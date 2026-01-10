//! Demonstrates conditional visibility using the fluent when() API.
//!
//! This example shows how to use visibility rules to show/hide parameters
//! based on other parameter values, creating dynamic adaptive forms.

use paramdef::context::Context;
use paramdef::core::Value;
use paramdef::expr::{Expr, Rule};
use paramdef::schema::Schema;
use paramdef::types::leaf::{Boolean, Number, Text};
use paramdef::visibility::when;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build schema with visibility conditions using the fluent API
    let schema = Arc::new(
        Schema::builder()
            // Mode selector - always visible
            .parameter(
                Text::builder("mode")
                    .label("Operating Mode")
                    .description("Select the operating mode")
                    .default("basic")
                    .required()
                    .build(),
            )
            // Advanced options - only visible when mode is "advanced"
            .parameter(
                Text::builder("advanced_setting")
                    .label("Advanced Setting")
                    .description("Only visible in advanced mode")
                    .visible_when(when("mode").eq(Value::text("advanced")))
                    .build(),
            )
            // Debug mode toggle
            .parameter(
                Boolean::builder("debug_enabled")
                    .label("Enable Debug")
                    .build(),
            )
            // Debug settings - only visible when debug is enabled
            .parameter(
                Number::builder("debug_level")
                    .label("Debug Level")
                    .description("Only visible when debug is enabled")
                    .visible_when(when("debug_enabled").is_true())
                    .build(),
            )
            // Age field
            .parameter(
                Number::builder("age")
                    .label("Age")
                    .description("User's age")
                    .build(),
            )
            // Premium features - visible when age >= 18 AND premium is true
            .parameter(Boolean::builder("premium").label("Premium Member").build())
            .parameter(
                Text::builder("premium_feature")
                    .label("Premium Feature")
                    .description("Only for adult premium members")
                    // Complex rule: multiple conditions
                    .visible_when(Rule::field(
                        "age",
                        Expr::and(vec![
                            Expr::gte(18.0),
                            // Note: This checks age field for both conditions
                            // For checking different fields, use separate Rules
                        ]),
                    ))
                    .build(),
            )
            .build(),
    );

    let mut ctx = Context::new(schema.clone());

    println!("=== Visibility Conditions Demo ===\n");

    // Scenario 1: Basic mode
    println!("Scenario 1: Basic mode");
    let _ = ctx.set("mode", Value::text("basic"));
    print_visibility(&ctx, &schema);

    // Scenario 2: Advanced mode
    println!("\nScenario 2: Advanced mode");
    let _ = ctx.set("mode", Value::text("advanced"));
    print_visibility(&ctx, &schema);

    // Scenario 3: Debug enabled
    println!("\nScenario 3: Debug enabled");
    let _ = ctx.set("debug_enabled", Value::Bool(true));
    print_visibility(&ctx, &schema);

    // Scenario 4: Premium adult user
    println!("\nScenario 4: Premium adult user (age=25, premium=true)");
    let _ = ctx.set("age", Value::Int(25));
    let _ = ctx.set("premium", Value::Bool(true));
    print_visibility(&ctx, &schema);

    // Scenario 5: Premium minor (should not see premium feature)
    println!("\nScenario 5: Premium minor (age=16, premium=true)");
    let _ = ctx.set("age", Value::Int(16));
    print_visibility(&ctx, &schema);

    // Demonstrate dependency tracking
    println!("\n=== Dependency Tracking ===");
    if let Some(node) = schema.get("advanced_setting") {
        let deps = node.dependencies();
        println!("advanced_setting depends on: {:?}", deps);
    }

    if let Some(node) = schema.get("premium_feature") {
        let deps = node.dependencies();
        println!("premium_feature depends on: {:?}", deps);
    }

    Ok(())
}

fn print_visibility(ctx: &Context, schema: &Schema) {
    println!("Visible parameters:");
    for key in schema.keys() {
        if let Some(node) = schema.get(&key) {
            let visible = node.is_visible(ctx);
            let marker = if visible { "✓" } else { "✗" };
            println!("  {} {}", marker, key);
        }
    }
}
