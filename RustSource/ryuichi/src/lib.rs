
mod waveform_generation_module;
pub use waveform_generation_module::*;
mod sound_track_update;
pub use sound_track_update::*;
mod sound_play;
pub use sound_play::*;

use std::collections::BTreeMap;
use std::sync::Condvar;
use std::time::Duration;
use std::collections::VecDeque;

const CAPACITY_SAMPLES : usize = 144_000;
const CHANNELS: usize = 2;

enum TrackNumber {
    Zero,
    One,
    Two,
    Three,
}
pub struct CircularBuffer {
    producer : Option<Producer<f32>>, //디코더/프로듀서가 push할 핸들
    consumer : Option<Consumer<f32>>, //소비(믹서/출력)가 pop할 핸들
}

pub struct TrackConfig {
    track_number : TrackNumber,
    volume : f32,
    muted : bool,
    pan : f32,
    reverb : bool,
    delay : bool,
    circularbuffer : CircularBuffer,
}
impl TrackConfig {
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
        volume : 0.5,
        muted : false,
        pan : 0.0,
        reverb : false,
        delay : false,
        circularbuffer : circularbuffer,
      })
    }
}

pub struct Parameters  {
    volume : Vec<AtomicU32>,
    pan : Vec<AtomicU32>,
    muted : Vec<AtomicBool>,
    bpm : AtomicU32,
}
impl Parameters {
    fn from_tracks(track: &Vec<TrackConfig>) -> Self {
        let volume = track.iter().map(|t| AtomicU32::new(t.volume.to_bits())).collect();
        let pan = track.iter().map(|t| AtomicU32::new(t.pan.to_bits())).collect();
        let muted = track.iter().map(|t| AtomicBool::new(t.muted)).collect();
        let bpm = AtomicU32::new((60.0f32).to_bits());
        Self { volume, pan, muted ,bpm}
    }
}

pub struct Clip {
    file_path : String,
    src_sr : u32,
    tl_start : u64,
    tl_len : u64,
}
pub struct TrackTimeline{
    clips : BTreeMap<u64,Clip>, //시작시간,클립 
    write_pos_samples : u64, //현재 재생 위치
}
pub struct DecoderState {
    format: Box<dyn symphonia::core::formats::FormatReader>,
    decoder: Box<dyn symphonia::core::codecs::Decoder>,
    sample_buf: SampleBuffer<f32>,
    src_sr: u32,
    src_pos_samples: u64,
    fille_path: String,
}

pub struct Transport {
    playing: AtomicBool,
    playhead_samples: AtomicU64,
    sample_rate : AtomicU32,
}
impl Transport {
    fn new(sr: u32) -> Self {
        Self {
            playing: AtomicBool::new(false),
            playhead_samples: AtomicU64::new(0),
            sample_rate: AtomicU32::new(sr),
        }
    }
    fn set_sr(&self, sr: u32) { //샘플링 레이트 설정
        self.sample_rate.store(sr, Ordering::Relaxed);
    }
    fn sr(&self)-> u32 { //샘플링 레이트
        self.sample_rate.load(Ordering::Relaxed)
    }
    fn start(&self) { //재생
        self.playing.store(true, Ordering::Relaxed);
    }
    fn stop(&self) { //정지
        self.playing.store(false, Ordering::Relaxed);
    }
    fn in_playing(&self) -> bool { //재생중인지
        self.playing.load(Ordering::Relaxed)
    }
    fn seek_samples(&self, s: u64){ //재생 위치를 s로 이동
        self.playhead_samples.store(s, Ordering::Relaxed);
    }
    fn pos_samples(&self) -> u64 { //현재 재생 위치
        self.playhead_samples.load(Ordering::Relaxed)
    }
    fn advance_samples(&self, s: u64) { //재생 위치를 s만큼 증가
        self.playhead_samples.fetch_add(s, Ordering::Relaxed);
    }
}

pub struct RateState {
    frac: f32,
    prev_l: f32,
    prev_r: f32,
    next_l: f32,
    next_r: f32,
    primed: bool,
}

pub struct Engine {
    track : Vec<TrackConfig>, //pull 해올것
    producers: Arc<Vec<Mutex<Producer<f32>>>>,   // 워커가 push 할 대상
    consumers: Arc<Vec<Mutex<Consumer<f32>>>>,   // cpal 이 사용할 핸들
    playout_producers: Arc<Vec<Mutex<Producer<f32>>>>,   // BPM조절된 샘플 저장
    playout_consumers: Arc<Vec<Mutex<Consumer<f32>>>>,   // playout 출구
    thread_worker: Vec<JoinHandle<()>>, //스레드 워커 노동자
    thread_stop: Arc<AtomicBool>,    //정지 확인
    sound_output: Option<cpal::Stream>,               //출력 장치
    real_time_params: Arc<Parameters>, //파라미터 볼륨 패닝 뮤트
    track_run_time : Arc<Vec<Mutex<TrackTimeline>>>, //트랙별 런타임 정보
    thread_wait : Arc<(Mutex<bool>,Condvar)>, // 대기 플래그
    decod: Arc<Vec<Mutex<Option<DecoderState>>>>, //트랙별 디코더 상태
    play_time_manager: Arc<Transport>,
    seek_epoch : Arc<AtomicU64>, //에폭 
}   
impl Engine {
    fn new (mut tk : Vec<TrackConfig>) -> Self {
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

        
        let mut ply_prod_vec = Vec::with_capacity(tk.len());
        let mut ply_cons_vec = Vec::with_capacity(tk.len());
        for _ in 0..tk.len() {
            let (tx, rx) = RingBuffer::<f32>::new(CAPACITY_SAMPLES);
            ply_prod_vec.push(Mutex::new(tx));
            ply_cons_vec.push(Mutex::new(rx));
        }

        let playout_producers = Arc::new(ply_prod_vec);
        let playout_consumers = Arc::new(ply_cons_vec);

        //트랙에서 파라미터 생성
        let params = Arc::new(Parameters::from_tracks(&tk)); 
        //종료 플래그 및 대기 플래그
        let stop = Arc::new(AtomicBool::new(false));
        let wait = Arc::new((Mutex::new(true), Condvar::new()));
        //트랙별 런타임 정보 생성
        let rt: Arc<Vec<Mutex<TrackTimeline>>> = Arc::new((0..tk.len())
        .map(|_| Mutex::new(TrackTimeline { clips: BTreeMap::new(), write_pos_samples: 0 }))
        .collect());
        //트랙별 디코더 상태 생성
        let decs: Arc<Vec<Mutex<Option<DecoderState>>>> = Arc::new((0..tk.len()).map(|_| Mutex::new(None)).collect());
        //트랜스포트 생성
        let playing =Arc::new(Transport::new(48_000));

        //에폭 생성
        let seek_epoch = Arc::new(AtomicU64::new(0));
        //워커4개 생성(노동자) - recv()는 수신대기로 (Job 들어올 때까지)
        let decoding_workers :usize = 4;
        let replication_worker : usize = 1;
        let mut worker = Vec::with_capacity(decoding_workers + replication_worker); //인덱스 4개 공간 생성
        for i in 0..decoding_workers {
            let rt_c = Arc::clone(&rt); //스레드 포인터 공유할 rx출구
            let prod_c = Arc::clone(&producers); //quee 공유
            let stop_c = Arc::clone(&stop);        //종료 플래그 포인터 공유
            let wait_c = Arc::clone(&wait); //대기 플래그 포인터 공유
            let dec_c = Arc::clone(&decs); //디코더 상태 공유
            let playing_c = Arc::clone(&playing); //트랜스포트 공유
            worker.push(thread::spawn(move || {
               loop {
                   //정지 플래그
                    if stop_c.load(Ordering::Relaxed) { break; } 
                   
                    //대기 플래그
                    {
                        let (lock, cvar) = &*wait_c;
                        let mut waiting = lock.lock().unwrap();
                        while *waiting {
                            waiting =cvar.wait(waiting).unwrap();
                            if stop_c.load(Ordering::Relaxed) { return; }
                        }
                    }

                    //트랙이 없으면 대기
                    let ntracks = rt_c.len();
                    if ntracks == 0 {
                    std::thread::yield_now();
                    continue;
                    }
                    let track_idx = i % ntracks; //트랙 인덱스

                    //트랙별로 디코더 상태 및 링버퍼 프로듀서 가져오기
                    const FILL_FRAMES: usize = 4096;

                    let should_fill = match prod_c[track_idx].lock() {
                    Ok(prod) => !prod.is_full(),   // 링버퍼가 꽉 찼는지 확인
                    Err(_) => false,
                    };

                    if !should_fill {
                    std::thread::yield_now(); // 꽉 찼으면 대기
                    continue;
                    }

                    let frames_need = FILL_FRAMES; // 채워야 할 프레임 수

                    // 고정된 잠금 순서: rt -> dec -> prod
                    let mut tr   = match rt_c[track_idx].lock()  { Ok(g) => g, Err(_) => continue }; //트랙 타임라인 락
                    let mut dec  = match dec_c[track_idx].lock() { Ok(g) => g, Err(_) => continue }; //디코더 상태 락
                    let mut prod = match prod_c[track_idx].lock(){ Ok(g) => g, Err(_) => continue }; //프로듀서 락
                    
                    let engine_sr = playing_c.sr(); //엔진 샘플링 레이트

                     if let Err(e) = fill_track_once( //job 처리
                        &mut *tr,
                        &mut *dec,           // ★ Option<DecoderState> 넘김
                        &mut *prod,
                        frames_need,
                        engine_sr
                        ) {
                            eprintln!("[worker {i}] fill_track_once error: {e}");
                        }
                }
                std::thread::yield_now();
            })); //디코딩 스레드 생성
    }
    let rt_c = Arc::clone(&rt);
    let cons_c = Arc::clone(&consumers);
    let playout_prod_c = Arc::clone(&playout_producers);
    let stop_c = Arc::clone(&stop);
    let wait_c = Arc::clone(&wait);
    let params_c = Arc::clone(&params); // ★ 추가
    let seek_epoch_c = Arc::clone(&seek_epoch);

    let mut states: Vec<RateState> = Vec::new();
    let mut src_fifos: Vec<VecDeque<f32>> = Vec::new(); // ★추가: 트랙별 입력 FIFO
    const CHUNK_FRAMES: usize = 4096;
    {  //복제 스레드
     worker.push(thread::spawn(move|| {
                let mut last_epoch = seek_epoch_c.load(Ordering::Acquire);
                loop {
                   //정지 플래그
                    if stop_c.load(Ordering::Relaxed) { break; } 
                   
                    //대기 플래그
                    {
                        let (lock, cvar) = &*wait_c;
                        let mut waiting = lock.lock().unwrap();
                        while *waiting {
                            waiting =cvar.wait(waiting).unwrap();
                            if stop_c.load(Ordering::Relaxed) { return; }
                        }
                    }

                    //트랙이 없으면 대기
                    let ntracks = rt_c.len();
                    if ntracks == 0 {
                    std::thread::yield_now();
                    continue;
                    }

                    let cur_epoch = seek_epoch_c.load(Ordering::Acquire);
                
                   if states.len() != ntracks {
                    states = (0..ntracks).map(|_| RateState {
                    frac: 0.0, prev_l: 0.0, prev_r: 0.0, next_l: 0.0, next_r: 0.0, primed: false
                    }).collect();
                    }
                    if src_fifos.len() != ntracks {
                    src_fifos = (0..ntracks).map(|_| VecDeque::<f32>::new()).collect();
                    }

                    // ★ seek_epoch 변경 시 내부 상태 초기화
                    let cur_epoch = seek_epoch_c.load(Ordering::Acquire);
                    if cur_epoch != last_epoch {
                    for f in &mut src_fifos { f.clear(); }
                    for st in &mut states {
                    *st = RateState { frac: 0.0, prev_l: 0.0, prev_r: 0.0, next_l: 0.0, next_r: 0.0, primed: false };
                    }
                    last_epoch = cur_epoch;
                    }

                    // BPM → step
                    let bpm = f32::from_bits(params_c.bpm.load(Ordering::Relaxed)).clamp(20.0, 300.0);
                    let step = bpm / 60.0_f32;

                    for idx in 0..ntracks {
                    // 1) 입력 FIFO 채우기(가능한 만큼)
                    if let Ok(mut rc) = cons_c[idx].lock() {
                    for _ in 0..CHUNK_FRAMES { // 한 번에 너무 많이 잡아먹지 않도록 상한
                    match (rc.pop(), rc.pop()) {
                    (Ok(l), Ok(r)) => { src_fifos[idx].push_back(l); src_fifos[idx].push_back(r); }
                    _ => break,
                    }
                    }
                    }

                    // 2) priming (최소 2프레임 = 4샘플 필요)
                    let st = &mut states[idx];
                    if !st.primed {
                    if src_fifos[idx].len() >= 4 {
                    st.prev_l = src_fifos[idx].pop_front().unwrap();
                    st.prev_r = src_fifos[idx].pop_front().unwrap();
                    st.next_l = src_fifos[idx].pop_front().unwrap();
                    st.next_r = src_fifos[idx].pop_front().unwrap();
                    st.frac = 0.0;
                    st.primed = true;
                    } else {
                    continue; // 입력 더 필요
                    }
                    }

                    // 3) 출력: 링버퍼 여유·입력 보유 조건 하에서만 생성
                    if let Ok(mut pp) = playout_prod_c[idx].lock() {
                    let mut produced = 0usize;
                    while produced < CHUNK_FRAMES && !pp.is_full() {
                    // 선형 보간
                    let yl = st.prev_l + (st.next_l - st.prev_l) * st.frac;
                    let yr = st.prev_r + (st.next_r - st.prev_r) * st.frac;

                    if pp.push(yl).is_err() || pp.push(yr).is_err() { break; }
                    produced += 1;

                    // 페이즈 전진
                    st.frac += step;
                    while st.frac >= 1.0 {
                    st.frac -= 1.0;
                    // 다음 입력 프레임이 준비되어 있지 않으면 멈춤(버리지 않음)
                    if src_fifos[idx].len() < 2 {
                    // 다음 루프에서 입력 더 채워오도록 빠져나감
                    break;
                    }
                    st.prev_l = st.next_l; st.prev_r = st.next_r;
                    st.next_l = src_fifos[idx].pop_front().unwrap();
                    st.next_r = src_fifos[idx].pop_front().unwrap();
                    }

                    // 위 while에서 입력이 부족해 빠져나왔다면 이번 프레임 생산 종료
                    if src_fifos[idx].len() < 2 && st.frac >= 1.0 {
                    break;
            }
        }
    }
}
        std::thread::yield_now();
        }
        }));
    }
        Self { track: tk, producers, consumers,playout_producers,playout_consumers ,thread_worker: worker, thread_stop: stop, sound_output:None , real_time_params: params , track_run_time: rt ,thread_wait: wait , decod: decs , play_time_manager: playing,seek_epoch:seek_epoch}
}
    
    fn wake_workers(&self) { //워커 깨우기
        let (lock, cvar) = &*self.thread_wait;
        *lock.lock().unwrap() = false;
        cvar.notify_all();
    }

    fn pause_workers(&self) { //워커 대기
        let (lock, _) = &*self.thread_wait;
        *lock.lock().unwrap() = true;
    }

    fn align_write_pos_to_transport(&self) {
        let pos = self.play_time_manager.pos_samples();
        for tr_mx in self.track_run_time.iter() {
            if let Ok(mut tr) = tr_mx.lock() {
                tr.write_pos_samples = pos;
            }
        }
    }
    fn rebuild_all_ringbuffers(&mut self) {
        for i in 0..self.track.len() {
            let (tx, rx) = RingBuffer::<f32>::new(CAPACITY_SAMPLES);

            if let Ok(mut p) = self.producers[i].lock()  { *p = tx; }
            if let Ok(mut c) = self.consumers[i].lock()  { *c = rx; }

            // (선택) 트랙 안의 사본도 끊어버리거나 맞춰준다.
            if let Some(tr) = self.track.get_mut(i) {
                tr.circularbuffer.producer = None;  // ← 트랙 사본 비사용화 (추천)
                tr.circularbuffer.consumer = None;  //  or 필요하면 여기도 새 tx/rx로 교체
            }
        }

        // playout_*도 쓰면 동일하게 재생성
        for i in 0..self.playout_producers.len() {
            let (tx, rx) = RingBuffer::<f32>::new(CAPACITY_SAMPLES);
            if let Ok(mut p) = self.playout_producers[i].lock() { *p = tx; }
            if let Ok(mut c) = self.playout_consumers[i].lock() { *c = rx; }
        }
    }

    fn start_output_from_ringbuffer(&mut self) -> anyhow::Result<cpal::Stream> {
        let host = cpal::default_host(); //기본 오디오 호스트 (Windows: WASAPI, Linux: ALSA/PulseAudio 등)
        let device = host.default_output_device().ok_or_else(|| anyhow::anyhow!("no output device"))?; //기본 장비 조회
        
        // 장치 기본(Windows 믹스) 포맷 사용: WASAPI에서 가장 안정적
        let supported = device.default_output_config()?;

        // 지금 콜백이 f32 전용이므로, 기본 포맷이 f32가 아니면 bail
        if supported.sample_format() != SampleFormat::F32 {
        anyhow::bail!("default output format is not f32; adjust callback or add match");
        }
        let config =supported.config(); //기본 포맷 설정
        let sr = config.sample_rate.0; //샘플링 레이트
        self.play_time_manager.set_sr(sr); //트랜스포트에 샘플링 레이트 설정

        let channels = config.channels as usize; //채널 수
        if channels != CHANNELS { anyhow::bail!("not stereo output");} //2채널이 아니면 종료

        let err_fn = |e| eprintln!("[cpal] stream error: {e}"); //에러 콜백

        //콜백에 넘길 핸들/파라미터 스냅샷
        let consumers =Arc::clone(&self.playout_consumers);
        
        //활성화 트랙 저장
        let active_idxs: Arc<Vec<usize>> = Arc::new((0..consumers.len()).collect());

        let params    = Arc::clone(&self.real_time_params); //실시간 파라미터 핸들
        let active = active_idxs.clone(); //활성화 트랙 인덱스
        let transport_c = Arc::clone(&self.play_time_manager); //트랜스포트 핸들
        #[derive(Clone,Copy)]
        struct Resamp {  //선형보간 용 구조체
            frac :f32,
            s0_l :f32,
            s0_r :f32,
            s1_l :f32,
            s1_r :f32,
        }

        let mut last: Vec<(f32, f32)> = vec![(0.0, 0.0); active_idxs.len()]; // 마지막 정상 L/R
        let mut pend_l: Vec<Option<f32>> = vec![None; active_idxs.len()];     // 홀로 pop된 L 임시보관
        let mut ramp_pos: usize = 0;                                          // 페이드인 램프
        const RAMP: usize = 64; 

        let stream =device.build_output_stream(&config, //출력 스트림 생성
            move |data:&mut [f32], _|{             //출력 콜백 기본 구조 FnMut(&mut [T], &cpal::OutputCallbackInfo) 기본구조에 맞추어 data 버퍼와 콜백정보를 받음

                //예외처리 - 스테레오가 아니거나 활성화 트랙이 없으면 무음
                if channels != 2 || active.is_empty() {     //스테레오가 아니거나 활성화 트랙이 없으면 무음
                    for sample in data.iter_mut() { //data 버퍼 순환
                        *sample = 0.0; //0.0으로 채움
                    }
                    return;
                }
                //활성화 트랙이 없으면 무음
                if active.is_empty() {
                    for sample in data.iter_mut() {
                        *sample = 0.0;
                    }
                    return;
                }
            
                if transport_c.in_playing() {
                    transport_c.advance_samples((data.len() / channels) as u64); //재생중이면 재생 위치 증가
                }

                // 이번 콜백에서 필요한 프레임 수
                let nframes = data.len() / 2;

                // 믹스 누적 버퍼(한 번에 모아서 씀)
                let mut mix_l_buf = vec![0.0f32; nframes];
                let mut mix_r_buf = vec![0.0f32; nframes];

                // 트랙 단위로 한 번만 lock 해서 nframes 만큼 pop → 누적
                for &idx in active_idxs.iter() {
                    if idx >= params.volume.len() || idx >= params.pan.len() || idx >= params.muted.len() { continue; }

                    let muted = params.muted[idx].load(Ordering::Relaxed);
                    let vol   = f32::from_bits(params.volume[idx].load(Ordering::Relaxed)).clamp(0.0, 1.0);
                    let pan   = f32::from_bits(params.pan[idx].load(Ordering::Relaxed)).clamp(-1.0, 1.0);
                    let m = if muted { 0.0 } else { 1.0 };
                    let gl = m * vol * (1.0 - pan) * 0.5;
                    let gr = m * vol * (1.0 + pan) * 0.5;

                    if let Ok(mut c) = consumers[idx].lock() {
                        for f in 0..nframes {
                            // 반쪽 프레임 방지 + 마지막 정상 샘플 캐시
                            let (mut l, mut r) = last[idx];

                                if let Some(stashed_l) = pend_l[idx].take() {
                                    match c.pop() {
                                        Ok(rv) => { l = stashed_l; r = rv; last[idx] = (l, r); }
                                        Err(_) => { pend_l[idx] = Some(stashed_l); ramp_pos = 0; }
                                    }
                                } else {
                                        match (c.pop(), c.pop()) {
                                            (Ok(lv), Ok(rv)) => { l = lv; r = rv; last[idx] = (l, r); }
                                            (Ok(lv), Err(_)) => { pend_l[idx] = Some(lv); ramp_pos = 0; }
                                            _ => { ramp_pos = 0; /* 언더런: last 유지 */ }
                                        }
                                    }
                            mix_l_buf[f] += l * gl;
                            mix_r_buf[f] += r * gr;
                        }
                    } else {
                            // 락 실패 시에도 클릭 방지
                            ramp_pos = 0;
                        }
                }

                // 램프 게인 곱해서 한 번에 출력
                for (f, frame) in data.chunks_mut(2).enumerate() {
                    let m = if ramp_pos < RAMP { (ramp_pos as f32) / (RAMP as f32) } else { 1.0 };
                    ramp_pos = ramp_pos.saturating_add(1);

                    frame[0] = (mix_l_buf[f] * m).clamp(-1.0, 1.0);
                    frame[1] = (mix_r_buf[f] * m).clamp(-1.0, 1.0);
                }            
        }, err_fn, None)?;
    stream.play()?;
    Ok(stream)
}

    fn flush_ringbuffers(&self) {
        for cons_mx in self.consumers.iter() {
            if let Ok(mut cons) = cons_mx.lock() {
                while cons.pop().is_ok() {}
            }
        }
        
        for cons_playout in self.playout_consumers.iter() { 
            if let Ok (mut cons) =cons_playout.lock() {
                while cons.pop().is_ok() {}
            }
        }
    
    }

    fn prefill_all_tracks(&self, freames: usize) -> Result<(),String> {
        let sr = self.play_time_manager.sr();
        let n = self.track.len();

        for i in 0..n {
            let mut tr   = match self.track_run_time[i].lock()  { Ok(g) => g, Err(_) => continue };
            let mut dec  = match self.decod[i].lock()           { Ok(g) => g, Err(_) => continue };
            let mut prod = match self.producers[i].lock()       { Ok(g) => g, Err(_) => continue };
            fill_track_once(&mut tr,&mut dec,&mut prod, freames, sr)?;
        }
        Ok(())
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
           // 1) 워커에게 종료 신호
        self.thread_stop.store(true, Ordering::Relaxed);

        // 2) 대기 중인 워커를 깨워서 stop 플래그를 보게 함
        self.wake_workers();

        // 3) 오디오 스트림 정지 (있으면)
        if let Some(stream) = self.sound_output.take() {
            let _ = stream.pause();
        }

        // 4) 워커 종료 대기
        for h in self.thread_worker.drain(..) {
            let _ = h.join();
        }
    }
}

#[no_mangle]
pub extern "C" fn rust_audio_track_new(number : i32) -> *mut TrackConfig {
    let track = match TrackConfig::new(number) {
        Ok(data) => data,
        Err(_) => return std::ptr::null_mut(),
    };
    Box::into_raw(Box::new(track))
}

#[no_mangle]
pub extern  "C" fn rust_audio_track_free( tk :*mut TrackConfig) {
    if tk.is_null() {
        return;
    }
   unsafe { drop(Box::from_raw(tk));}
}

#[no_mangle]
pub extern "C" fn rust_audio_engine_new(track0:*mut TrackConfig,track1:*mut TrackConfig,track2:*mut TrackConfig,track3:*mut TrackConfig) -> *mut Engine {
      if track0.is_null() || track1.is_null() || track2.is_null() || track3.is_null() {
        return std::ptr::null_mut();
    }
    let t0 = unsafe { *Box::from_raw(track0) };
    let t1 = unsafe { *Box::from_raw(track1) };
    let t2 = unsafe { *Box::from_raw(track2) };
    let t3 = unsafe { *Box::from_raw(track3) };
    let  track :Vec<TrackConfig> = vec![t0,t1,t2,t3];
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

