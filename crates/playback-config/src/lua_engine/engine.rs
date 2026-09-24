//! Configuration ownership and file watching.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use parking_lot::RwLock;

use crate::schema::PlaybackConfig;

use super::loader::{ConfigError, LoadedConfig, load_builtin, load_from_path};

/// A thread-safe configuration store with optional file watching.
pub struct ConfigEngine {
    path: Option<PathBuf>,
    config: Arc<RwLock<PlaybackConfig>>,
    watcher: Option<RecommendedWatcher>,
}

impl std::fmt::Debug for ConfigEngine {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ConfigEngine")
            .field("path", &self.path)
            .field("config", &self.config.read())
            .field("watching", &self.watcher.is_some())
            .finish()
    }
}

impl Default for ConfigEngine {
    fn default() -> Self {
        Self {
            path: None,
            config: Arc::new(RwLock::new(PlaybackConfig::default())),
            watcher: None,
        }
    }
}

impl ConfigEngine {
    /// Creates an engine from an optional user path.
    ///
    /// Passing `None` uses the bundled defaults without touching the user's
    /// filesystem.
    pub fn from_path(path: Option<PathBuf>) -> Result<Self, ConfigError> {
        let loaded = match &path {
            Some(path) => load_from_path(path)?,
            None => load_builtin()?,
        };
        Ok(Self {
            path,
            config: Arc::new(RwLock::new(loaded.config)),
            watcher: None,
        })
    }

    /// Returns the configuration currently held by the engine.
    pub fn config(&self) -> PlaybackConfig {
        self.config.read().clone()
    }

    /// Returns the path being watched, if any.
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    /// Reloads the configured path and replaces the current document.
    pub fn reload(&mut self) -> Result<LoadedConfig, ConfigError> {
        let path = self.path.as_ref().ok_or(ConfigError::MissingPath)?;
        let loaded = load_from_path(path)?;
        *self.config.write() = loaded.config.clone();
        Ok(loaded)
    }

    /// Starts watching the configured file and invokes `callback` after reloads.
    pub fn watch(
        &mut self,
        callback: impl Fn(&PlaybackConfig) + Send + Sync + 'static,
    ) -> Result<(), ConfigError> {
        let path = self.path.clone().ok_or(ConfigError::MissingPath)?;
        let config = Arc::clone(&self.config);
        let callback = Arc::new(callback);
        let watch_path = path.clone();
        let mut watcher = notify::recommended_watcher(
            move |event: notify::Result<notify::Event>| {
                if event.is_err() {
                    return;
                }
                match load_from_path(&watch_path) {
                    Ok(loaded) => {
                        callback(&loaded.config);
                        *config.write() = loaded.config;
                    }
                    Err(error) => {
                        tracing::warn!(error = %error, path = %watch_path.display(), "configuration reload failed");
                    }
                }
            },
        )?;
        watcher.watch(&path, RecursiveMode::NonRecursive)?;
        self.watcher = Some(watcher);
        Ok(())
    }

    /// Stops file watching, if active.
    pub fn stop_watching(&mut self) {
        self.watcher = None;
    }
}

#[cfg(test)]
mod tests {
    use super::ConfigEngine;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn engine_reloads_a_file() {
        let path = std::env::temp_dir().join(format!("playback-config-{}.lua", std::process::id()));
        fs::write(&path, "return { playback = { volume = 25 } }").expect("write config");
        let mut engine = ConfigEngine::from_path(Some(path.clone())).expect("load config");
        assert!((engine.config().playback().volume() - 25.0).abs() < f64::EPSILON);
        fs::write(&path, "return { playback = { volume = 75 } }").expect("rewrite config");
        engine.reload().expect("reload config");
        assert!((engine.config().playback().volume() - 75.0).abs() < f64::EPSILON);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn default_engine_is_usable_without_a_file() {
        let engine = ConfigEngine::default();
        assert_eq!(engine.config().interface().theme, "macos");
        assert!(engine.path().is_none());
    }

    #[test]
    fn path_type_is_stable() {
        let path: PathBuf = PathBuf::from("init.lua");
        assert_eq!(
            path.file_name().and_then(|name| name.to_str()),
            Some("init.lua")
        );
    }
}
