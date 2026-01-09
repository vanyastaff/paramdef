//! Validation example.
//!
//! Demonstrates:
//! - Creating validation rules
//! - Validating values
//! - Handling validation errors

#[cfg(feature = "validation")]
use paramdef::context::Context;
#[cfg(feature = "validation")]
use paramdef::core::Value;
#[cfg(feature = "validation")]
use paramdef::schema::Schema;
#[cfg(feature = "validation")]
use paramdef::validation::{Rule, Rules};
#[cfg(feature = "validation")]
use std::sync::Arc;

#[cfg(feature = "validation")]
fn main() {
    println!("=== Validation Example ===\n");

    // Example 1: Required rule
    let rule = Rule::required();
    let ctx = Context::new(Arc::new(Schema::builder().build()));

    println!("Required validation:");
    println!("  Valid: {:?}", rule.validate(&Value::text("hello"), &ctx));
    println!("  Invalid: {:?}\n", rule.validate(&Value::Null, &ctx));

    // Example 2: Length validation
    let rule = Rule::min_length(5);
    println!("Min length (5):");
    println!("  Valid: {:?}", rule.validate(&Value::text("hello"), &ctx));
    println!("  Invalid: {:?}\n", rule.validate(&Value::text("hi"), &ctx));

    // Example 3: Pattern matching
    let rule = Rule::pattern(r"^[a-z]+$");
    println!("Pattern (lowercase only):");
    println!("  Valid: {:?}", rule.validate(&Value::text("hello"), &ctx));
    println!(
        "  Invalid: {:?}\n",
        rule.validate(&Value::text("Hello123"), &ctx)
    );

    // Example 4: Email validation
    let rule = Rule::email();
    println!("Email validation:");
    println!(
        "  Valid: {:?}",
        rule.validate(&Value::text("user@example.com"), &ctx)
    );
    println!(
        "  Invalid: {:?}\n",
        rule.validate(&Value::text("not-an-email"), &ctx)
    );

    // Example 5: Multiple rules (Rules pipeline)
    let rules = Rules::from_rules([
        Rule::required(),
        Rule::min_length(3),
        Rule::max_length(20),
        Rule::pattern(r"^[a-zA-Z0-9_]+$"),
    ]);

    println!("Username validation (required, 3-20 chars, alphanumeric):");
    println!(
        "  'alice': {:?}",
        rules.validate(&Value::text("alice"), &ctx)
    );
    println!("  'al': {:?}", rules.validate(&Value::text("al"), &ctx));
    println!(
        "  'alice@123': {:?}",
        rules.validate(&Value::text("alice@123"), &ctx)
    );
    println!("  null: {:?}", rules.validate(&Value::Null, &ctx));
}

#[cfg(not(feature = "validation"))]
fn main() {
    println!("This example requires the 'validation' feature.");
    println!("Run with: cargo run --example 03_validation --features validation");
}
