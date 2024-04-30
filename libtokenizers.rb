class Libtokenizers < Formula
  desc "libtokenizers for haskell-bindings"
  homepage "https://github.com/hasktorch/tokenizers"
  url "https://github.com/hasktorch/tokenizers/releases/download/libtokenizers-v0.1/libtokenizers-macos-arm64.zip"
  sha256 "4700b99f07b5cc7c11d8d441d4070639222db9db729221afe38286ad8bcc6b98"
  license "Apache-2.0"

  def install
    system "bash", "-c", "install -d #{lib}"
    system "bash", "-c", "cp -a lib/* #{lib}"
  end
end
