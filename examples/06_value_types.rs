//! Value types example.
//!
//! Demonstrates:
//! - Creating different value types
//! - Type checking and conversion
//! - Working with arrays and objects

use paramdef::core::Value;

fn main() {
    println!("=== Value Types Example ===\n");

    // Primitive types
    println!("Primitive values:");
    let null = Value::Null;
    let bool_val = Value::Bool(true);
    let int_val = Value::Int(42);
    let float_val = Value::Float(3.14);
    let text_val = Value::text("hello");

    println!("  Null: {:?}", null);
    println!("  Bool: {:?}", bool_val);
    println!("  Int: {:?}", int_val);
    println!("  Float: {:?}", float_val);
    println!("  Text: {:?}\n", text_val);

    // Type checking
    println!("Type checking:");
    println!("  int_val.is_null(): {}", int_val.is_null());
    println!("  text_val.is_text(): {}", text_val.is_text());
    println!("  bool_val.is_bool(): {}", bool_val.is_bool());
    println!("  array.is_array(): {}", Value::array([]).is_array());

    // Type names
    println!("Type names:");
    println!("  null type: {}", null.type_name());
    println!("  bool type: {}", bool_val.type_name());
    println!("  int type: {}", int_val.type_name());
    println!("  float type: {}", float_val.type_name());
    println!("  text type: {}\n", text_val.type_name());

    // Arrays
    println!("Arrays:");
    let array = Value::array([Value::Int(1), Value::Int(2), Value::Int(3)]);
    println!("  Array: {:?}", array);
    println!("  Is array: {}", array.is_array());
    if let Some(arr) = array.as_array() {
        println!("  Length: {}", arr.len());
        println!("  First: {:?}\n", arr.first());
    }

    // Objects
    println!("Objects:");
    let object = Value::object([
        ("name", Value::text("Alice")),
        ("age", Value::Int(30)),
        ("active", Value::Bool(true)),
    ]);
    println!("  Object: {:?}", object);
    println!("  Is object: {}", object.is_object());
    if let Some(obj) = object.as_object() {
        println!("  Keys: {:?}", obj.keys().collect::<Vec<_>>());
        println!("  Has 'name': {}", obj.contains_key("name"));
        println!("  name value: {:?}\n", obj.get("name"));
    }

    // Value conversions
    println!("Value access:");
    println!("  int_val.as_i64(): {:?}", int_val.as_i64());
    println!("  float_val.as_f64(): {:?}", float_val.as_f64());
    println!("  text_val.as_text(): {:?}", text_val.as_text());
    println!("  bool_val.as_bool(): {:?}", bool_val.as_bool());
}
