//! Playback state shared by the CLI, media backend, and UI model.

use std::sync::Arc;

use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

/// A non-negative media position in seconds.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Seconds(pub f64);

impl Seconds {
    /// Creates a duration or position from seconds.
    pub const fn new(value: f64) -> Self {
        Self(value)
    }

    /// Returns the value in seconds.
    pub const fn as_f64(self) -> f64 {
        self.0
    }

    /// Returns `true` when the value is finite and non-negative.
    pub const fn is_valid(self) -> bool {
        self.0.is_finite() && self.0 >= 0.0
    }
}

/// The kind of media track represented by a selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrackKind {
    /// An audio track.
    Audio,
    /// A subtitle track.
    Subtitle,
    /// A video track.
    Video,
}

/// A selected external or embedded track.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrackSelection {
    /// The selected track kind.
    pub kind: TrackKind,
    /// The backend-specific track identifier.
    pub id: String,
}

/// One item in the current playlist.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaylistItem {
    /// The file or URL represented by this item.
    pub path: String,
    /// A display title, usually derived from the file name.
    pub title: String,
    /// A known duration, when metadata is available.
    pub duration: Option<Seconds>,
}

/// An immutable view of playback state suitable for rendering or IPC.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaybackSnapshot {
    /// Whether playback is paused.
    pub paused: bool,
    /// Whether a seek operation is active.
    pub seeking: bool,
    /// Current playback position.
    pub position: Seconds,
    /// Current media duration, when known.
    pub duration: Option<Seconds>,
    /// Buffered range, when known.
    pub buffered: Option<(Seconds, Seconds)>,
    /// Current volume from 0 to 100.
    pub volume: f64,
    /// Current playback speed.
    pub speed: f64,
    /// Whether the current file repeats.
    pub loop_file: bool,
    /// Current audio track.
    pub audio_track: Option<TrackSelection>,
    /// Current subtitle track.
    pub subtitle_track: Option<TrackSelection>,
    /// Current playlist.
    pub playlist: Vec<PlaylistItem>,
}

/// The mutable playback state owned by the media thread.
#[derive(Debug, Clone)]
pub struct PlaybackState {
    paused: bool,
    seeking: bool,
    position: Seconds,
    duration: Option<Seconds>,
    buffered: Option<(Seconds, Seconds)>,
    volume: f64,
    speed: f64,
    loop_file: bool,
    audio_track: Option<TrackSelection>,
    subtitle_track: Option<TrackSelection>,
    playlist: Vec<PlaylistItem>,
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            paused: false,
            seeking: false,
            position: Seconds::new(0.0),
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
}

impl PlaybackState {
    /// Creates an empty state with sensible playback defaults.
    pub const fn new() -> Self {
        Self {
            paused: false,
            seeking: false,
            position: Seconds::new(0.0),
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

    /// Returns a serializable snapshot.
    pub fn snapshot(&self) -> PlaybackSnapshot {
        PlaybackSnapshot {
            paused: self.paused,
            seeking: self.seeking,
            position: self.position,
            duration: self.duration,
            buffered: self.buffered,
            volume: self.volume,
            speed: self.speed,
            loop_file: self.loop_file,
            audio_track: self.audio_track.clone(),
            subtitle_track: self.subtitle_track.clone(),
            playlist: self.playlist.clone(),
        }
    }

    /// Marks the beginning of a media load and clears transient state.
    pub fn begin_load(&mut self, duration: Option<Seconds>) {
        self.paused = false;
        self.seeking = false;
        self.position = Seconds::new(0.0);
        self.duration = duration.filter(|value| value.is_valid());
        self.buffered = None;
        self.audio_track = None;
        self.subtitle_track = None;
    }

    /// Updates the current position.
    pub fn set_position(&mut self, position: Seconds) -> bool {
        if !position.is_valid() {
            return false;
        }
        self.position = position;
        true
    }

    /// Updates the buffered range.
    pub fn set_buffered(&mut self, start: Seconds, end: Seconds) -> bool {
        if !start.is_valid() || !end.is_valid() || end < start {
            return false;
        }
        self.buffered = Some((start, end));
        true
    }

    /// Toggles paused state and returns the new value.
    pub fn toggle_pause(&mut self) -> bool {
        self.paused = !self.paused;
        self.paused
    }

    /// Sets the paused state.
    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    /// Sets the seeking state.
    pub fn set_seeking(&mut self, seeking: bool) {
        self.seeking = seeking;
    }

    /// Sets volume, clamped to mpv's 0–100 range.
    pub fn set_volume(&mut self, volume: f64) -> bool {
        if !volume.is_finite() {
            return false;
        }
        self.volume = volume.clamp(0.0, 100.0);
        true
    }

    /// Sets playback speed, returning `false` for invalid values.
    pub fn set_speed(&mut self, speed: f64) -> bool {
        if !speed.is_finite() || speed <= 0.0 {
            return false;
        }
        self.speed = speed;
        true
    }

    /// Enables or disables file looping.
    pub fn set_loop_file(&mut self, enabled: bool) {
        self.loop_file = enabled;
    }

    /// Selects a track, replacing the previous track of the same kind.
    pub fn select_track(&mut self, selection: TrackSelection) {
        match selection.kind {
            TrackKind::Audio => self.audio_track = Some(selection),
            TrackKind::Subtitle => self.subtitle_track = Some(selection),
            TrackKind::Video => {}
        }
    }

    /// Adds an item to the end of the playlist.
    pub fn add_playlist_item(&mut self, item: PlaylistItem) {
        self.playlist.push(item);
    }

    /// Moves a playlist item to a new index, returning `false` for invalid indices.
    pub fn move_playlist_item(&mut self, from: usize, to: usize) -> bool {
        if from >= self.playlist.len() || to >= self.playlist.len() {
            return false;
        }
        let item = self.playlist.remove(from);
        self.playlist.insert(to, item);
        true
    }

    /// Returns the current snapshot without exposing mutable internals.
    pub fn current_snapshot(&self) -> PlaybackSnapshot {
        self.snapshot()
    }
}

/// A thread-safe handle to the shared playback state.
#[derive(Debug, Clone)]
pub struct PlaybackStateHandle {
    inner: Arc<Mutex<PlaybackState>>,
}

impl Default for PlaybackStateHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl PlaybackStateHandle {
    /// Creates a shared state handle.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(PlaybackState::new())),
        }
    }

    /// Executes a read operation under the state lock.
    pub fn with_state<R>(&self, operation: impl FnOnce(&PlaybackState) -> R) -> R {
        operation(&self.inner.lock())
    }

    /// Executes a mutation under the state lock.
    pub fn with_state_mut<R>(&self, operation: impl FnOnce(&mut PlaybackState) -> R) -> R {
        operation(&mut self.inner.lock())
    }

    /// Returns a consistent snapshot under the state lock.
    pub fn snapshot(&self) -> PlaybackSnapshot {
        self.inner.lock().current_snapshot()
    }
}

#[cfg(test)]
mod tests {
    use super::{PlaybackState, PlaybackStateHandle, Seconds, TrackKind, TrackSelection};

    #[test]
    fn defaults_are_playable() {
        let state = PlaybackState::new();
        assert!(!state.current_snapshot().paused);
        assert!((state.current_snapshot().volume - 100.0).abs() < f64::EPSILON);
        assert!((state.current_snapshot().speed - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn rejects_invalid_time_values() {
        let mut state = PlaybackState::new();
        assert!(!state.set_position(Seconds::new(-1.0)));
        assert!(!state.set_buffered(Seconds::new(3.0), Seconds::new(2.0)));
    }

    #[test]
    fn clamps_volume_and_rejects_invalid_speed() {
        let mut state = PlaybackState::new();
        assert!(state.set_volume(140.0));
        assert!((state.current_snapshot().volume - 100.0).abs() < f64::EPSILON);
        assert!(!state.set_speed(0.0));
        assert!(state.set_speed(1.5));
        assert!((state.current_snapshot().speed - 1.5).abs() < f64::EPSILON);
    }

    #[test]
    fn replaces_tracks_by_kind() {
        let mut state = PlaybackState::new();
        state.select_track(TrackSelection {
            kind: TrackKind::Audio,
            id: "a-1".to_owned(),
        });
        state.select_track(TrackSelection {
            kind: TrackKind::Audio,
            id: "a-2".to_owned(),
        });
        let snapshot = state.current_snapshot();
        assert_eq!(
            snapshot.audio_track.map(|track| track.id),
            Some("a-2".to_owned())
        );
        assert!(snapshot.subtitle_track.is_none());
    }

    #[test]
    fn shared_handle_synchronizes_mutations() {
        let handle = PlaybackStateHandle::new();
        let writer = handle.clone();
        let thread = std::thread::spawn(move || {
            writer.with_state_mut(|state| state.set_paused(true));
        });
        thread.join().expect("writer thread should finish");
        assert!(handle.with_state(|state| state.current_snapshot().paused));
    }
}
