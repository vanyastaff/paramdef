//! Schema iteration example.
//!
//! Demonstrates:
//! - Iterating over schema parameters
//! - Accessing parameter metadata
//! - Filtering parameters

use paramdef::schema::Schema;
use paramdef::types::leaf::{Boolean, Number, Text};

fn main() {
    println!("=== Schema Iteration Example ===\n");

    // Create a schema with multiple parameters
    let schema = Schema::builder()
        .parameter(
            Text::builder("username")
                .label("Username")
                .description("Your username")
                .group("Account")
                .build(),
        )
        .parameter(
            Text::builder("email")
                .label("Email")
                .description("Your email address")
                .group("Account")
                .build(),
        )
        .parameter(
            Boolean::builder("newsletter")
                .label("Subscribe to Newsletter")
                .description("Receive updates via email")
                .group("Preferences")
                .default(false)
                .build(),
        )
        .parameter(
            Number::integer("age")
                .label("Age")
                .description("Your age")
                .group("Profile")
                .build(),
        )
        .build();

    // Basic iteration
    println!("All parameters:");
    for param in schema.iter() {
        println!("  - {}", param.key());
    }

    // Access metadata
    println!("\nParameter details:");
    for param in schema.iter() {
        println!("\n{}:", param.key());
        if let Some(label) = param.metadata().label() {
            println!("  Label: {}", label);
        }
        if let Some(desc) = param.metadata().description() {
            println!("  Description: {}", desc);
        }
        if let Some(group) = param.metadata().group() {
            println!("  Group: {}", group);
        }
    }

    // Get specific parameter
    println!("\nGet by key:");
    if let Some(param) = schema.get("username") {
        println!("  Found: {}", param.key());
    }

    // Check existence
    println!("\nParameter existence:");
    println!("  Has 'username': {}", schema.get("username").is_some());
    println!("  Has 'invalid': {}", schema.get("invalid").is_some());

    // Count parameters
    println!("\nSchema statistics:");
    println!("  Total parameters: {}", schema.len());
    println!("  Is empty: {}", schema.is_empty());

    // Collect keys
    println!("\nAll keys:");
    let keys: Vec<_> = schema.keys().collect();
    println!("  {:?}", keys);
}
