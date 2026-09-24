# PlayBack reference documentation

This page contains the configuration, CLI, and architecture references for
PlayBack.

## Configuration

PlayBack configuration is Lua, not a `.conf` file. The bundled defaults live
in `crates/playback-config/src/defaults.lua`; a user file is loaded from
`~/.config/playback/init.lua` when present. Use `--config=PATH` to select a
different file.

### Complete example

```lua
return {
  playback = {
    volume = 80,
    speed = 1.0,
    loop = false,
    autoplay = true,
    start_paused = false,
  },
  video = {
    hardware_decoding = "auto-safe",
    scale = "bilinear",
    deinterlace = false,
    aspect_ratio = "auto",
  },
  audio = {
    channel_layout = "auto",
    normalize = false,
  },
  subtitles = {
    enabled = true,
    font = "SF Pro Display",
    font_size = 52,
    color = "#FFFFFF",
    border_color = "#000000",
    border_size = 3,
  },
  interface = {
    theme = "macos",
    animations = true,
    blur_background = true,
    show_playlist = true,
    show_controls = true,
    controls_timeout_ms = 2500,
  },
  keybindings = {
    toggle_play = "Space",
    seek_forward = "Right",
    seek_backward = "Left",
    volume_up = "Up",
    volume_down = "Down",
    fullscreen = "f",
    quit = "q",
    next_track = "n",
    prev_track = "p",
  },
  custom = {
    fs = "minimal",
    vo = "gpu",
  },
}
```

### Typed keys

#### `playback`

| Key | Type | Default | Meaning |
| --- | --- | --- | --- |
| `volume` | number | `100` | Initial volume from 0 to 100. |
| `speed` | number | `1.0` | Initial playback speed. |
| `loop` | boolean | `false` | Repeat the current file. |
| `autoplay` | boolean | `true` | Start playback when media is loaded. |
| `start_paused` | boolean | `false` | Load media in a paused state. |

#### `video`

| Key | Type | Default | Meaning |
| --- | --- | --- | --- |
| `hardware_decoding` | string | `"auto-safe"` | mpv hardware-decoding policy. |
| `scale` | string | `"bilinear"` | mpv scaler name. |
| `deinterlace` | boolean | `false` | Enable deinterlacing. |
| `aspect_ratio` | string | `"auto"` | Video aspect-ratio policy. |

#### `audio`

| Key | Type | Default | Meaning |
| --- | --- | --- | --- |
| `channel_layout` | string | `"auto"` | mpv channel-layout policy. |
| `normalize` | boolean | `false` | Apply loudness normalization. |

#### `subtitles`

| Key | Type | Default | Meaning |
| --- | --- | --- | --- |
| `enabled` | boolean | `true` | Render subtitle cues. |
| `font` | string | `"SF Pro Display"` | Subtitle font family. |
| `font_size` | integer | `52` | Font size in points. |
| `color` | string | `"#FFFFFF"` | Foreground color. |
| `border_color` | string | `"#000000"` | Border color. |
| `border_size` | integer | `3` | Border width in points. |

#### `interface`

| Key | Type | Default | Meaning |
| --- | --- | --- | --- |
| `theme` | string | `"macos"` | UI theme name. |
| `animations` | boolean | `true` | Enable interaction animations. |
| `blur_background` | boolean | `true` | Enable translucent/blurred materials. |
| `show_playlist` | boolean | `true` | Show the playlist initially. |
| `show_controls` | boolean | `true` | Show controls initially. |
| `controls_timeout_ms` | integer | `2500` | Idle timeout before controls fade. |

#### `keybindings`

Every binding is a string interpreted by the active frontend:

| Key | Default |
| --- | --- |
| `toggle_play` | `"Space"` |
| `seek_forward` | `"Right"` |
| `seek_backward` | `"Left"` |
| `volume_up` | `"Up"` |
| `volume_down` | `"Down"` |
| `fullscreen` | `"f"` |
| `quit` | `"q"` |
| `next_track` | `"n"` |
| `prev_track` | `"p"` |

### Custom mpv properties

Use `custom` for any mpv property not represented by the typed schema:

```lua
return {
  custom = {
    fs = "minimal",
    vo = "gpu",
    ["af"] = "lavfi=[loudnorm]",
  },
}
```

Scalar values are forwarded as `--property=value` arguments. Arrays and
objects are retained in the typed configuration but are not flattened into a
single CLI argument; a frontend can interpret them directly.

Unknown top-level or section keys are reported as warnings and do not make the
configuration invalid. This keeps live reload forward-compatible with newer
mpv properties and newer PlayBack releases.

## CLI

`playback` accepts media paths and mpv-style options. Both `--option=value` and
`--option value` are supported, and legacy single-dash forms are preserved:

```bash
playback video.mp4
playback --no-video video.mp4
playback --volume=80 video.mp4
playback -volume 80 video.mp4
playback --sub-file=subs.srt video.mp4
playback --start=00:01:30 video.mp4
playback --loop-file=inf video.mp4
playback --speed=1.5 video.mp4
playback --audio-file=track2.flac video.mp4
playback --fullscreen video.mp4
playback --ontop video.mp4
playback --geometry=1280x720 video.mp4
```

### Typed options

| Option | Value | Behavior |
| --- | --- | --- |
| `--config PATH` | path | Load a specific Lua file. |
| `--volume VALUE` | number | Set the initial volume. |
| `--speed VALUE` | positive number | Set playback speed. |
| `--start TIME` | seconds, `MM:SS`, or `HH:MM:SS` | Seek after loading. |
| `--loop-file MODE` | `no`, `inf`, or count | Configure file looping. |
| `--sub-file PATH` | path | Add and select an external subtitle file. |
| `--audio-file PATH` | path | Add and select an external audio file. |
| `--no-video` | flag | Disable video output. |
| `--fullscreen` | flag | Start fullscreen. |
| `--ontop` | flag | Keep the window above other windows. |
| `--geometry WIDTHxHEIGHT` | dimensions | Set the initial window geometry. |

Boolean options also accept the legacy single-dash spelling, such as
`-fullscreen`.

### Forward-compatible options

Unknown options are retained rather than rejected:

```bash
playback --fs=minimal --hwdec=auto-safe --autoload video.mp4
playback --my-option=value video.mp4
```

An unknown option with `=` becomes a property assignment. A valueless unknown
option becomes a boolean flag and is forwarded to mpv. This is the deliberate
compatibility boundary for mpv options that PlayBack has not typed yet; consult
the [mpv manual](https://mpv.io/manual/master/) for the backend meaning of a
property.

### Shell completions

```bash
playback --completion bash
playback --completion zsh
playback --completion fish
```

### Process behavior

The default backend launches the system `mpv` executable and waits for it to
finish. This keeps installation lightweight and makes the CLI usable on
headless systems. Applications embedding `playback-core` can select a different
`MpvBackend`; the optional `native-mpv` feature provides a libmpv-backed
adapter when the platform SDK is available.

## Architecture

PlayBack separates command parsing, configuration, media-engine access, and UI
state so each concern can evolve without forcing a platform SDK on every
consumer.

### Thread and data-flow diagram

```text
                         ┌──────────────────────────────┐
                         │        main / UI thread      │
                         │                              │
                         │  ConfigEngine                │
                         │    └─ notify watcher         │
                         │       Lua -> JSON -> schema   │
                         │                              │
                         │  UiState / PlayerViewModel   │
                         │       ▲              │        │
                         │       │              ▼        │
                         │  events       MpvController    │
                         └───────┬──────────────┬─────────┘
                                 │              │
                       bounded channel     Arc<Mutex<State>>
                                 │              │
                                 ▼              ▼
                         ┌──────────────────────────────┐
                         │       media / render thread   │
                         │                              │
                         │  MpvBackend                  │
                         │    ├─ MpvProcessBackend        │
                         │    └─ NativeMpvBackend        │
                         │         (libmpv2 adapter)     │
                         └──────────────────────────────┘
```

### Parser and backend boundary

`playback-core::cli` parses command-line input at the process boundary. Common
options become typed `MpvArg` values; unknown options remain typed properties or
flags. `MpvController` translates those values into `MpvBackend` operations and
updates `PlaybackStateHandle` in the same transaction boundary.

The default `MpvProcessBackend` delegates to the installed `mpv` executable.
The `native-mpv` feature adds a `NativeMpvBackend` backed by the current
`libmpv2` release (the maintained crates.io package from the requested
`Stremio/libmpv2-rs` project). No raw libmpv pointers are exposed by PlayBack's
public API.

### Configuration

`playback-config` evaluates Lua with vendored Lua 5.4 through `mlua`. The
returned table is converted to JSON and then to the typed `PlaybackConfig`
schema. Unknown keys are collected as diagnostics instead of failing the load.
`ConfigEngine::watch` uses `notify` to reload a file and invoke a callback.

The default schema is embedded from `src/defaults.lua`, so a user file only
needs to contain the values it wants to override.

### UI integration

`playback-ui` keeps the domain model renderer-neutral while providing a macOS
native shell behind the `native` feature. It owns:

- macOS HIG-inspired color, typography, motion, sizing, and elevation tokens;
- playback and library view models;
- component models for controls, timeline, playlist, subtitles, and settings;
- a bounded `crossbeam-channel` message boundary;
- a `HasWindowHandle` bridge for native frontends.

On macOS, the native feature links GPUI, `gpui-design` with
`DesignSystem::apple_hig()`, `gpui-ui-kit`, and `lucide-icons`. The current
shell owns the native window and control surface; it deliberately does not
claim to embed video frames yet. `libmpv2` 6.0 provides a control/event API
and an OpenGL render API, while the documented `mpv --wid` embedding path is
not a macOS integration contract and GPUI 0.2.2 does not expose a compatible
public framebuffer bridge. The verified playback path therefore remains the
process-backed CLI until that renderer adapter is implemented. The portable
build keeps the same models usable on Linux CI and in environments without a
display server.

### State ownership

`PlaybackStateHandle` is an `Arc<parking_lot::Mutex<PlaybackState>>`. The lock
is held only for short state updates or snapshot reads. A native frontend can
move larger messages over a bounded channel and use the shared state as the
latest-value snapshot rather than as an event bus.

### Safety and release policy

All first-party crates use `#![forbid(unsafe_code)]`. The optional dependency
that contains the FFI boundary is isolated behind `native-mpv`; its safety
proof belongs to the upstream crate. PlayBack validates values before crossing
that boundary and does not manufacture raw handles or pointers.
