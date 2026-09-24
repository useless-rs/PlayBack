//! mpv-compatible command-line parsing.
//!
//! Unknown options are retained as typed properties or flags so the parser
//! remains forward-compatible with mpv while common PlayBack options are
//! validated at the process boundary.

mod parse;
mod types;

pub use types::{LoopMode, MpvArg, MpvArgs, WindowGeometry};
