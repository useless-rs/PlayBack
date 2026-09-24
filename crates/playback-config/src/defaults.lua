-- PlayBack default configuration
-- This file is Lua, not .conf. Every key here maps to an mpv property.

return {
  playback = {
    volume = 100,
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
}
