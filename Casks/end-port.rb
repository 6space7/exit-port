cask "end-port" do
  version "0.4.0"
  sha256 "9019b00f12c616d1c42f74ae20e7c4e3346859a6562fa58d2a8fb85a2c165847"

  url "https://github.com/6space7/end-port/releases/download/v#{version}/End-Port-#{version}-macos-arm64.zip"
  name "End Port"
  desc "Menu bar utility for ending local development ports"
  homepage "https://github.com/6space7/end-port"

  depends_on arch: :arm64
  depends_on macos: :big_sur

  app "End Port.app"

  uninstall quit: "com.6space7.end-port"

  zap trash: [
    "~/Library/Application Support/End Port",
    "~/Library/Caches/com.6space7.end-port",
    "~/Library/HTTPStorages/com.6space7.end-port",
    "~/Library/Preferences/com.6space7.end-port.plist",
    "~/Library/Saved Application State/com.6space7.end-port.savedState",
  ]
end
