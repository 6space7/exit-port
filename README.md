# End Port

End Port is a tiny native tray/menu-bar utility for stopping stuck local web development ports.

It scans listening TCP sockets with OS APIs, filters for likely local web dev servers, shows them in the system tray, and lets you stop the owning process from the menu.

## Features

- Native tray/menu-bar app for macOS, Windows, and Linux.
- Lists likely web dev listeners such as Node, Vite, Next, Python, Bun, Deno, Rails, PHP, Go, and Java.
- Avoids obvious infrastructure services such as Postgres, Redis, MongoDB, MySQL, SSH, and Docker.
- Ignores End Port itself and unknown high ephemeral listeners so the app does not offer to stop its own helper ports.
- Stops a port by terminating its owning process from a compact tray menu.
- Auto-refreshes every 5 seconds and includes a manual Refresh menu item.
- Checks GitHub releases for updates from the menu bar and CLI, then points you to the Homebrew upgrade command.
- Includes CLI commands for quick inspection and scripting.

## Install

Install the macOS app into Applications:

```sh
brew tap 6space7/end-port https://github.com/6space7/end-port
brew trust 6space7/end-port
brew install --cask end-port
```

Then open **End Port.app** from Applications. It runs as a menu bar app, so it appears in the menu bar rather than the Dock. Release builds are Developer ID signed and notarized on macOS.

## Updates

End Port checks the latest public GitHub release when the menu-bar app starts. You can also choose **Check for Updates** from the menu or run:

```sh
end-port --check-updates
```

Because Homebrew owns the installed app bundle, End Port does not silently replace itself. When an update is available, update the cask with:

```sh
brew upgrade --cask end-port
```

## CLI

For the command-line binary instead:

```sh
brew install end-port
```

## Run from Source

```sh
cargo run
```

The default command starts the tray/menu-bar utility.

```sh
cargo run -- --list
cargo run -- --stop-pid 12345
cargo run -- --check-updates
```

## Build

```sh
cargo build --release
```

The release binary will be at `target/release/end-port`.

Build the macOS `.app` cask asset:

```sh
./scripts/package-macos-app.sh
```

On macOS, the packaging script uses an installed Developer ID Application identity when available. To notarize a release build, pass a signing environment file with Apple notarization credentials:

```sh
END_PORT_SIGNING_ENV_FILE=/path/to/.env.mac-signing ./scripts/package-macos-app.sh
```

The generated cask zip is written to `target/dist/`.

## Homebrew Tap

The Homebrew cask and formula live in this same repository at `Casks/end-port.rb` and `Formula/end-port.rb`. Because this is not a `homebrew-*` tap repo, use the explicit tap URL shown above.

## Linux Notes

The tray backend uses GTK/AppIndicator through `tray-icon`. On Debian/Ubuntu, install the tray libraries before building:

```sh
sudo apt install libgtk-3-dev libayatana-appindicator3-dev
```

Some Linux desktops hide tray icons unless an AppIndicator/status-notifier extension is enabled.
