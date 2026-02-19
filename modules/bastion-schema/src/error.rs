use thiserror::Error;

use crate::types::{FieldType, ValidationRule};

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ValidationError {
    /// A required field is missing from the payload.
    #[error("field '{field}' is required but missing")]
    MissingField { field: String },

    /// A field has the wrong type.
    #[error("field '{field}' expected type {expected} but got {actual}")]
    InvalidType {
        field: String,
        expected: FieldType,
        actual: FieldType,
    },

    /// A field failed a validation rule.
    #[error("field '{field}' failed rule {rule:?}: {message}")]
    RuleViolation {
        field: String,
        rule: ValidationRule,
        message: String,
    },

    /// A null value was received for a non-nullable field.
    #[error("field '{field}' is not nullable but received null")]
    NullValue { field: String },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{FieldType, ValidationRule};

    #[test]
    fn missing_field_error_message() {
        let err = ValidationError::MissingField {
            field: "user_id".to_string(),
        };
        assert_eq!(err.to_string(), "field 'user_id' is required but missing");
    }

    #[test]
    fn type_mismatch_error_message() {
        let err = ValidationError::InvalidType {
            field: "age".to_string(),
            expected: FieldType::Integer,
            actual: FieldType::String,
        };
        assert_eq!(
            err.to_string(),
            "field 'age' expected type integer but got string"
        );
    }

    #[test]
    fn null_value_error_message() {
        let err = ValidationError::NullValue {
            field: "email".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "field 'email' is not nullable but received null"
        );
    }

    #[test]
    fn rule_violation_error_message() {
        let err = ValidationError::RuleViolation {
            field: "email".to_string(),
            rule: ValidationRule::Pattern(r"^[^@]+@[^@]+$".to_string()),
            message: "value 'not-an-email' does not match pattern".to_string(),
        };
        assert!(err.to_string().contains("email"));
        assert!(err.to_string().contains("failed rule"));
    }
}
