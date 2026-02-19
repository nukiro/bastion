use serde::{Deserialize, Serialize};

/// The data type of a schema field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
// `rename_all` to lowercase, so that the JSON representation is more concise and consistent with common conventions.
// From the dashboard we want to send "string", "integer", etc. instead of "String", "Integer".
pub enum FieldType {
    String,
    Integer,
    Float,
    Boolean,
    DateTime,
    Object,
    Array,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DateTimeFormat {
    Iso8601,
    UnixTimestamp,
}

/// A validation rule that can be applied to a field. It describes how to validate the field's value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "rule", content = "value", rename_all = "snake_case")]
// Serde tag and content produces a JSON object like {"rule": "regex", "value": "^\d+$"} instead of {"Regex": "^\d+$"},
// which is easier to work with from the python dashboard and more consistent with common JSON conventions.
pub enum ValidationRule {
    /// String must match this regex pattern.
    Pattern(String),
    /// String minimum length (inclusive).
    MinLength(usize),
    /// String maximum length (inclusive).
    MaxLength(usize),
    /// Numeric minimum value (inclusive).
    MinValue(f64),
    /// Numeric maximum value (inclusive).
    MaxValue(f64),
    /// DateTime format constraint. DateTime must match this format: "iso8601" | "unix_timestamp".
    DateTimeFormat(DateTimeFormat),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn field_type_serializes_lowercase() {
        let json = serde_json::to_string(&FieldType::Integer).unwrap();
        assert_eq!(json, "\"integer\"");
    }

    #[test]
    fn validation_rule_serializes_tagged() {
        let rule = ValidationRule::Pattern(r"^\d+$".to_string());
        let json = serde_json::to_string(&rule).unwrap();
        assert_eq!(json, r#"{"rule":"pattern","value":"^\\d+$"}"#);
    }

    #[test]
    fn field_type_round_trips() {
        let original = FieldType::String;
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: FieldType = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn datetime_format_serializes_correctly() {
        let rule = ValidationRule::DateTimeFormat(DateTimeFormat::Iso8601);
        let json = serde_json::to_string(&rule).unwrap();
        assert_eq!(json, r#"{"rule":"date_time_format","value":"iso8601"}"#);
    }
}
