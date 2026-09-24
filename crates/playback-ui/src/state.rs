//! UI state and interaction model independent of a rendering backend.

use playback_config::PlaybackConfig;
use playback_core::PlaybackSnapshot;
use serde::{Deserialize, Serialize};

use crate::theme::Theme;

/// A settings-sheet section.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettingsTab {
    /// Playback controls and startup behavior.
    Playback,
    /// Video rendering options.
    Video,
    /// Audio options.
    Audio,
    /// Subtitle options.
    Subtitles,
    /// Interface and appearance options.
    Interface,
    /// Keyboard shortcuts.
    Keybindings,
}

/// Whether playback controls are currently visible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControlVisibility {
    /// Controls are visible.
    Visible,
    /// Controls are hidden after inactivity.
    Hidden,
}

/// A user interaction understood by the UI model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UiCommand {
    /// Toggles play and pause.
    TogglePlay,
    /// Seeks by a relative number of seconds.
    SeekRelative(f64),
    /// Sets the absolute position.
    SeekAbsolute(f64),
    /// Sets volume as a percentage.
    SetVolume(f64),
    /// Toggles subtitle visibility.
    ToggleSubtitles,
    /// Toggles fullscreen.
    ToggleFullscreen,
    /// Toggles playlist visibility.
    TogglePlaylist,
    /// Opens the settings sheet.
    OpenSettings(SettingsTab),
    /// Closes the settings sheet.
    CloseSettings,
    /// Records user activity.
    Activity,
}

/// The render-ready state shared by the player and library views.
#[derive(Debug, Clone, PartialEq)]
pub struct UiState {
    /// Active visual theme.
    pub theme: Theme,
    /// Latest playback snapshot.
    pub playback: PlaybackSnapshot,
    /// Current controls visibility.
    pub controls: ControlVisibility,
    /// Whether the playlist sidebar is visible.
    pub playlist_visible: bool,
    /// Whether the settings sheet is visible.
    pub settings_visible: bool,
    /// Active settings section.
    pub settings_tab: SettingsTab,
    /// Whether the player is in fullscreen.
    pub fullscreen: bool,
    /// Last activity timestamp in milliseconds.
    pub last_activity_ms: u64,
}

impl UiState {
    /// Creates a UI state from playback and configuration data.
    pub fn new(playback: PlaybackSnapshot, config: &PlaybackConfig) -> Self {
        Self {
            theme: Theme::macos_hig(),
            playback,
            controls: ControlVisibility::Visible,
            playlist_visible: config.interface().show_playlist,
            settings_visible: false,
            settings_tab: SettingsTab::Playback,
            fullscreen: false,
            last_activity_ms: 0,
        }
    }

    /// Applies one interaction and returns whether state changed.
    pub fn apply(&mut self, command: UiCommand) -> bool {
        match command {
            UiCommand::TogglePlay => {
                self.playback.paused = !self.playback.paused;
                true
            }
            UiCommand::SeekRelative(seconds) => {
                let next = self.playback.position.as_f64() + seconds;
                let maximum = self
                    .playback
                    .duration
                    .map_or(f64::MAX, playback_core::Seconds::as_f64);
                self.playback.position = playback_core::Seconds::new(next.clamp(0.0, maximum));
                true
            }
            UiCommand::SeekAbsolute(seconds) => {
                let maximum = self
                    .playback
                    .duration
                    .map_or(f64::MAX, playback_core::Seconds::as_f64);
                self.playback.position = playback_core::Seconds::new(seconds.clamp(0.0, maximum));
                true
            }
            UiCommand::SetVolume(volume) => {
                if !volume.is_finite() {
                    return false;
                }
                self.playback.volume = volume.clamp(0.0, 100.0);
                true
            }
            UiCommand::ToggleSubtitles => {
                self.playback.subtitle_track = None;
                true
            }
            UiCommand::ToggleFullscreen => {
                self.fullscreen = !self.fullscreen;
                self.controls = ControlVisibility::Visible;
                true
            }
            UiCommand::TogglePlaylist => {
                self.playlist_visible = !self.playlist_visible;
                true
            }
            UiCommand::OpenSettings(tab) => {
                self.settings_tab = tab;
                self.settings_visible = true;
                true
            }
            UiCommand::CloseSettings => {
                self.settings_visible = false;
                true
            }
            UiCommand::Activity => {
                self.last_activity_ms = self.last_activity_ms.saturating_add(1);
                self.controls = ControlVisibility::Visible;
                true
            }
        }
    }

    /// Advances the inactivity timer and hides fullscreen controls when idle.
    pub fn tick(&mut self, now_ms: u64) -> bool {
        if !self.fullscreen {
            return false;
        }
        let timeout = self.theme.animation.controls_reveal.milliseconds;
        if now_ms.saturating_sub(self.last_activity_ms) >= timeout {
            let changed = self.controls != ControlVisibility::Hidden;
            self.controls = ControlVisibility::Hidden;
            changed
        } else {
            false
        }
    }

    /// Returns a stable snapshot for rendering or diagnostics.
    #[must_use]
    pub fn render_snapshot(&self) -> UiState {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::{ControlVisibility, SettingsTab, UiCommand, UiState};
    use playback_config::PlaybackConfig;
    use playback_core::PlaybackSnapshot;

    #[test]
    fn initializes_from_interface_configuration() {
        let state = UiState::new(
            PlaybackSnapshot {
                paused: false,
                seeking: false,
                position: playback_core::Seconds::new(0.0),
                duration: Some(playback_core::Seconds::new(120.0)),
                buffered: None,
                volume: 100.0,
                speed: 1.0,
                loop_file: false,
                audio_track: None,
                subtitle_track: None,
                playlist: Vec::new(),
            },
            &PlaybackConfig::default(),
        );
        assert!(state.playlist_visible);
        assert_eq!(state.controls, ControlVisibility::Visible);
    }

    #[test]
    fn clamps_seek_and_volume_commands() {
        let mut state = UiState::new(
            PlaybackSnapshot {
                paused: false,
                seeking: false,
                position: playback_core::Seconds::new(10.0),
                duration: Some(playback_core::Seconds::new(20.0)),
                buffered: None,
                volume: 50.0,
                speed: 1.0,
                loop_file: false,
                audio_track: None,
                subtitle_track: None,
                playlist: Vec::new(),
            },
            &PlaybackConfig::default(),
        );
        state.apply(UiCommand::SeekRelative(-100.0));
        assert!((state.playback.position.as_f64() - 0.0).abs() < f64::EPSILON);
        state.apply(UiCommand::SetVolume(400.0));
        assert!((state.playback.volume - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn fullscreen_controls_hide_after_timeout() {
        let mut state = UiState::new(
            PlaybackSnapshot {
                paused: false,
                seeking: false,
                position: playback_core::Seconds::new(0.0),
                duration: None,
                buffered: None,
                volume: 100.0,
                speed: 1.0,
                loop_file: false,
                audio_track: None,
                subtitle_track: None,
                playlist: Vec::new(),
            },
            &PlaybackConfig::default(),
        );
        state.apply(UiCommand::ToggleFullscreen);
        state.tick(350);
        assert_eq!(state.controls, ControlVisibility::Hidden);
    }

    #[test]
    fn settings_tab_is_selectable() {
        let mut state = UiState::new(
            PlaybackSnapshot {
                paused: false,
                seeking: false,
                position: playback_core::Seconds::new(0.0),
                duration: None,
                buffered: None,
                volume: 100.0,
                speed: 1.0,
                loop_file: false,
                audio_track: None,
                subtitle_track: None,
                playlist: Vec::new(),
            },
            &PlaybackConfig::default(),
        );
        state.apply(UiCommand::OpenSettings(SettingsTab::Keybindings));
        assert!(state.settings_visible);
        assert_eq!(state.settings_tab, SettingsTab::Keybindings);
    }
}
