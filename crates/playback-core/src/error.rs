//! Error types shared by the PlayBack core crate.

use std::ffi::OsString;
use std::path::PathBuf;

use thiserror::Error;

/// Errors produced while parsing or applying media-player arguments.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CoreError {
    /// An option was supplied without its required value.
    #[error("option `{option}` requires a value")]
    MissingValue {
        /// The option that was missing a value.
        option: String,
    },
    /// An option value could not be parsed.
    #[error("invalid value `{value}` for `{option}`: {reason}")]
    InvalidValue {
        /// The option being parsed.
        option: String,
        /// The value supplied by the caller.
        value: String,
        /// A human-readable explanation.
        reason: String,
    },
    /// A path-like argument could not be represented as UTF-8 text.
    #[error("argument is not valid UTF-8: {argument:?}")]
    NonUnicodeArgument {
        /// The original operating-system argument.
        argument: OsString,
    },
    /// A media path was empty.
    #[error("media path must not be empty")]
    EmptyPath,
    /// A media backend operation failed.
    #[error("media backend operation failed: {message}")]
    Backend {
        /// Backend-specific diagnostic text.
        message: String,
    },
    /// The media backend could not be started.
    #[error("could not start media backend `{program}`: {source}")]
    BackendStart {
        /// The executable that could not be started.
        program: String,
        /// The operating-system error.
        #[source]
        source: std::io::Error,
    },
    /// A media backend exited unsuccessfully.
    #[error("media backend exited with status {status}")]
    BackendExit {
        /// The process exit status.
        status: String,
    },
    /// A configuration file could not be read.
    #[error("could not read configuration file {path}: {source}")]
    ReadConfig {
        /// The configuration path.
        path: PathBuf,
        /// The operating-system error.
        #[source]
        source: std::io::Error,
    },
}

/// Convenient result alias for core operations.
pub type CoreResult<T> = Result<T, CoreError>;

#[cfg(test)]
mod tests {
    use super::CoreError;
    use std::ffi::OsString;

    #[test]
    fn error_messages_name_the_invalid_option() {
        let error = CoreError::InvalidValue {
            option: "--volume".to_owned(),
            value: "loud".to_owned(),
            reason: "expected a number".to_owned(),
        };
        assert_eq!(
            error.to_string(),
            "invalid value `loud` for `--volume`: expected a number"
        );
    }

    #[test]
    fn non_unicode_argument_error_keeps_debug_context() {
        let error = CoreError::NonUnicodeArgument {
            argument: OsString::from("invalid"),
        };
        assert!(error.to_string().contains("not valid UTF-8"));
    }
}
