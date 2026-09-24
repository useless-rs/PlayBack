//! Typed configuration schema and defaults.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The complete PlayBack configuration document.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlaybackConfig {
    /// General playback behavior.
    pub playback: PlaybackSettings,
    /// Video rendering behavior.
    pub video: VideoSettings,
    /// Audio behavior.
    pub audio: AudioSettings,
    /// Subtitle rendering behavior.
    pub subtitles: SubtitleSettings,
    /// User-interface behavior.
    pub interface: InterfaceSettings,
    /// Keyboard bindings.
    pub keybindings: Keybindings,
    /// Values forwarded to arbitrary mpv properties.
    pub custom: BTreeMap<String, Value>,
}

/// General playback settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlaybackSettings {
    /// Initial volume from 0 to 100.
    pub volume: f64,
    /// Initial playback speed.
    pub speed: f64,
    /// Whether playback loops.
    pub r#loop: bool,
    /// Whether playback starts automatically.
    pub autoplay: bool,
    /// Whether playback starts paused.
    pub start_paused: bool,
}

impl Default for PlaybackSettings {
    fn default() -> Self {
        Self {
            volume: 100.0,
            speed: 1.0,
            r#loop: false,
            autoplay: true,
            start_paused: false,
        }
    }
}

impl PlaybackSettings {
    /// Returns the configured volume.
    pub const fn volume(&self) -> f64 {
        self.volume
    }

    /// Returns the configured speed.
    pub const fn speed(&self) -> f64 {
        self.speed
    }
}

/// Video settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VideoSettings {
    /// Hardware decoding policy.
    pub hardware_decoding: String,
    /// mpv scaler name.
    pub scale: String,
    /// Whether deinterlacing is enabled.
    pub deinterlace: bool,
    /// Aspect-ratio policy.
    pub aspect_ratio: String,
}

impl Default for VideoSettings {
    fn default() -> Self {
        Self {
            hardware_decoding: "auto-safe".to_owned(),
            scale: "bilinear".to_owned(),
            deinterlace: false,
            aspect_ratio: "auto".to_owned(),
        }
    }
}

impl VideoSettings {
    /// Returns the hardware decoding policy.
    pub fn hardware_decoding(&self) -> &str {
        &self.hardware_decoding
    }
}

/// Audio settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AudioSettings {
    /// mpv channel-layout policy.
    pub channel_layout: String,
    /// Whether audio normalization is enabled.
    pub normalize: bool,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            channel_layout: "auto".to_owned(),
            normalize: false,
        }
    }
}

impl AudioSettings {
    /// Returns the channel-layout policy.
    pub fn channel_layout(&self) -> &str {
        &self.channel_layout
    }
}

/// Subtitle settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SubtitleSettings {
    /// Whether subtitles are enabled.
    pub enabled: bool,
    /// Font family.
    pub font: String,
    /// Font size in points.
    pub font_size: u32,
    /// Foreground color.
    pub color: String,
    /// Border color.
    pub border_color: String,
    /// Border width in points.
    pub border_size: u32,
}

impl Default for SubtitleSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            font: "SF Pro Display".to_owned(),
            font_size: 52,
            color: "#FFFFFF".to_owned(),
            border_color: "#000000".to_owned(),
            border_size: 3,
        }
    }
}

impl SubtitleSettings {
    /// Returns whether subtitles are enabled.
    pub const fn enabled(&self) -> bool {
        self.enabled
    }
}

/// Interface settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct InterfaceSettings {
    /// Theme name.
    pub theme: String,
    /// Whether animations are enabled.
    pub animations: bool,
    /// Whether blurred materials are enabled.
    pub blur_background: bool,
    /// Whether the playlist is visible initially.
    pub show_playlist: bool,
    /// Whether playback controls are visible initially.
    pub show_controls: bool,
    /// Idle timeout before controls fade, in milliseconds.
    pub controls_timeout_ms: u64,
}

impl Default for InterfaceSettings {
    fn default() -> Self {
        Self {
            theme: "macos".to_owned(),
            animations: true,
            blur_background: true,
            show_playlist: true,
            show_controls: true,
            controls_timeout_ms: 2500,
        }
    }
}

impl InterfaceSettings {
    /// Returns the control idle timeout.
    pub const fn controls_timeout_ms(&self) -> u64 {
        self.controls_timeout_ms
    }
}

/// Keyboard bindings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Keybindings {
    /// Binding that toggles play and pause.
    pub toggle_play: String,
    /// Binding that seeks forward.
    pub seek_forward: String,
    /// Binding that seeks backward.
    pub seek_backward: String,
    /// Binding that raises volume.
    pub volume_up: String,
    /// Binding that lowers volume.
    pub volume_down: String,
    /// Binding that toggles fullscreen.
    pub fullscreen: String,
    /// Binding that quits PlayBack.
    pub quit: String,
    /// Binding that selects the next track.
    pub next_track: String,
    /// Binding that selects the previous track.
    pub prev_track: String,
}

impl Default for Keybindings {
    fn default() -> Self {
        Self {
            toggle_play: "Space".to_owned(),
            seek_forward: "Right".to_owned(),
            seek_backward: "Left".to_owned(),
            volume_up: "Up".to_owned(),
            volume_down: "Down".to_owned(),
            fullscreen: "f".to_owned(),
            quit: "q".to_owned(),
            next_track: "n".to_owned(),
            prev_track: "p".to_owned(),
        }
    }
}

impl PlaybackConfig {
    /// Returns general playback settings.
    pub const fn playback(&self) -> &PlaybackSettings {
        &self.playback
    }

    /// Returns video settings.
    pub const fn video(&self) -> &VideoSettings {
        &self.video
    }

    /// Returns audio settings.
    pub const fn audio(&self) -> &AudioSettings {
        &self.audio
    }

    /// Returns subtitle settings.
    pub const fn subtitles(&self) -> &SubtitleSettings {
        &self.subtitles
    }

    /// Returns interface settings.
    pub const fn interface(&self) -> &InterfaceSettings {
        &self.interface
    }

    /// Returns keybindings.
    pub const fn keybindings(&self) -> &Keybindings {
        &self.keybindings
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AudioSettings, InterfaceSettings, Keybindings, PlaybackConfig, PlaybackSettings,
        SubtitleSettings, VideoSettings,
    };

    #[test]
    fn defaults_populate_every_section() {
        let config = PlaybackConfig::default();
        assert_eq!(config.playback(), &PlaybackSettings::default());
        assert_eq!(config.video(), &VideoSettings::default());
        assert_eq!(config.audio(), &AudioSettings::default());
        assert_eq!(config.subtitles(), &SubtitleSettings::default());
        assert_eq!(config.interface(), &InterfaceSettings::default());
        assert_eq!(config.keybindings(), &Keybindings::default());
        assert!(config.custom.is_empty());
    }

    #[test]
    fn getters_expose_scalar_values() {
        let config = PlaybackConfig::default();
        assert!((config.playback().volume() - 100.0).abs() < f64::EPSILON);
        assert!((config.playback().speed() - 1.0).abs() < f64::EPSILON);
        assert_eq!(config.video().hardware_decoding(), "auto-safe");
        assert_eq!(config.audio().channel_layout(), "auto");
        assert!(config.subtitles().enabled());
        assert_eq!(config.interface().controls_timeout_ms(), 2500);
    }

    #[test]
    fn missing_sections_use_defaults_when_deserializing() {
        let config: PlaybackConfig = serde_json::from_str("{}").expect("valid sparse config");
        assert!((config.playback().volume() - 100.0).abs() < f64::EPSILON);
    }
}
