<h1 align="center">Ryuichi DAW — JUCE × Rust (FFI)</h1>

<p align="center">
  <em>JUCE 기반 C++ UI + Rust 오디오 엔진(DLL) — 디코딩 · 리샘플 · 믹싱 · 출력(cpal)</em><br/>
  <sub>Lock-free ring buffer(rtrb), Symphonia 디코더, CPAL 오디오 출력</sub>
</p>

<p align="center">
  <a href="https://www.rust-lang.org/"><img alt="Rust" src="https://img.shields.io/badge/Rust-stable-blue?logo=rust" /></a>
  <a href="https://juce.com/"><img alt="JUCE" src="https://img.shields.io/badge/JUCE-C%2B%2B-8A2BE2" /></a>
  <img alt="Platform" src="https://img.shields.io/badge/Platform-Windows%20x64-black" />
  <img alt="Audio" src="https://img.shields.io/badge/Audio-48kHz%20stereo-1abc9c" />
</p>

<hr/>

## ✨ 특징
- C++ ↔ Rust **직접 FFI** (`#[no_mangle] extern "C"`)
- **rtrb**(lock-free ring buffer)로 트랙별 파이프라인
- **symphonia**로 디코딩, **cpal**로 출력
- 볼륨/뮤트/팬 파라미터, 타임라인/클립 구조(개발 중)
- 언더런 튜닝을 위한 **프레임 묶음 크기(FILL_FRAMES / CHUNK_FRAMES)** 및 **버퍼 용량(CAPACITY_SAMPLES)** 노출

---

## 🗂️ 폴더 구성
