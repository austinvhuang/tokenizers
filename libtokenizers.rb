class Libtokenizers < Formula
  desc "libtokenizers for haskell-bindings"
  homepage "https://github.com/hasktorch/tokenizers"
  url "https://github.com/hasktorch/tokenizers/releases/download/libtokenizers-v0.1/libtokenizers-macos-arm64.zip"
  sha256 "83e0f74daea43e5cbda80b32edacfed6ce9bec1037fc7fe413690e332777626d"
  license "Apache-2.0"

  def install
    system "bash", "-c", "install -d #{lib}"
    system "bash", "-c", "cp -a lib/* #{lib}"
  end
end
