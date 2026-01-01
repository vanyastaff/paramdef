//! Type conversion and accessor methods for Value.

use std::collections::HashMap;

use super::{Key, Value};

impl Value {
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
