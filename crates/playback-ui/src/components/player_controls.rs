//! Models for the bottom playback control bar and timeline.

use playback_core::{PlaybackSnapshot, Seconds};

/// A control-bar button action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlButton {
    /// Toggle play and pause.
    PlayPause,
    /// Seek backward ten seconds.
    SkipBack,
    /// Seek forward ten seconds.
    SkipForward,
    /// Toggle subtitles.
    Subtitles,
    /// Open settings.
    Settings,
    /// Toggle fullscreen.
    Fullscreen,
}

/// A timeline model with buffered and played ranges.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimelineModel {
    /// Current position in seconds.
    pub position: Seconds,
    /// Media duration in seconds, if known.
    pub duration: Option<Seconds>,
    /// Buffered range, if known.
    pub buffered: Option<(Seconds, Seconds)>,
    /// Whether a drag or seek is active.
    pub seeking: bool,
}

impl TimelineModel {
    /// Creates a timeline from a playback snapshot.
    pub fn from_snapshot(snapshot: &PlaybackSnapshot) -> Self {
        Self {
            position: snapshot.position,
            duration: snapshot.duration,
            buffered: snapshot.buffered,
            seeking: snapshot.seeking,
        }
    }

    /// Returns the played fraction from 0 to 1.
    pub fn progress(&self) -> f64 {
        self.duration
            .filter(|duration| duration.as_f64() > 0.0)
            .map_or(0.0, |duration| {
                (self.position.as_f64() / duration.as_f64()).clamp(0.0, 1.0)
            })
    }
}

/// A model for the volume control.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VolumeModel {
    /// Current volume from 0 to 100.
    pub value: f64,
    /// Whether the pointer is over the control.
    pub hovered: bool,
}

impl VolumeModel {
    /// Creates a volume model.
    pub const fn new(value: f64, hovered: bool) -> Self {
        Self { value, hovered }
    }

    /// Returns the knob scale used by the renderer.
    pub const fn knob_scale(&self) -> f32 {
        if self.hovered { 1.15 } else { 1.0 }
    }
}

/// The complete control-bar model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ControlBarModel {
    /// Whether play or pause is currently active.
    pub paused: bool,
    /// Whether the bar is visible.
    pub visible: bool,
    /// Timeline values.
    pub timeline: TimelineModel,
    /// Volume values.
    pub volume: VolumeModel,
}

impl ControlBarModel {
    /// Creates a control-bar model.
    pub const fn new(
        paused: bool,
        visible: bool,
        timeline: TimelineModel,
        volume: VolumeModel,
    ) -> Self {
        Self {
            paused,
            visible,
            timeline,
            volume,
        }
    }

    /// Returns the ordered buttons rendered in the bar.
    pub const fn buttons(&self) -> [ControlButton; 6] {
        [
            ControlButton::PlayPause,
            ControlButton::SkipBack,
            ControlButton::SkipForward,
            ControlButton::Subtitles,
            ControlButton::Settings,
            ControlButton::Fullscreen,
        ]
    }
}

/// Formats a timestamp as `HH:MM:SS` or `MM:SS`.
pub fn format_timestamp(seconds: Seconds) -> String {
    let total = format!("{:.0}", seconds.as_f64().max(0.0))
        .parse::<u64>()
        .unwrap_or(u64::MAX);
    let hours = total / 3600;
    let minutes = (total % 3600) / 60;
    let remaining = total % 60;
    if hours == 0 {
        format!("{minutes:02}:{remaining:02}")
    } else {
        format!("{hours:02}:{minutes:02}:{remaining:02}")
    }
}

#[cfg(test)]
mod tests {
    use super::{ControlBarModel, ControlButton, TimelineModel, VolumeModel, format_timestamp};
    use playback_core::Seconds;

    #[test]
    fn timeline_clamps_progress() {
        let timeline = TimelineModel {
            position: Seconds::new(15.0),
            duration: Some(Seconds::new(10.0)),
            buffered: None,
            seeking: false,
        };
        assert!((timeline.progress() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn volume_knob_scales_on_hover() {
        assert!((VolumeModel::new(50.0, false).knob_scale() - 1.0).abs() < f32::EPSILON);
        assert!((VolumeModel::new(50.0, true).knob_scale() - 1.15).abs() < f32::EPSILON);
    }

    #[test]
    fn control_bar_has_stable_button_order() {
        let bar = ControlBarModel::new(
            false,
            true,
            TimelineModel {
                position: Seconds::new(0.0),
                duration: None,
                buffered: None,
                seeking: false,
            },
            VolumeModel::new(100.0, false),
        );
        assert_eq!(bar.buttons()[0], ControlButton::PlayPause);
        assert_eq!(bar.buttons()[5], ControlButton::Fullscreen);
    }

    #[test]
    fn timestamps_are_human_readable() {
        assert_eq!(format_timestamp(Seconds::new(65.0)), "01:05");
        assert_eq!(format_timestamp(Seconds::new(3661.0)), "01:01:01");
    }
}
