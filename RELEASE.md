# 새 버전 릴리스 가이드

프로젝트를 업데이트하고 새 버전을 릴리스하는 과정입니다.

## 📋 체크리스트

- [ ] 코드 변경 및 테스트 완료
- [ ] 버전 업데이트
- [ ] 변경사항 커밋 및 태그 생성
- [ ] GitHub Actions 완료 대기
- [ ] SHA256 계산
- [ ] Homebrew Formula 업데이트
- [ ] 사용자에게 업데이트 안내

---

## 1단계: 버전 업데이트

`Cargo.toml` 파일에서 버전을 업데이트합니다:

```toml
[package]
version = "0.1.2"  # 예: 0.1.1 → 0.1.2
```

### 버전 규칙 (Semantic Versioning)

```
MAJOR.MINOR.PATCH
  ↓     ↓     ↓
  1  .  2  .  3
```

- **MAJOR**: 호환되지 않는 API 변경
- **MINOR**: 하위 호환되는 기능 추가
- **PATCH**: 하위 호환되는 버그 수정

예시:
- 버그 수정: `0.1.1` → `0.1.2`
- 새 기능: `0.1.2` → `0.2.0`
- 대규모 변경: `0.2.0` → `1.0.0`

---

## 2단계: 변경사항 커밋 및 태그 생성

```bash
# 1. 로컬 빌드 테스트
cargo build --release
./target/release/gradle-build-profiler --help

# 2. 변경사항 커밋
git add .
git commit -m "Release v0.1.2: <변경 내용 요약>"

# 3. 태그 생성 (버전 번호 확인!)
git tag v0.1.2

# 4. 푸시 (GitHub Actions 자동 실행)
git push origin master
git push origin v0.1.2
```

---

## 3단계: GitHub Actions 완료 대기

1. GitHub 저장소 접속: https://github.com/Chaebin-Park/GradleBuildProfiler
2. **Actions** 탭 클릭
3. 최신 워크플로우 실행 확인
4. 모든 작업이 ✅ 완료될 때까지 대기 (약 5-10분)

완료되면:
- **Releases** 탭에 새 릴리스 생성됨
- 각 플랫폼별 바이너리가 업로드됨

---

## 4단계: SHA256 계산

릴리스가 완료되면 소스 코드의 SHA256을 계산합니다:

```bash
# 버전 번호를 실제 버전으로 변경!
curl -sL https://github.com/Chaebin-Park/GradleBuildProfiler/archive/refs/tags/v0.1.2.tar.gz | shasum -a 256
```

출력 예시:
```
abc123def456789... -
```

앞부분 해시 값을 복사해두세요: `abc123def456789...`

---

## 5단계: Homebrew Formula 업데이트

### 5-1. homebrew-tap 레포지토리로 이동

```bash
cd ~/Developments/homebrew-tap
```

### 5-2. Formula 파일 수정

`Formula/gradle-build-profiler.rb` 파일을 열어서 다음 두 줄만 수정:

```ruby
class GradleBuildProfiler < Formula
  desc "Analyze Gradle build profiles and provide performance insights"
  homepage "https://github.com/Chaebin-Park/GradleBuildProfiler"
  url "https://github.com/Chaebin-Park/GradleBuildProfiler/archive/refs/tags/v0.1.2.tar.gz"  # ← 버전 변경
  sha256 "abc123def456789..."  # ← 4단계에서 계산한 SHA256으로 변경
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
    bin.install_symlink "gradle-build-profiler" => "gbp"
  end

  test do
    assert_match "gradle-profiler", shell_output("#{bin}/gradle-build-profiler --help")
    assert_match "gradle-profiler", shell_output("#{bin}/gbp --help")
  end
end
```

**변경할 내용:**
1. `url` 라인: 버전 번호 변경 (예: `v0.1.1` → `v0.1.2`)
2. `sha256` 라인: 4단계에서 계산한 값으로 변경

### 5-3. 커밋 및 푸시

```bash
git add Formula/gradle-build-profiler.rb
git commit -m "Update gradle-build-profiler to v0.1.2"
git push origin main
```

---

## 6단계: 사용자 업데이트 확인

사용자들이 업데이트하는 방법:

```bash
brew update
brew upgrade gradle-build-profiler
```

직접 테스트:

```bash
# 새 터미널에서
brew update
brew upgrade gradle-build-profiler
gradle-build-profiler --version  # 또는 gbp --version
```

---

## 🚀 빠른 참조 (Quick Reference)

```bash
# 1. 버전 업데이트 (Cargo.toml)
vim Cargo.toml  # version = "0.1.x"

# 2. 커밋 및 태그
git add .
git commit -m "Release v0.1.x"
git tag v0.1.x
git push origin master && git push origin v0.1.x

# 3. SHA256 계산 (GitHub Actions 완료 후)
curl -sL https://github.com/Chaebin-Park/GradleBuildProfiler/archive/refs/tags/v0.1.x.tar.gz | shasum -a 256

# 4. Formula 업데이트
cd ~/Developments/homebrew-tap
vim Formula/gradle-build-profiler.rb  # url, sha256 수정
git add Formula/gradle-build-profiler.rb
git commit -m "Update gradle-build-profiler to v0.1.x"
git push origin main

# 5. 테스트
brew update && brew upgrade gradle-build-profiler
```

---

## ⚠️ 주의사항

1. **버전 번호 일치**: Cargo.toml, git tag, Formula의 버전이 모두 일치해야 합니다
2. **GitHub Actions 대기**: 태그 푸시 후 반드시 Actions 완료를 확인하세요
3. **SHA256 정확성**: SHA256은 정확해야 합니다. 틀리면 설치 실패합니다
4. **테스트**: Formula 푸시 후 실제로 `brew upgrade`로 테스트하세요

---

## 🔧 문제 해결

### GitHub Actions 실패

- Actions 탭에서 로그 확인
- 일반적인 원인: Cargo.toml 문법 오류, 빌드 실패

### SHA256 불일치 오류

```
Error: SHA256 mismatch
Expected: abc123...
  Actual: def456...
```

**해결 방법**: SHA256을 다시 계산하고 Formula 업데이트

```bash
curl -sL https://github.com/Chaebin-Park/GradleBuildProfiler/archive/refs/tags/v0.1.x.tar.gz | shasum -a 256
```

### 설치 실패

```bash
# Homebrew 캐시 정리
brew cleanup gradle-build-profiler

# 재설치
brew uninstall gradle-build-profiler
brew install gradle-build-profiler
```

---

## 📝 릴리스 노트 작성 (선택사항)

GitHub Release 페이지에서 릴리스 노트를 작성하면 더 전문적입니다:

1. https://github.com/Chaebin-Park/GradleBuildProfiler/releases
2. 해당 릴리스 "Edit" 클릭
3. 변경사항 작성:

```markdown
## 🎉 v0.1.2

### ✨ 새로운 기능
- 새 기능 1
- 새 기능 2

### 🐛 버그 수정
- 버그 수정 1
- 버그 수정 2

### 📝 기타 변경사항
- 문서 업데이트
- 성능 개선
```

---

## 🎯 요약

1. **Cargo.toml** 버전 업데이트
2. **Git 태그** 생성 및 푸시
3. **GitHub Actions** 완료 대기
4. **SHA256** 계산
5. **Formula** 업데이트 (url, sha256)
6. **테스트** 및 확인

이 과정은 약 **10-15분** 정도 소요됩니다.
