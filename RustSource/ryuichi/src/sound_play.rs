use crate::unit::*;
use crate::Clip;
use crate::DecoderState;
use crate::Engine;
use crate::TrackTimeline;
pub use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
pub use cpal::{Sample, SampleFormat};
pub use rtrb::{Consumer, Producer, RingBuffer};
pub use std::{
    ffi::CStr,
    fs::File,
    path::Path,
    sync::{
        atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering},
        mpsc, Arc, Mutex,
    },
    thread::{self, JoinHandle},
};
pub use symphonia::core::{
    audio::{SampleBuffer, SignalSpec},
    codecs::DecoderOptions,
    formats::{FormatOptions, SeekMode, SeekTo},
    io::MediaSourceStream,
    meta::MetadataOptions,
    probe::Hint,
    units::Time,
};
pub use symphonia::default::{get_codecs, get_probe};

#[no_mangle]
pub extern "C" fn rust_sound_play(engine: *mut Engine) -> bool {
    if engine.is_null() {
        return false;
    }
    let eng = unsafe { &mut *engine };
    {
        let seek_lock = eng.seek_lock.clone();
        let _guard = seek_lock.lock().unwrap();
        // 0) 초기 정렬 + 기존 큐 비우기
        eng.pause_workers();
        eng.align_write_pos_to_transport();
        eng.flush_ringbuffers();
        eng.budget.reset();
    }

    // 1) 충분히 프리필 (최소 0.5s~1.0s 권장)
    // 48k 기준 24_576(≈0.512s) ~ 49_152(≈1.024s) 정도
    let _ = eng.prefill_rb1_blocking(PREFILL_ON_START);
    let target_rb2 = PREFILL_RB2_FRAMES.min(RB2_FRAMES.saturating_sub(2048));
    let _ = eng.prefill_rb2_blocking(target_rb2);

    // 2) 복제 스레드 시작(이제 1차에 데이터가 있음)
    if eng.copythread_worker.is_none() {
        eng.spawn_copy_thread();
    }

    std::thread::sleep(std::time::Duration::from_millis(80));

    // 스트림 보장
    if eng.sound_output.is_none() {
        match eng.start_output_from_ringbuffer() {
            Ok(stream) => {
                eng.sound_output = Some(stream);
            }
            Err(_) => return false,
        }
    } else if let Some(stream) = eng.sound_output.as_ref() {
        let _ = stream.play();
    }

    eng.flush_flag
        .store(false, std::sync::atomic::Ordering::Release);

    // 4) 트랜스포트 ON + 워커 깨우기
    eng.play_time_manager.start();
    eng.wake_workers();
    true
}

#[no_mangle]
pub extern "C" fn rust_sound_stop(engine: *mut Engine) -> bool {
    if engine.is_null() {
        return false;
    }
    let eng = unsafe { &mut *engine };
    {
        let seek_lock = eng.seek_lock.clone();
        let _guard = seek_lock.lock().unwrap();
        // 1) 재생 정지 + 워커 대기
        eng.play_time_manager.stop();
        eng.pause_workers();
    }

    // 2) CPAL 스트림을 엔진에서 떼어내서 drop (콜백이 들고 있던 2차 Consumer도 같이 drop됨)
    if let Some(stream) = eng.sound_output.take() {
        let _ = stream.pause();
    }

    // 3) 복제 스레드 종료 → 2차 링버퍼 재생성
    eng.stop_copy_thread();
    eng.budget.reset();

    true
}

#[no_mangle]
pub extern "C" fn rust_sound_seek(engine: *mut Engine, pos_frames: u64) -> bool {
    if engine.is_null() {
        return false;
    }
    let eng = unsafe { &mut *engine };

    {
        let seek_lock = eng.seek_lock.clone();
        let _guard = seek_lock.lock().unwrap();

        let was_playing = eng.play_time_manager.in_playing();
        if was_playing {
            eng.play_time_manager.stop();
        }

        eng.pause_workers();
        eng.play_time_manager.seek_frames(pos_frames);
        eng.align_write_pos_to_transport();
        eng.seek_epoch
            .fetch_add(1, std::sync::atomic::Ordering::Release);
        eng.flush_flag
            .store(true, std::sync::atomic::Ordering::Release);
        eng.flush_ringbuffers();
        eng.budget.reset();
    }
    let _ = eng.prefill_rb1_blocking(PREFILL_ON_SEEK);
    let target_rb2 = PREFILL_RB2_FRAMES.min(RB2_FRAMES.saturating_sub(2048));
    let _ = eng.prefill_rb2_blocking(target_rb2);
    eng.flush_flag
        .store(false, std::sync::atomic::Ordering::Release);
    eng.wake_workers();
    eng.play_time_manager.start();
    true
}

#[inline]
fn push_silence(prod: &mut Producer<f32>, frames: usize) -> usize {
    let mut wrote = 0usize;
    while wrote < frames {
        if prod.push(0.0).is_err() {
            break;
        } // L
        if prod.push(0.0).is_err() {
            break;
        } // R
        wrote += 1;
    }
    wrote
}
fn refill_packet(d: &mut DecoderState) -> Result<usize, String> {
    let pkt = match d.format.next_packet() {
        Ok(p) => p,
        Err(_) => return Ok(0), // <-- 에러도 0프레임(EOF 취급)
    };
    let decoded = match d.decoder.decode(&pkt) {
        Ok(x) => x,
        Err(_) => return Ok(0), // <-- 디코드 에러도 EOF 취급
    };
    let dec_spec = *decoded.spec();
    let cap = decoded.capacity();
    if d.sample_buf.capacity() < cap {
        d.sample_buf = SampleBuffer::<f32>::new(cap as u64, *decoded.spec());
    }
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
            //참조를 풀어버리고
            let l = samples[*si]; //엔진기준 시작해야할 위치에 원본 프레임을 가져온다
            let r = if *ch >= 2 { samples[*si + 1] } else { l }; //채널이 모노가 아니면 시작프레임에서 1칸더가서 진행 [L],[R]
                                                                 //모노일경우 l동일한 위치 가져온다
            *si += *ch; //다음프레임으로 이동
            d.src_pos_samples += 1; //재생위치도 한칸 이동
            return Ok(Some((l, r))); //해당 패킷을 전달한다
        }
        // 버퍼 고갈 → refill
        *ch = refill_packet(d)?; //다시 패키지 읽어온다
        *si = 0; //다시 초기화
        if d.sample_buf.samples().is_empty() {
            return Ok(None);
        } //완전히 다출력했다면
    }
}

pub fn seek_decoder_to_src_samples(dec: &mut DecoderState, src_off: u64) -> anyhow::Result<()> {
    // 시각(초)으로 변환
    let time = Time::from((src_off as f64) / (dec.src_sr as f64));

    // 1) 먼저 track_id만 뽑아서 immutable borrow를 즉시 drop
    let track_id = {
        let t = dec
            .format
            .default_track()
            .ok_or_else(|| anyhow::anyhow!("no default track"))?;
        t.id
    };

    // 2) 이제 mutable borrow로 seek 가능
    dec.format.seek(
        SeekMode::Accurate,
        SeekTo::Time {
            time,
            track_id: Some(track_id),
        },
    )?;

    // (선택) 디코더 내부상태 초기화: 없으면 무시돼요
    let _ = dec.decoder.reset();

    // 3) seek이 끝났으니 다시 immutable borrow로 채널 정보만 읽기
    let chans = dec
        .format
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

#[inline]
fn ensure_decoder_for(dec: &mut Option<DecoderState>, clip: &Clip) -> bool {
    match dec {
        None => match open_decoder_for(&clip.file_path) {
            Ok(d) => {
                *dec = Some(d);
                true
            }
            Err(_) => false,
        },
        Some(d0) => {
            if d0.file_path != clip.file_path || d0.src_sr != clip.src_sr {
                match open_decoder_for(&clip.file_path) {
                    Ok(nd) => {
                        *dec = Some(nd);
                        true
                    }
                    Err(_) => false,
                }
            } else {
                true
            }
        }
    }
}

pub fn fill_track_once(
    tr: &mut TrackTimeline,
    dec: &mut Option<DecoderState>,
    prod: &mut Producer<f32>,
    mut frames_need: usize,
    engine_sr: u32,
) -> Result<usize, String> {
    if frames_need == 0 || prod.is_full() {
        //할 일이 없음
        return Ok(0);
    }
    let mut pos = tr.write_pos_frames; //현재 쓰기 위치
    let mut produced_total = 0usize; //마지막에 사용량 저장을 위해

    while frames_need > 0 {
        // 1) 현재 pos에 활성 클립 찾기
        let active_clip = tr
            .clips
            .range(..=pos) //<= 0부터 pos 까지 키값을 가진 클립들중에서
            .next_back() // 그범위에 가장 뒤에있는거 (즉, pos에 가장 가까운 시작점)
            .and_then(|(_, c)| {
                //and_then 는 Option이 Some일때만 실행 _ 시작부분 , c 클립
                let end = c.tl_start.saturating_add(c.tl_len); //클립의 끝 위치 시작시작 + 길이 = 끝(saturating_add 오버플로우 방지)
                if pos < end {
                    Some(c)
                } else {
                    None
                } //pos가 클립의 끝보다 작으면(즉, 클립 구간 안에 있으면) Some(c) 반환 아니면 None
            });

        match active_clip {
            None => {
                // 2) 빈 구간 → 다음 클립 시작 전까지 무음
                let next_start = tr
                    .clips
                    .range((pos + 1)..)
                    .next() // 다음 클립 시작 위치찾기
                    .map(|(s, _)| *s) // 키값(시작 위치) 추출
                    .unwrap_or(u64::MAX); // 없으면 무한대
                let gap = if next_start == u64::MAX {
                    frames_need // 무한대면 다 채움
                } else {
                    ((next_start.saturating_sub(pos)) as usize) // 다음 클립 시작 전까지
                        .min(frames_need) // 필요한 프레임
                };

                let wrote = push_silence(prod, gap);
                if wrote == 0 {
                    break;
                } // 링버퍼 만땅
                produced_total += wrote;
                pos += wrote as u64; // 진행 시킴
                frames_need -= wrote; // 남은 필요량 감소
            }
            Some(clip) => {
                // 3) 클립 구간 → 필요한 만큼만 디코드 후 리샘플해서 push
                let clip_end = clip.tl_start.saturating_add(clip.tl_len);
                let can_write = ((clip_end.saturating_sub(pos)) as usize).min(frames_need);

                // (1) 디코더 열기/재열기
                if !ensure_decoder_for(dec, clip) {
                    // 디코더를 못 열면 'can_write' 만큼 무음으로 채우고 다음 루프로 (스핀 방지)
                    let wrote = push_silence(prod, can_write.min(frames_need));
                    produced_total += wrote;
                    pos += wrote as u64;
                    frames_need = frames_need.saturating_sub(wrote);
                    continue;
                }

                // 여기부터는 안전
                let d = dec.as_mut().unwrap();

                // (2) 타임라인 pos → 소스 좌표(src_sr)로 매핑
                let src_begin = (((pos.saturating_sub(clip.tl_start)) as f64) * (d.src_sr as f64)
                    / (engine_sr as f64))
                    .floor() as u64;

                // (3) 정확 시킹(필요 시)
                if d.src_pos_samples != src_begin {
                    if let Err(_) = seek_decoder_to_src_samples(d, src_begin) {
                        // 실패: 디코더 폐기 + 무음으로 메우고 다음 루프
                        *dec = None;
                        let wrote = push_silence(prod, can_write.min(frames_need));
                        produced_total += wrote;
                        pos += wrote as u64;
                        frames_need = frames_need.saturating_sub(wrote);
                        continue;
                    }
                }

                // (4) 디코드/리샘플
                // 위에서 정확 시킹을 했으므로, decode 쪽에서 추가 스킵 없게 src_begin=0 전달
                match decode_resample_into_ring(d, can_write, engine_sr, prod, 0) {
                    Ok(wrote) if wrote > 0 => {
                        produced_total += wrote;
                        pos += wrote as u64;
                        frames_need -= wrote;
                    }
                    _ => {
                        // EOF/에러/포화 → 무음으로 진행 유지
                        *dec = None;
                        let wrote = push_silence(prod, can_write.min(frames_need));
                        produced_total += wrote;
                        pos += wrote as u64;
                        frames_need = frames_need.saturating_sub(wrote);
                        continue;
                    }
                }
            }
        }

        if prod.is_full() {
            break;
        } //링버퍼 꽉 참
    }

    tr.write_pos_frames = pos; //트랙의 '공식' 쓰기 위치를 갱신
    Ok(produced_total) //종료
}

// -------------------------
// 디코더 열기
// -------------------------

fn open_decoder_for(path: &str) -> Result<DecoderState, String> {
    let file = File::open(Path::new(path)).map_err(|e| e.to_string())?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    //Symphonia가 읽을 수 있는 미디어 소스 래퍼 파일/메모리/커서 등 가진 집합체 / Default 는 옵션값들

    let probed = get_probe()
        .format(
            //파일을 Symphonia가 읽을 수 있는 포맷으로 변환
            &Hint::new(), //힌트를 줄수 있다(확장자/ MIME 등)
            mss,
            &FormatOptions::default(),   // 컨테이너 포맷 탐색 옵션
            &MetadataOptions::default(), // 메타데이터(부가적인 내용) 읽기 옵션
        )
        .map_err(|e| e.to_string())?; //이건 작업중 에러나면 return

    let format = probed.format;
    let track = format
        .tracks()
        .iter() //컨테이너 안의 트랙을 순환시킨다
        .find(|t| t.codec_params.channels.is_some()) //찾기 코덱 파라미터 안에 정보중 채널이 있는지 없는지 체크
        .ok_or_else(|| "no audio track".to_string())?; //있으면 오디오로 판단하여 진행
    let chans = track
        .codec_params
        .channels
        .ok_or_else(|| "no channels".to_string())?; //채널 정보
    let src_sr = track
        .codec_params
        .sample_rate
        .ok_or_else(|| "unknown sample rate".to_string())?; //샘플링 레이트 정보

    let decoder = get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .map_err(|e| e.to_string())?; //코덱 파라미터 바탕으로 디코더 객체 생성

    // 빈 SampleBuffer로 시작(첫 decode에서 스펙 맞춰 채움)
    let spec = SignalSpec {
        rate: src_sr,
        channels: chans,
    };
    let sample_buf = SampleBuffer::<f32>::new(0, spec);

    Ok(DecoderState {
        format,
        decoder,
        sample_buf,
        src_sr,
        src_pos_samples: 0,
        file_path: path.to_string(),
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
    let step = (d.src_sr as f32) / (engine_sr as f32); //디코더에서 덜읽어야할 sr 수치

    // 로컬 커서
    let mut ch: usize = refill_packet(d)?; // 첫 패킷 적재 & 채널수 확보
    if ch == 0 {
        return Ok(0);
    } // 채널이없으면 에러로
    let mut si: usize = 0;

    // A) src_begin 까지 프레임 스킵(정확 시킹 대체)
    while d.src_pos_samples < src_begin {
        //샘플에 현재위치를 트랙에 현재위치까지 이동시켜 맞춤
        let samples = d.sample_buf.samples(); //샘플 전체를 전달
        if si + ch <= samples.len() {
            si += ch;
            d.src_pos_samples += 1;
        } else {
            //예외처리
            ch = refill_packet(d)?;
            si = 0;
            if d.sample_buf.samples().is_empty() {
                return Ok(0);
            }
        }
    }

    // B) 선형보간 준비
    let mut s0 = match fetch_lr_once(d, &mut si, &mut ch)? {
        Some(fr) => fr,
        None => return Ok(0),
    }; //현재
    let mut s1 = match fetch_lr_once(d, &mut si, &mut ch)? {
        Some(fr) => fr,
        None => return Ok(0),
    }; //다음
    let mut frac = 0.0f32; //s0 , s1 에 정규화된 위치 0~1

    // C) 출력 프레임 생성
    while wrote < out_frames {
        //총프레임 만큼 동작
        let out_l = (s0.0 + (s1.0 - s0.0) * frac).clamp(-1.0, 1.0); // 선형보간 계산법 A + (B - A) *frac
        let out_r = (s0.1 + (s1.1 - s0.1) * frac).clamp(-1.0, 1.0);
        let out_l = if out_l.is_finite() { out_l } else { 0.0 };
        let out_r = if out_r.is_finite() { out_r } else { 0.0 };

        if prod.push(out_l).is_err() {
            break;
        } //링버퍼에 넣기 에러나면 종료
        if prod.push(out_r).is_err() {
            break;
        }
        wrote += 1;

        frac += step; //step 읽어 나가야할 값
        while frac >= 1.0 {
            // 1초 보다 크면
            frac -= 1.0; // 1초 보다 작게 만들고
            if let Some(fr) = fetch_lr_once(d, &mut si, &mut ch)? {
                //s1까지왔다면 s0을 s1로 새로 샘플가져와서 넣어주기
                s0 = s1;
                s1 = fr;
            } else {
                return Ok(wrote);
            }
        }
    }

    Ok(wrote)
}
