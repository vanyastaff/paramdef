//! Unified runtime value representation.
//!
//! The [`Value`] enum is the runtime representation for all parameter values.
//! It provides a type-safe way to store and manipulate parameter data.
//!
//! # Organization
//!
//! - [`Value`] - Main enum definition and constructors
//! - [`convert`] - Type conversion methods (as_*, From impls)
//! - [`ops`] - Utility operations
//! - [`serde`] - Serialization support (feature-gated)

mod convert;
mod ops;

#[cfg(feature = "serde")]
mod serde_support;

use std::sync::Arc;

use super::{IndexMap, Key, SmartStr};

// Re-export conversion traits (used by inherent methods and external users)
#[allow(unused_imports)]
pub use convert::*;
#[allow(unused_imports)]
pub use ops::*;

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
    Text(SmartStr),

    /// Ordered array of values.
    Array(Arc<[Value]>),

    /// Key-value object with insertion-order preservation.
    ///
    /// Uses [`IndexMap`] to maintain the order in which fields were inserted,
    /// providing consistent serialization and iteration order.
    Object(Arc<IndexMap<Key, Value>>),

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
    pub fn text(s: impl Into<SmartStr>) -> Self {
        Self::Text(s.into())
    }

    /// Creates an array value from an iterator.
    ///
    /// Uses the iterator's `size_hint()` to pre-allocate the Vec,
    /// avoiding reallocation during construction.
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
        let iter = values.into_iter();
        let (lower_bound, _) = iter.size_hint();

        let mut vec = Vec::with_capacity(lower_bound);
        vec.extend(iter);

        Self::Array(Arc::from(vec.into_boxed_slice()))
    }

    /// Creates an array value with pre-allocated capacity.
    ///
    /// Use this when you know the number of elements in advance
    /// to avoid reallocation. For dynamic sizes, use [`Value::array`] which
    /// uses the iterator's `size_hint()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::core::Value;
    ///
    /// // Pre-allocate for 1000 elements
    /// let mut values = Vec::with_capacity(1000);
    /// for i in 0..1000 {
    ///     values.push(Value::Int(i));
    /// }
    /// let value = Value::array_with_capacity(1000, values);
    /// ```
    pub fn array_with_capacity(capacity: usize, values: impl IntoIterator<Item = Value>) -> Self {
        let mut vec = Vec::with_capacity(capacity);
        vec.extend(values);

        Self::Array(Arc::from(vec.into_boxed_slice()))
    }

    /// Creates an object value from key-value pairs.
    ///
    /// Uses the iterator's `size_hint()` to pre-allocate the `IndexMap`,
    /// avoiding rehashing during construction. Field order is preserved.
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
        let iter = pairs.into_iter();
        let (lower_bound, _) = iter.size_hint();

        let mut map = IndexMap::with_capacity(lower_bound);
        map.extend(iter.map(|(k, v)| (k.into(), v)));

        Self::Object(Arc::new(map))
    }

    /// Creates an object value with pre-allocated capacity.
    ///
    /// Use this when you know the number of key-value pairs in advance
    /// to avoid rehashing. For dynamic sizes, use [`Value::object`] which
    /// uses the iterator's `size_hint()`. Field order is preserved.
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::core::Value;
    ///
    /// // Pre-allocate for 100 entries
    /// let mut pairs = Vec::with_capacity(100);
    /// for i in 0..100 {
    ///     pairs.push((format!("key_{i}"), Value::Int(i)));
    /// }
    /// let value = Value::object_with_capacity(100, pairs);
    /// ```
    pub fn object_with_capacity(
        capacity: usize,
        pairs: impl IntoIterator<Item = (impl Into<Key>, Value)>,
    ) -> Self {
        let mut map = IndexMap::with_capacity(capacity);
        map.extend(pairs.into_iter().map(|(k, v)| (k.into(), v)));

        Self::Object(Arc::new(map))
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
}

#[cfg(test)]
mod tests {
    use super::Value;

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

    #[test]
    fn test_value_default() {
        assert_eq!(Value::default(), Value::Null);
    }

    #[test]
    fn test_value_clone() {
        let original = Value::object([("key", Value::array([Value::Int(1), Value::Int(2)]))]);
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    // === Capacity optimization tests ===

    #[test]
    fn test_array_with_capacity() {
        // Create array with explicit capacity
        let values: Vec<Value> = (0..100).map(Value::Int).collect();
        let array = Value::array_with_capacity(100, values);

        assert!(array.is_array());
        assert_eq!(array.as_array().map(|a| a.len()), Some(100));

        // Verify values are correct
        let arr = array.as_array().unwrap();
        assert_eq!(arr[0].as_int(), Some(0));
        assert_eq!(arr[99].as_int(), Some(99));
    }

    #[test]
    fn test_array_with_capacity_small() {
        // Test with small array
        let array = Value::array_with_capacity(3, [Value::Int(1), Value::Int(2), Value::Int(3)]);

        assert_eq!(array.as_array().map(|a| a.len()), Some(3));
    }

    #[test]
    fn test_object_with_capacity() {
        // Create object with explicit capacity
        let pairs: Vec<(String, Value)> = (0..50)
            .map(|i| (format!("key_{i}"), Value::Int(i)))
            .collect();

        let object = Value::object_with_capacity(50, pairs);

        assert!(object.is_object());
        let obj = object.as_object().unwrap();
        assert_eq!(obj.len(), 50);

        // Verify values are correct
        assert_eq!(obj.get("key_0").and_then(|v| v.as_int()), Some(0));
        assert_eq!(obj.get("key_49").and_then(|v| v.as_int()), Some(49));
    }

    #[test]
    fn test_object_with_capacity_small() {
        // Test with small object
        let object = Value::object_with_capacity(
            2,
            [("name", Value::text("Alice")), ("age", Value::Int(30))],
        );

        let obj = object.as_object().unwrap();
        assert_eq!(obj.len(), 2);
        assert_eq!(obj.get("name").and_then(|v| v.as_text()), Some("Alice"));
    }

    #[test]
    fn test_array_size_hint_optimization() {
        // Verify that array() uses size_hint correctly
        // Vec has exact size_hint, so this should pre-allocate correctly
        let values: Vec<Value> = (0..1000).map(Value::Int).collect();
        let array = Value::array(values);

        assert_eq!(array.as_array().map(|a| a.len()), Some(1000));
    }

    #[test]
    fn test_object_size_hint_optimization() {
        // Verify that object() uses size_hint correctly
        // Vec has exact size_hint, so this should pre-allocate correctly
        let pairs: Vec<(String, Value)> = (0..100)
            .map(|i| (format!("key_{i}"), Value::Int(i)))
            .collect();

        let object = Value::object(pairs);

        assert_eq!(object.as_object().map(|o| o.len()), Some(100));
    }
}
