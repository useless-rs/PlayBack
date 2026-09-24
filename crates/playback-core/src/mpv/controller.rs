//! Typed action controller and shared-state synchronization.

use std::path::Path;

use crate::cli::{LoopMode, MpvArg};
use crate::error::{CoreError, CoreResult};
use crate::state::{PlaybackStateHandle, TrackKind, TrackSelection};

use super::{MpvBackend, MpvValue};

/// A controller that translates typed actions into backend operations and keeps
/// the shared playback state synchronized.
#[derive(Debug)]
pub struct MpvController<B> {
    backend: B,
    state: PlaybackStateHandle,
}

impl<B> MpvController<B>
where
    B: MpvBackend,
{
    /// Creates a controller with a backend and shared state.
    pub const fn new(backend: B, state: PlaybackStateHandle) -> Self {
        Self { backend, state }
    }

    /// Returns the backend reference.
    pub const fn backend(&self) -> &B {
        &self.backend
    }

    /// Applies one parsed action.
    pub fn apply(&mut self, action: &MpvArg) -> CoreResult<()> {
        match action {
            MpvArg::Volume(volume) => {
                self.backend
                    .set_property("volume", MpvValue::Number(*volume))?;
                self.state.with_state_mut(|state| state.set_volume(*volume));
            }
            MpvArg::Speed(speed) => {
                self.backend
                    .set_property("speed", MpvValue::Number(*speed))?;
                self.state.with_state_mut(|state| state.set_speed(*speed));
            }
            MpvArg::Start(position) => {
                self.backend.command(
                    "seek",
                    &[position.as_f64().to_string(), "absolute".to_owned()],
                )?;
                self.state
                    .with_state_mut(|state| state.set_position(*position));
            }
            MpvArg::LoopFile(mode) => {
                let value = match mode {
                    LoopMode::Once => "no".to_owned(),
                    LoopMode::Infinite => "inf".to_owned(),
                    LoopMode::Count(count) => count.to_string(),
                };
                self.backend
                    .set_property("loop-file", MpvValue::Text(value))?;
                self.state
                    .with_state_mut(|state| state.set_loop_file(*mode != LoopMode::Once));
            }
            MpvArg::SubFile(path) => {
                self.backend.command(
                    "sub-add",
                    &[path.to_string_lossy().into_owned(), "select".to_owned()],
                )?;
            }
            MpvArg::AudioFile(path) => {
                self.backend.command(
                    "audio-add",
                    &[path.to_string_lossy().into_owned(), "select".to_owned()],
                )?;
            }
            MpvArg::NoVideo => {
                self.backend
                    .set_property("vid", MpvValue::Text("no".to_owned()))?;
            }
            MpvArg::Fullscreen => {
                self.backend
                    .set_property("fullscreen", MpvValue::Boolean(true))?;
            }
            MpvArg::OnTop => {
                self.backend
                    .set_property("ontop", MpvValue::Boolean(true))?;
            }
            MpvArg::Geometry(geometry) => {
                self.backend
                    .set_property("geometry", MpvValue::Text(geometry.as_mpv_value()))?;
            }
            MpvArg::Property { name, value } => {
                self.backend
                    .set_property(name, MpvValue::Text(value.clone()))?;
            }
            MpvArg::Flag(name) => {
                self.backend.set_property(name, MpvValue::Boolean(true))?;
            }
        }
        Ok(())
    }

    /// Applies all actions in order.
    pub fn apply_all(&mut self, actions: &[MpvArg]) -> CoreResult<()> {
        for action in actions {
            self.apply(action)?;
        }
        Ok(())
    }

    /// Loads the first media path and records it in the playlist.
    pub fn load_media(&mut self, path: &Path) -> CoreResult<()> {
        if path.as_os_str().is_empty() {
            return Err(CoreError::EmptyPath);
        }
        self.backend.load_file(path)?;
        self.state.with_state_mut(|state| {
            state.begin_load(None);
            state.add_playlist_item(crate::state::PlaylistItem {
                path: path.to_string_lossy().into_owned(),
                title: path.file_name().map_or_else(
                    || path.to_string_lossy().into_owned(),
                    |name| name.to_string_lossy().into_owned(),
                ),
                duration: None,
            });
        });
        Ok(())
    }

    /// Selects a track in both the backend and shared state.
    pub fn select_track(&mut self, selection: TrackSelection) -> CoreResult<()> {
        let property = match selection.kind {
            TrackKind::Audio => "aid",
            TrackKind::Subtitle => "sid",
            TrackKind::Video => "vid",
        };
        self.backend
            .set_property(property, MpvValue::Text(selection.id.clone()))?;
        self.state
            .with_state_mut(|state| state.select_track(selection));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::MpvController;
    use crate::cli::MpvArgs;
    use crate::mpv::RecordingBackend;
    use crate::state::{PlaybackStateHandle, TrackKind, TrackSelection};

    #[test]
    fn translates_actions_to_backend_operations() {
        let args =
            MpvArgs::parse_from(["--volume=80", "--start=00:01:30", "--no-video", "movie.mkv"])
                .expect("valid args");
        let state = PlaybackStateHandle::new();
        let mut controller = MpvController::new(RecordingBackend::new(), state.clone());
        controller.apply_all(&args.actions).expect("actions apply");
        let snapshot = state.snapshot();
        assert!((snapshot.volume - 80.0).abs() < f64::EPSILON);
        assert!((snapshot.position.as_f64() - 90.0).abs() < f64::EPSILON);
        assert!(
            controller
                .backend()
                .operations()
                .iter()
                .any(|operation| operation == "set volume=80")
        );
        assert!(
            controller
                .backend()
                .operations()
                .iter()
                .any(|operation| operation == "command seek 90 absolute")
        );
    }

    #[test]
    fn selects_tracks_through_backend() {
        let mut controller =
            MpvController::new(RecordingBackend::new(), PlaybackStateHandle::new());
        controller
            .select_track(TrackSelection {
                kind: TrackKind::Subtitle,
                id: "sub-1".to_owned(),
            })
            .expect("track selection");
        assert!(
            controller
                .backend()
                .operations()
                .iter()
                .any(|operation| operation == "set sid=sub-1")
        );
    }
}
