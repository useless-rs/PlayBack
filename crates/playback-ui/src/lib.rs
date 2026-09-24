//! PlayBack UI models and platform integration boundary.

#![forbid(unsafe_code)]

pub mod app;
pub mod components;
pub mod messaging;
#[cfg(feature = "native")]
pub mod native;
#[cfg(all(feature = "native", target_os = "macos"))]
pub mod native_app;
pub mod state;
pub mod theme;
pub mod views;

pub use app::PlaybackApplication;
pub use components::{
    ControlBarModel, ControlButton, PlaylistPanelModel, PlaylistRow, SettingsWindowModel,
    SubtitleOverlayModel, TimelineModel, VolumeModel,
};
pub use messaging::{UiChannelError, UiMessage, UiReceiver, UiSender, ui_channel};
pub use state::{ControlVisibility, SettingsTab, UiCommand, UiState};
pub use theme::{AnimationCurve, AnimationTokens, Color, ColorTokens, Duration, Theme};
pub use views::{LibraryRow, LibraryViewModel, PlayerViewModel};

/// Creates the default macOS-inspired application state.
pub fn default_state(
    playback: playback_core::PlaybackSnapshot,
    config: &playback_config::PlaybackConfig,
) -> UiState {
    UiState::new(playback, config)
}

#[cfg(test)]
mod tests {
    use super::default_state;
    use playback_config::PlaybackConfig;
    use playback_core::PlaybackSnapshot;

    #[test]
    fn creates_default_state() {
        let state = default_state(
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
        assert!((state.theme.playlist_width - 280.0).abs() < f32::EPSILON);
    }
}
