use crate::Engine;
pub use rtrb::{Consumer, Producer, RingBuffer};
pub use std::{fs::File, path::Path,ffi::CStr, sync::{mpsc, Arc, Mutex, atomic::{AtomicBool, Ordering}} , thread::{self,JoinHandle}};
pub use symphonia::core::{
    audio::SampleBuffer, codecs::DecoderOptions, formats::FormatOptions,
    io::MediaSourceStream, meta::MetadataOptions, probe::Hint,
};
pub use symphonia::default::{get_codecs, get_probe};


#[no_mangle]
pub extern "C" fn rust_sound_play(engine : *mut Engine) -> bool {
    if engine.is_null(){
        return false;
    }
    let _eng = unsafe { &mut *engine};
    
    true
}

#[no_mangle]
pub extern "C" fn rust_sound_stop(_eng : *mut Engine) -> bool {
true
}


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
                let ch = audio_buf.spec().channels.count(); //추출한 오디오 패킷에서 채널 종류에 따라 숫자 출력
                buf.copy_interleaved_ref(audio_buf); //audio_buf 내용을 복사해가면서 소유권도 Move 안에 데이터도 변환 작업
                let interleaved = buf.samples(); //samples 배열인데 이안에 PCM 데이터가 있고 [L0,R0,L1,R1] 이런느낌

                let tmp;
                let stereo: &[f32] = match ch{
                    2 => interleaved,
                    1 => {
                        tmp = interleaved.iter().flat_map(|&s| [s,s]).collect::<Vec<_>>();
                        &tmp
                    }
                    _ => {
                        tmp = interleaved
                            .chunks(ch)
                            .flat_map(|frm|{
                                let m =frm.iter().copied().sum::<f32>() / (ch as f32);
                                [m,m]
                            })
                            .collect::<Vec<_>>();
                        &tmp 
                    }
                };
                let mut prod = prod_mx.lock().map_err(|_|"producer lock poisoned".to_string())?;
                for &s in stereo{
                    while let Err(_full) = prod.push(s) {
                        if stop.load(Ordering::Relaxed) { break;}
                        std::thread::yield_now();
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