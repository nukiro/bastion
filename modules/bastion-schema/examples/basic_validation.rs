use bastion_schema::schema::Schema;
use serde_json::Value;

fn main() {
    // Example payload to validate: a message from a client
    let valid_message = r#"
    {
        "age": 30
    }
    "#;

    let valid_message =
        serde_json::from_str::<Value>(valid_message).expect("Failed to parse payload");

    let invalid_message = r#"
    {
        "age": "30"
    }
    "#;

    let invalid_message =
        serde_json::from_str::<Value>(invalid_message).expect("Failed to parse payload");

    let schema = r#"
    {
        "fields": {
            "age": {
                "field_type": "integer"
            }
        }
    }
    "#;

    let schema = serde_json::from_str::<Schema>(schema).expect("Failed to parse schema");

    match bastion_schema::validate(&schema, &valid_message) {
        Ok(_) => println!("Validation passed!"),
        Err(errors) => {
            println!("Validation failed with the following errors:");
            for error in errors {
                println!("- {}", error);
            }
        }
    }

    match bastion_schema::validate(&schema, &invalid_message) {
        Ok(_) => println!("Validation passed!"),
        Err(errors) => {
            println!("Validation failed with the following errors:");
            for error in errors {
                println!("- {}", error);
            }
        }
    }
}
