use regex::Regex;
use serde_json::Value;

use crate::error::ValidationError;
use crate::schema::Schema;
use crate::types::{DateTimeFormat, FieldType, ValidationRule};

/// Public API for validating a JSON payload against a schema. Returns Ok(()) if valid, or a list of validation errors if invalid.
pub fn validate(schema: &Schema, payload: &Value) -> Result<(), Vec<ValidationError>> {
    // payload is data coming from outside

    let mut errors: Vec<ValidationError> = vec![];

    // Per field in schema we need to validate the corresponding value in the payload
    // One iteration per field in the schema,
    // so that we can report all errors in one go instead of failing fast on the first error.
    for (field_name, definition) in &schema.fields {
        // Get the value from the payload for this field, if it exists
        let value = payload.get(field_name);

        // Field is missing from the payload
        if value.is_none() {
            if definition.required {
                errors.push(ValidationError::MissingField {
                    field: field_name.clone(),
                });
            }
            // If type is not correct, does not make sense to apply rules,
            // so we can skip to the next field
            continue;
        }

        // We already know the value is not None,
        // so we can safely unwrap it for the rest of the checks
        let value = value.unwrap();

        // Field is present but null
        if value.is_null() {
            if !definition.nullable {
                errors.push(ValidationError::NullValue {
                    field: field_name.clone(),
                });
            }
            // If value is null, does not make sense to apply rules,
            // so we can skip to the next field
            continue;
        }

        // Type check
        if let Some(err) = check_type(field_name, &definition.field_type, value) {
            errors.push(err);
            continue; // no point applying rules if the type is wrong
        }

        // Validation rules
        // For each rule defined for this field, check if the value satisfies the rule. If not, add an error to the list.
        for rule in &definition.rules {
            if let Some(err) = check_rule(field_name, rule, value) {
                errors.push(err);
            }
        }
    }

    // Return all errors found, or Ok if no errors
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn check_type(field: &str, expected: &FieldType, value: &Value) -> Option<ValidationError> {
    fn json_type_name(value: &Value) -> FieldType {
        match value {
            Value::String(_) => FieldType::String,
            Value::Number(n) if n.is_f64() => FieldType::Float,
            Value::Number(_) => FieldType::Integer,
            Value::Bool(_) => FieldType::Boolean,
            Value::Array(_) => FieldType::Array,
            Value::Object(_) => FieldType::Object,
            Value::Null => FieldType::String, // never reached, null handled before
        }
    }

    let matches = match expected {
        FieldType::String => value.is_string(),
        FieldType::Integer => value.is_i64() || value.is_u64(),
        FieldType::Float => value.is_f64() || value.is_i64() || value.is_u64(),
        FieldType::Boolean => value.is_boolean(),
        FieldType::Array => value.is_array(),
        FieldType::Object => value.is_object(),
        FieldType::DateTime => value.is_string(), // format validated via rule
    };

    if matches {
        None
    } else {
        Some(ValidationError::InvalidType {
            field: field.to_string(),
            expected: expected.clone(),
            actual: json_type_name(value),
        })
    }
}

fn check_rule(field: &str, rule: &ValidationRule, value: &Value) -> Option<ValidationError> {
    match rule {
        ValidationRule::Pattern(pattern) => {
            let s = value.as_str().unwrap_or("");
            // It compiles the regex in every call, which is inefficient.
            // Move to: lazy_static or OnceLock.
            let re = Regex::new(pattern).ok()?;
            if !re.is_match(s) {
                Some(ValidationError::RuleViolation {
                    field: field.to_string(),
                    rule: rule.clone(),
                    message: format!("value '{}' does not match pattern '{}'", s, pattern),
                })
            } else {
                None
            }
        }

        ValidationRule::MinLength(min) => {
            let s = value.as_str().unwrap_or("");
            if s.len() < *min {
                Some(ValidationError::RuleViolation {
                    field: field.to_string(),
                    rule: rule.clone(),
                    message: format!("length {} is less than minimum {}", s.len(), min),
                })
            } else {
                None
            }
        }

        ValidationRule::MaxLength(max) => {
            let s = value.as_str().unwrap_or("");
            if s.len() > *max {
                Some(ValidationError::RuleViolation {
                    field: field.to_string(),
                    rule: rule.clone(),
                    message: format!("length {} exceeds maximum {}", s.len(), max),
                })
            } else {
                None
            }
        }

        ValidationRule::MinValue(min) => {
            let n = value.as_f64().unwrap_or(f64::MAX);
            if n < *min {
                Some(ValidationError::RuleViolation {
                    field: field.to_string(),
                    rule: rule.clone(),
                    message: format!("value {} is less than minimum {}", n, min),
                })
            } else {
                None
            }
        }

        ValidationRule::MaxValue(max) => {
            let n = value.as_f64().unwrap_or(f64::MIN);
            if n > *max {
                Some(ValidationError::RuleViolation {
                    field: field.to_string(),
                    rule: rule.clone(),
                    message: format!("value {} exceeds maximum {}", n, max),
                })
            } else {
                None
            }
        }

        ValidationRule::DateTimeFormat(format) => {
            let s = value.as_str().unwrap_or("");
            let valid = match format {
                DateTimeFormat::Iso8601 => chrono::DateTime::parse_from_rfc3339(s).is_ok(),
                DateTimeFormat::UnixTimestamp => s.parse::<i64>().is_ok(),
            };
            if !valid {
                Some(ValidationError::RuleViolation {
                    field: field.to_string(),
                    rule: rule.clone(),
                    message: format!("value '{}' is not a valid {:?}", s, format),
                })
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{FieldDefinition, Schema};
    use crate::types::{DateTimeFormat, FieldType, ValidationRule};
    use serde_json::json;

    // ── Helpers ─────────────────────────────────────────────────────────────

    fn user_schema() -> Schema {
        Schema::new("user")
            .field(
                "user_id",
                FieldDefinition::new(FieldType::Integer).required(),
            )
            .field(
                "email",
                FieldDefinition::new(FieldType::String)
                    .required()
                    .rule(ValidationRule::Pattern(r"^[^@]+@[^@]+$".to_string())),
            )
            .field(
                "age",
                FieldDefinition::new(FieldType::Integer)
                    .nullable()
                    .rule(ValidationRule::MinValue(0.0))
                    .rule(ValidationRule::MaxValue(120.0)),
            )
            .field(
                "username",
                FieldDefinition::new(FieldType::String)
                    .required()
                    .rule(ValidationRule::MinLength(3))
                    .rule(ValidationRule::MaxLength(20)),
            )
    }

    // ── Happy path ───────────────────────────────────────────────────────────

    #[test]
    fn valid_payload_passes() {
        let schema = user_schema();
        let payload = json!({
            "user_id": 1,
            "email": "carlos@example.com",
            "username": "carlos",
            "age": 30
        });
        assert!(validate(&schema, &payload).is_ok());
    }

    #[test]
    fn optional_field_absent_passes() {
        let schema = user_schema();
        let payload = json!({
            "user_id": 1,
            "email": "carlos@example.com",
            "username": "carlos"
            // age is optional — not required
        });
        assert!(validate(&schema, &payload).is_ok());
    }

    #[test]
    fn nullable_field_with_null_passes() {
        let schema = user_schema();
        let payload = json!({
            "user_id": 1,
            "email": "carlos@example.com",
            "username": "carlos",
            "age": null
        });
        assert!(validate(&schema, &payload).is_ok());
    }

    // ── Missing fields ───────────────────────────────────────────────────────

    #[test]
    fn missing_required_field_returns_error() {
        let schema = user_schema();
        let payload = json!({
            "email": "carlos@example.com",
            "username": "carlos"
            // user_id missing
        });
        let errors = validate(&schema, &payload).unwrap_err();
        assert!(
            errors.iter().any(
                |e| matches!(e, ValidationError::MissingField { field } if field == "user_id")
            )
        );
    }

    #[test]
    fn multiple_missing_fields_returns_all_errors() {
        let schema = user_schema();
        let payload = json!({});
        let errors = validate(&schema, &payload).unwrap_err();
        let missing: Vec<_> = errors
            .iter()
            .filter(|e| matches!(e, ValidationError::MissingField { .. }))
            .collect();
        assert_eq!(missing.len(), 3); // user_id, email, username
    }

    // ── Null values ──────────────────────────────────────────────────────────

    #[test]
    fn null_on_non_nullable_field_returns_error() {
        let schema = user_schema();
        let payload = json!({
            "user_id": null,
            "email": "carlos@example.com",
            "username": "carlos"
        });
        let errors = validate(&schema, &payload).unwrap_err();
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::NullValue { field } if field == "user_id"))
        );
    }

    // ── Type checks ──────────────────────────────────────────────────────────

    #[test]
    fn wrong_type_returns_invalid_type_error() {
        let schema = user_schema();
        let payload = json!({
            "user_id": "not-an-integer",
            "email": "carlos@example.com",
            "username": "carlos"
        });
        let errors = validate(&schema, &payload).unwrap_err();
        assert!(errors.iter().any(|e| matches!(
            e,
            ValidationError::InvalidType { field, expected, .. }
            if field == "user_id" && *expected == FieldType::Integer
        )));
    }

    #[test]
    fn wrong_type_skips_rule_validation() {
        // If type is wrong, rules should not produce additional errors for that field
        let schema = user_schema();
        let payload = json!({
            "user_id": "not-an-integer",
            "email": "carlos@example.com",
            "username": "carlos"
        });
        let errors = validate(&schema, &payload).unwrap_err();
        let user_id_errors: Vec<_> = errors
            .iter()
            .filter(|e| match e {
                ValidationError::InvalidType { field, .. } => field == "user_id",
                ValidationError::RuleViolation { field, .. } => field == "user_id",
                _ => false,
            })
            .collect();
        assert_eq!(user_id_errors.len(), 1); // only InvalidType, no RuleViolation
    }

    // ── Pattern ──────────────────────────────────────────────────────────────

    #[test]
    fn invalid_email_pattern_returns_error() {
        let schema = user_schema();
        let payload = json!({
            "user_id": 1,
            "email": "not-an-email",
            "username": "carlos"
        });
        let errors = validate(&schema, &payload).unwrap_err();
        assert!(errors.iter().any(|e| matches!(
            e,
            ValidationError::RuleViolation { field, rule: ValidationRule::Pattern(_), .. }
            if field == "email"
        )));
    }

    // ── MinLength / MaxLength ────────────────────────────────────────────────

    #[test]
    fn username_too_short_returns_error() {
        let schema = user_schema();
        let payload = json!({
            "user_id": 1,
            "email": "carlos@example.com",
            "username": "ab" // less than 3
        });
        let errors = validate(&schema, &payload).unwrap_err();
        assert!(errors.iter().any(|e| matches!(
            e,
            ValidationError::RuleViolation { field, rule: ValidationRule::MinLength(_), .. }
            if field == "username"
        )));
    }

    #[test]
    fn username_too_long_returns_error() {
        let schema = user_schema();
        let payload = json!({
            "user_id": 1,
            "email": "carlos@example.com",
            "username": "this_username_is_way_too_long" // more than 20
        });
        let errors = validate(&schema, &payload).unwrap_err();
        assert!(errors.iter().any(|e| matches!(
            e,
            ValidationError::RuleViolation { field, rule: ValidationRule::MaxLength(_), .. }
            if field == "username"
        )));
    }

    // ── MinValue / MaxValue ──────────────────────────────────────────────────

    #[test]
    fn age_below_minimum_returns_error() {
        let schema = user_schema();
        let payload = json!({
            "user_id": 1,
            "email": "carlos@example.com",
            "username": "carlos",
            "age": -1
        });
        let errors = validate(&schema, &payload).unwrap_err();
        assert!(errors.iter().any(|e| matches!(
            e,
            ValidationError::RuleViolation { field, rule: ValidationRule::MinValue(_), .. }
            if field == "age"
        )));
    }

    #[test]
    fn age_above_maximum_returns_error() {
        let schema = user_schema();
        let payload = json!({
            "user_id": 1,
            "email": "carlos@example.com",
            "username": "carlos",
            "age": 121
        });
        let errors = validate(&schema, &payload).unwrap_err();
        assert!(errors.iter().any(|e| matches!(
            e,
            ValidationError::RuleViolation { field, rule: ValidationRule::MaxValue(_), .. }
            if field == "age"
        )));
    }

    // ── DateTime ─────────────────────────────────────────────────────────────

    #[test]
    fn valid_iso8601_datetime_passes() {
        let schema = Schema::new("event").field(
            "created_at",
            FieldDefinition::new(FieldType::DateTime)
                .required()
                .rule(ValidationRule::DateTimeFormat(DateTimeFormat::Iso8601)),
        );
        let payload = json!({ "created_at": "2024-01-15T10:30:00Z" });
        assert!(validate(&schema, &payload).is_ok());
    }

    #[test]
    fn invalid_iso8601_datetime_returns_error() {
        let schema = Schema::new("event").field(
            "created_at",
            FieldDefinition::new(FieldType::DateTime)
                .required()
                .rule(ValidationRule::DateTimeFormat(DateTimeFormat::Iso8601)),
        );
        let payload = json!({ "created_at": "not-a-date" });
        let errors = validate(&schema, &payload).unwrap_err();
        assert!(errors.iter().any(|e| matches!(
            e,
            ValidationError::RuleViolation { field, .. }
            if field == "created_at"
        )));
    }

    #[test]
    fn valid_unix_timestamp_passes() {
        let schema = Schema::new("event").field(
            "created_at",
            FieldDefinition::new(FieldType::DateTime).required().rule(
                ValidationRule::DateTimeFormat(DateTimeFormat::UnixTimestamp),
            ),
        );
        let payload = json!({ "created_at": "1705312200" });
        assert!(validate(&schema, &payload).is_ok());
    }

    // ── Accumulation ─────────────────────────────────────────────────────────

    #[test]
    fn all_errors_accumulated_not_short_circuited() {
        let schema = user_schema();
        let payload = json!({
            // user_id missing
            "email": "not-an-email",
            "username": "ab", // too short
            "age": 200        // too high
        });
        let errors = validate(&schema, &payload).unwrap_err();
        assert!(errors.len() >= 4); // MissingField + Pattern + MinLength + MaxValue
    }
}
