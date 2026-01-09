//! Validation example.
//!
//! Demonstrates:
//! - Creating validation rules
//! - Validating values with standalone rules
//! - Handling validation errors

#[cfg(feature = "validation")]
use paramdef::core::Value;
#[cfg(feature = "validation")]
use paramdef::validation::Rule;

#[cfg(feature = "validation")]
fn main() {
    println!("=== Validation Example ===\n");

    // Example 1: Required rule
    let rule = Rule::required();

    println!("Required validation:");
    println!("  Valid: {:?}", rule.check(&Value::text("hello")));
    println!("  Invalid: {:?}\n", rule.check(&Value::Null));

    // Example 2: Length validation
    let rule = Rule::min_length(5);
    println!("Min length (5):");
    println!("  Valid: {:?}", rule.check(&Value::text("hello")));
    println!("  Invalid: {:?}\n", rule.check(&Value::text("hi")));

    let rule = Rule::max_length(10);
    println!("Max length (10):");
    println!("  Valid: {:?}", rule.check(&Value::text("hello")));
    println!(
        "  Invalid: {:?}\n",
        rule.check(&Value::text("hello world!!"))
    );

    // Example 3: Pattern matching
    let rule = Rule::pattern(r"^[a-z]+$");
    println!("Pattern (lowercase only):");
    println!("  Valid: {:?}", rule.check(&Value::text("hello")));
    println!("  Invalid: {:?}\n", rule.check(&Value::text("Hello123")));

    // Example 4: Email validation
    let rule = Rule::email();
    println!("Email validation:");
    println!(
        "  Valid: {:?}",
        rule.check(&Value::text("user@example.com"))
    );
    println!(
        "  Invalid: {:?}\n",
        rule.check(&Value::text("not-an-email"))
    );

    // Example 5: URL validation
    let rule = Rule::url();
    println!("URL validation:");
    println!(
        "  Valid: {:?}",
        rule.check(&Value::text("https://example.com"))
    );
    println!("  Invalid: {:?}\n", rule.check(&Value::text("not a url")));

    // Example 6: Number range
    let rule = Rule::min(0.0);
    println!("Min value (0.0):");
    println!("  Valid: {:?}", rule.check(&Value::Int(5)));
    println!("  Invalid: {:?}\n", rule.check(&Value::Int(-5)));

    let rule = Rule::max(100.0);
    println!("Max value (100.0):");
    println!("  Valid: {:?}", rule.check(&Value::Int(50)));
    println!("  Invalid: {:?}\n", rule.check(&Value::Int(150)));

    println!("Note: For complex validation pipelines with multiple rules,");
    println!("use Rules::from_rules() and validate with a ValidationContext.");
}

#[cfg(not(feature = "validation"))]
fn main() {
    println!("This example requires the 'validation' feature.");
    println!("Run with: cargo run --example 03_validation --features validation");
}
