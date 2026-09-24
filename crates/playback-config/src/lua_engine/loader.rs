//! Lua source evaluation, schema conversion, and unknown-key diagnostics.

use std::path::{Path, PathBuf};

use mlua::{Lua, LuaSerdeExt, Value};
use thiserror::Error;

use crate::schema::PlaybackConfig;

/// Errors produced while loading or watching configuration.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ConfigError {
    /// A configuration file could not be read.
    #[error("could not read configuration file {path}: {source}")]
    Read {
        /// The path being read.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },
    /// Lua source could not be evaluated or serialized.
    #[error("Lua configuration error: {0}")]
    Lua(#[from] mlua::Error),
    /// The Lua document did not match the typed schema.
    #[error("configuration schema error: {0}")]
    Schema(#[from] serde_json::Error),
    /// The file watcher could not be created or configured.
    #[error("configuration watcher error: {0}")]
    Watch(#[from] notify::Error),
    /// A watcher was requested without a source path.
    #[error("a configuration path is required for watching")]
    MissingPath,
}

/// The result of loading a configuration document.
#[derive(Debug, Clone, PartialEq)]
pub struct LoadedConfig {
    /// The typed configuration.
    pub config: PlaybackConfig,
    /// Non-fatal diagnostics for unknown keys.
    pub warnings: Vec<String>,
}

/// Loads the bundled `defaults.lua` document.
pub fn load_builtin() -> Result<LoadedConfig, ConfigError> {
    load_source(include_str!("../defaults.lua"), "playback:defaults.lua")
}

/// Loads a Lua document from a file.
pub fn load_from_path(path: &Path) -> Result<LoadedConfig, ConfigError> {
    let source = std::fs::read_to_string(path).map_err(|source| ConfigError::Read {
        path: path.to_path_buf(),
        source,
    })?;
    load_source(&source, &path.to_string_lossy())
}

/// Evaluates a Lua document and converts it to the typed schema.
pub fn load_source(source: &str, source_name: &str) -> Result<LoadedConfig, ConfigError> {
    let lua = Lua::new();
    let value: Value = lua.load(source).set_name(source_name).eval()?;
    let json: serde_json::Value = lua.from_value(value)?;
    let config: PlaybackConfig = serde_json::from_value(json.clone())?;
    let warnings = unknown_key_warnings(&json);
    for warning in &warnings {
        tracing::warn!(warning = %warning, "unknown configuration key");
    }
    Ok(LoadedConfig { config, warnings })
}

/// Returns the conventional user configuration path without creating it.
pub fn default_config_path() -> Option<PathBuf> {
    if let Some(path) = std::env::var_os("PLAYBACK_CONFIG") {
        return Some(PathBuf::from(path));
    }
    let home = std::env::var_os("HOME")?;
    Some(
        PathBuf::from(home)
            .join(".config")
            .join("playback")
            .join("init.lua"),
    )
}

fn unknown_key_warnings(value: &serde_json::Value) -> Vec<String> {
    let mut warnings = Vec::new();
    collect_unknown_keys(value, "", known_keys(""), &mut warnings);
    warnings
}

fn collect_unknown_keys(
    value: &serde_json::Value,
    prefix: &str,
    known: &[&str],
    warnings: &mut Vec<String>,
) {
    let Some(object) = value.as_object() else {
        return;
    };
    for (key, child) in object {
        let path = if prefix.is_empty() {
            key.clone()
        } else {
            format!("{prefix}.{key}")
        };
        if !known.contains(&key.as_str()) {
            warnings.push(path);
            continue;
        }
        if key == "custom" {
            continue;
        }
        collect_unknown_keys(child, &path, known_keys(key), warnings);
    }
}

fn known_keys(section: &str) -> &'static [&'static str] {
    match section {
        "playback" => &["volume", "speed", "loop", "autoplay", "start_paused"],
        "video" => &["hardware_decoding", "scale", "deinterlace", "aspect_ratio"],
        "audio" => &["channel_layout", "normalize"],
        "subtitles" => &[
            "enabled",
            "font",
            "font_size",
            "color",
            "border_color",
            "border_size",
        ],
        "interface" => &[
            "theme",
            "animations",
            "blur_background",
            "show_playlist",
            "show_controls",
            "controls_timeout_ms",
        ],
        "keybindings" => &[
            "toggle_play",
            "seek_forward",
            "seek_backward",
            "volume_up",
            "volume_down",
            "fullscreen",
            "quit",
            "next_track",
            "prev_track",
        ],
        "custom" => &[],
        _ => &[
            "playback",
            "video",
            "audio",
            "subtitles",
            "interface",
            "keybindings",
            "custom",
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::{load_builtin, load_source};

    #[test]
    fn bundled_defaults_populate_every_field() {
        let loaded = load_builtin().expect("bundled defaults are valid");
        assert!((loaded.config.playback().volume() - 100.0).abs() < f64::EPSILON);
        assert!((loaded.config.playback().speed() - 1.0).abs() < f64::EPSILON);
        assert!(!loaded.config.playback().r#loop);
        assert!(loaded.config.playback().autoplay);
        assert_eq!(loaded.config.video().hardware_decoding(), "auto-safe");
        assert_eq!(loaded.config.audio().channel_layout(), "auto");
        assert!(loaded.config.subtitles().enabled());
        assert_eq!(loaded.config.interface().controls_timeout_ms(), 2500);
        assert!(loaded.warnings.is_empty());
    }

    #[test]
    fn unknown_keys_warn_without_failing() {
        let loaded = load_source(
            "return { playback = { volume = 50, mystery = true }, custom = { x = 1 } }",
            "test.lua",
        )
        .expect("unknown keys are non-fatal");
        assert!((loaded.config.playback().volume() - 50.0).abs() < f64::EPSILON);
        assert_eq!(loaded.warnings, vec!["playback.mystery"]);
    }
}
