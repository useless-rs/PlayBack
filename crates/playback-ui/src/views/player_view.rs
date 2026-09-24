//! Player view composition.

use crate::components::{
    ControlBarModel, PlaylistPanelModel, SubtitleOverlayModel, TimelineModel, VolumeModel,
};
use crate::state::{ControlVisibility, UiState};

/// The complete player-view model.
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerViewModel {
    /// Current UI state.
    pub state: UiState,
    /// Bottom control bar.
    pub controls: ControlBarModel,
    /// Optional playlist sidebar.
    pub playlist: PlaylistPanelModel,
    /// Optional subtitle overlay.
    pub subtitles: SubtitleOverlayModel,
}

impl PlayerViewModel {
    /// Creates a player view from UI state.
    pub fn from_state(state: UiState) -> Self {
        let timeline = TimelineModel::from_snapshot(&state.playback);
        let controls = ControlBarModel::new(
            state.playback.paused,
            state.controls == ControlVisibility::Visible,
            timeline,
            VolumeModel::new(state.playback.volume, false),
        );
        let playlist =
            PlaylistPanelModel::new(state.playlist_visible, &state.playback.playlist, None);
        let subtitles =
            SubtitleOverlayModel::from_settings(&playback_config::SubtitleSettings::default());
        Self {
            state,
            controls,
            playlist,
            subtitles,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PlayerViewModel;
    use crate::state::UiState;
    use playback_config::PlaybackConfig;
    use playback_core::PlaybackSnapshot;

    #[test]
    fn composes_player_components() {
        let state = UiState::new(
            PlaybackSnapshot {
                paused: true,
                seeking: false,
                position: playback_core::Seconds::new(0.0),
                duration: Some(playback_core::Seconds::new(60.0)),
                buffered: None,
                volume: 80.0,
                speed: 1.0,
                loop_file: false,
                audio_track: None,
                subtitle_track: None,
                playlist: Vec::new(),
            },
            &PlaybackConfig::default(),
        );
        let view = PlayerViewModel::from_state(state);
        assert!(view.controls.paused);
        assert!((view.controls.volume.value - 80.0).abs() < f64::EPSILON);
    }
}
