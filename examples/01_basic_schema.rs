//! Basic schema creation and usage example.
//!
//! Demonstrates:
//! - Creating a simple schema with text and number parameters
//! - Creating a context from the schema
//! - Setting and getting values
//! - Checking dirty state

use paramdef::context::Context;
use paramdef::core::Value;
use paramdef::schema::Schema;
use paramdef::types::leaf::{Number, Text};
use std::sync::Arc;

fn main() {
    // 1. Create a schema with basic parameters
    let schema = Schema::builder()
        .parameter(
            Text::builder("username")
                .label("Username")
                .description("Your display name")
                .default("anonymous")
                .build(),
        )
        .parameter(
            Text::builder("email")
                .label("Email Address")
                .description("Your email for notifications")
                .build(),
        )
        .parameter(
            Number::integer("age")
                .label("Age")
                .description("Your age in years")
                .default(18.0)
                .build(),
        )
        .build();

    println!("Schema created with {} parameters", schema.len());
    println!("Parameters: {:?}", schema.keys().collect::<Vec<_>>());

    // 2. Create a runtime context
    let mut ctx = Context::new(Arc::new(schema));
    println!("\nInitial context state:");
    println!("  Is dirty: {}", ctx.is_dirty());
    println!("  Is valid: {}", ctx.is_valid());

    // 3. Set values
    ctx.set("username", Value::text("alice")).unwrap();
    ctx.set("email", Value::text("alice@example.com")).unwrap();
    ctx.set("age", Value::Int(25)).unwrap();

    println!("\nAfter setting values:");
    println!("  Is dirty: {}", ctx.is_dirty());
    println!("  Username: {:?}", ctx.get("username"));
    println!("  Email: {:?}", ctx.get("email"));
    println!("  Age: {:?}", ctx.get("age"));

    // 4. Collect all values
    let values = ctx.collect_values();
    println!("\nAll values:");
    for (key, value) in values {
        println!("  {}: {:?}", key, value);
    }

    // 5. Mark as clean
    ctx.mark_all_clean();
    println!("\nAfter marking clean:");
    println!("  Is dirty: {}", ctx.is_dirty());

    // 6. Modify one field
    ctx.set("age", Value::Int(26)).unwrap();
    println!("\nAfter modifying age:");
    println!("  Is dirty: {}", ctx.is_dirty());
    let dirty = ctx.collect_dirty_values();
    println!("  Dirty fields: {:?}", dirty.keys().collect::<Vec<_>>());

    // 7. Reset context
    ctx.reset();
    println!("\nAfter reset:");
    println!("  Is dirty: {}", ctx.is_dirty());
    println!("  Values cleared: {}", ctx.collect_values().is_empty());
}
