//! Expression parser example.
//!
//! Demonstrates parsing validation rules from strings using the expression parser.

#[cfg(feature = "validation")]
fn main() {
    use paramdef::expr::{Expr, Rule};
    use paramdef::parser::parse;

    println!("=== Expression Parser Demo ===\n");

    // Simple comparisons
    println!("1. Simple Comparisons:");
    let expr = parse("age >= 18").unwrap();
    println!("   'age >= 18' → {:?}", expr);

    let expr = parse(r#"name == "admin""#).unwrap();
    println!(r#"   'name == "admin"' → {:?}"#, expr);

    let expr = parse("active == true").unwrap();
    println!("   'active == true' → {:?}\n", expr);

    // Function calls
    println!("2. Function Calls:");
    let expr = parse("email()").unwrap();
    println!("   'email()' → {:?}", expr);

    let expr = parse("min_length(8)").unwrap();
    println!("   'min_length(8)' → {:?}", expr);

    let expr = parse(r#"starts_with("admin")"#).unwrap();
    println!(r#"   'starts_with("admin")' → {:?}"#, expr);
    println!();

    // Logical operators
    println!("3. Logical Operators:");
    let expr = parse("age >= 18 AND premium == true").unwrap();
    println!("   'age >= 18 AND premium == true' → {:?}", expr);

    let expr = parse("admin == true OR moderator == true").unwrap();
    println!("   'admin == true OR moderator == true' → {:?}", expr);

    let expr = parse("NOT empty()").unwrap();
    println!("   'NOT empty()' → {:?}\n", expr);

    // Complex expressions
    println!("4. Complex Expressions:");
    let expr = parse("email() AND min_length(5) AND max_length(100)").unwrap();
    println!("   'email() AND min_length(5) AND max_length(100)'");
    println!("   → {:?}\n", expr);

    // Parentheses
    println!("5. Parentheses:");
    let expr = parse("(age >= 18 OR premium == true) AND active == true").unwrap();
    println!("   '(age >= 18 OR premium == true) AND active == true'");
    println!("   → {:?}\n", expr);

    // Using Expr::parse()
    println!("6. Using Expr::parse():");
    let expr = Expr::parse("positive() AND integer()").unwrap();
    println!("   'positive() AND integer()' → {:?}\n", expr);

    // Using Rule::parse()
    println!("7. Using Rule::parse():");
    let rule = Rule::parse("url() AND max_length(2048)").unwrap();
    println!("   'url() AND max_length(2048)' → {:?}", rule);
    println!("   Target: {:?}, Local: {}", rule.target, rule.is_local());
    println!();

    // Real-world example
    println!("8. Real-World Example - Password Validation:");
    let password_rule = Rule::parse("min_length(8) AND max_length(128)").unwrap();
    println!("   Rule: 'min_length(8) AND max_length(128)'");
    println!("   Parsed: {:?}", password_rule);
    println!();

    // Error handling
    println!("9. Error Handling:");
    let result = parse("age = 18"); // Invalid: should be ==
    match result {
        Ok(_) => println!("   Unexpectedly succeeded"),
        Err(e) => println!("   'age = 18' → Error: {}", e),
    }

    let result = parse("(age >= 18"); // Missing closing paren
    match result {
        Ok(_) => println!("   Unexpectedly succeeded"),
        Err(e) => println!("   '(age >= 18' → Error: {}", e),
    }
}

#[cfg(not(feature = "validation"))]
fn main() {
    println!("This example requires the 'validation' feature.");
    println!("Run with: cargo run --example 15_expression_parser --features validation");
}
