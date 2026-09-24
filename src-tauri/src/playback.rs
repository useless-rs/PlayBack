use std::io::Write;
use std::path::Path;
use std::process::{Child, ChildStdin, Command, Stdio};

use anyhow::{Context, Result};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, State};

use playback_config::{ConfigEngine, PlaybackConfig};

const STATE_EVENT: &str = "playback://state";

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FrontendSnapshot {
    pub paused: bool,
    pub position: f64,
    pub duration: Option<f64>,
    pub volume: f64,
    pub speed: f64,
    pub loop_file: bool,
    pub subtitle_visible: bool,
    pub fullscreen: bool,
    pub media_title: String,
    pub media_path: Option<String>,
    pub backend: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaItem {
    pub id: String,
    pub path: String,
    pub title: String,
    pub subtitle: String,
    pub kind: String,
    pub duration: String,
    pub added_at: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaybackStateEvent {
    pub snapshot: FrontendSnapshot,
    pub playlist: Vec<MediaItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub theme: String,
    pub animations: bool,
    pub blur_background: bool,
    pub show_playlist: bool,
    pub show_controls: bool,
    pub hardware_decoding: String,
    pub scale: String,
    pub subtitle_size: u32,
    pub subtitle_font: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "dark".to_owned(),
            animations: true,
            blur_background: true,
            show_playlist: true,
            show_controls: true,
            hardware_decoding: "auto-safe".to_owned(),
            scale: "bilinear".to_owned(),
            subtitle_size: 52,
            subtitle_font: "SF Pro Display".to_owned(),
        }
    }
}

impl AppConfig {
    fn from_playback(config: &PlaybackConfig) -> Self {
        Self {
            theme: if config.interface().theme.eq_ignore_ascii_case("light") {
                "light".to_owned()
            } else {
                "dark".to_owned()
            },
            animations: config.interface().animations,
            blur_background: config.interface().blur_background,
            show_playlist: config.interface().show_playlist,
            show_controls: config.interface().show_controls,
            hardware_decoding: config.video().hardware_decoding.clone(),
            scale: config.video().scale.clone(),
            subtitle_size: config.subtitles().font_size,
            subtitle_font: config.subtitles().font.clone(),
        }
    }
}

struct MpvSession {
    child: Child,
    stdin: ChildStdin,
}

impl MpvSession {
    fn start(paths: &[String]) -> Result<Self> {
        let mut child = Command::new("mpv")
            .args([
                "--idle=yes",
                "--force-window=yes",
                "--keep-open=yes",
                "--no-terminal",
                "--input-terminal",
                "--msg-level=all=no",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .context("could not start mpv")?;
        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| anyhow::anyhow!("mpv stdin was not available"))?;
        let mut session = Self { child, stdin };
        if let Some(path) = paths.first() {
            session.load_path(path)?;
        }
        Ok(session)
    }

    fn send(&mut self, command: &[&str]) -> Result<()> {
        let line = format!("{}\n", command.join(" "));
        self.stdin
            .write_all(line.as_bytes())
            .context("could not send command to mpv")?;
        self.stdin.flush().context("could not flush mpv command")
    }

    fn load_path(&mut self, path: &str) -> Result<()> {
        self.send(&["loadfile", path, "replace"])
    }

    fn shutdown(&mut self) {
        let _ = self.send(&["quit"]);
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

impl Drop for MpvSession {
    fn drop(&mut self) {
        self.shutdown();
    }
}

pub struct BackendState {
    session: Mutex<Option<MpvSession>>,
    snapshot: Mutex<FrontendSnapshot>,
    playlist: Mutex<Vec<MediaItem>>,
    config: Mutex<AppConfig>,
}

impl BackendState {
    pub fn new() -> Self {
        let config = ConfigEngine::from_path(
            playback_config::default_config_path().filter(|path| path.is_file()),
        )
        .map(|engine| AppConfig::from_playback(&engine.config()))
        .unwrap_or_default();
        Self {
            session: Mutex::new(None),
            snapshot: Mutex::new(FrontendSnapshot {
                paused: true,
                position: 0.0,
                duration: None,
                volume: 100.0,
                speed: 1.0,
                loop_file: false,
                subtitle_visible: true,
                fullscreen: false,
                media_title: "Nothing playing".to_owned(),
                media_path: None,
                backend: "mpv".to_owned(),
            }),
            playlist: Mutex::new(Vec::new()),
            config: Mutex::new(config),
        }
    }

    fn event(&self) -> PlaybackStateEvent {
        PlaybackStateEvent {
            snapshot: self.snapshot.lock().clone(),
            playlist: self.playlist.lock().clone(),
        }
    }
}

impl Default for BackendState {
    fn default() -> Self {
        Self::new()
    }
}

fn media_item(path: &str, index: usize) -> MediaItem {
    let title = Path::new(path).file_name().map_or_else(
        || path.to_owned(),
        |name| name.to_string_lossy().into_owned(),
    );
    MediaItem {
        id: format!("{path}-{index}"),
        path: path.to_owned(),
        title,
        subtitle: "Added from your library".to_owned(),
        kind: "video".to_owned(),
        duration: "--:--".to_owned(),
        added_at: "Just now".to_owned(),
    }
}

fn update_media(snapshot: &mut FrontendSnapshot, path: &str) {
    snapshot.media_path = Some(path.to_owned());
    snapshot.media_title = Path::new(path).file_name().map_or_else(
        || path.to_owned(),
        |name| name.to_string_lossy().into_owned(),
    );
    snapshot.position = 0.0;
    snapshot.paused = false;
    snapshot.backend = "mpv".to_owned();
}

fn emit_state(app: &AppHandle, state: &BackendState) {
    if let Err(error) = app.emit(STATE_EVENT, state.event()) {
        eprintln!("PlayBack could not emit playback state: {error}");
    }
}

fn ensure_session(state: &BackendState, paths: &[String]) -> Result<()> {
    let mut session = state.session.lock();
    if let Some(current) = session.as_mut() {
        if let Some(path) = paths.first() {
            current.load_path(path)?;
        }
    } else {
        *session = Some(MpvSession::start(paths)?);
    }
    Ok(())
}

fn load_existing(
    app: &AppHandle,
    state: &BackendState,
    path: String,
) -> Result<PlaybackStateEvent, String> {
    ensure_session(state, std::slice::from_ref(&path)).map_err(|error| error.to_string())?;
    update_media(&mut state.snapshot.lock(), &path);
    let event = state.event();
    emit_state(app, state);
    Ok(event)
}

#[tauri::command]
pub fn get_playback_snapshot(state: State<'_, BackendState>) -> FrontendSnapshot {
    state.snapshot.lock().clone()
}

#[tauri::command]
pub fn get_app_config(state: State<'_, BackendState>) -> AppConfig {
    state.config.lock().clone()
}

#[tauri::command]
pub fn save_app_config(state: State<'_, BackendState>, config: AppConfig) -> AppConfig {
    *state.config.lock() = config.clone();
    config
}

#[tauri::command]
pub fn open_media(
    app: AppHandle,
    state: State<'_, BackendState>,
    paths: Vec<String>,
) -> Result<PlaybackStateEvent, String> {
    if paths.is_empty() {
        return Err("no media path was provided".to_owned());
    }
    ensure_session(&state, &paths).map_err(|error| error.to_string())?;
    let items = paths
        .iter()
        .enumerate()
        .map(|(index, path)| media_item(path, index))
        .collect::<Vec<_>>();
    {
        let mut playlist = state.playlist.lock();
        for item in items {
            if !playlist.iter().any(|existing| existing.path == item.path) {
                playlist.push(item);
            }
        }
    }
    {
        let mut snapshot = state.snapshot.lock();
        update_media(&mut snapshot, &paths[0]);
    }
    let event = state.event();
    emit_state(&app, &state);
    Ok(event)
}

#[tauri::command]
pub fn toggle_playback(
    app: AppHandle,
    state: State<'_, BackendState>,
) -> Result<PlaybackStateEvent, String> {
    let paused = {
        let snapshot = state.snapshot.lock();
        snapshot.paused
    };
    if let Some(session) = state.session.lock().as_mut() {
        let value = if paused { "no" } else { "yes" };
        session
            .send(&["set", "pause", value])
            .map_err(|error| error.to_string())?;
    }
    state.snapshot.lock().paused = !paused;
    let event = state.event();
    emit_state(&app, &state);
    Ok(event)
}

#[tauri::command]
pub fn seek_relative(
    app: AppHandle,
    state: State<'_, BackendState>,
    seconds: f64,
) -> Result<PlaybackStateEvent, String> {
    if let Some(session) = state.session.lock().as_mut() {
        session
            .send(&["seek", &seconds.to_string(), "relative"])
            .map_err(|error| error.to_string())?;
    }
    {
        let mut snapshot = state.snapshot.lock();
        let maximum = snapshot.duration.unwrap_or(f64::MAX);
        snapshot.position = (snapshot.position + seconds).clamp(0.0, maximum);
    }
    let event = state.event();
    emit_state(&app, &state);
    Ok(event)
}

#[tauri::command]
pub fn seek_absolute(
    app: AppHandle,
    state: State<'_, BackendState>,
    seconds: f64,
) -> Result<PlaybackStateEvent, String> {
    if let Some(session) = state.session.lock().as_mut() {
        session
            .send(&["seek", &seconds.to_string(), "absolute"])
            .map_err(|error| error.to_string())?;
    }
    state.snapshot.lock().position = seconds.max(0.0);
    let event = state.event();
    emit_state(&app, &state);
    Ok(event)
}

#[tauri::command]
pub fn set_volume(
    app: AppHandle,
    state: State<'_, BackendState>,
    volume: f64,
) -> Result<PlaybackStateEvent, String> {
    let volume = volume.clamp(0.0, 100.0);
    if let Some(session) = state.session.lock().as_mut() {
        session
            .send(&["set", "volume", &volume.to_string()])
            .map_err(|error| error.to_string())?;
    }
    state.snapshot.lock().volume = volume;
    let event = state.event();
    emit_state(&app, &state);
    Ok(event)
}

#[tauri::command]
pub fn toggle_fullscreen(
    app: AppHandle,
    state: State<'_, BackendState>,
) -> Result<PlaybackStateEvent, String> {
    let fullscreen = !state.snapshot.lock().fullscreen;
    if let Some(session) = state.session.lock().as_mut() {
        session
            .send(&["set", "fullscreen", if fullscreen { "yes" } else { "no" }])
            .map_err(|error| error.to_string())?;
    }
    state.snapshot.lock().fullscreen = fullscreen;
    let event = state.event();
    emit_state(&app, &state);
    Ok(event)
}

#[tauri::command]
pub fn next_track(
    app: AppHandle,
    state: State<'_, BackendState>,
) -> Result<PlaybackStateEvent, String> {
    let current = state.snapshot.lock().media_path.clone();
    let next = state
        .playlist
        .lock()
        .iter()
        .find(|item| Some(&item.path) != current.as_ref())
        .map(|item| item.path.clone());
    if let Some(path) = next {
        return load_existing(&app, &state, path);
    }
    Ok(state.event())
}

#[tauri::command]
pub fn previous_track(
    app: AppHandle,
    state: State<'_, BackendState>,
) -> Result<PlaybackStateEvent, String> {
    let current = state.snapshot.lock().media_path.clone();
    let previous = state
        .playlist
        .lock()
        .iter()
        .rev()
        .find(|item| Some(&item.path) != current.as_ref())
        .map(|item| item.path.clone());
    if let Some(path) = previous {
        return load_existing(&app, &state, path);
    }
    Ok(state.event())
}

#[tauri::command]
pub fn shutdown_session(state: State<'_, BackendState>) {
    if let Some(mut session) = state.session.lock().take() {
        session.shutdown();
    }
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .manage(BackendState::new())
        .invoke_handler(tauri::generate_handler![
            get_playback_snapshot,
            get_app_config,
            save_app_config,
            open_media,
            toggle_playback,
            seek_relative,
            seek_absolute,
            set_volume,
            toggle_fullscreen,
            next_track,
            previous_track,
            shutdown_session,
        ])
        .run(tauri::generate_context!())
        .expect("error while running PlayBack");
}
