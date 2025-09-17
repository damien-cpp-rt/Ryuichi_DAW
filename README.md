Ryuichi DAW (JUCE + Rust FFI)

JUCE 기반 C++ UI와 Rust 오디오 엔진을 FFI(DLL)로 연결한 DAW 실험 프로젝트입니다.
JUCE 쪽에는 파일 브라우저/리스트 등 UI 컴포넌트가 있고(예: VSTFileUI.h), Rust 쪽에는 디코딩・리샘플・믹싱・출력(cpal) 엔진이 들어 있습니다. 

acc1c0cf-5109-457d-9f08-2bb5ad7…

 

5ea77f29-6134-415f-bf17-45f2f7c…

구성 개요

JUCE(C++) 앱

Visual Studio(Win) 타깃.

오디오 디바이스/플러그인 UI 등.

예: VSTFileUI.h – 리스트/패널로 구성된 파일 UI. 

acc1c0cf-5109-457d-9f08-2bb5ad7…

Rust 오디오 엔진 (DLL)

#[no_mangle] extern "C"로 C++에서 바로 호출.

링버퍼(rtrb) + 디코딩(symphonia) + 출력(cpal).

기본 버퍼 크기와 채널 수는 다음 상수로 정의:

CAPACITY_SAMPLES = 144_000 (약 1.5초 @ 48kHz, 스테레오 인터리브드 샘플 기준)

CHANNELS = 2

디코더/플레이아웃 배치 크기: FILL_FRAMES, CHUNK_FRAMES (성능/언더런 튜닝 포인트) 

5ea77f29-6134-415f-bf17-45f2f7c…

사전 준비(Windows)

Visual Studio 2019/2022 (Desktop development with C++)

Rust(Stable) + MSVC toolchain

rustup default stable-x86_64-pc-windows-msvc
rustup update


(선택) JUCE Projucer 로 VS 솔루션 생성/관리

아키텍처는 x64 통일이 필수입니다. Visual Studio 솔루션 구성(Release x64)과 Rust 타깃이 서로 맞아야 합니다.

Rust 엔진 빌드(DLL)

Rust 크레이트의 Cargo.toml에 아래가 들어 있어야 DLL이 생성됩니다.

[lib]
crate-type = ["cdylib"]


빌드:

cd <rust-crate-root>
cargo build --release


산출물(예시):

target\release\your_rust_engine.dll
target\release\your_rust_engine.lib   # VS 링커가 사용하는 import lib

C++ ↔ Rust FFI 헤더

JUCE C++ 코드에서 Rust 함수를 쓰려면, 아래처럼 간단한 헤더를 만들어 포함합니다(프로토타입은 Rust 쪽 #[no_mangle] extern "C"와 일치해야 함).

// rust_audio.h
#pragma once
#include <cstdint>

extern "C" {
    struct TrackConfig;
    struct Engine;

    TrackConfig* rust_audio_track_new(int32_t number);
    void         rust_audio_track_free(TrackConfig* tk);

    Engine* rust_audio_engine_new(TrackConfig* t0, TrackConfig* t1,
                                  TrackConfig* t2, TrackConfig* t3);
    void    rust_audio_engine_free(Engine* e);
    // 필요 시 다른 extern "C" API도 여기에 추가
}


Rust 파일에는 #[no_mangle] extern "C"로 rust_audio_track_new, rust_audio_engine_new 등이 이미 정의되어 있습니다. 

5ea77f29-6134-415f-bf17-45f2f7c…

Visual Studio 설정 (JUCE 프로젝트)
1) 링커 경로/라이브러리

프로젝트 속성 → 구성(Release x64) 고른 상태

Linker → General → Additional Library Directories

<repo>\rust\<crate>\target\release


Linker → Input → Additional Dependencies

your_rust_engine.lib


.lib는 DLL의 import 라이브러리입니다. 링커는 .lib로 심볼을 해결하고, 실행 시점에는 .dll이 로딩됩니다.