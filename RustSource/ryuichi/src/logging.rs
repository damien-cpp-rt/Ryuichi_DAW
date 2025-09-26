use std::{
    fs::{OpenOptions, create_dir_all},
    io::Write,
    path::Path,
    sync::{mpsc, OnceLock},
    thread,
    time::Duration,
};

pub(crate) static LOG_SENDER: OnceLock<mpsc::Sender<String>> = OnceLock::new();

/// 파일로 로깅 시작. 이미 시작돼 있으면 조용히 리턴.
pub fn init_file_log<P: AsRef<Path>>(path: P) {
    if LOG_SENDER.get().is_some() {
        return;
    }

    let path = path.as_ref();

    // 폴더가 없다면 만들어줌(예: C:\temp)
    if let Some(dir) = path.parent() {
        let _ = create_dir_all(dir);
    }

    let file = match OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        Ok(f) => f,
        Err(e) => {
            // 최후 수단: 콘솔로라도 알림 (JUCE에서도 디버그 출력창에 보일 수 있음)
            eprintln!("[rlog] failed to open log file {:?}: {}", path, e);
            return;
        }
    };

    let (tx, rx) = mpsc::channel::<String>();
    // OnceLock에 담아두어 송신자(tx)가 살아있게 함.
    let _ = LOG_SENDER.set(tx);

    // 백그라운드 라이터 스레드
    thread::spawn(move || {
        let mut f = file;
        let mut buf = Vec::<String>::with_capacity(256);

        loop {
            // 최소 1개는 블로킹으로 기다림
            let first = match rx.recv() {
                Ok(s) => s,
                Err(_) => break, // 채널 닫힘 → 스레드 종료
            };
            buf.push(first);

            // 2ms 동안 더 긁어모아 배치 쓰기
            let deadline = std::time::Instant::now() + Duration::from_millis(2);
            while let Ok(s) = rx.try_recv() {
                buf.push(s);
                if std::time::Instant::now() >= deadline {
                    break;
                }
            }

            // 쓰기 + 플러시
            for s in buf.drain(..) {
                let _ = f.write_all(s.as_bytes());
                let _ = f.write_all(b"\n");
            }
            let _ = f.flush();
        }
    });
}

/// 매크로가 호출하는 함수(경로 꼬임 방지용).
pub fn rlog_send<S: Into<String>>(s: S) {
    if let Some(tx) = LOG_SENDER.get() {
        let _ = tx.send(s.into());
    }
}

/// 전역 매크로. `$crate::rlog_send(..)`만 호출하도록 고정.
#[macro_export]
macro_rules! rlog {
    ($($t:tt)*) => {{
        $crate::rlog_send(format!($($t)*));
    }};
}
