# Exit Port

Exit Port is a tiny native tray/menu-bar utility for stopping stuck local web development ports.

It scans listening TCP sockets with OS APIs, filters for likely local web dev servers, shows them in the system tray, and lets you stop the owning process from the menu.

## Features

- Native tray/menu-bar app for macOS, Windows, and Linux.
- Lists likely web dev listeners such as Node, Vite, Next, Python, Bun, Deno, Rails, PHP, Go, and Java.
- Avoids obvious infrastructure services such as Postgres, Redis, MongoDB, MySQL, SSH, and Docker.
- Stops a port by terminating its owning process from a compact tray menu.
- Auto-refreshes every 5 seconds and includes a manual Refresh menu item.
- Includes CLI commands for quick inspection and scripting.

## Install

```sh
brew install 6space7/exit-port/exit-port
```

Then start the tray app:

```sh
exit-port
```

## Run from Source

```sh
cargo run
```

The default command starts the tray/menu-bar utility.

```sh
cargo run -- --list
cargo run -- --stop-pid 12345
```

## Build

```sh
cargo build --release
```

The release binary will be at `target/release/exit-port`.

## Homebrew Tap

Homebrew installs from the companion tap at [`6space7/homebrew-exit-port`](https://github.com/6space7/homebrew-exit-port). The formula points back to tagged source releases in this repository.

## Linux Notes

The tray backend uses GTK/AppIndicator through `tray-icon`. On Debian/Ubuntu, install the tray libraries before building:

```sh
sudo apt install libgtk-3-dev libayatana-appindicator3-dev
```

Some Linux desktops hide tray icons unless an AppIndicator/status-notifier extension is enabled.
