cask "end-port" do
  version "0.3.0"
  sha256 "APP_SHA_PLACEHOLDER"

  url "https://github.com/6space7/end-port/releases/download/v#{version}/End-Port-#{version}-macos-arm64.zip",
      verified: "github.com/6space7/end-port/"
  name "End Port"
  desc "Menu bar utility for ending local development ports"
  homepage "https://github.com/6space7/end-port"

  depends_on arch: :arm64
  depends_on macos: ">= :big_sur"

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
