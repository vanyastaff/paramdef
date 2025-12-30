//! Unified runtime value representation.
//!
//! The [`Value`] enum is the runtime representation for all parameter values.
//! It provides a type-safe way to store and manipulate parameter data.

use std::collections::HashMap;
use std::sync::Arc;

use smartstring::{LazyCompact, SmartString};

use super::Key;

/// Unified runtime representation for all parameter values.
///
/// This enum covers all possible value types that parameters can hold.
/// Collections use [`Arc`] for cheap cloning and thread-safe sharing.
///
/// # Examples
///
/// ```
/// use paramdef::core::Value;
///
/// // Primitive values
/// let null = Value::Null;
/// let boolean = Value::Bool(true);
/// let integer = Value::Int(42);
/// let float = Value::Float(3.14);
/// let text = Value::text("hello");
///
/// // Type checking
/// assert!(null.is_null());
/// assert!(boolean.is_bool());
/// assert_eq!(integer.as_int(), Some(42));
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Value {
    /// Absence of a value.
    #[default]
    Null,

    /// Boolean value.
    Bool(bool),

    /// 64-bit signed integer.
    Int(i64),

    /// 64-bit floating point.
    Float(f64),

    /// Text string using stack-optimized storage.
    Text(SmartString<LazyCompact>),

    /// Ordered array of values.
    Array(Arc<[Value]>),

    /// Key-value object.
    Object(Arc<HashMap<Key, Value>>),

    /// Binary data.
    Binary(Arc<[u8]>),
}

impl Value {
    /// Creates a text value from a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::core::Value;
    ///
    /// let value = Value::text("hello");
    /// assert_eq!(value.as_text(), Some("hello"));
    /// ```
    pub fn text(s: impl Into<SmartString<LazyCompact>>) -> Self {
        Self::Text(s.into())
    }

    /// Creates an array value from an iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::core::Value;
    ///
    /// let value = Value::array([Value::Int(1), Value::Int(2), Value::Int(3)]);
    /// assert_eq!(value.as_array().map(|a| a.len()), Some(3));
    /// ```
    pub fn array(values: impl IntoIterator<Item = Value>) -> Self {
        Self::Array(values.into_iter().collect())
    }

    /// Creates an object value from key-value pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::core::Value;
    ///
    /// let value = Value::object([
    ///     ("name", Value::text("Alice")),
    ///     ("age", Value::Int(30)),
    /// ]);
    /// ```
    pub fn object(pairs: impl IntoIterator<Item = (impl Into<Key>, Value)>) -> Self {
        Self::Object(Arc::new(
            pairs.into_iter().map(|(k, v)| (k.into(), v)).collect(),
        ))
    }

    /// Creates a binary value from bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::core::Value;
    ///
    /// let value = Value::binary([0x00, 0x01, 0x02]);
    /// assert_eq!(value.as_binary().map(|b| b.len()), Some(3));
    /// ```
    pub fn binary(bytes: impl IntoIterator<Item = u8>) -> Self {
        Self::Binary(bytes.into_iter().collect())
    }

    // === Type checking methods ===

    /// Returns `true` if this is a `Null` value.
    #[inline]
    #[must_use]
    pub const fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Returns `true` if this is a `Bool` value.
    #[inline]
    #[must_use]
    pub const fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(_))
    }

    /// Returns `true` if this is an `Int` value.
    #[inline]
    #[must_use]
    pub const fn is_int(&self) -> bool {
        matches!(self, Self::Int(_))
    }

    /// Returns `true` if this is a `Float` value.
    #[inline]
    #[must_use]
    pub const fn is_float(&self) -> bool {
        matches!(self, Self::Float(_))
    }

    /// Returns `true` if this is a `Text` value.
    #[inline]
    #[must_use]
    pub const fn is_text(&self) -> bool {
        matches!(self, Self::Text(_))
    }

    /// Returns `true` if this is an `Array` value.
    #[inline]
    #[must_use]
    pub const fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    /// Returns `true` if this is an `Object` value.
    #[inline]
    #[must_use]
    pub const fn is_object(&self) -> bool {
        matches!(self, Self::Object(_))
    }

    /// Returns `true` if this is a `Binary` value.
    #[inline]
    #[must_use]
    pub const fn is_binary(&self) -> bool {
        matches!(self, Self::Binary(_))
    }

    /// Returns `true` if this is a numeric value (Int or Float).
    #[inline]
    #[must_use]
    pub const fn is_numeric(&self) -> bool {
        matches!(self, Self::Int(_) | Self::Float(_))
    }

    // === Accessor methods ===

    /// Returns the boolean value if this is a `Bool`.
    #[inline]
    #[must_use]
    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Returns the integer value if this is an `Int`.
    #[inline]
    #[must_use]
    pub const fn as_int(&self) -> Option<i64> {
        match self {
            Self::Int(i) => Some(*i),
            _ => None,
        }
    }

    /// Returns the float value if this is a `Float`.
    #[inline]
    #[must_use]
    pub const fn as_float(&self) -> Option<f64> {
        match self {
            Self::Float(f) => Some(*f),
            _ => None,
        }
    }

    /// Returns the text value if this is a `Text`.
    #[inline]
    #[must_use]
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(s) => Some(s.as_str()),
            _ => None,
        }
    }

    /// Returns the array if this is an `Array`.
    #[inline]
    #[must_use]
    pub fn as_array(&self) -> Option<&[Value]> {
        match self {
            Self::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// Returns the object if this is an `Object`.
    #[inline]
    #[must_use]
    pub fn as_object(&self) -> Option<&HashMap<Key, Value>> {
        match self {
            Self::Object(obj) => Some(obj),
            _ => None,
        }
    }

    /// Returns the binary data if this is a `Binary`.
    #[inline]
    #[must_use]
    pub fn as_binary(&self) -> Option<&[u8]> {
        match self {
            Self::Binary(bytes) => Some(bytes),
            _ => None,
        }
    }

    /// Returns the numeric value as f64, converting if necessary.
    #[inline]
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Float(f) => Some(*f),
            Self::Int(i) => Some(*i as f64),
            _ => None,
        }
    }

    /// Returns the numeric value as i64, converting if possible.
    #[inline]
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Int(i) => Some(*i),
            Self::Float(f) => Some(*f as i64),
            _ => None,
        }
    }

    // === Utility methods ===

    /// Returns `true` if this value is considered empty.
    ///
    /// - `Null` is empty
    /// - Empty string is empty
    /// - Empty array is empty
    /// - Empty object is empty
    /// - Empty binary is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Null => true,
            Self::Text(s) => s.is_empty(),
            Self::Array(arr) => arr.is_empty(),
            Self::Object(obj) => obj.is_empty(),
            Self::Binary(bytes) => bytes.is_empty(),
            _ => false,
        }
    }

    /// Returns the type name as a string.
    #[must_use]
    pub const fn type_name(&self) -> &'static str {
        match self {
            Self::Null => "null",
            Self::Bool(_) => "bool",
            Self::Int(_) => "int",
            Self::Float(_) => "float",
            Self::Text(_) => "text",
            Self::Array(_) => "array",
            Self::Object(_) => "object",
            Self::Binary(_) => "binary",
        }
    }
}

// === From implementations ===

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Self::Int(v)
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Self::Int(v.into())
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Self::Float(v)
    }
}

impl From<f32> for Value {
    fn from(v: f32) -> Self {
        Self::Float(v.into())
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Self::text(v)
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Self::text(v)
    }
}

impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(v: Vec<T>) -> Self {
        Self::array(v.into_iter().map(Into::into))
    }
}

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(v: Option<T>) -> Self {
        match v {
            Some(v) => v.into(),
            None => Self::Null,
        }
    }
}

// === serde support ===

#[cfg(feature = "serde")]
mod serde_impl {
    use super::Value;
    use serde::{Deserialize, Serialize};
    use std::fmt;
    use std::str::FromStr;

    impl Serialize for Value {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match self {
                Value::Null => serializer.serialize_none(),
                Value::Bool(b) => serializer.serialize_bool(*b),
                Value::Int(i) => serializer.serialize_i64(*i),
                Value::Float(f) => serializer.serialize_f64(*f),
                Value::Text(s) => serializer.serialize_str(s),
                Value::Array(arr) => arr.serialize(serializer),
                Value::Object(obj) => obj.serialize(serializer),
                Value::Binary(bytes) => serializer.serialize_bytes(bytes),
            }
        }
    }

    impl<'de> Deserialize<'de> for Value {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            // Deserialize as serde_json::Value first, then convert
            let json: serde_json::Value = Deserialize::deserialize(deserializer)?;
            Ok(Value::from(json))
        }
    }

    impl From<Value> for serde_json::Value {
        fn from(value: Value) -> Self {
            match value {
                Value::Null => serde_json::Value::Null,
                Value::Bool(b) => serde_json::Value::Bool(b),
                Value::Int(i) => serde_json::Value::Number(i.into()),
                Value::Float(f) => serde_json::Number::from_f64(f)
                    .map_or(serde_json::Value::Null, serde_json::Value::Number),
                Value::Text(s) => serde_json::Value::String(s.to_string()),
                Value::Array(arr) => {
                    serde_json::Value::Array(arr.iter().cloned().map(Into::into).collect())
                }
                Value::Object(obj) => serde_json::Value::Object(
                    obj.iter()
                        .map(|(k, v)| (k.to_string(), v.clone().into()))
                        .collect(),
                ),
                Value::Binary(bytes) => {
                    use base64::Engine;
                    let encoded = base64::engine::general_purpose::STANDARD.encode(&*bytes);
                    serde_json::Value::String(encoded)
                }
            }
        }
    }

    impl From<serde_json::Value> for Value {
        fn from(json: serde_json::Value) -> Self {
            match json {
                serde_json::Value::Null => Value::Null,
                serde_json::Value::Bool(b) => Value::Bool(b),
                serde_json::Value::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        Value::Int(i)
                    } else if let Some(f) = n.as_f64() {
                        Value::Float(f)
                    } else {
                        Value::Null
                    }
                }
                serde_json::Value::String(s) => Value::text(s),
                serde_json::Value::Array(arr) => Value::array(arr.into_iter().map(Value::from)),
                serde_json::Value::Object(obj) => {
                    Value::object(obj.into_iter().map(|(k, v)| (k, Value::from(v))))
                }
            }
        }
    }

    impl FromStr for Value {
        type Err = serde_json::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let json: serde_json::Value = serde_json::from_str(s)?;
            Ok(Value::from(json))
        }
    }

    impl fmt::Display for Value {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let json: serde_json::Value = self.clone().into();
            if f.alternate() {
                write!(f, "{}", serde_json::to_string_pretty(&json).unwrap())
            } else {
                write!(f, "{}", serde_json::to_string(&json).unwrap())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Key, Value};

    // Value creation tests
    #[test]
    fn test_value_null() {
        let value = Value::Null;
        assert!(value.is_null());
        assert!(!value.is_bool());
    }

    #[test]
    fn test_value_bool() {
        let value = Value::Bool(true);
        assert!(value.is_bool());
        assert_eq!(value.as_bool(), Some(true));

        let value = Value::Bool(false);
        assert_eq!(value.as_bool(), Some(false));
    }

    #[test]
    fn test_value_int() {
        let value = Value::Int(42);
        assert!(value.is_int());
        assert_eq!(value.as_int(), Some(42));
    }

    #[test]
    fn test_value_float() {
        let value = Value::Float(3.14);
        assert!(value.is_float());
        assert_eq!(value.as_float(), Some(3.14));
    }

    #[test]
    fn test_value_text() {
        let value = Value::text("hello");
        assert!(value.is_text());
        assert_eq!(value.as_text(), Some("hello"));
    }

    #[test]
    fn test_value_array() {
        let value = Value::array([Value::Int(1), Value::Int(2), Value::Int(3)]);
        assert!(value.is_array());
        assert_eq!(value.as_array().map(|a| a.len()), Some(3));
    }

    #[test]
    fn test_value_object() {
        let value = Value::object([("key", Value::text("value"))]);
        assert!(value.is_object());
        assert!(value.as_object().is_some());
    }

    #[test]
    fn test_value_binary() {
        let value = Value::binary([0x00, 0x01, 0x02]);
        assert!(value.is_binary());
        assert_eq!(value.as_binary(), Some([0x00, 0x01, 0x02].as_slice()));
    }

    // Type checking tests
    #[test]
    fn test_value_is_null() {
        assert!(Value::Null.is_null());
        assert!(!Value::Bool(true).is_null());
    }

    #[test]
    fn test_value_is_bool() {
        assert!(Value::Bool(true).is_bool());
        assert!(!Value::Null.is_bool());
    }

    #[test]
    fn test_value_is_numeric() {
        assert!(Value::Int(1).is_numeric());
        assert!(Value::Float(1.0).is_numeric());
        assert!(!Value::text("1").is_numeric());
    }

    // Accessor tests
    #[test]
    fn test_value_as_bool() {
        assert_eq!(Value::Bool(true).as_bool(), Some(true));
        assert_eq!(Value::Int(1).as_bool(), None);
    }

    #[test]
    fn test_value_as_f64() {
        assert_eq!(Value::Float(3.14).as_f64(), Some(3.14));
        assert_eq!(Value::Int(42).as_f64(), Some(42.0));
        assert_eq!(Value::text("hello").as_f64(), None);
    }

    #[test]
    fn test_value_as_i64() {
        assert_eq!(Value::Int(42).as_i64(), Some(42));
        assert_eq!(Value::Float(3.9).as_i64(), Some(3));
        assert_eq!(Value::text("hello").as_i64(), None);
    }

    // Utility tests
    #[test]
    fn test_value_is_empty() {
        assert!(Value::Null.is_empty());
        assert!(Value::text("").is_empty());
        assert!(Value::array([]).is_empty());
        assert!(Value::object(std::iter::empty::<(Key, Value)>()).is_empty());
        assert!(Value::binary([]).is_empty());

        assert!(!Value::text("hello").is_empty());
        assert!(!Value::Bool(false).is_empty());
    }

    #[test]
    fn test_value_type_name() {
        assert_eq!(Value::Null.type_name(), "null");
        assert_eq!(Value::Bool(true).type_name(), "bool");
        assert_eq!(Value::Int(1).type_name(), "int");
        assert_eq!(Value::Float(1.0).type_name(), "float");
        assert_eq!(Value::text("").type_name(), "text");
        assert_eq!(Value::array([]).type_name(), "array");
        assert_eq!(
            Value::object(std::iter::empty::<(Key, Value)>()).type_name(),
            "object"
        );
        assert_eq!(Value::binary([]).type_name(), "binary");
    }

    #[test]
    fn test_value_default() {
        assert_eq!(Value::default(), Value::Null);
    }

    // From implementations
    #[test]
    fn test_value_from_bool() {
        let value: Value = true.into();
        assert_eq!(value.as_bool(), Some(true));
    }

    #[test]
    fn test_value_from_i64() {
        let value: Value = 42i64.into();
        assert_eq!(value.as_int(), Some(42));
    }

    #[test]
    fn test_value_from_i32() {
        let value: Value = 42i32.into();
        assert_eq!(value.as_int(), Some(42));
    }

    #[test]
    fn test_value_from_f64() {
        let value: Value = 3.14f64.into();
        assert_eq!(value.as_float(), Some(3.14));
    }

    #[test]
    fn test_value_from_str() {
        let value: Value = "hello".into();
        assert_eq!(value.as_text(), Some("hello"));
    }

    #[test]
    fn test_value_from_string() {
        let value: Value = String::from("hello").into();
        assert_eq!(value.as_text(), Some("hello"));
    }

    #[test]
    fn test_value_from_vec() {
        let value: Value = vec![1i64, 2, 3].into();
        assert!(value.is_array());
        assert_eq!(value.as_array().map(|a| a.len()), Some(3));
    }

    #[test]
    fn test_value_from_option() {
        let some: Value = Some(42i64).into();
        assert_eq!(some.as_int(), Some(42));

        let none: Value = Option::<i64>::None.into();
        assert!(none.is_null());
    }

    #[test]
    fn test_value_clone() {
        let original = Value::object([("key", Value::array([Value::Int(1), Value::Int(2)]))]);
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use super::*;

    #[test]
    fn test_value_to_json() {
        let value = Value::object([("name", Value::text("Alice")), ("age", Value::Int(30))]);

        let json: serde_json::Value = value.into();
        assert!(json.is_object());
        assert_eq!(json["name"], "Alice");
        assert_eq!(json["age"], 30);
    }

    #[test]
    fn test_json_to_value() {
        let json = serde_json::json!({
            "name": "Bob",
            "active": true,
            "score": 95.5
        });

        let value: Value = json.into();
        assert!(value.is_object());

        let obj = value.as_object().unwrap();
        assert_eq!(obj.get("name").and_then(|v| v.as_text()), Some("Bob"));
        assert_eq!(obj.get("active").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(obj.get("score").and_then(|v| v.as_float()), Some(95.5));
    }

    #[test]
    fn test_value_from_str() {
        let value: Value = r#"{"key": "value"}"#.parse().unwrap();
        assert!(value.is_object());
    }

    #[test]
    fn test_value_display() {
        let value = Value::object([("a", Value::Int(1))]);
        let display = format!("{}", value);
        assert!(display.contains("\"a\""));
        assert!(display.contains("1"));
    }

    #[test]
    fn test_value_display_pretty() {
        let value = Value::object([("a", Value::Int(1))]);
        let display = format!("{:#}", value);
        assert!(display.contains('\n')); // Pretty print has newlines
    }

    #[test]
    fn test_value_serialize_deserialize() {
        let original = Value::array([Value::Int(1), Value::text("two"), Value::Bool(true)]);

        let json_str = serde_json::to_string(&original).unwrap();
        let restored: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(original, restored);
    }
}
