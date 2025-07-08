class Waspswithbazookas < Formula
  desc "Distributed load testing tool - like bees with machine guns, but way more power!"
  homepage "https://github.com/Phara0h/WaspsWithBazookas"
  version "2.0.0"
  license "GPL-2.0"
  
  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/Phara0h/WaspsWithBazookas/releases/download/v#{version}/waspswithbazookas-macos-aarch64.tar.gz"
      sha256 "PLACEHOLDER_SHA256" # This will be updated by the release process
    else
      url "https://github.com/Phara0h/WaspsWithBazookas/releases/download/v#{version}/waspswithbazookas-macos-x86_64.tar.gz"
      sha256 "PLACEHOLDER_SHA256" # This will be updated by the release process
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/Phara0h/WaspsWithBazookas/releases/download/v#{version}/waspswithbazookas-linux-aarch64.tar.gz"
      sha256 "PLACEHOLDER_SHA256" # This will be updated by the release process
    else
      url "https://github.com/Phara0h/WaspsWithBazookas/releases/download/v#{version}/waspswithbazookas-linux-x86_64.tar.gz"
      sha256 "PLACEHOLDER_SHA256" # This will be updated by the release process
    end
  end

  def install
    bin.install "hive"
    bin.install "wasp"
    bin.install "test-dummy"
  end

  test do
    system "#{bin}/hive", "--version"
    system "#{bin}/wasp", "--version"
    system "#{bin}/test-dummy", "--version"
  end
end 