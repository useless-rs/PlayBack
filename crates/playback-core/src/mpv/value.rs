//! Values exchanged with mpv properties.

use std::fmt;

use serde::Serialize;
use serde_json::Value;

/// A value sent to an mpv property.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum MpvValue {
    /// A floating-point property value.
    Number(f64),
    /// A string property value.
    Text(String),
    /// A boolean property value.
    Boolean(bool),
}

impl MpvValue {
    /// Converts the value to JSON for IPC or diagnostics.
    pub fn to_json(&self) -> Value {
        match self {
            Self::Number(value) => {
                serde_json::Number::from_f64(*value).map_or(Value::Null, Value::Number)
            }
            Self::Text(value) => Value::String(value.clone()),
            Self::Boolean(value) => Value::Bool(*value),
        }
    }
}

impl fmt::Display for MpvValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(value) => write!(formatter, "{value}"),
            Self::Text(value) => write!(formatter, "{value}"),
            Self::Boolean(value) => write!(formatter, "{value}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MpvValue;

    #[test]
    fn converts_values_to_json() {
        assert_eq!(MpvValue::Number(2.0).to_json(), serde_json::json!(2.0));
        assert_eq!(
            MpvValue::Text("gpu".to_owned()).to_json(),
            serde_json::json!("gpu")
        );
        assert_eq!(MpvValue::Boolean(true).to_json(), serde_json::json!(true));
    }
}
