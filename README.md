Ryuichi DAW — JUCE × Rust (FFI)

JUCE 기반 C++ UI와 Rust 오디오 엔진(DLL) 을 FFI로 연결한 실험적 DAW 프로젝트입니다.
Rust 쪽에서 디코딩/리샘플/믹싱/출력(cpal)을 담당하고, JUCE(C++) 쪽에서 파일/타임라인 등 UI를 제공합니다.

특징

C++ ↔ Rust 직접 FFI (#[no_mangle] extern "C")

rtrb(lock-free ring buffer)로 트랙별 오디오 파이프라인 구성

symphonia로 오디오 디코딩, cpal로 출력

간단한 볼륨/뮤트/팬 파라미터, 타임라인/클립 구조(진행 중)

폴더 구성 (요약)

JUCE/ — C++ 앱(UI)

rust/ — Rust 오디오 엔진(Cargo 크레이트, DLL 생성)

include/ — JUCE에서 포함하는 FFI 헤더(예: rust_audio.h)

사전 준비 (Windows)

Visual Studio 2019/2022 (Desktop development with C++)

Rust (stable) + MSVC toolchain

rustup default stable-x86_64-pc-windows-msvc
rustup update


(선택) Projucer로 JUCE 솔루션 생성/관리

중요: 아키텍처는 x64 통일 필수입니다.
Visual Studio 구성(Release x64)과 Rust 타깃이 일치해야 합니다.

Rust 엔진 빌드(DLL)

Cargo.toml 설정:

[lib]
crate-type = ["cdylib"]


빌드:

cd rust\your-crate
cargo build --release


산출물(예):

rust\your-crate\target\release\your_rust_engine.dll
rust\your-crate\target\release\your_rust_engine.lib   # VS 링커용 import lib

C++ ↔ Rust FFI 헤더

JUCE 프로젝트에서 Rust 함수를 사용하려면 아래와 같은 헤더를 포함합니다:

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

    // 필요한 다른 extern "C" API를 여기에 추가
}


Rust 쪽에는 동일 시그니처로 #[no_mangle] extern "C" 함수들이 구현되어 있어야 합니다.

Visual Studio 설정 (JUCE 프로젝트)

구성: Release | x64

1) 링커 설정

Linker → General → Additional Library Directories

<repo>\rust\your-crate\target\release


Linker → Input → Additional Dependencies

your_rust_engine.lib


.lib는 DLL의 import 라이브러리입니다.
빌드 시 링커가 심볼을 해결하고, 실행 시 실제로는 .dll이 로드됩니다.

2) C/C++ 설정 (권장)

C/C++ → General → Additional Include Directories

<repo>\include        // rust_audio.h 등이 위치한 곳

3) 실행 파일 옆에 DLL 배치

반드시 JUCE 실행 폴더($(OutDir))에 your_rust_engine.dll이 있어야 로드됩니다.

간단한 방법:

빌드 후 이벤트로 DLL 복사:

Build Events → Post-Build Event → Command Line

xcopy /Y /D "<repo>\rust\your-crate\target\release\your_rust_engine.dll" "$(OutDir)"


혹은 JUCE 프로젝트의 BinaryData/Installer 스텝에서 배포에 포함.

런타임/튜닝 포인트

엔진 코드에는 다음과 같은 상수/크기가 등장합니다:

CAPACITY_SAMPLES = 144_000
: 링버퍼 용량(샘플 단위). 48kHz 스테레오 기준 약 1.5초 버퍼 여유입니다.

CHANNELS = 2
: 스테레오(인터리브드).

FILL_FRAMES, CHUNK_FRAMES
: 디코딩/플레이아웃 시 한 번에 처리하는 프레임 묶음 크기입니다.
: 언더런/지연/CPU의 균형을 맞추는 핵심 튜닝 포인트입니다.

Tip
언더런이 보이면

FILL_FRAMES/CHUNK_FRAMES를 늘려 더 크게 채우거나,

CAPACITY_SAMPLES를 키워 버퍼 여유를 늘려보세요.
반대로 지연이 체감되면 조금씩 줄이며 균형을 맞춥니다.

빠른 체크리스트

 VS 구성: Release | x64

 Rust 빌드: cargo build --release (MSVC toolchain)

 링커: .lib 경로/파일 추가 완료

 실행 폴더에 .dll 배치 완료

 FFI 헤더 포함 및 시그니처 일치 확인
