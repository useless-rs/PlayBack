//! Conversion from typed actions to mpv command-line arguments.

use crate::cli::{LoopMode, MpvArg, MpvArgs};

/// Converts parsed arguments into a complete mpv invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MpvInvocation {
    /// Executable arguments, including media paths and options.
    pub arguments: Vec<String>,
}

impl MpvInvocation {
    /// Builds an invocation from parsed arguments.
    pub fn from_args(args: &MpvArgs) -> Self {
        let mut arguments = Vec::new();
        for media in &args.media {
            arguments.push(media.to_string_lossy().into_owned());
        }
        for action in &args.actions {
            arguments.extend(action_to_arguments(action));
        }
        Self { arguments }
    }
}

pub(super) fn action_to_arguments(action: &MpvArg) -> Vec<String> {
    match action {
        MpvArg::Volume(value) => vec![format!("--volume={value}")],
        MpvArg::Speed(value) => vec![format!("--speed={value}")],
        MpvArg::Start(value) => vec![format!("--start={value}")],
        MpvArg::LoopFile(mode) => vec![format!(
            "--loop-file={}",
            match mode {
                LoopMode::Once => "no",
                LoopMode::Infinite => "inf",
                LoopMode::Count(count) => return vec![format!("--loop-file={count}")],
            }
        )],
        MpvArg::SubFile(path) | MpvArg::AudioFile(path) => {
            let option = if matches!(action, MpvArg::SubFile(_)) {
                "--sub-file"
            } else {
                "--audio-file"
            };
            vec![format!("{option}={}", path.to_string_lossy())]
        }
        MpvArg::NoVideo => vec!["--no-video".to_owned()],
        MpvArg::Fullscreen => vec!["--fullscreen".to_owned()],
        MpvArg::OnTop => vec!["--ontop".to_owned()],
        MpvArg::Geometry(geometry) => {
            vec![format!("--geometry={}", geometry.as_mpv_value())]
        }
        MpvArg::Property { name, value } => vec![format!("--{name}={value}")],
        MpvArg::Flag(name) => vec![format!("--{name}")],
    }
}

#[cfg(test)]
mod tests {
    use super::{MpvInvocation, action_to_arguments};
    use crate::cli::{MpvArg, MpvArgs, WindowGeometry};

    #[test]
    fn builds_a_complete_process_invocation() {
        let args = MpvArgs::parse_from([
            "--geometry=1280x720",
            "--audio-file=track.flac",
            "movie.mkv",
        ])
        .expect("valid args");
        let invocation = MpvInvocation::from_args(&args);
        assert_eq!(
            invocation.arguments,
            vec![
                "movie.mkv".to_owned(),
                "--geometry=1280x720".to_owned(),
                "--audio-file=track.flac".to_owned(),
            ]
        );
    }

    #[test]
    fn generic_actions_are_preserved_in_invocations() {
        let action = MpvArg::Property {
            name: "fs".to_owned(),
            value: "minimal".to_owned(),
        };
        let invocation = MpvInvocation {
            arguments: action_to_arguments(&action),
        };
        assert_eq!(invocation.arguments, vec!["--fs=minimal"]);
        let geometry = WindowGeometry {
            width: 1,
            height: 2,
        };
        assert_eq!(geometry.as_mpv_value(), "1x2");
    }
}
