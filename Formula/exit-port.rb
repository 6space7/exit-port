class ExitPort < Formula
  desc "Tiny native tray utility for finding and stopping local development ports"
  homepage "https://github.com/6space7/exit-port"
  url "https://github.com/6space7/exit-port/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "a2ebb4a8ee2f4863b8265c80af35c89f3dd37dd118e10f5b2e13ba0393a747be"
  license any_of: ["MIT", "Apache-2.0"]
  head "https://github.com/6space7/exit-port.git", branch: "main"

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
    assert_match "Start the tray/menu-bar utility", shell_output("#{bin}/exit-port --help")
  end
end
