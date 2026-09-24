//! Typed mpv-style argument values.

use std::fmt;
use std::path::PathBuf;

use crate::error::{CoreError, CoreResult};
use crate::state::Seconds;

use super::parse::{
    is_option, parse_f64, parse_geometry, parse_loop_mode, parse_path, parse_timecode,
    split_option, take_value,
};

/// A parsed PlayBack command line.
#[derive(Debug, Clone, PartialEq)]
pub struct MpvArgs {
    /// Media files and URLs in playback order.
    pub media: Vec<PathBuf>,
    /// Typed options and commands to apply after media loading.
    pub actions: Vec<MpvArg>,
    /// An optional user-provided Lua configuration path.
    pub config: Option<PathBuf>,
}

/// A single typed media-player action.
///
/// Unknown mpv options are retained as [`MpvArg::Property`] or
/// [`MpvArg::Flag`] so callers can forward them to the active backend.
#[derive(Debug, Clone, PartialEq)]
pub enum MpvArg {
    /// Set the mpv `volume` property.
    Volume(f64),
    /// Set the mpv `speed` property.
    Speed(f64),
    /// Seek to an absolute position after loading.
    Start(Seconds),
    /// Configure looping for the current file.
    LoopFile(LoopMode),
    /// Add an external subtitle file.
    SubFile(PathBuf),
    /// Add an external audio track.
    AudioFile(PathBuf),
    /// Disable video output.
    NoVideo,
    /// Start in fullscreen mode.
    Fullscreen,
    /// Keep the player window above other windows.
    OnTop,
    /// Set the initial window geometry.
    Geometry(WindowGeometry),
    /// Set an arbitrary mpv property.
    Property {
        /// The property name without leading dashes.
        name: String,
        /// The raw property value.
        value: String,
    },
    /// Forward a boolean or valueless mpv option.
    Flag(String),
}

/// A validated window geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowGeometry {
    /// Window width in pixels.
    pub width: u32,
    /// Window height in pixels.
    pub height: u32,
}

/// The loop behavior for one media file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopMode {
    /// Do not repeat the file.
    Once,
    /// Repeat the file forever.
    Infinite,
    /// Repeat the file a fixed number of times.
    Count(u32),
}

impl MpvArgs {
    /// Parses arguments from an iterator, excluding the executable name.
    pub fn parse_from<I, S>(arguments: I) -> CoreResult<Self>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut iterator = arguments.into_iter().map(Into::into);
        let mut media = Vec::new();
        let mut actions = Vec::new();
        let mut config = None;
        let mut options_enabled = true;

        while let Some(argument) = iterator.next() {
            if options_enabled && argument == "--" {
                options_enabled = false;
                continue;
            }

            if options_enabled && is_option(&argument) {
                let (name, inline_value) = split_option(&argument);
                let normalized = name.trim_start_matches('-').to_ascii_lowercase();
                match normalized.as_str() {
                    "volume" => {
                        let value = take_value(&name, inline_value, &mut iterator)?;
                        actions.push(MpvArg::Volume(parse_f64(&name, &value)?));
                    }
                    "speed" => {
                        let value = take_value(&name, inline_value, &mut iterator)?;
                        let speed = parse_f64(&name, &value)?;
                        if speed <= 0.0 || !speed.is_finite() {
                            return Err(CoreError::InvalidValue {
                                option: name,
                                value,
                                reason: "speed must be a finite positive number".to_owned(),
                            });
                        }
                        actions.push(MpvArg::Speed(speed));
                    }
                    "start" => {
                        let value = take_value(&name, inline_value, &mut iterator)?;
                        actions.push(MpvArg::Start(parse_timecode(&name, &value)?));
                    }
                    "loop-file" => {
                        let value = take_value(&name, inline_value, &mut iterator)?;
                        actions.push(MpvArg::LoopFile(parse_loop_mode(&name, &value)?));
                    }
                    "sub-file" => {
                        let value = take_value(&name, inline_value, &mut iterator)?;
                        actions.push(MpvArg::SubFile(parse_path(&name, value)?));
                    }
                    "audio-file" => {
                        let value = take_value(&name, inline_value, &mut iterator)?;
                        actions.push(MpvArg::AudioFile(parse_path(&name, value)?));
                    }
                    "geometry" => {
                        let value = take_value(&name, inline_value, &mut iterator)?;
                        actions.push(MpvArg::Geometry(parse_geometry(&name, &value)?));
                    }
                    "config" => {
                        let value = take_value(&name, inline_value, &mut iterator)?;
                        config = Some(parse_path(&name, value)?);
                    }
                    "no-video" => actions.push(MpvArg::NoVideo),
                    "fullscreen" => actions.push(MpvArg::Fullscreen),
                    "ontop" => actions.push(MpvArg::OnTop),
                    _ if inline_value.is_some() => actions.push(MpvArg::Property {
                        name: normalized,
                        value: inline_value.unwrap_or_default(),
                    }),
                    _ => actions.push(MpvArg::Flag(normalized)),
                }
            } else {
                media.push(PathBuf::from(argument));
            }
        }

        Ok(Self {
            media,
            actions,
            config,
        })
    }

    /// Parses the current process arguments.
    pub fn from_env() -> CoreResult<Self> {
        let mut arguments = Vec::new();
        for argument in std::env::args_os().skip(1) {
            let value = argument
                .clone()
                .into_string()
                .map_err(|_| CoreError::NonUnicodeArgument { argument })?;
            arguments.push(value);
        }
        Self::parse_from(arguments)
    }

    /// Returns `true` when no media files were supplied.
    pub fn is_empty(&self) -> bool {
        self.media.is_empty()
    }
}

impl fmt::Display for Seconds {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:.3}", self.0)
    }
}

impl WindowGeometry {
    /// Parses a geometry in mpv's `WIDTHxHEIGHT` form.
    pub fn parse(value: &str) -> CoreResult<Self> {
        parse_geometry("--geometry", value)
    }

    /// Returns the geometry in mpv's textual form.
    pub fn as_mpv_value(self) -> String {
        format!("{}x{}", self.width, self.height)
    }
}

impl LoopMode {
    /// Parses `inf`, `no`, or a non-negative repeat count.
    pub fn parse(value: &str) -> CoreResult<Self> {
        parse_loop_mode("--loop-file", value)
    }
}

#[cfg(test)]
mod tests {
    use super::{LoopMode, MpvArg, MpvArgs, WindowGeometry};
    use crate::error::CoreError;
    use crate::state::Seconds;
    use std::path::PathBuf;

    #[test]
    fn parses_volume_in_equals_form() {
        let args = MpvArgs::parse_from(["--volume=80", "movie.mkv"]).expect("valid args");
        assert_eq!(args.actions, vec![MpvArg::Volume(80.0)]);
        assert_eq!(args.media, vec![PathBuf::from("movie.mkv")]);
    }

    #[test]
    fn parses_legacy_single_dash_form() {
        let args = MpvArgs::parse_from(["-volume", "55", "movie.mkv"]).expect("valid legacy args");
        assert_eq!(args.actions, vec![MpvArg::Volume(55.0)]);
    }

    #[test]
    fn parses_timecode_and_media_options() {
        let args = MpvArgs::parse_from([
            "--start=00:01:30",
            "--sub-file=subs.srt",
            "--audio-file=track2.flac",
            "movie.mkv",
        ])
        .expect("valid media args");
        assert_eq!(args.actions[0], MpvArg::Start(Seconds::new(90.0)));
        assert_eq!(args.actions[1], MpvArg::SubFile(PathBuf::from("subs.srt")));
        assert_eq!(
            args.actions[2],
            MpvArg::AudioFile(PathBuf::from("track2.flac"))
        );
    }

    #[test]
    fn parses_boolean_and_geometry_options() {
        let args = MpvArgs::parse_from([
            "--no-video",
            "--fullscreen",
            "--ontop",
            "--geometry=1280x720",
            "movie.mkv",
        ])
        .expect("valid display args");
        assert_eq!(args.actions[0], MpvArg::NoVideo);
        assert_eq!(args.actions[1], MpvArg::Fullscreen);
        assert_eq!(args.actions[2], MpvArg::OnTop);
        assert_eq!(
            args.actions[3],
            MpvArg::Geometry(WindowGeometry {
                width: 1280,
                height: 720
            })
        );
    }

    #[test]
    fn preserves_unknown_properties_and_flags() {
        let args = MpvArgs::parse_from(["--fs=minimal", "--autoload", "movie.mkv"])
            .expect("forward-compatible args");
        assert_eq!(
            args.actions,
            vec![
                MpvArg::Property {
                    name: "fs".to_owned(),
                    value: "minimal".to_owned(),
                },
                MpvArg::Flag("autoload".to_owned()),
            ]
        );
    }

    #[test]
    fn parses_config_path() {
        let args = MpvArgs::parse_from(["--config=/tmp/init.lua", "movie.mkv"])
            .expect("valid config path");
        assert_eq!(args.config, Some(PathBuf::from("/tmp/init.lua")));
    }

    #[test]
    fn rejects_missing_value() {
        let error = MpvArgs::parse_from(["--volume"]).expect_err("missing value");
        assert!(matches!(error, CoreError::MissingValue { .. }));
    }

    #[test]
    fn parses_loop_modes() {
        let infinite = MpvArgs::parse_from(["--loop-file=inf"])
            .expect("valid infinite loop")
            .actions;
        assert_eq!(infinite, vec![MpvArg::LoopFile(LoopMode::Infinite)]);
        let count = MpvArgs::parse_from(["--loop-file=3"])
            .expect("valid finite loop")
            .actions;
        assert_eq!(count, vec![MpvArg::LoopFile(LoopMode::Count(3))]);
    }

    #[test]
    fn rejects_invalid_geometry() {
        let error = MpvArgs::parse_from(["--geometry=0x0"]).expect_err("invalid geometry");
        assert!(matches!(error, CoreError::InvalidValue { .. }));
    }
}
