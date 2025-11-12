# Gradle Build Profiler

Gradle 빌드 프로파일을 분석하여 빌드 성능 개선 인사이트를 제공하는 Rust 기반 CLI 도구입니다.

## 기능

- Gradle 빌드 프로파일 HTML 파일 자동 분석
- 가장 느린 태스크 Top 10 식별
- 프로젝트별 빌드 시간 요약
- 빌드 최적화 팁 자동 제공
- 컬러풀한 터미널 출력 및 테이블 형식 리포트

## 설치

### Homebrew (macOS 추천)

```bash
brew tap Chaebin-Park/tap
brew install gradle-build-profiler
```

### Cargo (모든 플랫폼)

```bash
cargo install gradle_build_profiler
```

### 바이너리 다운로드

[GitHub Releases](https://github.com/Chaebin-Park/GradleBuildProfiler/releases)에서 플랫폼에 맞는 바이너리를 다운로드할 수 있습니다.

### 소스에서 빌드

```bash
git clone https://github.com/Chaebin-Park/GradleBuildProfiler
cd gradle_build_profiler
cargo build --release
```

빌드된 바이너리는 `target/release/gradle_build_profiler`에 생성됩니다.

## 사용법

### 1. Gradle 빌드 프로파일 생성

먼저 Android 프로젝트에서 `--profile` 플래그를 사용하여 빌드를 실행합니다:

```bash
cd <your-android-project>
./gradlew build --profile
```

이 명령은 `build/reports/profile/` 디렉토리에 HTML 프로파일 파일을 생성합니다.

### 2. 프로파일 분석

#### 최신 프로파일 자동 분석

```bash
gradle_build_profiler analyze --project /path/to/android/project
```

현재 디렉토리가 Android 프로젝트인 경우:

```bash
gradle_build_profiler analyze
```

#### 특정 프로파일 파일 분석

```bash
gradle_build_profiler analyze --file /path/to/profile-2024-01-01-12-34-56.html
```

### 명령어 옵션

```
analyze [OPTIONS]

Options:
  -p, --project <PROJECT>  Android 프로젝트 경로 (기본값: 현재 디렉토리)
  -f, --file <FILE>        특정 프로파일 파일 경로
  -h, --help               도움말 표시
```

## 출력 예시

```
📂 Reading profile: build/reports/profile/profile-2024-01-01-12-34-56.html

📊 Gradle Build Profile Analysis
============================================================

Build Summary
Total build time: 2m 15s

🐌 Top 10 Slowest Tasks
╭──────────────────────────────────────────────────┬──────────┬────────────╮
│ Task                                             │ Duration │ % of Total │
├──────────────────────────────────────────────────┼──────────┼────────────┤
│ :app:kaptDebugKotlin                             │ 45.2s    │ 33.5%      │
│ :app:compileDebugKotlin                          │ 32.1s    │ 23.8%      │
│ :data:compileDebugKotlin                         │ 18.5s    │ 13.7%      │
│ :app:dexBuilderDebug                             │ 12.3s    │ 9.1%       │
│ :domain:compileDebugKotlin                       │ 8.7s     │ 6.4%       │
╰──────────────────────────────────────────────────┴──────────┴────────────╯

📦 Project Summary
  :app - 1m 28s (24 tasks)
  :data - 25s (18 tasks)
  :domain - 12s (15 tasks)

💡 Optimization Tips
  • Kapt takes 33.5% of build time. Consider migrating to KSP for ~40% improvement
  • Module ':app' has slow Kotlin compilation. Consider splitting into smaller modules
  • Enable Gradle parallel execution: org.gradle.parallel=true
  • Enable configuration cache: org.gradle.configuration-cache=true
```

## 프로젝트 구조

```
src/
├── main.rs       # CLI 진입점
├── parser.rs     # HTML 프로파일 파싱
├── analyzer.rs   # 빌드 데이터 분석
├── report.rs     # 리포트 생성 및 출력
├── models.rs     # 데이터 모델
└── lib.rs        # 라이브러리 인터페이스
```

## 주요 분석 항목

### 1. 가장 느린 태스크 (Top 10)
빌드 시간이 가장 오래 걸리는 태스크들을 식별하여, 어느 부분을 최적화해야 하는지 우선순위를 제공합니다.

### 2. 프로젝트별 요약
각 Gradle 모듈별로 총 빌드 시간과 태스크 개수를 표시하여, 어느 모듈이 빌드 시간에 가장 큰 영향을 미치는지 파악합니다.

### 3. 최적화 팁
- **Kapt 사용 검사**: Kapt 빌드 시간이 전체의 20%를 초과하면 KSP 마이그레이션을 제안
- **컴파일 시간 검사**: Kotlin 컴파일이 30초를 초과하는 모듈에 대해 모듈 분리를 제안
- **Gradle 설정 제안**: 병렬 실행 및 구성 캐시 활성화 권장

## 의존성

주요 라이브러리:
- `clap` - CLI 인터페이스 구현
- `regex` - HTML 파싱을 위한 정규표현식
- `colored` - 터미널 컬러 출력
- `tabled` - 테이블 형식 리포트
- `walkdir` - 디렉토리 탐색
- `serde` / `serde_json` - JSON 직렬화/역직렬화
- `anyhow` / `thiserror` - 에러 처리

## 제한사항

- 현재 HTML 형식의 Gradle 프로파일만 지원합니다
- Android 프로젝트를 주요 대상으로 개발되었지만, 다른 Gradle 프로젝트에서도 사용 가능합니다

## 라이선스

MIT

## 기여

이슈 및 PR은 언제든 환영합니다!
