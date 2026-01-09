//! Example demonstrating JSON Schema export functionality.
//!
//! Shows how to:
//! - Export paramdef schemas to JSON Schema
//! - Configure export options (strict mode)
//! - Handle required fields
//! - Work with different parameter types

use paramdef::export::JsonSchemaExporter;
use paramdef::schema::Schema;
use paramdef::types::leaf::{Boolean, Number, Text};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a schema representing a user registration form
    let schema = Schema::builder()
        // Required text field (email)
        .parameter(
            Text::builder("email")
                .subtype(paramdef::subtype::Email)
                .label("Email Address")
                .description("User's email address")
                .required()
                .build(),
        )
        // Required text field (username)
        .parameter(
            Text::builder("username")
                .label("Username")
                .description("Unique username (3-20 characters)")
                .required()
                .build(),
        )
        // Required password field
        .parameter(
            Text::builder("password")
                .subtype(paramdef::subtype::Password)
                .label("Password")
                .description("Account password (min 8 characters)")
                .required()
                .sensitive()
                .build(),
        )
        // Optional text field
        .parameter(
            Text::builder("bio")
                .label("Bio")
                .description("User biography (optional)")
                .build(),
        )
        // Required number field (age)
        .parameter(
            Number::integer("age")
                .label("Age")
                .description("User's age in years")
                .required()
                .build(),
        )
        // Optional boolean field
        .parameter(
            Boolean::builder("newsletter")
                .label("Subscribe to Newsletter")
                .description("Receive email updates")
                .default(false)
                .build(),
        )
        .build();

    // Export to JSON Schema
    let exporter = JsonSchemaExporter::new()
        .with_title("User Registration")
        .with_description("Schema for user registration form")
        .strict(true); // additionalProperties: false

    let json_schema = exporter.export(&schema)?;

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&json_schema)?;
    println!("{json}");

    // Verify structure
    println!("\nSchema validation:");
    println!("- Title: {:?}", json_schema.title);
    println!("- Type: {}", json_schema.type_);

    let prop_count = json_schema
        .properties
        .as_ref()
        .map(|p| p.len())
        .unwrap_or(0);
    println!("- Properties count: {prop_count}");

    let req_count = json_schema.required.as_ref().map(|r| r.len()).unwrap_or(0);
    println!("- Required fields: {req_count}");

    if let Some(required) = &json_schema.required {
        println!("  Required: {required:?}");
    }

    Ok(())
}
