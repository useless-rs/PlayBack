//! Primitive parsers used by the typed command-line model.

use std::path::PathBuf;

use crate::error::{CoreError, CoreResult};
use crate::state::Seconds;

use super::types::{LoopMode, WindowGeometry};

pub(super) fn is_option(argument: &str) -> bool {
    argument.len() > 1 && argument.starts_with('-')
}

pub(super) fn split_option(argument: &str) -> (String, Option<String>) {
    match argument.split_once('=') {
        Some((name, value)) => (name.to_owned(), Some(value.to_owned())),
        None => (argument.to_owned(), None),
    }
}

pub(super) fn take_value<I>(
    option: &str,
    inline: Option<String>,
    iterator: &mut I,
) -> CoreResult<String>
where
    I: Iterator<Item = String>,
{
    if let Some(value) = inline {
        return Ok(value);
    }
    iterator.next().ok_or_else(|| CoreError::MissingValue {
        option: option.to_owned(),
    })
}

pub(super) fn parse_f64(option: &str, value: &str) -> CoreResult<f64> {
    value.parse::<f64>().map_err(|_| CoreError::InvalidValue {
        option: option.to_owned(),
        value: value.to_owned(),
        reason: "expected a floating-point number".to_owned(),
    })
}

pub(super) fn parse_timecode(option: &str, value: &str) -> CoreResult<Seconds> {
    let parts: Vec<&str> = value.split(':').collect();
    let seconds = match parts.as_slice() {
        [seconds] => parse_f64(option, seconds)?,
        [minutes, seconds] => {
            let minutes = parse_f64(option, minutes)?;
            let seconds = parse_f64(option, seconds)?;
            minutes.mul_add(60.0, seconds)
        }
        [hours, minutes, seconds] => {
            let hours = parse_f64(option, hours)?;
            let minutes = parse_f64(option, minutes)?;
            let seconds = parse_f64(option, seconds)?;
            hours.mul_add(3600.0, minutes.mul_add(60.0, seconds))
        }
        _ => {
            return Err(CoreError::InvalidValue {
                option: option.to_owned(),
                value: value.to_owned(),
                reason: "expected seconds, MM:SS, or HH:MM:SS".to_owned(),
            });
        }
    };
    if !seconds.is_finite() || seconds < 0.0 {
        return Err(CoreError::InvalidValue {
            option: option.to_owned(),
            value: value.to_owned(),
            reason: "time must be finite and non-negative".to_owned(),
        });
    }
    Ok(Seconds::new(seconds))
}

pub(super) fn parse_loop_mode(option: &str, value: &str) -> CoreResult<LoopMode> {
    match value.to_ascii_lowercase().as_str() {
        "inf" | "infinite" => Ok(LoopMode::Infinite),
        "no" | "none" | "0" => Ok(LoopMode::Once),
        _ => value
            .parse::<u32>()
            .map(LoopMode::Count)
            .map_err(|_| CoreError::InvalidValue {
                option: option.to_owned(),
                value: value.to_owned(),
                reason: "expected `inf`, `no`, or a non-negative integer".to_owned(),
            }),
    }
}

pub(super) fn parse_geometry(option: &str, value: &str) -> CoreResult<WindowGeometry> {
    let (width, height) = value
        .split_once('x')
        .ok_or_else(|| CoreError::InvalidValue {
            option: option.to_owned(),
            value: value.to_owned(),
            reason: "expected WIDTHxHEIGHT".to_owned(),
        })?;
    let width = width.parse::<u32>().map_err(|_| CoreError::InvalidValue {
        option: option.to_owned(),
        value: value.to_owned(),
        reason: "width must be a positive integer".to_owned(),
    })?;
    let height = height.parse::<u32>().map_err(|_| CoreError::InvalidValue {
        option: option.to_owned(),
        value: value.to_owned(),
        reason: "height must be a positive integer".to_owned(),
    })?;
    if width == 0 || height == 0 {
        return Err(CoreError::InvalidValue {
            option: option.to_owned(),
            value: value.to_owned(),
            reason: "width and height must be greater than zero".to_owned(),
        });
    }
    Ok(WindowGeometry { width, height })
}

pub(super) fn parse_path(option: &str, value: String) -> CoreResult<PathBuf> {
    if value.trim().is_empty() {
        return Err(CoreError::InvalidValue {
            option: option.to_owned(),
            value,
            reason: "path must not be empty".to_owned(),
        });
    }
    Ok(PathBuf::from(value))
}

#[cfg(test)]
mod tests {
    use super::{parse_geometry, parse_loop_mode, parse_timecode};
    use crate::cli::{LoopMode, WindowGeometry};

    #[test]
    fn parses_primitive_values() {
        assert!(
            (parse_timecode("--start", "00:01:30")
                .expect("time")
                .as_f64()
                - 90.0)
                .abs()
                < f64::EPSILON
        );
        assert_eq!(
            parse_loop_mode("--loop-file", "inf").expect("loop"),
            LoopMode::Infinite
        );
        assert_eq!(
            parse_geometry("--geometry", "640x480").expect("geometry"),
            WindowGeometry {
                width: 640,
                height: 480
            }
        );
    }
}
