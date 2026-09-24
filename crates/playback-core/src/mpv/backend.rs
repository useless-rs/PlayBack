//! Media-engine backend implementations.

use std::path::Path;
use std::process::Command;

use crate::error::{CoreError, CoreResult};

use super::MpvValue;

/// A backend capable of loading media and executing mpv operations.
///
/// Implementations may use libmpv, an IPC connection, or a subprocess. The
/// controller never exposes backend-specific handles, which keeps the core
/// crate portable and makes the FFI boundary replaceable.
pub trait MpvBackend {
    /// Sets an mpv property.
    fn set_property(&mut self, name: &str, value: MpvValue) -> CoreResult<()>;

    /// Executes an mpv command.
    fn command(&mut self, name: &str, arguments: &[String]) -> CoreResult<()>;

    /// Loads a media path.
    fn load_file(&mut self, path: &Path) -> CoreResult<()>;
}

/// A command-line backend that delegates playback to the installed `mpv`
/// executable.
#[derive(Debug, Clone)]
pub struct MpvProcessBackend {
    program: String,
}

impl MpvProcessBackend {
    /// Creates a process backend using the `mpv` executable.
    pub fn new() -> Self {
        Self::with_program("mpv")
    }

    /// Creates a process backend with a custom executable name.
    pub fn with_program(program: impl Into<String>) -> Self {
        Self {
            program: program.into(),
        }
    }

    /// Runs mpv with the supplied arguments and waits for it to exit.
    pub fn run(&self, arguments: &[String]) -> CoreResult<()> {
        let status = Command::new(&self.program)
            .args(arguments)
            .status()
            .map_err(|source| CoreError::BackendStart {
                program: self.program.clone(),
                source,
            })?;
        if status.success() {
            Ok(())
        } else {
            Err(CoreError::BackendExit {
                status: status.to_string(),
            })
        }
    }

    /// Returns the executable name used by this backend.
    pub fn program(&self) -> &str {
        &self.program
    }
}

impl Default for MpvProcessBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl MpvBackend for MpvProcessBackend {
    fn set_property(&mut self, name: &str, value: MpvValue) -> CoreResult<()> {
        self.run(&[format!("--{name}={value}")])
    }

    fn command(&mut self, name: &str, arguments: &[String]) -> CoreResult<()> {
        let mut command = vec![name.to_owned()];
        command.extend_from_slice(arguments);
        self.run(&command)
    }

    fn load_file(&mut self, path: &Path) -> CoreResult<()> {
        self.run(&[path.to_string_lossy().into_owned()])
    }
}

/// A backend that records operations for tests and embedding hosts.
#[derive(Debug, Default)]
pub struct RecordingBackend {
    operations: Vec<String>,
}

impl RecordingBackend {
    /// Creates an empty recording backend.
    pub const fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    /// Returns the recorded operations in execution order.
    pub fn operations(&self) -> &[String] {
        &self.operations
    }
}

impl MpvBackend for RecordingBackend {
    fn set_property(&mut self, name: &str, value: MpvValue) -> CoreResult<()> {
        self.operations.push(format!("set {name}={value}"));
        Ok(())
    }

    fn command(&mut self, name: &str, arguments: &[String]) -> CoreResult<()> {
        let mut operation = format!("command {name}");
        for argument in arguments {
            operation.push(' ');
            operation.push_str(argument);
        }
        self.operations.push(operation);
        Ok(())
    }

    fn load_file(&mut self, path: &Path) -> CoreResult<()> {
        self.operations
            .push(format!("load {}", path.to_string_lossy()));
        Ok(())
    }
}

/// A libmpv-backed implementation enabled by the `native-mpv` feature.
#[cfg(all(feature = "native-mpv", target_os = "macos"))]
pub struct NativeMpvBackend {
    mpv: libmpv2::Mpv,
}

#[cfg(all(feature = "native-mpv", target_os = "macos"))]
impl std::fmt::Debug for NativeMpvBackend {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("NativeMpvBackend")
    }
}

#[cfg(all(feature = "native-mpv", target_os = "macos"))]
impl NativeMpvBackend {
    /// Creates and initializes a libmpv context.
    pub fn new() -> CoreResult<Self> {
        libmpv2::Mpv::new()
            .map(|mpv| Self { mpv })
            .map_err(|error| CoreError::Backend {
                message: error.to_string(),
            })
    }
}

#[cfg(all(feature = "native-mpv", target_os = "macos"))]
impl MpvBackend for NativeMpvBackend {
    fn set_property(&mut self, name: &str, value: MpvValue) -> CoreResult<()> {
        let result = match value {
            MpvValue::Number(value) => self.mpv.set_property(name, value),
            MpvValue::Text(value) => self.mpv.set_property(name, value),
            MpvValue::Boolean(value) => self.mpv.set_property(name, value),
        };
        result.map_err(|error| CoreError::Backend {
            message: error.to_string(),
        })
    }

    fn command(&mut self, name: &str, arguments: &[String]) -> CoreResult<()> {
        let borrowed: Vec<&str> = arguments.iter().map(String::as_str).collect();
        self.mpv
            .command(name, &borrowed)
            .map_err(|error| CoreError::Backend {
                message: error.to_string(),
            })
    }

    fn load_file(&mut self, path: &Path) -> CoreResult<()> {
        let path = path.to_string_lossy();
        self.mpv
            .command("loadfile", &[path.as_ref(), "replace"])
            .map_err(|error| CoreError::Backend {
                message: error.to_string(),
            })
    }
}

#[cfg(all(test, feature = "native-mpv", target_os = "macos"))]
mod native_tests {
    use super::NativeMpvBackend;

    #[test]
    fn native_backend_type_is_available() {
        let backend: Option<NativeMpvBackend> = None;
        assert!(backend.is_none());
    }
}

#[cfg(test)]
mod tests {
    use super::super::MpvValue;
    use super::{MpvBackend, MpvProcessBackend, RecordingBackend};
    use std::path::Path;

    #[test]
    fn process_backend_uses_requested_program() {
        let backend = MpvProcessBackend::with_program("definitely-not-a-real-mpv");
        let error = backend
            .run(&["--version".to_owned()])
            .expect_err("program missing");
        assert!(error.to_string().contains("definitely-not-a-real-mpv"));
    }

    #[test]
    fn recording_backend_captures_operations() {
        let mut backend = RecordingBackend::new();
        backend
            .set_property("volume", MpvValue::Number(80.0))
            .expect("set");
        backend
            .command("seek", &["10".to_owned()])
            .expect("command");
        backend.load_file(Path::new("movie.mkv")).expect("load");
        assert_eq!(backend.operations().len(), 3);
    }
}
