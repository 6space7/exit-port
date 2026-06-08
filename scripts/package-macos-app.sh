#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERSION="$(cargo metadata --no-deps --format-version 1 --manifest-path "$ROOT_DIR/Cargo.toml" | sed -n 's/.*"version":"\([^"]*\)".*/\1/p')"
ARCH="$(uname -m)"

if [[ "$ARCH" != "arm64" ]]; then
  echo "This release packaging script currently builds the macOS arm64 cask asset." >&2
  exit 1
fi

APP_NAME="End Port"
BUNDLE_ID="com.6space7.end-port"
DIST_DIR="$ROOT_DIR/target/dist"
APP_DIR="$DIST_DIR/$APP_NAME.app"
CONTENTS_DIR="$APP_DIR/Contents"
MACOS_DIR="$CONTENTS_DIR/MacOS"
RESOURCES_DIR="$CONTENTS_DIR/Resources"
ZIP_PATH="$DIST_DIR/End-Port-$VERSION-macos-arm64.zip"

cargo build --release --manifest-path "$ROOT_DIR/Cargo.toml"

rm -rf "$APP_DIR" "$ZIP_PATH"
mkdir -p "$MACOS_DIR" "$RESOURCES_DIR"

cp "$ROOT_DIR/target/release/end-port" "$MACOS_DIR/end-port"
chmod 755 "$MACOS_DIR/end-port"

cat > "$CONTENTS_DIR/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "https://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDevelopmentRegion</key>
  <string>en</string>
  <key>CFBundleDisplayName</key>
  <string>$APP_NAME</string>
  <key>CFBundleExecutable</key>
  <string>end-port</string>
  <key>CFBundleIconFile</key>
  <string>AppIcon</string>
  <key>CFBundleIdentifier</key>
  <string>$BUNDLE_ID</string>
  <key>CFBundleInfoDictionaryVersion</key>
  <string>6.0</string>
  <key>CFBundleName</key>
  <string>$APP_NAME</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleShortVersionString</key>
  <string>$VERSION</string>
  <key>CFBundleVersion</key>
  <string>$VERSION</string>
  <key>LSApplicationCategoryType</key>
  <string>public.app-category.developer-tools</string>
  <key>LSMinimumSystemVersion</key>
  <string>11.0</string>
  <key>LSUIElement</key>
  <true/>
  <key>NSHighResolutionCapable</key>
  <true/>
</dict>
</plist>
PLIST

if command -v node >/dev/null 2>&1; then
  ICONSET="$DIST_DIR/AppIcon.iconset"
  rm -rf "$ICONSET"
  mkdir -p "$ICONSET"

  node "$ROOT_DIR/scripts/render-icon-png.mjs" "$ICONSET"
  iconutil -c icns "$ICONSET" -o "$RESOURCES_DIR/AppIcon.icns"
  rm -rf "$ICONSET"
else
  echo "Node.js not found; packaging without a custom app icon." >&2
fi

plutil -lint "$CONTENTS_DIR/Info.plist" >/dev/null
codesign --force --sign - "$APP_DIR" >/dev/null
codesign --verify --deep --strict "$APP_DIR"

(
  cd "$DIST_DIR"
  COPYFILE_DISABLE=1 /usr/bin/ditto -c -k --norsrc --noqtn --keepParent "$APP_NAME.app" "$ZIP_PATH"
)

echo "$APP_DIR"
echo "$ZIP_PATH"
