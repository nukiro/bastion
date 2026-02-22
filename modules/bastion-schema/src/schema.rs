use serde::Deserialize;

use crate::types::FieldType;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct FieldDefinition {
    pub field_type: FieldType,
}

#[derive(Deserialize)]
pub struct Schema {
    pub fields: HashMap<String, FieldDefinition>,
}
