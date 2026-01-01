//! Utility operations for Value.

use super::Value;

impl Value {
    /// Returns `true` if this value is considered empty.
    ///
    /// - `Null` is empty
    /// - Empty string is empty
    /// - Empty array is empty
    /// - Empty object is empty
    /// - Empty binary is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::core::Value;
    ///
    /// assert!(Value::Null.is_empty());
    /// assert!(Value::text("").is_empty());
    /// assert!(Value::array([]).is_empty());
    /// assert!(!Value::text("hello").is_empty());
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::core::Value;
    ///
    /// assert_eq!(Value::Null.type_name(), "null");
    /// assert_eq!(Value::Bool(true).type_name(), "bool");
    /// assert_eq!(Value::Int(42).type_name(), "int");
    /// assert_eq!(Value::text("hello").type_name(), "text");
    /// ```
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Key;

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
}
