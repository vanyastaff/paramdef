//! Integration tests for the Value system.

use paramdef::core::{Key, Value};

#[test]
fn test_value_primitives() {
    // Null
    let null = Value::Null;
    assert!(null.is_null());
    assert!(null.is_empty());

    // Boolean
    let bool_true = Value::Bool(true);
    let bool_false = Value::Bool(false);
    assert!(bool_true.is_bool());
    assert_eq!(bool_true.as_bool(), Some(true));
    assert_eq!(bool_false.as_bool(), Some(false));

    // Integer
    let int = Value::Int(42);
    assert!(int.is_numeric());
    assert_eq!(int.as_i64(), Some(42));
    assert_eq!(int.as_f64(), Some(42.0));

    // Float
    let float = Value::Float(3.14);
    assert!(float.is_numeric());
    assert_eq!(float.as_f64(), Some(3.14));

    // Text
    let text = Value::text("hello");
    assert_eq!(text.as_text(), Some("hello"));
}

#[test]
fn test_value_collections() {
    // Array
    let array = Value::array([Value::Int(1), Value::Int(2), Value::Int(3)]);
    assert!(!array.is_empty());
    let arr = array.as_array().unwrap();
    assert_eq!(arr.len(), 3);
    assert_eq!(arr[0], Value::Int(1));

    // Empty array
    let empty_array = Value::array(Vec::<Value>::new());
    assert!(empty_array.is_empty());

    // Object
    let object = Value::object([("name", Value::text("Alice")), ("age", Value::Int(30))]);
    let obj = object.as_object().unwrap();
    assert_eq!(obj.len(), 2);
    assert_eq!(obj.get(&Key::new("name")), Some(&Value::text("Alice")));

    // Empty object
    let empty_obj = Value::object(Vec::<(&str, Value)>::new());
    assert!(empty_obj.is_empty());
}

#[test]
fn test_value_binary() {
    let data = vec![0x00, 0x01, 0x02, 0xFF];
    let binary = Value::binary(data.clone());

    let retrieved = binary.as_binary().unwrap();
    assert_eq!(retrieved.as_ref(), data.as_slice());
}

#[test]
fn test_value_from_conversions() {
    // From bool
    let from_bool: Value = true.into();
    assert_eq!(from_bool, Value::Bool(true));

    // From i32
    let from_i32: Value = 42i32.into();
    assert_eq!(from_i32, Value::Int(42));

    // From i64
    let from_i64: Value = 100i64.into();
    assert_eq!(from_i64, Value::Int(100));

    // From f64
    let from_f64: Value = 3.14f64.into();
    assert_eq!(from_f64, Value::Float(3.14));

    // From &str
    let from_str: Value = "hello".into();
    assert_eq!(from_str, Value::text("hello"));

    // From String
    let from_string: Value = String::from("world").into();
    assert_eq!(from_string, Value::text("world"));

    // From Option<T>
    let from_some: Value = Some(42i32).into();
    assert_eq!(from_some, Value::Int(42));

    let from_none: Value = Option::<i32>::None.into();
    assert_eq!(from_none, Value::Null);
}

#[test]
fn test_value_type_names() {
    assert_eq!(Value::Null.type_name(), "null");
    assert_eq!(Value::Bool(true).type_name(), "bool");
    assert_eq!(Value::Int(0).type_name(), "int");
    assert_eq!(Value::Float(0.0).type_name(), "float");
    assert_eq!(Value::text("").type_name(), "text");
    assert_eq!(Value::array(Vec::<Value>::new()).type_name(), "array");
    assert_eq!(
        Value::object(Vec::<(&str, Value)>::new()).type_name(),
        "object"
    );
    assert_eq!(Value::binary(Vec::<u8>::new()).type_name(), "binary");
}

#[test]
fn test_value_equality() {
    // Same values are equal
    assert_eq!(Value::Int(42), Value::Int(42));
    assert_eq!(Value::text("hello"), Value::text("hello"));

    // Different values are not equal
    assert_ne!(Value::Int(42), Value::Int(43));
    assert_ne!(Value::Int(42), Value::Float(42.0));
    assert_ne!(Value::text("hello"), Value::text("world"));
}

#[test]
fn test_value_clone() {
    let original = Value::object([
        ("name", Value::text("Test")),
        (
            "values",
            Value::array([Value::Int(1), Value::Int(2), Value::Int(3)]),
        ),
    ]);

    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_nested_structures() {
    // Complex nested structure
    let config = Value::object([
        ("name", Value::text("MyApp")),
        ("version", Value::Int(1)),
        ("enabled", Value::Bool(true)),
        (
            "settings",
            Value::object([
                ("timeout", Value::Float(30.5)),
                (
                    "retries",
                    Value::array([Value::Int(1), Value::Int(2), Value::Int(3)]),
                ),
            ]),
        ),
        (
            "tags",
            Value::array([Value::text("prod"), Value::text("stable")]),
        ),
    ]);

    // Verify structure
    let obj = config.as_object().unwrap();
    assert_eq!(obj.get(&Key::new("name")), Some(&Value::text("MyApp")));
    assert_eq!(obj.get(&Key::new("version")), Some(&Value::Int(1)));

    // Access nested object
    let settings = obj.get(&Key::new("settings")).unwrap();
    let settings_obj = settings.as_object().unwrap();
    assert_eq!(
        settings_obj.get(&Key::new("timeout")),
        Some(&Value::Float(30.5))
    );
}
