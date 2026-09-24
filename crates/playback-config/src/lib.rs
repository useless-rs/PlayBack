//! Lua-backed configuration for PlayBack.
//!
//! Configuration is intentionally data-oriented: Lua evaluates to a table,
//! the table is converted to JSON, and the typed [`PlaybackConfig`] schema is
//! the only value exposed to the rest of the application.

#![forbid(unsafe_code)]

pub mod lua_engine;
pub mod schema;

pub use lua_engine::{
    ConfigEngine, ConfigError, LoadedConfig, default_config_path, load_builtin, load_from_path,
    load_source,
};
pub use schema::{
    AudioSettings, InterfaceSettings, Keybindings, PlaybackConfig, PlaybackSettings,
    SubtitleSettings, VideoSettings,
};

#[cfg(test)]
mod tests {
    use super::{PlaybackConfig, load_builtin};

    #[test]
    fn bundled_config_is_parseable() {
        let config = load_builtin().expect("bundled config is valid").config;
        assert_eq!(config, PlaybackConfig::default());
    }
}
