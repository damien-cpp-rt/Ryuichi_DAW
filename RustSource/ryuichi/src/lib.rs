
mod waveform_generation_module;
pub use waveform_generation_module::*;
mod sound_track_update;
pub use sound_track_update::*;
mod sound_thread_job;
pub use sound_thread_job::*;

use std::time::Duration;
use std::sync::mpsc::RecvTimeoutError;

const CAPACITY_SAMPLES : usize = 48_000;
const CHANNELS: usize = 2;

enum TrackNumber {
    Zero,
    One,
    Two,
    Three,
}
enum Job {
    DecodeFile{track: usize, path: String},
}
pub struct CircularBuffer {
    producer : Option<Producer<f32>>, //디코더/프로듀서가 push할 핸들
    consumer : Option<Consumer<f32>>, //소비(믹서/출력)가 pop할 핸들
}
pub struct TrackDatas {
    track_number : TrackNumber,
    file_path : Vec<String>,
    volume : f32,
    muted : bool,
    pan : f32,
    sound_balance : f32,
    reverb : bool,
    delay : bool,
    circularbuffer : CircularBuffer,
}

impl TrackDatas {
    fn new(number : i32) -> Result<Self,String> {
      let track_num = match number {
        0 => TrackNumber::Zero,
        1 => TrackNumber::One,
        2 => TrackNumber::Two,
        3 => TrackNumber::Three,
        _ => return Err("not a valid track number".to_string()),
      };
      let (tx,rx) = RingBuffer::<f32>::new(CAPACITY_SAMPLES);
      //f32 타입에 배열을 생성 [CAPACITY_SAMPLES] 만큼
      let circularbuffer = CircularBuffer {
        producer : Some(tx),
        consumer : Some(rx),
      };
      Ok (Self {
        track_number : track_num,
        file_path : Vec::new(),
        volume : 0.5,
        muted : false,
        pan : 0.0,
        sound_balance : 0.0,
        reverb : false,
        delay : false,
        circularbuffer : circularbuffer,
      })
    }
}

pub struct Engine {
    track : Vec<TrackDatas>, //pull 해올것
    producers: Arc<Vec<Mutex<Producer<f32>>>>,   // 워커가 push 할 대상
    consumers: Arc<Vec<Mutex<Consumer<f32>>>>,   // cpal 이 사용할 핸들
    worker: Vec<JoinHandle<()>>, //스레드 워커 노동자
    queue: Option<mpsc::Sender<Job>>, //입구
    job_rx_shared: Arc<Mutex<mpsc::Receiver<Job>>>, //quee 공유를 위한거임
    stop: Arc<AtomicBool>,    //정지 확인
    output: Option<cpal::Stream>,               //출력 장치
}   
impl Engine {
    fn new (mut tk : Vec<TrackDatas>) -> Self {
        //트랙별 producer를 꺼내 엔진이 보관 (워커 전용)
        let mut prod_vec = Vec::with_capacity(tk.len()); //사전에 백터에 인덱스 만큼 공간 제작
        for tks in &mut tk {
            let prod = tks.circularbuffer.producer.take().expect("producer already taken");
            prod_vec.push(Mutex::new(prod));
        }
        let producers = Arc::new(prod_vec);

        let mut cons_vec =Vec::with_capacity(tk.len());
        for tks in &mut tk {
            let cons = tks.circularbuffer.consumer.take().expect("consumer already taken");
            cons_vec.push(Mutex::new(cons));
        }
        let consumers =Arc::new(cons_vec);

        //큐(채널)생성 채널 생성시 sender,Receiver 생성된다 현 프로젝트는 큐1개에 워커4개를 가동하기때문에 Arc포인터로 쉐어 필요
        let(tx,rx) =mpsc::channel::<Job>();
        let job_rx_shared = Arc::new(Mutex::new(rx));

        //종료 플래그
        let stop = Arc::new(AtomicBool::new(false));

        //워커4개 생성(노동자) - recv()는 수신대기로 (Job 들어올 때까지)
        let mut worker = Vec::with_capacity(4); //인덱스 4개 공간 생성
        for _ in 0..4 {
            let rx_c = Arc::clone(&job_rx_shared); //스레드 포인터 공유할 rx출구
            let stop_c = Arc::clone(&stop);        //종료 플래그 포인터 공유
            let prod_c = Arc::clone(&producers); //quee 공유

            worker.push(thread::spawn(move || {
               loop {
                    // 50ms 마다 깨어나 stop 플래그/종료여부 점검
                    let job = {
                    let rx = rx_c.lock().unwrap();
                    rx.recv_timeout(Duration::from_millis(50))
                    };

                    match job {
                    Ok(Job::DecodeFile { track, path }) => {
                    // 디코더 내부에서 stop_c.load()를 확인하며 중단
                    let _ = sound_thread_job::decode_and_push_into_track_ringbuffer(
                    track, &path, &prod_c, &stop_c
                    );
                    }
                    Err(RecvTimeoutError::Timeout) => {
                    // 그냥 대기 계속 (stop=true여도 워커는 죽지 않음)
                    // 필요하면 여기서 약간 sleep 없이 다음 루프로
                    continue;
                    }
                    Err(RecvTimeoutError::Disconnected) => {
                    // 채널이 닫힘(엔진 drop 시) → 진짜 종료
                    break;
                    }
                }
            }
        }));
    }
        Self { track: tk, producers, consumers, worker, queue: Some(tx), job_rx_shared, stop, output:None }
}

    fn enqueue_decode(&self) -> Result<(),String>{
        let Some(q) = &self.queue else { return Err("engine queue not available".to_string());};
        //엔진에 queue 를 가져오는데 실패시 스트링 전달
        for (track_index,tk) in self.track.iter().enumerate(){ 
            //트랙 배열을 순환하여 트랙을 가져와서 사용
            if tk.file_path.is_empty() { continue; }
            for path in &tk.file_path { //트랙에 파일 주소를 가져와서 queue에 send job을 전달
            q.send(Job::DecodeFile { track: track_index, path: path.clone() }).map_err(|_| "engine worker stopped".to_string())?;
            }
        }
        Ok(())
    }

    fn start_output_from_ringbuffer(&mut self) -> anyhow::Result<cpal::Stream> {
        let host = cpal::default_host();
        let device = host.default_output_device().ok_or_else(|| anyhow::anyhow!("no output device"))?;
        
        // 장치 기본(Windows 믹스) 포맷 사용: WASAPI에서 가장 안정적
        let supported = device.default_output_config()?;

        // 지금 콜백이 f32 전용이므로, 기본 포맷이 f32가 아니면 bail
        if supported.sample_format() != SampleFormat::F32 {
        anyhow::bail!("default output format is not f32; adjust callback or add match");
        }
        let config =supported.config();

        let channels = config.channels as usize;
        if channels != CHANNELS { anyhow::bail!("not stereo output");}

        let err_fn = |e| eprintln!("[cpal] stream error: {e}");

        //콜백에 넘길 핸들/파라미터 스냅샷
        let consumers =Arc::clone(&self.consumers);
        
        //빈 파일 트랙에서 제외
        let active_idxs: Arc<Vec<usize>> = Arc::new(
        self.track.iter()
            .enumerate()
            .filter(|(_, t)| !t.file_path.is_empty())
            .map(|(i, _)| i)
            .collect()
        );

        let vols: Arc<Vec<f32>> = Arc::new(self.track.iter().map(|t|t.volume).collect());
        let pans: Arc<Vec<f32>> = Arc::new(self.track.iter().map(|t|t.pan).collect());
        let mutes: Arc<Vec<bool>> = Arc::new(self.track.iter().map(|t|t.muted).collect());
        
        let active = active_idxs.clone();

        let stream =device.build_output_stream(&config,
            move |data:&mut [f32], _|{
                for frame in data.chunks_mut(2){
                    let (mut mix_l, mut mix_r) = (0.0f32,0.0f32);

                    for &idx in active.iter(){
                        if idx >= consumers.len() {continue;}
                        if idx >= vols.len() || idx >= pans.len() || idx >=mutes.len() {continue;}

                        let (sl,sr) = {
                            let mut cons = match consumers[idx].try_lock() {
                                Ok(g) => g,
                                Err(_) => continue,
                            };
                            (cons.pop().unwrap_or(0.0),cons.pop().unwrap_or(0.0))
                        };
                        let m = if mutes[idx] {0.0} else {1.0};
                        let v = vols[idx].clamp(0.0,1.0);
                        let p = pans[idx].clamp(-1.0,1.0);

                        let gl = m * v * (1.0 -p)*0.5;
                        let gr = m * v * (1.0 +p)*0.5;

                        mix_l += sl * gl;
                        mix_r += sr * gr;
                    }
                    frame[0]= mix_l.clamp(-1.0, 1.0);
                    frame[1]= mix_r.clamp(-1.0,1.0);
                }
            }, err_fn,None,)?;
            stream.play()?;
            Ok(stream)
    }

    fn flush_ringbuffers(&self) {
        for cons_mx in self.consumers.iter() {
            if let Ok(mut cons) = cons_mx.lock() {
                while cons.pop().is_ok() {}
            }
        }
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        self.stop.store(true,Ordering::Relaxed); //Engine에 stop 을 true로 변경하여 스레드 종료

       let _ = self.queue.take(); //큐 안에 Job이 들어오지않게 소유권을 버린다

       for h in self.worker.drain(..) { //엔진에서 스레드 핸들의 소유권을 하나씩 꺼내오기
        let _ = h.join(); //워커들은 스레드가 완전히 종료될 때까지 대기.
       }

       //엔진에서 큐,워크에 소유권을 버리기위한 작업;
    }
}

#[no_mangle]
pub extern "C" fn rust_audio_track_new(number : i32) -> *mut TrackDatas {
    let track = match TrackDatas::new(number) {
        Ok(data) => data,
        Err(_) => return std::ptr::null_mut(),
    };
    Box::into_raw(Box::new(track))
}

#[no_mangle]
pub extern  "C" fn rust_audio_track_free( tk :*mut TrackDatas) {
    if tk.is_null() {
        return;
    }
   unsafe { drop(Box::from_raw(tk));}
}

#[no_mangle]
pub extern "C" fn rust_audio_engine_new(track0:*mut TrackDatas,track1:*mut TrackDatas,track2:*mut TrackDatas,track3:*mut TrackDatas) -> *mut Engine {
      if track0.is_null() || track1.is_null() || track2.is_null() || track3.is_null() {
        return std::ptr::null_mut();
    }
    let t0 = unsafe { *Box::from_raw(track0) };
    let t1 = unsafe { *Box::from_raw(track1) };
    let t2 = unsafe { *Box::from_raw(track2) };
    let t3 = unsafe { *Box::from_raw(track3) };
    let  track :Vec<TrackDatas> = vec![t0,t1,t2,t3];
    let eng = Engine::new(track);
    Box::into_raw(Box::new(eng))
}

#[no_mangle]
pub extern  "C" fn rust_audio_engine_free( eng :*mut Engine) {
    if eng.is_null() {
        return;
    }
   unsafe { drop(Box::from_raw(eng));}
}