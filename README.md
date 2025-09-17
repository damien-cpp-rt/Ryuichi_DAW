Ryuichi DAW (JUCE + Rust FFI)

JUCE 기반 C++ UI와 Rust 오디오 엔진을 FFI(DLL)로 연결한 DAW 실험 프로젝트입니다.
JUCE 쪽에는 파일 브라우저/리스트 등 UI 컴포넌트가 있고(예: VSTFileUI.h), Rust 쪽에는 디코딩・리샘플・믹싱・출력(cpal) 엔진이 들어 있습니다.

구성 개요
JUCE (C++) 앱

Visual Studio(Windows) 타깃

오디오 디바이스/플러그인 UI 등

Rust 오디오 엔진 (DLL)

#[no_mangle] extern "C" 로 C++에서 직접 호출

링버퍼(rtrb) + 디코딩(symphonia) + 출력(cpal)

기본 상수

CAPACITY_SAMPLES = 144_000 (약 1.5초 @ 48kHz, 스테레오 인터리브드 샘플 기준)

CHANNELS = 2

디코더/플레이아웃 처리 묶음 크기: FILL_FRAMES, CHUNK_FRAMES (성능/언더런 튜닝 포인트)

사전 준비(Windows)

Visual Studio 2019/2022 (Desktop development with C++ 워크로드)

Rust(Stable) + MSVC toolchain

rustup default stable-x86_64-pc-windows-msvc
rustup update


(선택) JUCE Projucer로 VS 솔루션 생성/관리

중요: 아키텍처는 x64 통일이 필수입니다. Visual Studio 구성(Release x64)과 Rust 타깃이 반드시 일치해야 합니다.

Rust 엔진 빌드(DLL)

Cargo.toml에 DLL 산출 설정:

[lib]
crate-type = ["cdylib"]


빌드:

cd <rust-crate-root>
cargo build --release


산출물(예시):

target\release\your_rust_engine.dll
target\release\your_rust_engine.lib   # VS 링커가 사용하는 import lib

C++ ↔ Rust FFI 헤더

JUCE C++ 코드에서 Rust 함수를 사용하려면, 아래 헤더를 포함합니다
(프로토타입은 Rust 쪽 #[no_mangle] extern "C" 시그니처와 일치해야 합니다).

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


Rust 코드에는 rust_audio_track_new, rust_audio_engine_new 등이 이미 #[no_mangle] extern "C"로 정의되어 있다고 가정합니다.

Visual Studio 설정 (JUCE 프로젝트)
1) 링커 경로/라이브러리

프로젝트 속성 → 구성(Release x64) 선택

Linker → General → Additional Library Directories

<repo>\rust\<crate>\target\release


Linker → Input → Additional Dependencies

your_rust_engine.lib


.lib 는 DLL의 import 라이브러리입니다. 빌드 시 링커가 .lib로 심볼을 해결하고, 실행 시점에 .dll이 로드됩니다.
실행 폴더($(OutDir))에 your_rust_engine.dll이 존재해야 합니다.
