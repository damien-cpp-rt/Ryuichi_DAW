use crate::Engine;
pub use rtrb::{Consumer, Producer, RingBuffer};
pub use std::{fs::File, path::Path,ffi::CStr, sync::{mpsc, Arc, Mutex, atomic::{AtomicU32, AtomicU64,AtomicBool, Ordering}} , thread::{self,JoinHandle}};
pub use symphonia::core::{
    audio::{SampleBuffer, SignalSpec}, codecs::DecoderOptions, formats::FormatOptions,
    io::MediaSourceStream, meta::MetadataOptions, probe::Hint,
};
pub use symphonia::default::{get_codecs, get_probe};
pub use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
pub use cpal::{Sample, SampleFormat};
use crate::TrackTimeline;
use crate::DecoderState;



#[no_mangle]
pub extern "C" fn rust_sound_play(engine : *mut Engine) -> bool {
    if engine.is_null(){
        return false;
    }
    let eng = unsafe { &mut *engine};

    eng.thread_stop.store(false,Ordering::Relaxed); //정지 플래그 해제
    eng.align_write_pos_to_transport();  //트랜스포트 위치에 맞게 각 트랙의 쓰기 위치를 맞춤
    eng.flush_ringbuffers(); //링버퍼 비우기

    eng.play_time_manager.start(); //재생 시작
    eng.wake_workers();

     // ★ 이미 존재하면 play()만 눌러 재개
    if let Some(stream) = eng.sound_output.as_ref() {
        if let Err(_) = stream.play() { return false; }
        return true;
    }

    // ★ 없으면 새로 만들고(내부에서 play() 호출됨) 보관
    match eng.start_output_from_ringbuffer() {
        Ok(stream) => { eng.sound_output = Some(stream); }
        Err(_) => { return false; }
    }
    true
}

#[no_mangle]
pub extern "C" fn rust_sound_stop(engine : *mut Engine) -> bool {
      if engine.is_null(){
        return false;
    }
    let eng = unsafe { &mut *engine};
    eng.play_time_manager.stop(); //재생 정지
    eng.pause_workers(); //작업자 스레드 대기
    if let Some(stream) = eng.sound_output.as_ref() 
    { let _ = stream.pause(); } //출력 정지
    true
}

#[no_mangle]
pub extern "C" fn rust_sound_seek(engine : *mut Engine, pos_samples:u64) -> bool {
    if engine.is_null(){
        return false;
    }
    let eng = unsafe { &mut *engine};
    eng.play_time_manager.seek_samples(pos_samples);
    eng.flush_ringbuffers();
    eng.align_write_pos_to_transport();
    for d_mx in eng.decod.iter() {
        if let Ok(mut d) = d_mx.lock() { *d = None; }
    }
    eng.wake_workers();
    true
}

#[inline]
fn push_silence(prod: &mut Producer<f32>, frames: usize) -> usize {
    let mut wrote = 0usize;
    while wrote < frames {
        if prod.push(0.0).is_err() { break; } // L
        if prod.push(0.0).is_err() { break; } // R
        wrote += 1;
    }
    wrote
}
fn refill_packet(d: &mut DecoderState) -> Result<usize, String> {
    let pkt      = match d.format.next_packet() { Ok(p) => p, Err(_) => return Ok(0) };
    let decoded  = d.decoder.decode(&pkt).map_err(|e| e.to_string())?;
    let dec_spec = *decoded.spec();
    let cap      = decoded.capacity() as u64;

    d.sample_buf = SampleBuffer::<f32>::new(cap, dec_spec);
    d.sample_buf.copy_interleaved_ref(decoded);

    Ok(dec_spec.channels.count())
}

fn fetch_lr_once(
    d: &mut DecoderState,
    si: &mut usize,
    ch: &mut usize,
) -> Result<Option<(f32, f32)>, String> {
    loop {
        let samples = d.sample_buf.samples(); // 짧게 빌림
        if *si + *ch <= samples.len() {
            let l = samples[*si];
            let r = if *ch >= 2 { samples[*si + 1] } else { l };
            *si += *ch;
            d.src_pos_samples += 1;
            return Ok(Some((l, r)));
        }
        // 버퍼 고갈 → refill
        *ch = refill_packet(d)?;
        *si = 0;
        if d.sample_buf.samples().is_empty() { return Ok(None); }
    }
}

pub fn fill_track_once(
    tr: &mut TrackTimeline,
    dec: &mut Option<DecoderState>,
    prod: &mut Producer<f32>,
    mut frames_need: usize,
    engine_sr: u32,
) -> Result<(), String> {
    if frames_need == 0 || prod.is_full() {
        return Ok(());
    }
    let mut pos = tr.write_pos_samples;

    while frames_need > 0 {
        // 1) 현재 pos에 활성 클립 찾기
        let active_clip = tr.clips.range(..=pos).next_back()
            .and_then(|(_, c)| {
                let end = c.tl_start.saturating_add(c.tl_len);
                if pos < end { Some(c) } else { None }
            });

        match active_clip {
            None => {
                // 2) 빈 구간 → 다음 클립 시작 전까지 무음
                let next_start = tr.clips.range((pos + 1)..).next().map(|(s, _)| *s).unwrap_or(u64::MAX);
                let gap = if next_start == u64::MAX {
                    frames_need
                } else {
                    ((next_start.saturating_sub(pos)) as usize).min(frames_need)
                };

                let wrote = push_silence(prod, gap);
                if wrote == 0 { break; } // 링버퍼 만땅
                pos += wrote as u64;
                frames_need -= wrote;
            }
            Some(clip) => {
                // 3) 클립 구간 → 필요한 만큼만 디코드 후 리샘플해서 push
                let clip_end  = clip.tl_start.saturating_add(clip.tl_len);
                let can_write = ((clip_end.saturating_sub(pos)) as usize).min(frames_need);

                // 디코더 준비(없으면 오픈 / SR 불일치면 재오픈)
                if dec.is_none() {
                    *dec = Some(open_decoder_for(&clip.file_path)?);
                }
                if let Some(d) = dec.as_ref() {
                    if d.src_sr != clip.src_sr {
                        *dec = Some(open_decoder_for(&clip.file_path)?);
                    }
                }
                let d = dec.as_mut().expect("decoder must exist here");

                // 타임라인 pos → 소스 좌표(src_sr)로 매핑 (정확시킹 대신 프레임 스킵)
                let src_begin = (((pos.saturating_sub(clip.tl_start)) as f64)
                    * (d.src_sr as f64) / (engine_sr as f64))
                    .floor() as u64;

                let wrote = decode_resample_into_ring(d, can_write, engine_sr, prod, src_begin)?;
                if wrote == 0 {
                    // EOF나 링버퍼 포화 시 남은 구간은 무음으로 메워서 시간축만 진행
                    let fallback = push_silence(prod, can_write.min(frames_need));
                    pos += fallback as u64;
                    frames_need = frames_need.saturating_sub(fallback);
                    break;
                }

                pos += wrote as u64;
                frames_need -= wrote;
            }
        }

        if prod.is_full() { break; }
    }

    tr.write_pos_samples = pos;
    Ok(())
}

// -------------------------
// 디코더 열기
// -------------------------

fn open_decoder_for(path: &str) -> Result<DecoderState, String> {
    let file = File::open(Path::new(path)).map_err(|e| e.to_string())?;
    let mss  = MediaSourceStream::new(Box::new(file), Default::default());
    let probed = get_probe().format(
        &Hint::new(),
        mss,
        &FormatOptions::default(),
        &MetadataOptions::default(),
    ).map_err(|e| e.to_string())?;

    let mut format = probed.format;
    let track = format.tracks().iter()
        .find(|t| t.codec_params.channels.is_some())
        .ok_or_else(|| "no audio track".to_string())?;
    let chans  = track.codec_params.channels.ok_or_else(|| "no channels".to_string())?;
    let src_sr = track.codec_params.sample_rate.ok_or_else(|| "unknown sample rate".to_string())?;

    let decoder = get_codecs().make(&track.codec_params, &DecoderOptions::default())
        .map_err(|e| e.to_string())?;

    // 빈 SampleBuffer로 시작(첫 decode에서 스펙 맞춰 채움)
    let spec = SignalSpec { rate: src_sr, channels: chans };
    let sample_buf = SampleBuffer::<f32>::new(0, spec);

    Ok(DecoderState {
        format,
        decoder,
        sample_buf,
        src_sr,
        src_pos_samples: 0,
    })
}

// -------------------------
// 디코드→리샘플→링버퍼 push (정확시킹 대신 프레임 스킵)
// -------------------------

fn decode_resample_into_ring(
    d: &mut DecoderState,
    out_frames: usize,
    engine_sr: u32,
    prod: &mut Producer<f32>,
    src_begin: u64,
) -> Result<usize, String> {
    let mut wrote = 0usize;
    let step = (d.src_sr as f32) / (engine_sr as f32);

    // 로컬 커서
    let mut ch: usize = refill_packet(d)?; // 첫 패킷 적재 & 채널수 확보
    if ch == 0 { return Ok(0); }
    let mut si: usize = 0;

    // A) src_begin 까지 프레임 스킵(정확 시킹 대체)
    while d.src_pos_samples < src_begin {
        let samples = d.sample_buf.samples();
        if si + ch <= samples.len() {
            si += ch;
            d.src_pos_samples += 1;
        } else {
            ch = refill_packet(d)?;
            si = 0;
            if d.sample_buf.samples().is_empty() { return Ok(0); }
        }
    }

    // B) 선형보간 준비
    let mut s0 = match fetch_lr_once(d, &mut si, &mut ch)? { Some(fr) => fr, None => return Ok(0) };
    let mut s1 = match fetch_lr_once(d, &mut si, &mut ch)? { Some(fr) => fr, None => return Ok(0) };
    let mut frac = 0.0f32;

    // C) 출력 프레임 생성
    while wrote < out_frames {
        let out_l = s0.0 + (s1.0 - s0.0) * frac;
        let out_r = s0.1 + (s1.1 - s0.1) * frac;

        if prod.push(out_l).is_err() { break; }
        if prod.push(out_r).is_err() { break; }
        wrote += 1;

        frac += step;
        while frac >= 1.0 {
            frac -= 1.0;
            if let Some(fr) = fetch_lr_once(d, &mut si, &mut ch)? {
                s0 = s1; s1 = fr;
            } else {
                return Ok(wrote);
            }
        }
    }

    Ok(wrote)
}

#[allow(dead_code)]
pub fn decode_and_push_into_track_ringbuffer(track : usize, path :&str ,producers: &Arc<Vec<Mutex<Producer<f32>>>>,stop: &Arc<AtomicBool>) -> Result<usize,String> {
    let prod_mx = producers.get(track).ok_or_else(|| "invalid track index".to_string())?;
    //전달 받은 일거리에 트랙에 해당되는 링버퍼 생성자를 가져온다 ok_or_else 는 get에서 나오는데이터가 Option 이여서 그걸 성공하면 Result로 변환
    let file = File::open(Path::new(path)).map_err(|e| e.to_string())?;
    //file 불러온다 경로를 넣고 가져오는데 성공하면 무시 실패하면 경로에대한 주소를 에러로 return
    let mss = MediaSourceStream::new(Box::new(file),Default::default());
    //Symphonia가 읽을 수 있는 미디어 소스 래퍼 파일/메모리/커서 등 가진 집합체 / Default 는 옵션값들
    let probed = get_probe() //컨테이너 포맷 탐지기 "파일 껍데기"를 알아내는거 확장자같은
                .format(&Hint::new(),mss,&FormatOptions::default(),&MetadataOptions::default())
                //스트림(mss)에서 컨테이너를 판별하고 포맷 리더를 만든다. 힌트를 줄수 있다(확장자/ MIME 등), 탐색포맷, 포맷리더(오디오파일에 세부적인 내용) 동작 옵션 , 메타데이터(부가적인 내용) 읽기 옵션
                .map_err(|e|e.to_string())?; //이건 작업중 에러나면 return
    let mut format = probed.format; //만들어진 포맷을 format에 전달한다
    let (track_id, codec_params) = { //포맷리더가 가진 트랙들 중 기본 오디오 트랙 가져와
                                    let t = format.tracks()
                                    .iter() //순환시키고
                                    .find(|t| t.codec_params.channels.is_some()) //코덱 파라미터 안에 정보들중 channels를 통해 채널이있는지 없는지 체크해
                                                                                 //채널이있으면 오디오로 판단하여 진행한다
                                    .ok_or_else(|| "no audio track".to_string())?;
                                    (t.id, t.codec_params.clone())               //찾은 오디오에 id 와 코덱 파라미터를 전달한다
    };
    let mut decoder = get_codecs()//코덱 파라미터 바탕으로 디코더 객체 생성
                    .make(&codec_params, &DecoderOptions::default()) //만들어 내가 가져온 코덱 파라미터 바탕으로 ,기본값으로
                    .map_err(|e| e.to_string())?;
    let mut sample_buf:Option<SampleBuffer<f32>> = None; //디코더가 낸 오디오 버퍼를 연속적으로 인터리브드 형태로 복사하기 위한 작업 재사용 목적
    let mut pushed = 0usize; //링버퍼에 밀어넣은 샘플 카운트

    loop {
        if stop.load(Ordering::Relaxed) {break;} //정지될때까지 반복

        let packet = match format.next_packet() { //포맷에서 트랙(패킷)을 가져온다
            Ok(p) => p,
            Err(_) => break,
        };
        if packet.track_id() != track_id  { //사전에 추출한 오디오 트랙 id 와 비교한다
            continue;
        }

        match decoder.decode(&packet) {  //사전에 추출한 디코더 객체를 활용해 디코더를 통해 포맷에서 뽑은 패킷에 정보 추출
            Ok(audio_buf) => { //성공하면
                if sample_buf.is_none() { //안에 데이터가 비어있는지 체크 최초 1회 구조 설정을 위해
                    let spec = *audio_buf.spec(); //spec 말그대로 성능 추출한 패킷에 성능으로 생성
                    let cap = audio_buf.capacity() as u64; //그 오디오 패킷에 프레임 
                    sample_buf = Some(SampleBuffer::<f32>::new(cap,spec)); //구조를 셋팅
                }
                let buf = sample_buf.as_mut().unwrap(); //unwrap이 Option으로 반환해서 안에 데이터를 수정할려면
                                                        // 그냥 참조 시키면 &Option(SampleBuffer<f32>) 이런형식이라
                                                        // .as_mut()를 사용해서 Option(&mut SampleBuffer<f32>)가져온다
                let ch = audio_buf.spec().channels.count(); //추출한 오디오 패킷에서 채널 종류에 따라 숫자 출력 - 모노 : 1 스트레오 : 2
                buf.copy_interleaved_ref(audio_buf); //audio_buf 내용을 복사해가면서 소유권도 Move 안에 데이터도 변환 작업
                let interleaved = buf.samples(); //samples 배열인데 이안에 PCM 데이터가 있고 [L0,R0,L1,R1] 이런느낌

                let tmp;
                let stereo: &[f32] = match ch{ //채널을 매칭시켜서 1이면 모노로 2며 stereo 에 복사 
                    2 => interleaved,
                    1 => {
                        tmp = interleaved.iter().flat_map(|&s| [s,s]).collect::<Vec<_>>(); // 순환돌면서 기존 데이터를 2배로 늘린다. [s,s] 수정하는건가?
                        &tmp //그리 담은걸 반환?
                    }
                    _ => { //그외 체널
                        tmp = interleaved
                            .chunks(ch)  //채널안에 데이터를 가져오는데 채널 개수에 맞가 젤라서 가저온다 (2) [L0,R0] ,[L1,R1] 한프레임식 잘라서 배열로 저장
                            .flat_map(|frm|{ 
                                let m =frm.iter().copied().sum::<f32>() / (ch as f32); //map 자체가 &참조로 가져와서 copied 값자체를끄내 사용 [L0,R0] 이런거를 서로 더해서 /m 으로 만들고 채널 개수로 나눔 
                                [m,m] //다시 배열에 [m,m] 넣은다
                            })
                            .collect::<Vec<_>>();
                        &tmp 
                    }
                };
                let mut prod = prod_mx.lock().map_err(|_|"producer lock poisoned".to_string())?; 
                //lock 으로 받아오는것은 스마트포인터를 통해 받아온다 트랙에 링버퍼 생성자에 데이터를 넣을수 있게 mut통해 가져오고 lock으로 키얻으면 접근
                for &s in stereo{ //데이터 넣기위해 순환 
                    while let Err(_full) = prod.push(s) { // prod.push(s) 배열에 링버퍼에 넣는데 실패하면 반복문 시작
                        if stop.load(Ordering::Relaxed) { break;} //stop이 걸렸는지
                        std::thread::yield_now();  //스레드 대기 상태로 전환 다른 다른 스레드 작업을 우선으로 선정해줌
                    }
                    pushed += 1;
                    if stop.load(Ordering::Relaxed) {break;}
                }
            }
            Err(symphonia::core::errors::Error::DecodeError(_)) => continue,
            Err(e) => return Err(e.to_string()),
        }
    }
    Ok(pushed)
}