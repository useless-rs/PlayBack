//! Settings-sheet model.

use playback_config::PlaybackConfig;

use crate::state::SettingsTab;

/// A tabbed settings sheet model.
#[derive(Debug, Clone, PartialEq)]
pub struct SettingsWindowModel {
    /// Whether the sheet is visible.
    pub visible: bool,
    /// Active tab.
    pub active_tab: SettingsTab,
    /// Snapshot of the configuration being edited.
    pub config: PlaybackConfig,
}

impl SettingsWindowModel {
    /// Creates a settings model.
    pub const fn new(visible: bool, active_tab: SettingsTab, config: PlaybackConfig) -> Self {
        Self {
            visible,
            active_tab,
            config,
        }
    }

    /// Returns the available settings tabs in display order.
    pub const fn tabs() -> [SettingsTab; 6] {
        [
            SettingsTab::Playback,
            SettingsTab::Video,
            SettingsTab::Audio,
            SettingsTab::Subtitles,
            SettingsTab::Interface,
            SettingsTab::Keybindings,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::SettingsWindowModel;
    use crate::state::SettingsTab;
    use playback_config::PlaybackConfig;

    #[test]
    fn exposes_all_sections() {
        assert_eq!(SettingsWindowModel::tabs().len(), 6);
        let model =
            SettingsWindowModel::new(true, SettingsTab::Interface, PlaybackConfig::default());
        assert!(model.visible);
        assert_eq!(model.active_tab, SettingsTab::Interface);
    }
}
