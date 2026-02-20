pub mod error;
pub mod schema;
pub mod types;
pub mod validate;

// Re-exports for ergonomic top-level usage
pub use error::ValidationError;
pub use schema::{FieldDefinition, Schema};
pub use types::{DateTimeFormat, FieldType, ValidationRule};
pub use validate::validate;
