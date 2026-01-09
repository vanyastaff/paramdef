//! Default values example.
//!
//! Demonstrates:
//! - Setting default values in schema
//! - Using defaults in context
//! - Overriding defaults

use paramdef::context::Context;
use paramdef::core::Value;
use paramdef::schema::Schema;
use paramdef::types::leaf::{Boolean, Number, Text};
use std::sync::Arc;

fn main() {
    println!("=== Default Values Example ===\n");

    // Create schema with defaults
    let schema = Schema::builder()
        .parameter(
            Text::builder("username")
                .label("Username")
                .default("guest")
                .build(),
        )
        .parameter(
            Text::builder("theme")
                .label("Theme")
                .default("light")
                .build(),
        )
        .parameter(
            Boolean::builder("notifications")
                .label("Enable Notifications")
                .default(true)
                .build(),
        )
        .parameter(
            Number::integer("timeout")
                .label("Timeout (seconds)")
                .default(30.0)
                .build(),
        )
        .build();

    println!("Schema with defaults created\n");

    // Create context - defaults are not automatically applied
    let mut ctx = Context::new(Arc::new(schema));

    println!("Context state (defaults NOT auto-applied):");
    println!("  username: {:?}", ctx.get("username"));
    println!("  theme: {:?}", ctx.get("theme"));
    println!("  notifications: {:?}", ctx.get("notifications"));
    println!("  timeout: {:?}\n", ctx.get("timeout"));

    // Set some values (overriding defaults)
    ctx.set("username", Value::text("alice")).unwrap();
    ctx.set("theme", Value::text("dark")).unwrap();

    println!("After setting values:");
    println!("  username: {:?}", ctx.get("username"));
    println!("  theme: {:?}", ctx.get("theme"));
    println!("  notifications: {:?}", ctx.get("notifications"));
    println!("  timeout: {:?}\n", ctx.get("timeout"));

    // Clear a value
    ctx.clear("username").unwrap();
    println!("After clearing username:");
    println!("  username: {:?}\n", ctx.get("username"));

    // Note: Default values are defined in the schema
    println!("Schema defines defaults for: username, theme, notifications, timeout");
    println!("These can be accessed by downcasting parameters to their concrete types.");
}
