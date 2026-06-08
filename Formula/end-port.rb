class EndPort < Formula
  desc "Tiny native tray utility for ending local development ports"
  homepage "https://github.com/6space7/end-port"
  url "https://github.com/6space7/end-port/archive/refs/tags/v0.3.0.tar.gz"
  sha256 "b114b5f91c06befe91f659e2fa04021cb51cf5cc358be28fd7218d709abe0cf0"
  license any_of: ["MIT", "Apache-2.0"]
  head "https://github.com/6space7/end-port.git", branch: "main"

  depends_on "rust" => :build

  on_linux do
    depends_on "pkgconf" => :build
    depends_on "gtk+3"
    depends_on "libayatana-appindicator"
  end

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "Start the tray/menu-bar utility", shell_output("#{bin}/end-port --help")
  end
end
