//! Playlist sidebar model.

use playback_core::PlaylistItem;

/// A single playlist row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaylistRow {
    /// Stable row index.
    pub index: usize,
    /// Display title.
    pub title: String,
    /// Formatted duration, if known.
    pub duration: Option<String>,
    /// Whether this row is currently playing.
    pub playing: bool,
    /// Whether the equalizer animation should be shown.
    pub animated_playing_indicator: bool,
}

/// The playlist panel model.
#[derive(Debug, Clone, PartialEq)]
pub struct PlaylistPanelModel {
    /// Whether the panel is visible.
    pub visible: bool,
    /// Panel width in points.
    pub width: f32,
    /// Rows in playlist order.
    pub rows: Vec<PlaylistRow>,
}

impl PlaylistPanelModel {
    /// Creates a panel from playlist items.
    pub fn new(visible: bool, items: &[PlaylistItem], playing_index: Option<usize>) -> Self {
        let rows = items
            .iter()
            .enumerate()
            .map(|(index, item)| PlaylistRow {
                index,
                title: item.title.clone(),
                duration: item.duration.map(super::player_controls::format_timestamp),
                playing: playing_index == Some(index),
                animated_playing_indicator: playing_index == Some(index),
            })
            .collect();
        Self {
            visible,
            width: 280.0,
            rows,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PlaylistPanelModel;
    use playback_core::{PlaylistItem, Seconds};

    #[test]
    fn marks_playing_row() {
        let items = vec![
            PlaylistItem {
                path: "one.mkv".to_owned(),
                title: "One".to_owned(),
                duration: Some(Seconds::new(12.0)),
            },
            PlaylistItem {
                path: "two.mkv".to_owned(),
                title: "Two".to_owned(),
                duration: None,
            },
        ];
        let panel = PlaylistPanelModel::new(true, &items, Some(1));
        assert!(!panel.rows[0].playing);
        assert!(panel.rows[1].playing);
        assert!(panel.rows[1].animated_playing_indicator);
        assert_eq!(panel.rows[0].duration.as_deref(), Some("00:12"));
    }
}
