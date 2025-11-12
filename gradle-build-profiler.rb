# Homebrew Formula for gradle-build-profiler
#
# 사용법:
# 1. GitHub에서 homebrew-tap 레포지토리 생성 (예: https://github.com/Chaebin-Park/homebrew-tap)
# 2. 이 파일을 Formula/gradle-build-profiler.rb로 복사
# 3. URL과 SHA256을 실제 릴리스 정보로 업데이트
#
# 설치:
# brew tap Chaebin-Park/tap
# brew install gradle-build-profiler

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
  elsif OS.linux?
    url "https://github.com/Chaebin-Park/GradleBuildProfiler/releases/download/v0.1.0/gradle_build_profiler-linux-x86_64"
    sha256 "YOUR_SHA256_HERE"
  end

  def install
    bin.install Dir["*"].first => "gradle-build-profiler"
  end

  test do
    system "#{bin}/gradle-build-profiler", "--help"
  end
end
