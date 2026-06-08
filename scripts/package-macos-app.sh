#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERSION="$(cargo metadata --no-deps --format-version 1 --manifest-path "$ROOT_DIR/Cargo.toml" | sed -n 's/.*"version":"\([^"]*\)".*/\1/p')"
ARCH="$(uname -m)"
SIGNING_ENV_FILE="${END_PORT_SIGNING_ENV_FILE:-${MAC_SIGNING_ENV_FILE:-}}"
SIGNING_IDENTITY="${END_PORT_SIGNING_IDENTITY:-${APPLE_SIGNING_IDENTITY:-}}"

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

log() {
  printf '[end-port-package] %s\n' "$1"
}

load_env_file() {
  local env_file="$1"
  [[ -n "$env_file" && -f "$env_file" ]] || return 0

  set -a
  # shellcheck disable=SC1090
  source "$env_file"
  set +a
  log "Loaded signing environment from $env_file."
}

resolve_signing_identity() {
  if [[ -n "$SIGNING_IDENTITY" ]]; then
    return 0
  fi

  command -v security >/dev/null 2>&1 || return 0

  SIGNING_IDENTITY="$(
    security find-identity -v -p codesigning \
      | awk -F\" '/Developer ID Application/ { print $2; exit }'
  )"
}

sign_app() {
  if [[ -n "$SIGNING_IDENTITY" ]]; then
    log "Signing app with $SIGNING_IDENTITY."
    codesign \
      --force \
      --timestamp \
      --options runtime \
      --sign "$SIGNING_IDENTITY" \
      "$APP_DIR" >/dev/null
    return
  fi

  log "No Developer ID signing identity found; using ad-hoc signing."
  codesign --force --sign - "$APP_DIR" >/dev/null
}

has_apple_id_notarization() {
  [[ -n "${APPLE_ID:-}" && -n "${APPLE_PASSWORD:-${APPLE_APP_SPECIFIC_PASSWORD:-}}" && -n "${APPLE_TEAM_ID:-}" ]]
}

has_api_key_notarization() {
  [[ -n "${APPLE_API_KEY:-}" && -n "${APPLE_API_KEY_PATH:-}" && -n "${APPLE_API_ISSUER:-}" ]]
}

notarize_app_if_available() {
  [[ -n "$SIGNING_IDENTITY" ]] || return 0
  command -v xcrun >/dev/null 2>&1 || return 0

  local notary_zip="$DIST_DIR/End-Port-$VERSION-notary.zip"
  rm -f "$notary_zip"
  (
    cd "$DIST_DIR"
    COPYFILE_DISABLE=1 /usr/bin/ditto -c -k --norsrc --noqtn --keepParent "$APP_NAME.app" "$notary_zip"
  )

  if has_apple_id_notarization; then
    log "Submitting app for notarization with Apple ID credentials."
    xcrun notarytool submit "$notary_zip" \
      --apple-id "$APPLE_ID" \
      --password "${APPLE_PASSWORD:-$APPLE_APP_SPECIFIC_PASSWORD}" \
      --team-id "$APPLE_TEAM_ID" \
      --wait
  elif has_api_key_notarization; then
    log "Submitting app for notarization with App Store Connect API credentials."
    xcrun notarytool submit "$notary_zip" \
      --key "$APPLE_API_KEY_PATH" \
      --key-id "$APPLE_API_KEY" \
      --issuer "$APPLE_API_ISSUER" \
      --wait
  else
    log "Notarization credentials not provided; signed app will not be notarized."
    rm -f "$notary_zip"
    return
  fi

  xcrun stapler staple "$APP_DIR"
  rm -f "$notary_zip"
}

load_env_file "$ROOT_DIR/.env.mac-signing"
load_env_file "$SIGNING_ENV_FILE"
SIGNING_IDENTITY="${END_PORT_SIGNING_IDENTITY:-${APPLE_SIGNING_IDENTITY:-$SIGNING_IDENTITY}}"
resolve_signing_identity

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
sign_app
notarize_app_if_available
codesign --verify --deep --strict "$APP_DIR"
spctl -a -vv --type execute "$APP_DIR" || true

(
  cd "$DIST_DIR"
  COPYFILE_DISABLE=1 /usr/bin/ditto -c -k --norsrc --noqtn --keepParent "$APP_NAME.app" "$ZIP_PATH"
)

echo "$APP_DIR"
echo "$ZIP_PATH"
