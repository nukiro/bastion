# bastion-schema

Schema definition and validation engine for the Bastion Smart Data Gateway.

## Overview

`bastion-schema` is the data contract layer of Bastion. It defines what a valid
payload looks like before it enters the pipeline, catching malformed or unexpected
data at the point of entry — not downstream.

Think of it as a flexible, JSON-serializable schema with built-in validation rules
for business logic, not just types.

## Responsibilities

- Define schemas: field types, validation rules, required/optional/nullable fields
- Validate JSON payloads against a schema
- Return descriptive, accumulated errors — never fail on the first error only

## Usage

```rust
use bastion_schema::{Schema, validate};
use serde_json::json;

let schema = Schema::builder()
    .field("user_id", FieldType::Integer, |f| f.required())
    .field("email", FieldType::String, |f| f.required().regex(r"^[^@]+@[^@]+$"))
    .field("age", FieldType::Integer, |f| f.range(0, 120).nullable())
    .build();

let payload = json!({ "user_id": 42, "email": "carlos@example.com" });

match validate(&schema, &payload) {
    Ok(()) => println!("Payload is valid"),
    Err(errors) => errors.iter().for_each(|e| eprintln!("{}", e)),
}
```

## Validation Rules

| Rule         | Applies to     | Description                          |
| ------------ | -------------- | ------------------------------------ |
| `required`   | Any field      | Field must be present in the payload |
| `nullable`   | Any field      | Field can be `null`                  |
| `regex`      | String         | Value must match the pattern         |
| `min_length` | String         | Minimum character length             |
| `max_length` | String         | Maximum character length             |
| `range`      | Integer, Float | Inclusive numeric range [min, max]   |

## Error Model

Validation never short-circuits. All rule violations are collected and returned
as `Vec<ValidationError>`, so a client can fix every problem in a single round trip.

```
ValidationError {
    field: "email",
    rule: "regex",
    message: "value 'not-an-email' does not match pattern '^[^@]+@[^@]+$'",
}
```

## Architecture

```
types.rs    — FieldType, ValidationRule (pure enums, no logic)
schema.rs   — Schema, FieldDefinition (compose types)
validate.rs — validate() function (core logic)
error.rs    — ValidationError with thiserror
```

## Part of the Bastion Workspace

```
bastion/
└── modules/
    └── bastion-schema/
```

In the HTTP gateway (coming in week 3), validation is as simple as:

```rust
use bastion_schema::validate;
```

## License

Apache-2.0
