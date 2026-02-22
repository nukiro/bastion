use thiserror::Error;

use crate::types::FieldType;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Invalid Type: Field '{field}', expected '{expected}'")]
    InvalidType {
        field: String, // Name of the field that has the error
        expected: FieldType,
    },
}
