# Homebrew 배포 가이드

이 문서는 `gradle-build-profiler`를 Homebrew를 통해 배포하는 방법을 설명합니다.

## 사전 준비

### 1. GitHub Secrets 설정

GitHub 저장소의 Settings > Secrets and variables > Actions에서 다음 시크릿을 추가합니다:

- `CARGO_REGISTRY_TOKEN`: crates.io에서 발급받은 토큰
  - https://crates.io/settings/tokens 에서 생성

### 2. Homebrew Tap 레포지토리 생성

```bash
# 새 GitHub 레포지토리 생성: homebrew-tap
# 예: https://github.com/Chaebin-Park/homebrew-tap

# 로컬에 클론
git clone https://github.com/Chaebin-Park/homebrew-tap
cd homebrew-tap

# Formula 디렉토리 생성
mkdir Formula
```

## 릴리스 프로세스

### 1. 버전 업데이트

`Cargo.toml`에서 버전을 업데이트합니다:

```toml
[package]
version = "0.1.0"  # 새 버전으로 변경
```

### 2. Git 태그 생성 및 푸시

```bash
# 변경사항 커밋
git add Cargo.toml
git commit -m "Bump version to 0.1.0"

# 태그 생성
git tag v0.1.0

# 푸시 (GitHub Actions가 자동으로 실행됨)
git push origin master
git push origin v0.1.0
```

### 3. GitHub Actions 자동 실행

태그를 푸시하면 자동으로:
1. macOS, Linux, Windows용 바이너리 빌드
2. GitHub Release 생성
3. 바이너리를 Release에 업로드
4. crates.io에 자동 배포

### 4. SHA256 계산

릴리스가 완료되면 소스 코드의 SHA256을 계산합니다:

```bash
# 릴리스 tarball 다운로드
curl -L https://github.com/Chaebin-Park/GradleBuildProfiler/archive/refs/tags/v0.1.0.tar.gz -o v0.1.0.tar.gz

# SHA256 계산
shasum -a 256 v0.1.0.tar.gz
```

### 5. Homebrew Formula 업데이트

`Formula-source-build.rb` 파일을 복사하여 homebrew-tap 레포지토리에 추가합니다:

```bash
# homebrew-tap 레포지토리에서
cp /path/to/gradle_build_profiler/Formula-source-build.rb Formula/gradle-build-profiler.rb

# SHA256 및 버전 업데이트
vim Formula/gradle-build-profiler.rb
```

`gradle-build-profiler.rb` 파일 수정:

```ruby
class GradleBuildProfiler < Formula
  desc "Analyze Gradle build profiles and provide performance insights"
  homepage "https://github.com/Chaebin-Park/GradleBuildProfiler"
  url "https://github.com/Chaebin-Park/GradleBuildProfiler/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "YOUR_CALCULATED_SHA256_HERE"  # 위에서 계산한 값으로 변경
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    assert_match "gradle-profiler", shell_output("#{bin}/gradle_build_profiler --help")
  end
end
```

### 6. Formula 커밋 및 푸시

```bash
git add Formula/gradle-build-profiler.rb
git commit -m "Update gradle-build-profiler to v0.1.0"
git push origin main
```

## 사용자 설치 테스트

```bash
# Tap 추가
brew tap Chaebin-Park/tap

# 설치
brew install gradle-build-profiler

# 테스트
gradle-build-profiler --help
```

## 바이너리 기반 Formula (선택사항)

바이너리를 직접 배포하려면 `gradle-build-profiler.rb` 파일을 사용합니다:

```ruby
class GradleBuildProfiler < Formula
  desc "Analyze Gradle build profiles and provide performance insights"
  homepage "https://github.com/Chaebin-Park/GradleBuildProfiler"
  version "0.1.0"

  if OS.mac? && Hardware::CPU.intel?
    url "https://github.com/Chaebin-Park/GradleBuildProfiler/releases/download/v0.1.0/gradle_build_profiler-macos-x86_64"
    sha256 "YOUR_SHA256_HERE"
  elsif OS.mac? && Hardware::CPU.arm?
    url "https://github.com/Chaebin-Park/GradleBuildProfiler/releases/download/v0.1.0/gradle_build_profiler-macos-aarch64"
    sha256 "YOUR_SHA256_HERE"
  end

  def install
    bin.install Dir["*"].first => "gradle-build-profiler"
  end

  test do
    system "#{bin}/gradle-build-profiler", "--help"
  end
end
```

바이너리의 SHA256 계산:

```bash
# 각 플랫폼별 바이너리 다운로드 후
shasum -a 256 gradle_build_profiler-macos-x86_64
shasum -a 256 gradle_build_profiler-macos-aarch64
```

## 버전 업데이트 체크리스트

- [ ] `Cargo.toml`의 버전 업데이트
- [ ] 변경사항 커밋
- [ ] Git 태그 생성 (`v0.x.x`)
- [ ] 태그 푸시
- [ ] GitHub Actions 실행 확인
- [ ] Release 생성 확인
- [ ] SHA256 계산
- [ ] Homebrew Formula 업데이트
- [ ] Formula 푸시
- [ ] 설치 테스트

## 문제 해결

### GitHub Actions 실패

1. Actions 탭에서 로그 확인
2. CARGO_REGISTRY_TOKEN이 올바르게 설정되었는지 확인
3. Cargo.toml의 메타데이터가 완전한지 확인

### Homebrew 설치 실패

```bash
# Formula 문법 체크
brew audit --strict Formula/gradle-build-profiler.rb

# 로컬 테스트
brew install --build-from-source ./Formula/gradle-build-profiler.rb
```

### SHA256 불일치

- 다운로드한 파일이 손상되지 않았는지 확인
- 정확한 릴리스 URL을 사용하는지 확인
- 캐시를 지우고 다시 시도: `brew cleanup`

## 참고 자료

- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [GitHub Actions 문서](https://docs.github.com/en/actions)
- [crates.io 배포 가이드](https://doc.rust-lang.org/cargo/reference/publishing.html)
