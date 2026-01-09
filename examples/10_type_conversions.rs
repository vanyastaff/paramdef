//! Type conversions example.
//!
//! Demonstrates:
//! - Converting from Rust types to Value
//! - Converting from Value to Rust types
//! - Type safety

use paramdef::core::Value;

fn main() {
    println!("=== Type Conversions Example ===\n");

    // From Rust types to Value
    println!("From Rust types to Value:");

    let from_bool: Value = true.into();
    println!("  bool -> Value: {:?}", from_bool);

    let from_i32: Value = 42i32.into();
    println!("  i32 -> Value: {:?}", from_i32);

    let from_i64: Value = 42i64.into();
    println!("  i64 -> Value: {:?}", from_i64);

    let from_f32: Value = 3.14f32.into();
    println!("  f32 -> Value: {:?}", from_f32);

    let from_f64: Value = 3.14f64.into();
    println!("  f64 -> Value: {:?}", from_f64);

    let from_string: Value = String::from("hello").into();
    println!("  String -> Value: {:?}", from_string);

    let from_str: Value = Value::text("world");
    println!("  &str -> Value: {:?}\n", from_str);

    // From Value to Rust types
    println!("From Value to Rust types:");

    let int_val = Value::Int(42);
    if let Some(i) = int_val.as_i64() {
        println!("  Value::Int -> i64: {}", i);
    }

    let float_val = Value::Float(3.14);
    if let Some(f) = float_val.as_f64() {
        println!("  Value::Float -> f64: {}", f);
    }

    let text_val = Value::text("hello");
    if let Some(s) = text_val.as_text() {
        println!("  Value::Text -> &str: {}", s);
    }

    let bool_val = Value::Bool(true);
    if let Some(b) = bool_val.as_bool() {
        println!("  Value::Bool -> bool: {}\n", b);
    }

    // Option conversions
    println!("Option conversions:");

    let some_val: Value = Some(42i32).into();
    println!("  Some(42) -> Value: {:?}", some_val);

    let none_val: Value = Option::<i32>::None.into();
    println!("  None -> Value: {:?}\n", none_val);

    // Type checking before conversion
    println!("Safe conversions with type checking:");

    let value = Value::Int(42);
    if value.as_i64().is_some() {
        println!("  Value can convert to i64: {:?}", value.as_i64());
    }

    let value = Value::text("hello");
    if value.is_text() {
        println!("  Value is text: {:?}", value.as_text());
    }

    let value = Value::Null;
    if value.is_null() {
        println!("  Value is null");
    }

    // Failed conversions return None
    println!("\nFailed conversions:");
    let text_val = Value::text("not a number");
    println!("  text.as_i64(): {:?}", text_val.as_i64());
    println!("  text.as_f64(): {:?}", text_val.as_f64());
}
