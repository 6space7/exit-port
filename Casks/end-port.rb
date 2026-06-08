cask "end-port" do
  version "0.3.0"
  sha256 "4674f597b9997a35b2ab8554e1f45401800905122edbc0eb7eb9f434d8ea3b27"

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

  caveats <<~EOS
    End Port is ad-hoc signed. If macOS blocks the first launch, Control-click
    /Applications/End Port.app and choose Open.
  EOS
end
