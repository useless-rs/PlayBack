//! Subtitle overlay model.

use playback_config::SubtitleSettings;

/// The model used to render a subtitle overlay.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubtitleOverlayModel {
    /// Whether subtitles should be rendered.
    pub visible: bool,
    /// Current subtitle text, if any.
    pub text: Option<String>,
    /// Font size in points.
    pub font_size: u32,
    /// Foreground color.
    pub color: String,
    /// Border color.
    pub border_color: String,
    /// Border width in points.
    pub border_size: u32,
}

impl SubtitleOverlayModel {
    /// Creates an overlay model from configuration.
    pub fn from_settings(settings: &SubtitleSettings) -> Self {
        Self {
            visible: settings.enabled,
            text: None,
            font_size: settings.font_size,
            color: settings.color.clone(),
            border_color: settings.border_color.clone(),
            border_size: settings.border_size,
        }
    }

    /// Sets the currently active subtitle cue.
    pub fn set_text(&mut self, text: Option<String>) {
        self.text = text;
    }
}

#[cfg(test)]
mod tests {
    use super::SubtitleOverlayModel;
    use playback_config::SubtitleSettings;

    #[test]
    fn reflects_subtitle_configuration() {
        let model = SubtitleOverlayModel::from_settings(&SubtitleSettings::default());
        assert!(model.visible);
        assert_eq!(model.font_size, 52);
        assert_eq!(model.color, "#FFFFFF");
    }

    #[test]
    fn stores_active_cue() {
        let mut model = SubtitleOverlayModel::from_settings(&SubtitleSettings::default());
        model.set_text(Some("Hello".to_owned()));
        assert_eq!(model.text.as_deref(), Some("Hello"));
    }
}
