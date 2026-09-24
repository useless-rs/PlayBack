//! Core abstractions for the PlayBack media player.
//!
//! The crate separates parsing, state, and backend access so command-line and
//! graphical frontends can share the same behavior without coupling either
//! frontend to a particular media-engine implementation.

#![forbid(unsafe_code)]

pub mod cli;
pub mod error;
pub mod mpv;
pub mod state;

pub use cli::{LoopMode, MpvArg, MpvArgs, WindowGeometry};
pub use error::{CoreError, CoreResult};
pub use mpv::{
    MpvBackend, MpvController, MpvInvocation, MpvProcessBackend, MpvValue, RecordingBackend,
};
pub use state::{
    PlaybackSnapshot, PlaybackState, PlaybackStateHandle, PlaylistItem, Seconds, TrackKind,
    TrackSelection,
};

/// Returns whether this build includes the optional native mpv integration.
///
/// The default build uses the safe process backend and is portable. Native
/// FFI support is reserved for platforms that provide a compatible libmpv.
pub const fn native_mpv_enabled() -> bool {
    cfg!(all(feature = "native-mpv", target_os = "macos"))
}

#[cfg(test)]
mod tests {
    use super::{MpvArgs, native_mpv_enabled};

    #[test]
    fn core_exports_parse_args() {
        let args = MpvArgs::parse_from(["playback", "--volume=80"]).expect("valid args");
        assert_eq!(args.actions.len(), 1);
    }

    #[test]
    fn native_flag_matches_build_configuration() {
        assert_eq!(
            native_mpv_enabled(),
            cfg!(all(feature = "native-mpv", target_os = "macos"))
        );
    }
}
