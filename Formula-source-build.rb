# Homebrew Formula (Source Build Version)
#
# 이 방식은 소스에서 빌드합니다. Homebrew의 권장 방식입니다.
#
# 사용법:
# 1. GitHub에서 homebrew-tap 레포지토리 생성
# 2. 이 파일을 Formula/gradle-build-profiler.rb로 복사
# 3. 릴리스 생성 후 URL과 SHA256 업데이트
#
# SHA256 계산:
# shasum -a 256 v0.1.0.tar.gz

class GradleBuildProfiler < Formula
  desc "Analyze Gradle build profiles and provide performance insights"
  homepage "https://github.com/Chaebin-Park/GradleBuildProfiler"
  url "https://github.com/Chaebin-Park/GradleBuildProfiler/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "YOUR_SHA256_HERE"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    assert_match "gradle-profiler", shell_output("#{bin}/gradle_build_profiler --help")
  end
end
