//! Serialization example.
//!
//! Demonstrates:
//! - Serializing values to JSON
//! - Deserializing values from JSON
//! - Working with nested structures

#[cfg(feature = "serde")]
use paramdef::core::Value;

#[cfg(feature = "serde")]
fn main() {
    println!("=== Serialization Example ===\n");

    // Example 1: Simple value serialization
    let value = Value::text("hello");
    let json = serde_json::to_string(&value).unwrap();
    println!("Text value:");
    println!("  Value: {:?}", value);
    println!("  JSON: {}\n", json);

    // Example 2: Number serialization
    let value = Value::Int(42);
    let json = serde_json::to_string(&value).unwrap();
    println!("Number value:");
    println!("  Value: {:?}", value);
    println!("  JSON: {}\n", json);

    // Example 3: Array serialization
    let value = Value::array([Value::Int(1), Value::Int(2), Value::Int(3)]);
    let json = serde_json::to_string_pretty(&value).unwrap();
    println!("Array value:");
    println!("  Value: {:?}", value);
    println!("  JSON:\n{}\n", json);

    // Example 4: Object serialization
    let value = Value::object([
        ("name", Value::text("Alice")),
        ("age", Value::Int(30)),
        ("active", Value::Bool(true)),
    ]);
    let json = serde_json::to_string_pretty(&value).unwrap();
    println!("Object value:");
    println!("  JSON:\n{}\n", json);

    // Example 5: Nested structure
    let value = Value::object([
        (
            "user",
            Value::object([
                ("name", Value::text("Bob")),
                ("email", Value::text("bob@example.com")),
            ]),
        ),
        (
            "scores",
            Value::array([Value::Int(95), Value::Int(87), Value::Int(92)]),
        ),
    ]);
    let json = serde_json::to_string_pretty(&value).unwrap();
    println!("Nested structure:");
    println!("  JSON:\n{}\n", json);

    // Example 6: Deserialization
    let json = r#"{"name":"Charlie","age":25}"#;
    let value: Value = serde_json::from_str(json).unwrap();
    println!("Deserialization:");
    println!("  JSON: {}", json);
    println!("  Value: {:?}\n", value);

    // Example 7: Round-trip
    let original = Value::object([
        ("id", Value::Int(123)),
        ("status", Value::text("active")),
        (
            "tags",
            Value::array([Value::text("rust"), Value::text("paramdef")]),
        ),
    ]);
    let json = serde_json::to_string(&original).unwrap();
    let restored: Value = serde_json::from_str(&json).unwrap();

    println!("Round-trip test:");
    println!("  Original: {:?}", original);
    println!("  Restored: {:?}", restored);
    println!("  Equal: {}", original == restored);
}

#[cfg(not(feature = "serde"))]
fn main() {
    println!("This example requires the 'serde' feature.");
    println!("Run with: cargo run --example 07_serialization --features serde");
}
