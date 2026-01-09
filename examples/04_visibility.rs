//! Visibility expressions example.
//!
//! Demonstrates:
//! - Creating visibility conditions
//! - Evaluating expressions against context
//! - Logical operators (And, Or, Not)

#[cfg(feature = "visibility")]
use paramdef::context::Context;
#[cfg(feature = "visibility")]
use paramdef::core::Value;
#[cfg(feature = "visibility")]
use paramdef::schema::Schema;
#[cfg(feature = "visibility")]
use paramdef::types::leaf::{Boolean, Text};
#[cfg(feature = "visibility")]
use paramdef::visibility::Expr;
#[cfg(feature = "visibility")]
use std::sync::Arc;

#[cfg(feature = "visibility")]
fn main() {
    println!("=== Visibility Expressions Example ===\n");

    // Create a schema with some parameters
    let schema = Schema::builder()
        .parameter(Boolean::builder("show_advanced").default(false).build())
        .parameter(Text::builder("mode").default("basic").build())
        .parameter(Text::builder("user_role").default("guest").build())
        .build();

    let mut ctx = Context::new(Arc::new(schema));

    // Example 1: Simple equality check
    ctx.set("mode", Value::text("advanced"));
    let expr = Expr::eq("mode", Value::text("advanced"));
    println!("Eq expression (mode == 'advanced'):");
    println!("  Result: {}\n", expr.eval(&ctx));

    // Example 2: Boolean check
    ctx.set("show_advanced", Value::Bool(true));
    let expr = Expr::is_true("show_advanced");
    println!("IsTrue expression (show_advanced == true):");
    println!("  Result: {}\n", expr.eval(&ctx));

    // Example 3: AND logic
    let expr = Expr::and(vec![
        Expr::is_true("show_advanced"),
        Expr::eq("mode", Value::text("advanced")),
    ]);
    println!("And expression (show_advanced AND mode=='advanced'):");
    println!("  Result: {}\n", expr.eval(&ctx));

    // Example 4: OR logic
    let expr = Expr::or(vec![
        Expr::eq("user_role", Value::text("admin")),
        Expr::eq("user_role", Value::text("moderator")),
    ]);
    println!("Or expression (role=='admin' OR role=='moderator'):");
    println!("  Result (role=guest): {}", expr.eval(&ctx));

    ctx.set("user_role", Value::text("admin"));
    println!("  Result (role=admin): {}\n", expr.eval(&ctx));

    // Example 5: NOT logic
    let expr = Expr::negate(Expr::eq("mode", Value::text("basic")));
    println!("Not expression (mode != 'basic'):");
    println!("  Result (mode=advanced): {}\n", expr.eval(&ctx));

    // Example 6: Complex nested expression
    let expr = Expr::and(vec![
        Expr::or(vec![
            Expr::eq("user_role", Value::text("admin")),
            Expr::eq("user_role", Value::text("moderator")),
        ]),
        Expr::is_true("show_advanced"),
    ]);
    println!("Complex: (role=='admin' OR role=='moderator') AND show_advanced:");
    println!("  Result: {}\n", expr.eval(&ctx));

    // Example 7: Dependency tracking
    let expr = Expr::and(vec![
        Expr::is_true("show_advanced"),
        Expr::eq("mode", Value::text("advanced")),
        Expr::eq("user_role", Value::text("admin")),
    ]);
    let deps = expr.dependencies();
    println!("Expression dependencies:");
    for dep in deps {
        println!("  - {:?}", dep);
    }
}

#[cfg(not(feature = "visibility"))]
fn main() {
    println!("This example requires the 'visibility' feature.");
    println!("Run with: cargo run --example 04_visibility --features visibility");
}
