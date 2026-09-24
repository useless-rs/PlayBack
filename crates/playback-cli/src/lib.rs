//! Command-line application logic for PlayBack.

use std::io::Write;

use anyhow::{Context, Result};
use clap::{Arg, ArgAction, Command as ClapCommand};
use clap_complete::Shell;
use playback_config::{ConfigEngine, PlaybackConfig};
use playback_core::{MpvArgs, MpvInvocation, MpvProcessBackend};
use tracing_subscriber::EnvFilter;

/// Runs the application using the current process arguments.
pub fn run() -> Result<()> {
    let arguments = std::env::args().collect::<Vec<_>>();
    run_with_args(arguments)
}

/// Runs the application with an explicit argument list.
///
/// The first element, when present, is treated as the executable name. This
/// makes the function straightforward to exercise in integration tests.
pub fn run_with_arguments(arguments: Vec<String>) -> Result<()> {
    initialize_tracing();
    let user_arguments = arguments.into_iter().skip(1).collect::<Vec<_>>();

    if user_arguments
        .iter()
        .any(|argument| argument == "--help" || argument == "-h")
    {
        print_help();
        return Ok(());
    }
    if user_arguments
        .iter()
        .any(|argument| argument == "--version" || argument == "-V")
    {
        println!("PlayBack {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    if let Some(shell) = completion_shell(&user_arguments) {
        generate_completions(shell)?;
        return Ok(());
    }

    let args = MpvArgs::parse_from(user_arguments).context("could not parse playback arguments")?;
    if args.is_empty() {
        print_help();
        return Ok(());
    }

    let config = load_configuration(args.config.as_deref())?;
    let invocation = MpvInvocation::from_args(&args);
    let mut backend_arguments = config_arguments(&config);
    backend_arguments.extend(invocation.arguments);
    MpvProcessBackend::new()
        .run(&backend_arguments)
        .map_err(|error| {
            anyhow::anyhow!(
                "PlayBack could not run mpv. Install mpv and make sure it is on PATH. Details: {error}"
            )
        })?;
    Ok(())
}

/// Compatibility alias for callers that prefer the shorter name.
pub fn run_with_args(arguments: Vec<String>) -> Result<()> {
    run_with_arguments(arguments)
}

fn load_configuration(explicit_path: Option<&std::path::Path>) -> Result<PlaybackConfig> {
    let path = explicit_path.map_or_else(
        || playback_config::default_config_path().filter(|candidate| candidate.is_file()),
        |candidate| Some(candidate.to_path_buf()),
    );
    ConfigEngine::from_path(path)
        .map(|engine| engine.config())
        .map_err(|error| anyhow::anyhow!(error.to_string()))
}

fn config_arguments(config: &PlaybackConfig) -> Vec<String> {
    let mut arguments = vec![
        format!("--volume={}", config.playback().volume()),
        format!("--speed={}", config.playback().speed()),
        format!(
            "--loop-file={}",
            if config.playback().r#loop {
                "inf"
            } else {
                "no"
            }
        ),
        format!("--hwdec={}", config.video().hardware_decoding()),
        format!("--scale={}", config.video().scale),
        format!("--audio-channels={}", config.audio().channel_layout()),
    ];
    let aspect_ratio = config.video().aspect_ratio.trim();
    if !aspect_ratio.is_empty() && !aspect_ratio.eq_ignore_ascii_case("auto") {
        arguments.push(format!("--video-aspect-override={aspect_ratio}"));
    }
    if config.video().deinterlace {
        arguments.push("--deinterlace=yes".to_owned());
    }
    if config.audio().normalize {
        arguments.push("--af=lavfi=[loudnorm]".to_owned());
    }
    if !config.subtitles().enabled() {
        arguments.push("--sub-visibility=no".to_owned());
    }
    for (property, value) in &config.custom {
        if let Some(value) = scalar_to_string(value) {
            arguments.push(format!("--{property}={value}"));
        }
    }
    arguments
}

fn scalar_to_string(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::Null | serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
            None
        }
        serde_json::Value::Bool(value) => Some(value.to_string()),
        serde_json::Value::Number(value) => Some(value.to_string()),
        serde_json::Value::String(value) => Some(value.clone()),
    }
}

fn completion_shell(arguments: &[String]) -> Option<Shell> {
    let index = arguments
        .iter()
        .position(|argument| argument == "--completion")?;
    let value = arguments.get(index + 1)?.as_str();
    value.parse().ok()
}

fn generate_completions(shell: Shell) -> Result<()> {
    let mut command = command_definition();
    let mut output = std::io::stdout();
    clap_complete::generate(shell, &mut command, "playback", &mut output);
    output
        .flush()
        .context("could not write shell completions")?;
    Ok(())
}

fn command_definition() -> ClapCommand {
    ClapCommand::new("playback")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A friendly, configurable media player powered by mpv")
        .long_about("PlayBack is a keyboard-first media player with an mpv-compatible command line. Pass one or more files or URLs, or use --help to see the common options.")
        .after_help("Examples:\n  playback movie.mp4\n  playback --volume 80 movie.mkv\n  playback --config ~/.config/playback/init.lua movie.mkv\n  playback --completion bash > ~/.local/share/bash-completion/completions/playback")
        .arg(
            Arg::new("file")
                .help("Media file or URL")
                .num_args(1..)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("config")
                .long("config")
                .value_name("PATH")
                .help("Lua configuration path"),
        )
        .arg(
            Arg::new("volume")
                .long("volume")
                .value_name("VALUE")
                .help("Set volume"),
        )
        .arg(
            Arg::new("speed")
                .long("speed")
                .value_name("VALUE")
                .help("Set playback speed"),
        )
        .arg(
            Arg::new("start")
                .long("start")
                .value_name("TIME")
                .help("Seek on load"),
        )
        .arg(
            Arg::new("loop-file")
                .long("loop-file")
                .value_name("MODE")
                .help("Configure file looping"),
        )
        .arg(
            Arg::new("sub-file")
                .long("sub-file")
                .value_name("PATH")
                .help("Add a subtitle file"),
        )
        .arg(
            Arg::new("audio-file")
                .long("audio-file")
                .value_name("PATH")
                .help("Add an audio file"),
        )
        .arg(
            Arg::new("no-video")
                .long("no-video")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("fullscreen")
                .long("fullscreen")
                .action(ArgAction::SetTrue),
        )
        .arg(Arg::new("ontop").long("ontop").action(ArgAction::SetTrue))
        .arg(
            Arg::new("geometry")
                .long("geometry")
                .value_name("WIDTHxHEIGHT")
                .help("Set initial window geometry"),
        )
        .arg(
            Arg::new("completion")
                .long("completion")
                .value_name("SHELL")
                .value_parser(["bash", "elvish", "fish", "powershell", "zsh"])
                .help("Generate a shell completion script"),
        )
}

fn print_help() {
    let mut command = command_definition();
    println!("{}", command.render_help());
}

fn initialize_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));
    if let Err(error) = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .try_init()
    {
        tracing::debug!(error = %error, "tracing was already initialized");
    }
}

#[cfg(test)]
mod tests {
    use super::{command_definition, config_arguments, scalar_to_string};
    use playback_config::PlaybackConfig;

    #[test]
    fn command_definition_contains_common_mpv_options() {
        let mut command = command_definition();
        let help = command.render_help().to_string();
        assert!(help.contains("--volume"));
        assert!(help.contains("--sub-file"));
        assert!(help.contains("--geometry"));
        assert!(help.contains("--completion"));
        assert!(help.contains("Examples:"));
    }

    #[test]
    fn config_translates_to_mpv_arguments() {
        let arguments = config_arguments(&PlaybackConfig::default());
        assert!(arguments.contains(&"--volume=100".to_owned()));
        assert!(arguments.contains(&"--speed=1".to_owned()));
        assert!(arguments.contains(&"--hwdec=auto-safe".to_owned()));
    }

    #[test]
    fn config_omits_automatic_aspect_override() {
        let mut config = PlaybackConfig::default();
        assert!(
            !config_arguments(&config)
                .iter()
                .any(|argument| argument.starts_with("--video-aspect-override="))
        );

        config.video.aspect_ratio = "16:9".to_owned();
        assert!(
            config_arguments(&config)
                .iter()
                .any(|argument| argument == "--video-aspect-override=16:9")
        );
    }

    #[test]
    fn only_scalar_custom_values_are_forwarded() {
        assert_eq!(
            scalar_to_string(&serde_json::json!("gpu")),
            Some("gpu".to_owned())
        );
        assert_eq!(
            scalar_to_string(&serde_json::json!(true)),
            Some("true".to_owned())
        );
        assert_eq!(scalar_to_string(&serde_json::json!([1, 2])), None);
    }
}
