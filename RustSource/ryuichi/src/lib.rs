
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

pub const CAPACITY_SAMPLES : usize = 144_000;
pub const CHANNELS: usize = 2;

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
    producers: Arc<Vec<Mutex<Producer<f32>>>>,
    consumers: Arc<Vec<Mutex<Consumer<f32>>>>,
    playout_producers: Vec<Option<Producer<f32>>>,
    playout_consumers: Vec<Option<Consumer<f32>>>,

    thread_worker: Vec<JoinHandle<()>>,
    copythread_worker: Option<JoinHandle<()>>, // ★ 복제 스레드 핸들 저장

    thread_stop: Arc<AtomicBool>, //스레드 종료
    copythread_stop: Arc<AtomicBool>, //스레드 종료
    thread_wait: Arc<(Mutex<bool>, Condvar)>, //전체 대기
    flush_flag : Arc<AtomicBool>,

    real_time_params: Arc<Parameters>,
    track_run_time: Arc<Vec<Mutex<TrackTimeline>>>,
    decod: Arc<Vec<Mutex<Option<DecoderState>>>>,

    play_time_manager: Arc<Transport>,
    seek_epoch: Arc<AtomicU64>,
    sound_output: Option<cpal::Stream>,
    track: Vec<TrackConfig>,
}   
impl Engine {
     fn new(mut tk: Vec<TrackConfig>) -> Self {
        // 1) 1차 링버퍼 소유권: TrackConfig에서 꺼내 Engine이 보관
        let mut prod_vec = Vec::with_capacity(tk.len());
        let mut cons_vec = Vec::with_capacity(tk.len());
        for tks in &mut tk {
            let tx = tks.circularbuffer.producer.take().expect("producer already taken");
            let rx = tks.circularbuffer.consumer.take().expect("consumer already taken");
            prod_vec.push(Mutex::new(tx));
            cons_vec.push(Mutex::new(rx));
        }
        let producers = Arc::new(prod_vec);
        let consumers = Arc::new(cons_vec);

        // 2) 2차(pl*): 복제 스레드 → cpal 로 가는 링버퍼
        let mut playout_producers = Vec::with_capacity(tk.len());
        let mut playout_consumers = Vec::with_capacity(tk.len());
        for _ in 0..tk.len() {
            let (tx, rx) = RingBuffer::<f32>::new(CAPACITY_SAMPLES);
            playout_producers.push(Some(tx));
            playout_consumers.push(Some(rx));
        }

        // 3) 공유 상태들
        let params = Arc::new(Parameters::from_tracks(&tk));
        let stop   = Arc::new(AtomicBool::new(false));
        let wait   = Arc::new((Mutex::new(true), Condvar::new()));
        let rt: Arc<Vec<Mutex<TrackTimeline>>> = Arc::new(
            (0..tk.len()).map(|_| Mutex::new(TrackTimeline { clips: BTreeMap::new(), write_pos_samples: 0 })).collect()
        );
        let decs: Arc<Vec<Mutex<Option<DecoderState>>>> = Arc::new((0..tk.len()).map(|_| Mutex::new(None)).collect());
        let playing = Arc::new(Transport::new(48_000));
        let repl_stop = Arc::new(AtomicBool::new(false));
        let flush_flag = Arc::new(AtomicBool::new(false));
        let seek_epoch = Arc::new(AtomicU64::new(0));

        // 4) 디코딩 워커들
        let decoding_workers: usize = 4;
        let mut worker = Vec::with_capacity(decoding_workers + 1);
        for i in 0..decoding_workers {
            let rt_c      = Arc::clone(&rt);
            let prod_c    = Arc::clone(&producers);
            let stop_c    = Arc::clone(&stop);
            let wait_c    = Arc::clone(&wait);
            let dec_c     = Arc::clone(&decs);
            let playing_c = Arc::clone(&playing);

            worker.push(thread::spawn(move || {
                loop {
                    if stop_c.load(Ordering::Relaxed) { break; }

                    // 대기 플래그
                    {
                        let (lock, cvar) = &*wait_c;
                        let mut waiting = lock.lock().unwrap();
                        while *waiting {
                            waiting = cvar.wait(waiting).unwrap();
                            if stop_c.load(Ordering::Relaxed) { return; }
                        }
                    }

                    let ntracks = rt_c.len();
                    if ntracks == 0 {
                        std::thread::yield_now();
                        continue;
                    }
                    let track_idx = i % ntracks;

                    // 꽉 찼으면 잠깐 양보
                    let should_fill = match prod_c[track_idx].lock() {
                        Ok(prod) => !prod.is_full(),
                        Err(_) => false,
                    };
                    if !should_fill {
                        std::thread::yield_now();
                        continue;
                    }

                    const FILL_FRAMES: usize = 65536;
                    let mut tr   = match rt_c[track_idx].lock()   { Ok(g) => g, Err(_) => continue };
                    let mut dec  = match dec_c[track_idx].lock()  { Ok(g) => g, Err(_) => continue };
                    let mut prod = match prod_c[track_idx].lock() { Ok(g) => g, Err(_) => continue };

                    let engine_sr = playing_c.sr();
                    if let Err(e) = fill_track_once(&mut *tr, &mut *dec, &mut *prod, FILL_FRAMES, engine_sr) {
                        eprintln!("[worker {i}] fill_track_once error: {e}");
                    }
                }
                std::thread::yield_now();
            }));
        }
        // 7) Self
        return Self {
            track: tk,
            producers,
            consumers,
            playout_producers,
            playout_consumers,

            thread_worker: worker,
            copythread_worker: None,          // ★ 여기!
            thread_stop: stop,
            copythread_stop: repl_stop,      // ★ 이름 일관
            thread_wait: wait,
            flush_flag: flush_flag,

            real_time_params: params,
            track_run_time: rt,
            decod: decs,

            play_time_manager: playing,
            seek_epoch,
            sound_output: None,
        };
    }

    fn spawn_copy_thread(&mut self) {
    if self.copythread_worker.is_some() { return; } // 이미 돌고 있으면 패스

    // 2차 Producer들을 스레드로 move
    let mut outs: Vec<Producer<f32>> = Vec::with_capacity(self.playout_producers.len());
    for p in &mut self.playout_producers {
        outs.push(p.take().expect("playout producer already moved"));
    }

    // 캡처할 공유 상태들 (self 캡처 금지)
    let cons_c        = Arc::clone(&self.consumers);
    let rt_c          = Arc::clone(&self.track_run_time);
    let wait_c        = Arc::clone(&self.thread_wait);
    let repl_stop_c   = Arc::clone(&self.copythread_stop);
    let params_c      = Arc::clone(&self.real_time_params);
    let seek_epoch_c  = Arc::clone(&self.seek_epoch);

    self.copythread_stop.store(false, Ordering::Relaxed);

    let handle = std::thread::spawn(move || {
        let mut last_epoch = seek_epoch_c.load(Ordering::Acquire);
        let mut states: Vec<RateState> = Vec::new();
        let mut src_fifos: Vec<VecDeque<f32>> = Vec::new();
        const CHUNK_FRAMES: usize = 2048;
        const FIFO_MAX_FRAMES: usize = 4 * 2048;

        loop {
            if repl_stop_c.load(Ordering::Relaxed) { break; }

            // 대기 플래그
            {
                let (lock, cvar) = &*wait_c;
                let mut waiting = lock.lock().unwrap();
                while *waiting {
                    waiting = cvar.wait(waiting).unwrap();
                    if repl_stop_c.load(Ordering::Relaxed) { return; }
                }
            }

            let ntracks = rt_c.len();
            if ntracks == 0 {
                std::thread::yield_now();
                continue;
            }

            if states.len() != ntracks {
                states = (0..ntracks).map(|_| RateState {
                    frac: 0.0, prev_l: 0.0, prev_r: 0.0, next_l: 0.0, next_r: 0.0, primed: false
                }).collect();
            }
            if src_fifos.len() != ntracks {
                src_fifos = (0..ntracks).map(|_| VecDeque::<f32>::new()).collect();
            }

            // seek_epoch 바뀌면 내부상태 초기화
            let cur_epoch = seek_epoch_c.load(Ordering::Acquire);
            if cur_epoch != last_epoch {
                for f in &mut src_fifos { f.clear(); }
                for st in &mut states {
                    *st = RateState { frac: 0.0, prev_l: 0.0, prev_r: 0.0, next_l: 0.0, next_r: 0.0, primed: false };
                }
                last_epoch = cur_epoch;
            }

            // resample step
            let bpm  = f32::from_bits(params_c.bpm.load(Ordering::Relaxed)).clamp(20.0, 300.0);
            let step = bpm / 60.0_f32;

            for idx in 0..ntracks {
                // 1) 1차 consumer에서 가져와 FIFO에 채우기
                if let Ok(mut rc) = cons_c[idx].lock() {
                    for _ in 0..CHUNK_FRAMES {
                        match (rc.pop(), rc.pop()) {
                            (Ok(l), Ok(r)) => { src_fifos[idx].push_back(l); src_fifos[idx].push_back(r); }
                            _ => break,
                        }
                    }
                }

                // 2) priming
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
                        continue;
                    }
                }

                    // 3) 2차 producer로 출력
                    let pp = &mut outs[idx];
                    let mut produced = 0usize;
                    while produced < CHUNK_FRAMES && !pp.is_full() {
                        let yl = st.prev_l + (st.next_l - st.prev_l) * st.frac;
                        let yr = st.prev_r + (st.next_r - st.prev_r) * st.frac;

                        if pp.push(yl).is_err() || pp.push(yr).is_err() { break; }
                        produced += 1;

                        st.frac += step;
                        while st.frac >= 1.0 {
                        st.frac -= 1.0;
                        if src_fifos[idx].len() < 2 { break; }
                        st.prev_l = st.next_l; st.prev_r = st.next_r;
                        st.next_l = src_fifos[idx].pop_front().unwrap();
                        st.next_r = src_fifos[idx].pop_front().unwrap();
                        }
                        if src_fifos[idx].len() < 2 && st.frac >= 1.0 { break; }
                    }
                }

                std::thread::yield_now();
            }
        });

        self.copythread_worker = Some(handle);
    }

    // fn stop_replication(&mut self) {
    //     if self.copythread_worker.is_none() { return; }
    //     self.copythread_stop.store(true, Ordering::Relaxed);
    //     self.wake_workers(); // condvar 깨워서 루프 탈출하도록
    //     if let Some(h) = self.copythread_worker.take() {
    //         let _ = h.join();
    //     }
    //     self.copythread_stop.store(false, Ordering::Relaxed);
    // }

    fn stop_copy_thread(&mut self) {
    if self.copythread_worker.is_none() { return; }
    self.copythread_stop.store(true, Ordering::Relaxed);
    self.wake_workers(); // condvar 깨워서 루프 탈출
    if let Some(h) = self.copythread_worker.take() {
        let _ = h.join(); // 여기서 outs(2차 Producer) drop
    }
    self.copythread_stop.store(false, Ordering::Relaxed);

    // 스레드가 2차 P를 drop했으니, 다시 쓸 수 있도록 재생성
    self.rebuild_second_ringbuffers();
    }

    fn rebuild_second_ringbuffers(&mut self) {
        self.playout_producers.clear();
        self.playout_consumers.clear();
        for _ in 0..self.track.len() {
            let (tx, rx) = RingBuffer::<f32>::new(CAPACITY_SAMPLES);
            self.playout_producers.push(Some(tx));
            self.playout_consumers.push(Some(rx));
        }
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

        // // playout_*도 쓰면 동일하게 재생성
        // for i in 0..self.playout_producers.len() {
        //     let (tx, rx) = RingBuffer::<f32>::new(CAPACITY_SAMPLES);
        //     self.playout_producers[i] = Some(tx);
        //     self.playout_consumers[i] = Some(rx);
        // }
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
        let mut play_cons :Vec<Consumer<f32>> = Vec::with_capacity(self.playout_consumers.len());
        for slot in self.playout_consumers.iter_mut(){
            let c = slot.take().expect("playout consumer already moved");
            play_cons.push(c);
        }

        //활성화 트랙 저장
        let active_idxs: Arc<Vec<usize>> = Arc::new((0..play_cons.len()).collect());
        let flush_flag = Arc::clone(&self.flush_flag);
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
        let mut mix_l_buf: Vec<f32> = Vec::new();
        let mut mix_r_buf: Vec<f32> = Vec::new();
        let stream =device.build_output_stream(&config, //출력 스트림 생성
            move |data:&mut [f32], _|{             //출력 콜백 기본 구조 FnMut(&mut [T], &cpal::OutputCallbackInfo) 기본구조에 맞추어 data 버퍼와 콜백정보를 받음
                 if flush_flag.swap(false, std::sync::atomic::Ordering::AcqRel) {
                    for &idx in active_idxs.iter() {
                        let c = &mut play_cons[idx];
                        // L/R 인터리브드로 모두 비움
                        loop {
                                match (c.pop(), c.pop()) {
                                    (Ok(_), Ok(_)) => continue,
                                    _ => break,
                                }
                            }
                        last[idx] = (0.0, 0.0);
                        pend_l[idx] = None;
                    }
                ramp_pos = 0; // 클릭 방지용 짧은 페이드인
                }

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
                // 이번 콜백에서 필요한 프레임 수
                let nframes = data.len() / 2;
                
                if mix_l_buf.len() != nframes {
                mix_l_buf.resize(nframes, 0.0);
                mix_r_buf.resize(nframes, 0.0);
                } else {
                        for v in &mut mix_l_buf[..] { *v = 0.0; }
                        for v in &mut mix_r_buf[..] { *v = 0.0; }
                    }

                if transport_c.in_playing() {
                    transport_c.advance_samples((data.len() / channels) as u64); //재생중이면 재생 위치 증가
                }
                
                
                

                // // 믹스 누적 버퍼(한 번에 모아서 씀)
                // let mut mix_l_buf = vec![0.0f32; nframes];
                // let mut mix_r_buf = vec![0.0f32; nframes];

                // 트랙 단위로 한 번만 lock 해서 nframes 만큼 pop → 누적
                for &idx in active_idxs.iter() {
                    if idx >= params.volume.len() || idx >= params.pan.len() || idx >= params.muted.len() { continue; }

                    let muted = params.muted[idx].load(Ordering::Relaxed);
                    let vol   = f32::from_bits(params.volume[idx].load(Ordering::Relaxed)).clamp(0.0, 1.0);
                    let pan   = f32::from_bits(params.pan[idx].load(Ordering::Relaxed)).clamp(-1.0, 1.0);
                    let m = if muted { 0.0 } else { 1.0 };
                    let gl = m * vol * (1.0 - pan) * 0.5;
                    let gr = m * vol * (1.0 + pan) * 0.5;

                    let c = &mut play_cons[idx];
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

    fn flush_ringbuffers(&mut self) {
        for cons_mx in self.consumers.iter() {
            if let Ok(mut cons) = cons_mx.lock() {
                while cons.pop().is_ok() {}
            }
        }
        
        // for cons_playout in self.playout_consumers.iter_mut() { 
        //     if let Some (mut cons) =cons_playout.as_mut() {
        //         while cons.pop().is_ok() {}
        //     }
        // }
    
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
        // 복제 스레드부터 정리 (2차 P drop + 재생성 안 해도 됨: 어차피 drop 중)
        if self.copythread_worker.is_some() {
            self.stop_copy_thread();
        }

        self.thread_stop.store(true, Ordering::Relaxed);
        self.wake_workers();

        if let Some(stream) = self.sound_output.take() {
            let _ = stream.pause();
        }
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

