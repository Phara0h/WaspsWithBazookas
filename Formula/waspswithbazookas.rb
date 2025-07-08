class Waspswithbazookas < Formula
  desc "Distributed HTTP/S load testing tool with cluster-based architecture"
  homepage "https://github.com/Phara0h/WaspsWithBazookas"
  url "https://github.com/Phara0h/WaspsWithBazookas/archive/refs/tags/v2.0.1.tar.gz"
  sha256 "REPLACE_ME_WITH_SHA256"
  license "GPL-2.0-or-later"

  depends_on "rust" => :build

  def install
    # Build all binaries from source
    system "cargo", "build", "--release", "--bin", "hive", "--bin", "wasp", "--bin", "test-dummy"
    
    # Install the binaries
    bin.install "target/release/hive"
    bin.install "target/release/wasp"
    bin.install "target/release/test-dummy"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/hive --version")
    assert_match version.to_s, shell_output("#{bin}/wasp --version")
    assert_match version.to_s, shell_output("#{bin}/test-dummy --version")
  end
end
