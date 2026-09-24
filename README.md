# PlayBack

[![CI](https://github.com/useless-rs/PlayBack/actions/workflows/ci.yml/badge.svg)](https://github.com/useless-rs/PlayBack/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/playback.svg)](https://crates.io/crates/playback)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

PlayBack is a keyboard-first desktop media player for people who want a calm library, a focused playback surface, and the flexibility of mpv without being buried in configuration.

The project is being reorganized around **Tauri v2 + React + TypeScript + Vite**, while preserving the existing Rust playback, configuration, and CLI crates. The visual language is original: it borrows interaction ideas from modern media players and macOS conventions, but it does not copy IINA assets, branding, source, or trade dress.

> **Current architecture:** the Tauri window owns the library, playlist, settings, keyboard model, and playback controls. The Rust backend manages an `mpv` process and communicates through typed Tauri commands and events. The first release keeps mpv's native output window as the video surface; the renderer boundary is isolated so a future embedded surface can replace it without rewriting the product model.

## What is here

- **Library-first playback** — open local media, keep a lightweight queue, and return to the last useful context quickly.
- **Keyboard-first UX** — Space, arrow keys, `F`, `⌘K`, and `⌘O` are first-class actions, not hidden power-user features.
- **Tauri v2 backend** — typed commands, events, capabilities, window-state persistence, native dialogs, OS information, and opener support.
- **Rust core** — mpv-compatible parsing, process backend, shared playback state, Lua configuration, and CLI support remain reusable crates.
- **Cross-platform foundation** — the frontend and Tauri shell target macOS, Windows, and Linux; the CLI remains usable in headless environments.
- **Reduced-motion and accessibility basics** — visible focus, semantic buttons, keyboard seek controls, screen-reader labels, responsive desktop layouts, and `prefers-reduced-motion` support.

## Screenshots

> Screenshot placeholder: the Tauri shell is designed around a quiet media surface, a compact toolbar, a keyboard-first transport, a playlist rail, and a native-feeling settings sheet. Replace this block with release screenshots once the first desktop build is captured on all three platforms.

## Quick start

### Requirements

- Rust `1.85+`
- Node.js `22+`
- `mpv` available on `PATH`
- Tauri v2 platform prerequisites:
  - macOS: Xcode Command Line Tools
  - Windows: Microsoft C++ Build Tools and WebView2
  - Linux: WebKitGTK, AppIndicator, librsvg, and patchelf packages

### Run the desktop app

```bash
npm install
npm run desktop:dev
```

The Tauri configuration lives in [`src-tauri/tauri.conf.json`](src-tauri/tauri.conf.json). During development, Vite serves the frontend at `http://localhost:5173` and Tauri loads it in the native window.

### Run the CLI

The original Rust CLI remains available for scripting and headless environments:

```bash
cargo run -p playback -- video.mp4 --volume=80
cargo run -p playback -- --help
```

The CLI accepts both `--option=value` and `--option value`, plus mpv-compatible legacy single-dash forms. It forwards unknown options to mpv instead of silently dropping them.

## Project layout

```text
playback/
├── frontend/                    # React + TypeScript + Vite desktop frontend
│   ├── src/components/          # Sidebar, stage, transport, playlist, settings
│   ├── src/lib/ipc.ts            # Typed Tauri command/event boundary
│   └── src/styles.css            # Cross-platform visual tokens and layout
├── src-tauri/                   # Tauri v2 Rust application
│   ├── capabilities/             # Least-privilege window capabilities
│   ├── src/playback.rs           # Managed mpv process and typed commands
│   ├── src/lib.rs                # Tauri builder and plugin registration
│   └── tauri.conf.json           # v2 window, bundle, and security config
├── crates/
│   ├── playback-core/            # Parsing, state, mpv backends, invocations
│   ├── playback-config/          # Lua evaluation, schema, reload watcher
│   ├── playback-ui/              # Renderer-neutral UI models and legacy shell
│   └── playback-cli/             # CLI and shell completions
├── docs/README.md                # Detailed architecture and migration notes
└── Cargo.toml                    # Published Rust workspace
```

## UX direction

The interface is designed around four principles:

1. **The content leads.** Chrome stays quiet, the transport is close, and the playlist is always one action away.
2. **The keyboard is a first-class input.** Shortcuts are visible in context and every pointer action has a keyboard equivalent.
3. **State should never surprise you.** The active item, playback position, volume, and queue state are always easy to locate.
4. **Motion explains change.** Transitions are short, purposeful, and disabled when the system requests reduced motion.

The visual system uses a neutral graphite base, warm playback accent, system-first typography, restrained translucency, and compact controls. It is intentionally not a clone of any existing player.

## Tauri commands and events

The frontend talks to Rust through `@tauri-apps/api` v2 wrappers in [`frontend/src/lib/ipc.ts`](frontend/src/lib/ipc.ts). The backend currently exposes:

- `get_playback_snapshot`
- `get_app_config`
- `save_app_config`
- `open_media`
- `toggle_playback`
- `seek_relative`
- `seek_absolute`
- `set_volume`
- `toggle_fullscreen`
- `next_track`
- `previous_track`
- `shutdown_session`

The backend emits `playback://state` after state-changing commands. New capabilities must be added to [`src-tauri/capabilities/default.json`](src-tauri/capabilities/default.json); do not grant broad filesystem or shell access by default.

## Development

Install frontend dependencies once:

```bash
npm ci
```

Useful checks:

```bash
# Rust workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo build --release --workspace

# Tauri backend
cargo check --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings

# Frontend
npm run build
npm run desktop:info
```

CI runs the Rust workspace, frontend build, and Tauri backend check on Ubuntu, macOS, and Windows.

## Build desktop bundles

```bash
# macOS
npm run desktop:build

# Windows
npm run desktop:build

# Linux
npm run desktop:build
```

For a macOS universal build, install both Rust targets first:

```bash
rustup target add aarch64-apple-darwin x86_64-apple-darwin
npm run desktop:build -- --target universal-apple-darwin
```

## Publishing the Rust workspace

The Tauri desktop application is released as a desktop bundle. The reusable Rust workspace is published to crates.io in dependency order:

```bash
cargo publish -p playback-core
cargo publish -p playback-config
cargo publish -p playback-ui
cargo publish -p playback-cli
cargo publish -p playback
```

The GitHub Actions publish workflow performs the same order with Trusted Publishing. Do not publish dependent crates before their internal dependencies exist on crates.io.

## Migration notes

The original project used a macOS-only GPUI shell. The migration keeps the Rust domain crates and replaces the application shell with Tauri v2:

- `playback-core` remains the source of truth for mpv-style arguments, playback state, and backend operations.
- `playback-config` remains the source of truth for Lua configuration and live reload behavior.
- `playback-cli` remains available for automation and headless playback.
- `playback-ui` remains as a renderer-neutral model crate while the React frontend becomes the primary desktop UI.
- The managed Tauri backend starts `mpv` as a child process and communicates through typed commands/events.
- The macOS window uses an overlay titlebar, hidden title, traffic-light positioning, transparent material, and a rounded native effect through Tauri configuration.
- Linux and Windows use the same React surface and Tauri command boundary, with platform behavior delegated to the webview and operating system.

## Lucide icon inventory

The React UI uses Lucide icons only. Structural icons have accessible labels through the shared `IconButton` component.

| Area | Icons |
| --- | --- |
| Navigation | `Film`, `ListVideo`, `FolderOpen`, `Settings2`, `PanelLeftClose`, `PanelLeftOpen`, `HardDrive` |
| Window and actions | `Search`, `Command`, `Moon`, `Sun`, `PanelRight`, `MoreHorizontal`, `Plus`, `X` |
| Playback | `Play`, `Pause`, `RotateCcw`, `RotateCw`, `SkipBack`, `SkipForward`, `Volume2`, `VolumeX`, `Gauge`, `Captions`, `Maximize2` |
| Playlist and settings | `Check`, `Trash2`, `SlidersHorizontal`, `MonitorPlay`, `Subtitles`, `Palette`, `Keyboard`, `Cpu` |

## License

PlayBack is dual-licensed under either the [MIT License](LICENSE-MIT) or the [Apache License, Version 2.0](LICENSE-APACHE). See [`LICENSE`](LICENSE) for the project notice.
