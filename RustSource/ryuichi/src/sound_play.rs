use crate::Engine;
pub use rtrb::{Consumer, Producer, RingBuffer};
pub use std::{fs::File, path::Path,ffi::CStr, sync::{mpsc, Arc, Mutex, atomic::{AtomicU32, AtomicU64,AtomicBool, Ordering}} , thread::{self,JoinHandle}};
pub use symphonia::core::{
    audio::{SampleBuffer, SignalSpec}, codecs::DecoderOptions, formats::{FormatOptions,SeekMode, SeekTo},
    io::MediaSourceStream, meta::MetadataOptions, probe::Hint, units::Time,
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
        Ok(stream) => { eng.sound_output = Some(stream); std::thread::sleep(std::time::Duration::from_millis(20)); } //성공이면 eng.sound_output에 스트림 저장
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
    let was_playing = eng.play_time_manager.in_playing();

     if was_playing {
        eng.play_time_manager.stop();
    }
    eng.pause_workers();
    if let Some(stream) = eng.sound_output.as_ref() {
        let _ = stream.pause();
    }
    eng.play_time_manager.seek_samples(pos_samples);
    eng.align_write_pos_to_transport();
    eng.flush_on_resume.store(true, std::sync::atomic::Ordering::Release);
    eng.seek_epoch.fetch_add(1, Ordering::Release); // ★ 복제 스레드에 “큐 비워!” 신호
    eng.rebuild_all_ringbuffers();
    const FILL_FRAMES: usize = 65536;
    if let Err(e)=eng.prefill_all_tracks(FILL_FRAMES) {
        eprintln!("[seek] prefill_all_tracks err:{e}");
    }
    if was_playing {
        eng.play_time_manager.start();
        if let Some(stream) = eng.sound_output.as_ref() { let _ = stream.play(); }
        else if let Ok(stream) = eng.start_output_from_ringbuffer() { eng.sound_output = Some(stream); }
        eng.wake_workers();
    } else {
        eng.pause_workers();
    }
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
    let pkt      = match d.format.next_packet() { Ok(p) => p, Err(_) => return Ok(0) }; //컨테이너에서 패킷을 가져온다
    let decoded  = d.decoder.decode(&pkt).map_err(|e| e.to_string())?; //패킷을 디코딩시킨다 PCM으로 진행하다  
    let dec_spec = *decoded.spec(); //디코더에서 스팩을끄낸다.
    let cap = decoded.capacity(); // usize //총프레임량을 가져온다
    if d.sample_buf.capacity() < cap {  //비교
    d.sample_buf = SampleBuffer::<f32>::new(cap as u64, *decoded.spec()); //외곡방지
    }
    d.sample_buf.copy_interleaved_ref(decoded);//원본데이터와 동일하게 형식을 맞춘다

    Ok(dec_spec.channels.count()) //스팩에서 채널을 추출한다
}

fn fetch_lr_once(
    d: &mut DecoderState,
    si: &mut usize,
    ch: &mut usize,
) -> Result<Option<(f32, f32)>, String> {
    loop {
        let samples = d.sample_buf.samples(); // 짧게 빌림
        if *si + *ch <= samples.len() {      //참조를 풀어버리고 
            let l = samples[*si];       //엔진기준 시작해야할 위치에 원본 프레임을 가져온다 
            let r = if *ch >= 2 { samples[*si + 1] } else { l }; //채널이 모노가 아니면 시작프레임에서 1칸더가서 진행 [L],[R]
                                                                      //모노일경우 l동일한 위치 가져온다
            *si += *ch;   //다음프레임으로 이동 
            d.src_pos_samples += 1; //재생위치도 한칸 이동
            return Ok(Some((l, r))); //해당 패킷을 전달한다
        } 
        // 버퍼 고갈 → refill
        *ch = refill_packet(d)?; //다시 패키지 읽어온다
        *si = 0;                 //다시 초기화
        if d.sample_buf.samples().is_empty() { return Ok(None); } //완전히 다출력했다면
    }
}

pub fn seek_decoder_to_src_samples(dec: &mut DecoderState, src_off: u64) -> anyhow::Result<()> {
   // 시각(초)으로 변환
    let time = Time::from((src_off as f64) / (dec.src_sr as f64));

    // 1) 먼저 track_id만 뽑아서 immutable borrow를 즉시 drop
    let track_id = {
        let t = dec.format
            .default_track()
            .ok_or_else(|| anyhow::anyhow!("no default track"))?;
        t.id
    };

    // 2) 이제 mutable borrow로 seek 가능
    dec.format.seek(
        SeekMode::Accurate,
        SeekTo::Time { time, track_id: Some(track_id) }
    )?;

    // (선택) 디코더 내부상태 초기화: 없으면 무시돼요
    let _ = dec.decoder.reset();

    // 3) seek이 끝났으니 다시 immutable borrow로 채널 정보만 읽기
    let chans = dec.format
        .default_track()
        .and_then(|t| t.codec_params.channels)
        .ok_or_else(|| anyhow::anyhow!("unknown channel layout"))?;

    // 4) 샘플버퍼 클리어(스펙 유지)
    let spec = SignalSpec::new(dec.src_sr, chans);
    dec.sample_buf = SampleBuffer::<f32>::new(0, spec);

    // 5) 디코더의 진행 샘플 카운터를 목표 위치로 맞춤
    dec.src_pos_samples = src_off;

    Ok(())
}

pub fn fill_track_once(
    tr: &mut TrackTimeline,
    dec: &mut Option<DecoderState>,
    prod: &mut Producer<f32>,
    mut frames_need: usize,
    engine_sr: u32,
) -> Result<(), String> {
    if frames_need == 0 || prod.is_full() { //할 일이 없음
        return Ok(());
    }
    let mut pos = tr.write_pos_samples; //현재 쓰기 위치

    while frames_need > 0 {
        // 1) 현재 pos에 활성 클립 찾기
        let active_clip = tr.clips.range(..=pos)//<= 0부터 pos 까지 키값을 가진 클립들중에서
            .next_back()                                      // 그범위에 가장 뒤에있는거 (즉, pos에 가장 가까운 시작점)
            .and_then(|(_, c)| {                                            //and_then 는 Option이 Some일때만 실행 _ 시작부분 , c 클립
                let end = c.tl_start.saturating_add(c.tl_len);               //클립의 끝 위치 시작시작 + 길이 = 끝(saturating_add 오버플로우 방지)
                if pos < end { Some(c) } else { None }                           //pos가 클립의 끝보다 작으면(즉, 클립 구간 안에 있으면) Some(c) 반환 아니면 None
            });

        match active_clip {
            None => {
                // 2) 빈 구간 → 다음 클립 시작 전까지 무음
                let next_start = tr.clips.range((pos + 1)..).next() // 다음 클립 시작 위치찾기
                .map(|(s, _)| *s) // 키값(시작 위치) 추출
                .unwrap_or(u64::MAX);     // 없으면 무한대
                let gap = if next_start == u64::MAX {
                    frames_need                // 무한대면 다 채움
                } else {
                    ((next_start.saturating_sub(pos)) as usize) // 다음 클립 시작 전까지
                    .min(frames_need) // 필요한 프레임
                };

                let wrote = push_silence(prod, gap);
                if wrote == 0 { break; } // 링버퍼 만땅
                pos += wrote as u64;     // 진행 시킴
                frames_need -= wrote;    // 남은 필요량 감소
            }
            Some(clip) => {
                // 3) 클립 구간 → 필요한 만큼만 디코드 후 리샘플해서 push
                let clip_end  = clip.tl_start.saturating_add(clip.tl_len); //클립의 끝 위치
                let can_write = ((clip_end.saturating_sub(pos)) as usize).min(frames_need); //클립 끝까지 또는 필요한 프레임

                // 디코더 준비(없으면 오픈 / SR 불일치면 재오픈)
                if dec.is_none() {
                    *dec = Some(open_decoder_for(&clip.file_path)?);
                } else if let Some(d) = dec.as_ref() {  //디코더가 있고
                    if d.fille_path != clip.file_path || d.src_sr != clip.src_sr { //샘플링 레이트가 다르면
                        *dec = Some(open_decoder_for(&clip.file_path)?); //재디코딩준비
                    }
                }
                let d = dec.as_mut().expect("decoder must exist here"); //디코더가 반드시 있어야함 
                                                                                                // &dec.as_ref()로 가져오면 &Option(DecoderState) 이고
                                                                                                //as_mut()로 가져오면 &mut Option(DecoderState) 가져와서 내부를 수정가능

                // 타임라인 pos → 소스 좌표(src_sr)로 매핑 (정확시킹 대신 프레임 스킵)
                let src_begin = (((pos.saturating_sub(clip.tl_start)) as f64) * (d.src_sr as f64) / (engine_sr as f64)).floor() as u64;  //원본 파일의 맨 앞에서부터 몇 프레임을 버리고 시작할지
                //pos - clip.tl_start  클립기준에 현재위치 계산
                // 파일에서 추출한 1초 src d.src_sr  / 내가 설정한 1초당 src  engine_sr   = src 단위 계산
                
                // 파일/샘플레이트 바뀌면 무조건 재오픈
                let need_reopen = d.fille_path != clip.file_path || d.src_sr != clip.src_sr;
                if need_reopen {
                *dec = Some(open_decoder_for(&clip.file_path)?);
                }
                let d = dec.as_mut().expect("decoder must exist here");

                // (중요) 위치 차이 확인 → 차이가 크거나, 방향이 뒤든 앞이든 '정확 시킹' 실행
                let cur = d.src_pos_samples;
                let delta = if cur > src_begin { cur - src_begin } else { src_begin - cur };
                // 임계값은 상황에 맞게. 즉시 점프 원하면 그냥 `delta > 0`로 전부 시킹해도 됨.
                if delta > 0 {
                seek_decoder_to_src_samples(d, src_begin).map_err(|e| e.to_string())?;
                }

                let wrote = decode_resample_into_ring(d, can_write, engine_sr, prod, src_begin)?;
                if wrote == 0 { //클립이 없다면
                    // EOF나 링버퍼 포화 시 남은 구간은 무음으로 메워서 시간축만 진행
                    let fallback = push_silence(prod, can_write.min(frames_need)); //min 두갑중 더 작은걸 골르는것 
                    pos += fallback as u64; //무음 프레임 만큼 더하여 트랙 읽기 진행 업데이트
                    frames_need = frames_need.saturating_sub(fallback); // 무음으로 채운 만큼 남은 작업량(프레임)을 '포화 감산'으로 줄임.
                    break; // 종료
                }

                pos += wrote as u64;    // 실제로 오디오를 써 넣은 양(wrote 프레임)만큼 타임라인 쓰기 포인터를 전진.
                frames_need -= wrote;  // 남은 작업량(프레임)에서 방금 쓴 양을 차감. (여긴 일반 감산: 언더플로 안 나게 로직상 보장)
            }
        }

        if prod.is_full() { break; } //링버퍼 꽉 참
    }

    tr.write_pos_samples = pos; //트랙의 '공식' 쓰기 위치를 갱신
    Ok(()) //종료
}

// -------------------------
// 디코더 열기
// -------------------------

fn open_decoder_for(path: &str) -> Result<DecoderState, String> {
    let file = File::open(Path::new(path)).map_err(|e| e.to_string())?;
    let mss  = MediaSourceStream::new(Box::new(file), Default::default()); 
    //Symphonia가 읽을 수 있는 미디어 소스 래퍼 파일/메모리/커서 등 가진 집합체 / Default 는 옵션값들  

    let probed = get_probe().format( //파일을 Symphonia가 읽을 수 있는 포맷으로 변환
        &Hint::new(), //힌트를 줄수 있다(확장자/ MIME 등)
        mss, 
        &FormatOptions::default(), // 컨테이너 포맷 탐색 옵션
        &MetadataOptions::default(), // 메타데이터(부가적인 내용) 읽기 옵션
    ).map_err(|e| e.to_string())?; //이건 작업중 에러나면 return

    let mut format = probed.format; 
    let track = format.tracks().iter() //컨테이너 안의 트랙을 순환시킨다
        .find(|t| t.codec_params.channels.is_some()) //찾기 코덱 파라미터 안에 정보중 채널이 있는지 없는지 체크
        .ok_or_else(|| "no audio track".to_string())?;                  //있으면 오디오로 판단하여 진행
    let chans  = track.codec_params.channels.ok_or_else(|| "no channels".to_string())?; //채널 정보
    let src_sr = track.codec_params.sample_rate.ok_or_else(|| "unknown sample rate".to_string())?; //샘플링 레이트 정보

    let decoder = get_codecs().make(&track.codec_params, &DecoderOptions::default()) 
        .map_err(|e| e.to_string())?; //코덱 파라미터 바탕으로 디코더 객체 생성

    // 빈 SampleBuffer로 시작(첫 decode에서 스펙 맞춰 채움)
    let spec = SignalSpec { rate: src_sr, channels: chans };
    let sample_buf = SampleBuffer::<f32>::new(0, spec);

    Ok(DecoderState {
        format,
        decoder,
        sample_buf,
        src_sr,
        src_pos_samples: 0,
        fille_path: path.to_string(),
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
    let step = (d.src_sr as f32) / (engine_sr as f32);  //디코더에서 덜읽어야할 sr 수치

    // 로컬 커서
    let mut ch: usize = refill_packet(d)?; // 첫 패킷 적재 & 채널수 확보
    if ch == 0 { return Ok(0); } // 채널이없으면 에러로 
    let mut si: usize = 0;

    // A) src_begin 까지 프레임 스킵(정확 시킹 대체)
    while d.src_pos_samples < src_begin {    //샘플에 현재위치를 트랙에 현재위치까지 이동시켜 맞춤
        let samples = d.sample_buf.samples(); //샘플 전체를 전달
        if si + ch <= samples.len() { 
            si += ch;
            d.src_pos_samples += 1;
        } else { //예외처리
            ch = refill_packet(d)?;
            si = 0;
            if d.sample_buf.samples().is_empty() { return Ok(0); }
        }
    }

    // B) 선형보간 준비
    let mut s0 = match fetch_lr_once(d, &mut si, &mut ch)? { Some(fr) => fr, None => return Ok(0) }; //현재
    let mut s1 = match fetch_lr_once(d, &mut si, &mut ch)? { Some(fr) => fr, None => return Ok(0) }; //다음
    let mut frac = 0.0f32; //s0 , s1 에 정규화된 위치 0~1

    // C) 출력 프레임 생성
    while wrote < out_frames { //총프레임 만큼 동작 
        let out_l = s0.0 + (s1.0 - s0.0) * frac; // 선형보간 계산법 A + (B - A) *frac 
        let out_r = s0.1 + (s1.1 - s0.1) * frac;

        if prod.push(out_l).is_err() { break; }      //링버퍼에 넣기 에러나면 종료
        if prod.push(out_r).is_err() { break; }
        wrote += 1;

        frac += step;                                //step 읽어 나가야할 값 
        while frac >= 1.0 {                         // 1초 보다 크면
            frac -= 1.0;                            // 1초 보다 작게 만들고
            if let Some(fr) = fetch_lr_once(d, &mut si, &mut ch)? {  //s1까지왔다면 s0을 s1로 새로 샘플가져와서 넣어주기
                s0 = s1;
                s1 = fr; 
            } else {
                return Ok(wrote);
            }
        }
    }

    Ok(wrote)
}
