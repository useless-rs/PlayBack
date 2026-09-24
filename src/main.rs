//! The installable `playback` application entry point.

#[cfg(all(feature = "native", target_os = "macos"))]
fn main() -> anyhow::Result<()> {
    let arguments = std::env::args().collect::<Vec<_>>();
    let cli_only = arguments.iter().skip(1).any(|argument| {
        matches!(
            argument.as_str(),
            "--help" | "-h" | "--version" | "-V" | "--completion"
        )
    });
    if cli_only {
        return playback_cli::run_with_args(arguments);
    }

    let parsed = playback_core::MpvArgs::parse_from(arguments.into_iter().skip(1))
        .map_err(|error| anyhow::anyhow!(error.to_string()))?;
    let path = parsed
        .config
        .clone()
        .or_else(|| playback_config::default_config_path().filter(|candidate| candidate.is_file()));
    let mut config = playback_config::ConfigEngine::from_path(path)
        .map_err(|error| anyhow::anyhow!(error.to_string()))?
        .config();
    for action in &parsed.actions {
        match action {
            playback_core::MpvArg::Volume(volume) => config.playback.volume = *volume,
            playback_core::MpvArg::Speed(speed) => config.playback.speed = *speed,
            playback_core::MpvArg::LoopFile(mode) => {
                config.playback.r#loop = !matches!(mode, playback_core::LoopMode::Once);
            }
            _ => {}
        }
    }
    playback_ui::native_app::run_native(&parsed.media, &config)
}

#[cfg(not(all(feature = "native", target_os = "macos")))]
fn main() -> anyhow::Result<()> {
    playback_cli::run()
}
