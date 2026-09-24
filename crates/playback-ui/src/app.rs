//! Application-level UI state management.

use playback_config::PlaybackConfig;
use playback_core::PlaybackSnapshot;

use crate::messaging::UiMessage;
use crate::state::{UiCommand, UiState};
use crate::views::PlayerViewModel;

/// The platform-neutral application model.
#[derive(Debug, Clone, PartialEq)]
pub struct PlaybackApplication {
    state: UiState,
    config: PlaybackConfig,
}

impl PlaybackApplication {
    /// Creates an application model.
    pub fn new(playback: PlaybackSnapshot, config: PlaybackConfig) -> Self {
        let state = UiState::new(playback, &config);
        Self { state, config }
    }

    /// Applies a UI command.
    pub fn apply(&mut self, command: UiCommand) -> bool {
        self.state.apply(command)
    }

    /// Handles a message received from another thread.
    pub fn handle_message(&mut self, message: UiMessage) -> bool {
        match message {
            UiMessage::Command(command) => self.apply(command),
            UiMessage::Playback(snapshot) => {
                self.state.playback = snapshot;
                true
            }
            UiMessage::Config(config) => {
                self.replace_config(*config);
                true
            }
            UiMessage::Shutdown => false,
        }
    }

    /// Replaces the current configuration and reapplies interface defaults.
    pub fn replace_config(&mut self, config: PlaybackConfig) {
        self.config = config;
        self.state.playlist_visible = self.config.interface().show_playlist;
    }

    /// Returns the current configuration.
    pub const fn config(&self) -> &PlaybackConfig {
        &self.config
    }

    /// Builds a render-ready player view.
    pub fn player_view(&self) -> PlayerViewModel {
        PlayerViewModel::from_state(self.state.clone())
    }

    /// Returns the underlying UI state.
    pub const fn state(&self) -> &UiState {
        &self.state
    }
}

#[cfg(test)]
mod tests {
    use super::PlaybackApplication;
    use crate::messaging::UiMessage;
    use crate::state::UiCommand;
    use playback_config::PlaybackConfig;
    use playback_core::PlaybackSnapshot;

    fn snapshot() -> PlaybackSnapshot {
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
        }
    }

    #[test]
    fn applies_commands_to_state() {
        let mut app = PlaybackApplication::new(snapshot(), PlaybackConfig::default());
        assert!(app.apply(UiCommand::TogglePlay));
        assert!(app.state().playback.paused);
    }

    #[test]
    fn handles_thread_messages() {
        let mut app = PlaybackApplication::new(snapshot(), PlaybackConfig::default());
        assert!(app.handle_message(UiMessage::Command(UiCommand::TogglePlay)));
        assert!(app.state().playback.paused);
        assert!(!app.handle_message(UiMessage::Shutdown));
    }

    #[test]
    fn replaces_configuration() {
        let mut app = PlaybackApplication::new(snapshot(), PlaybackConfig::default());
        let mut config = PlaybackConfig::default();
        config.interface.show_playlist = false;
        app.replace_config(config);
        assert!(!app.state().playlist_visible);
    }
}
