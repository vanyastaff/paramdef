//! Visibility expressions example.
//!
//! Demonstrates:
//! - Creating visibility conditions with the fluent API
//! - Evaluating rules against context
//! - Logical operators (And, Or, Not)

#[cfg(feature = "visibility")]
use paramdef::context::Context;
#[cfg(feature = "visibility")]
use paramdef::core::Value;
#[cfg(feature = "visibility")]
use paramdef::expr::{Expr, Rule};
#[cfg(feature = "visibility")]
use paramdef::schema::Schema;
#[cfg(feature = "visibility")]
use paramdef::types::leaf::{Boolean, Text};
#[cfg(feature = "visibility")]
use paramdef::visibility::when;
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

    // Example 1: Simple equality check using fluent API
    ctx.set("mode", Value::text("advanced")).unwrap();
    let rule = when("mode").eq(Value::text("advanced"));
    println!("Eq expression (mode == 'advanced'):");
    println!("  Result: {}\n", rule.eval(&ctx));

    // Example 2: Boolean check
    ctx.set("show_advanced", Value::Bool(true)).unwrap();
    let rule = when("show_advanced").is_true();
    println!("IsTrue expression (show_advanced == true):");
    println!("  Result: {}\n", rule.eval(&ctx));

    // Example 3: AND logic
    let rule = Rule::field(
        "show_advanced",
        Expr::and(vec![Expr::is_true(), Expr::eq(Value::text("advanced"))]),
    );
    println!("Complex expression (show_advanced AND mode=='advanced'):");
    println!("  Note: This example shows manual Rule construction");
    println!("  Result: {}\n", rule.eval(&ctx));

    // Example 4: Multiple field checks (OR logic)
    println!("Or logic (role=='admin' OR role=='moderator'):");
    let is_admin = when("user_role").eq(Value::text("admin"));
    let is_mod = when("user_role").eq(Value::text("moderator"));
    println!("  Result (role=guest): {}", is_admin.eval(&ctx));
    println!("                       {}", is_mod.eval(&ctx));

    ctx.set("user_role", Value::text("admin")).unwrap();
    println!("  Result (role=admin): {}", is_admin.eval(&ctx));
    println!("                       {}\n", is_mod.eval(&ctx));

    // Example 5: NOT logic with when()
    let rule = when("mode").ne(Value::text("basic"));
    println!("Not-equal expression (mode != 'basic'):");
    println!("  Result (mode=advanced): {}\n", rule.eval(&ctx));

    // Example 6: Fluent API chaining
    println!("Fluent API examples:");

    let rule = when("show_advanced").is_true();
    println!("  show_advanced is true: {}", rule.eval(&ctx));

    let rule = when("mode").eq(Value::text("advanced"));
    println!("  mode == 'advanced': {}", rule.eval(&ctx));

    let rule = when("user_role").eq(Value::text("admin"));
    println!("  user_role == 'admin': {}\n", rule.eval(&ctx));

    // Example 7: Dependency tracking
    let rule = when("mode").eq(Value::text("advanced"));
    let deps = rule.dependencies();
    println!("Expression dependencies for 'when(\"mode\").eq(...)':");
    for dep in deps {
        println!("  - {:?}", dep);
    }
}

#[cfg(not(feature = "visibility"))]
fn main() {
    println!("This example requires the 'visibility' feature.");
    println!("Run with: cargo run --example 04_visibility --features visibility");
}
