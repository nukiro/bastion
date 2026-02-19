use serde::{Deserialize, Serialize};

use crate::types::{FieldType, ValidationRule};

/// Defines a single field in a schema: its type, rules, and presence constraints.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// Required and nullable are separate attributes:
// - A required field must be present in the payload, but can be null if nullable is true.
// - A non-required field can be omitted from the payload, but if present it must not be null if nullable is false.
// As an example, an user event may have `deleted_at` (required: false) but if present it may be null,
// if the user has not been deleted, or a datetime if it was (nullable: true). Without this distinction,
// we would not model this common pattern correctly.
pub struct FieldDefinition {
    pub field_type: FieldType,
    pub required: bool,
    pub nullable: bool,
    pub rules: Vec<ValidationRule>,
}

impl FieldDefinition {
    pub fn new(field_type: FieldType) -> Self {
        Self {
            field_type,
            required: false,
            nullable: false,
            rules: vec![],
        }
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    pub fn nullable(mut self) -> Self {
        self.nullable = true;
        self
    }

    pub fn rule(mut self, rule: ValidationRule) -> Self {
        self.rules.push(rule);
        self
    }
}

/// A named collection of field definitions that describes the expected shape of a payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schema {
    pub name: String,
    pub fields: std::collections::HashMap<String, FieldDefinition>,
}

impl Schema {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            fields: std::collections::HashMap::new(),
        }
    }

    pub fn field(mut self, name: impl Into<String>, definition: FieldDefinition) -> Self {
        self.fields.insert(name.into(), definition);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{FieldType, ValidationRule};

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
            .field("age", FieldDefinition::new(FieldType::Integer).nullable())
    }

    #[test]
    fn schema_has_correct_fields() {
        let schema = user_schema();
        assert_eq!(schema.name, "user");
        assert!(schema.fields.contains_key("user_id"));
        assert!(schema.fields.contains_key("email"));
        assert!(schema.fields.contains_key("age"));
    }

    #[test]
    fn required_field_is_marked_correctly() {
        let schema = user_schema();
        assert!(schema.fields["user_id"].required);
        assert!(!schema.fields["age"].required);
    }

    #[test]
    fn nullable_field_is_marked_correctly() {
        let schema = user_schema();
        assert!(schema.fields["age"].nullable);
        assert!(!schema.fields["user_id"].nullable);
    }

    #[test]
    fn field_rules_are_attached() {
        let schema = user_schema();
        assert_eq!(schema.fields["email"].rules.len(), 1);
    }

    #[test]
    fn schema_round_trips_via_json() {
        let schema = user_schema();
        let json = serde_json::to_string(&schema).unwrap();
        let deserialized: Schema = serde_json::from_str(&json).unwrap();
        assert_eq!(schema, deserialized);
    }
}
