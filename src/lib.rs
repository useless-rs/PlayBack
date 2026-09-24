//! Public facade for the PlayBack workspace.
//!
//! The root crate intentionally stays small. The implementation lives in the
//! focused `playback-core`, `playback-config`, `playback-ui`, and
//! `playback-cli` crates so applications can depend on only the pieces they
//! need.

/// The command-line application entry point.
pub use playback_cli as cli;
/// Lua-backed configuration types and loading support.
pub use playback_config as config;
/// Core playback types and command-line parsing.
pub use playback_core as core;
/// Platform-neutral UI models and integration boundaries.
pub use playback_ui as ui;
