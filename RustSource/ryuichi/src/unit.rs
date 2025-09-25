pub const CHANNELS: usize = 2;
#[inline] pub fn slots(frames: usize) -> usize { frames * CHANNELS } // frames → samples

// 링버퍼·FIFO 용량 (전부 frames 단위)
pub const RB1_FRAMES: usize      = 65_536;  // 트랙(1차) 링버퍼 용량
pub const RB2_FRAMES: usize      = 98_304;   // 재생(2차) 링버퍼 용량
pub const FIFO_MAX_FRAMES: usize = 98_304;   // 복제 스레드 내부 FIFO 상한

// 디코드/복제 청크 (frames)
pub const CHUNK_DECODE: usize = 12_288;      // 디코더 워커가 한 번에 밀어넣는 크기
pub const CHUNK_COPY:   usize = 8_192;      // 복제 스레드 보간 출력 단위

// 전역 워터마크 (frames) — 히스테리시스
pub const HIGH_FRAMES: usize = 49_152;
pub const LOW_FRAMES:  usize = 24_576;

// 출력 페이드인 길이 (frames)
pub const RAMP_FRAMES: usize = 256;