# Ryuichi

## 프로젝트 소개

**Ryuichi**는 JUCE(C++) 기반의 데스크탑 음악/사운드 편집 애플리케이션입니다.  
Rust로 구현된 사운드 처리 모듈과 연동하여 고성능 오디오 기능을 제공합니다.

---

## 주요 기능

- 멀티트랙 오디오 편집 및 믹싱
- 사운드 파일 및 VST 플러그인 관리(VST 플러그인 관리 개발 예정)
- 다양한 UI 컴포넌트(버튼, 슬라이더, 토글 등)
- Rust로 구현된 오디오 엔진 연동(FFI)
- 커스텀 LookAndFeel 및 이미지 리소스 활용

---

## 폴더 구조

```
Source/           # C++ JUCE 기반 메인 소스코드
RustSource/       # Rust 오디오 엔진 및 FFI 모듈
UI_Image/         # UI에 사용되는 이미지 리소스
Sound_Files/      # 샘플 사운드 파일
VST_Files/        # VST 플러그인 파일(비어있을 수 있음)
Builds/           # Visual Studio 프로젝트 및 빌드 산출물
JuceLibraryCode/  # JUCE 라이브러리 코드
```

---

## 빌드 및 실행 방법

### 1. C++ (JUCE) 빌드

- **필수:** Visual Studio 2022, JUCE 7.x 이상
- `Builds/VisualStudio2022/Ryuichi.sln` 파일을 열고 빌드

### 2. Rust 오디오 엔진 빌드

- **필수:** Rust toolchain (`cargo` 명령어 필요)
- `RustSource/ryuichi/` 폴더에서 아래 명령 실행
    ```
    cargo build --release
    ```
- 빌드 결과(`ryuichi.dll`)가 C++ 프로젝트에서 FFI로 사용됨

### 3. 실행

- 빌드 후 `Builds/x64/Debug/App/Ryuichi.exe` 실행
- UI에서 사운드 파일, VST 플러그인, 각종 컨트롤 사용 가능

---

## 주의사항

- 대용량 리소스(사운드, 이미지, 빌드 산출물)는 버전관리에서 제외(.gitignore 참고)
- Rust/C++ FFI 연동 시 빌드 경로 및 DLL 위치에 주의
- JUCE 라이선스 정책을 준수해야 합니다

---

## 라이선스

MIT License  
(상세 내용은 LICENSE 파일 참고)

---

## 문의

- 이슈 등록 또는 Pull Request로 문의/기여 바랍니다.
