use serde_json::Value;

use crate::error::ValidationError;
use crate::schema::Schema;
use crate::types::FieldType;

pub fn validate(schema: &Schema, message: &Value) -> Result<(), Vec<ValidationError>> {
    let mut errors: Vec<ValidationError> = vec![];

    for (field_name, field_definition) in &schema.fields {
        // Try to get the value of the field from the message
        let field_value = message.get(field_name);

        if field_value.is_none() {
            // If the field is missing, we can choose to ignore it or treat it as an error
            // For this example, we'll ignore missing fields
            continue;
        }

        // If the filed is present, validate it
        // We can safely unwrap here because we just checked that it's not None
        let field_value = field_value.unwrap();

        if field_value.is_null() {
            // If the field is null, we can choose to ignore it or treat it as an error
            // For this example, we'll ignore null fields
            continue;
        }

        // Validate the field type
        // Try to match the field type defined in the schema with the actual type of the value in the message
        let matches = match field_definition.field_type {
            FieldType::Integer => field_value.is_i64(),
        };

        if matches {
            // If the types match, we can consider this field valid
            continue;
        } else {
            // If the types don't match, we add an error to the list of errors
            errors.push(ValidationError::InvalidType {
                field: field_name.clone(),
                expected: field_definition.field_type.clone(),
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
