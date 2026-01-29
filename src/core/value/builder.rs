//! Builder API for constructing complex Value objects.
//!
//! This module provides a fluent builder API for creating nested object structures
//! without verbose array-of-tuples syntax.
//!
//! # Examples
//!
//! ```
//! use paramdef::core::Value;
//!
//! // Flat style
//! let config = Value::build_object()
//!     .text("host", "localhost")
//!     .int("port", 5432)
//!     .bool("ssl", true)
//!     .build();
//!
//! // Nested style with closures
//! let config = Value::build_object()
//!     .nested("database", |db| db
//!         .text("host", "localhost")
//!         .int("port", 5432)
//!     )
//!     .build();
//! ```

use crate::core::{IndexMap, Key, SmartStr, Value};

/// Builder for constructing object values with a fluent API.
///
/// Provides typed methods for common value types and supports nested object construction.
#[derive(Debug, Default)]
pub struct ObjectBuilder {
    map: IndexMap<Key, Value>,
}

impl ObjectBuilder {
    /// Creates a new empty object builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            map: IndexMap::new(),
        }
    }

    /// Creates a new object builder with pre-allocated capacity.
    ///
    /// Use this when you know the number of fields in advance to avoid rehashing.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: IndexMap::with_capacity(capacity),
        }
    }

    /// Adds a generic field with any Value type.
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::core::Value;
    ///
    /// let obj = Value::build_object()
    ///     .field("custom", Value::Int(42))
    ///     .build();
    /// ```
    #[must_use]
    pub fn field(mut self, key: impl Into<Key>, value: Value) -> Self {
        self.map.insert(key.into(), value);
        self
    }

    /// Adds a text field.
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::core::Value;
    ///
    /// let obj = Value::build_object()
    ///     .text("name", "Alice")
    ///     .build();
    /// ```
    #[must_use]
    pub fn text(self, key: impl Into<Key>, value: impl Into<SmartStr>) -> Self {
        self.field(key, Value::text(value))
    }

    /// Adds an integer field.
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::core::Value;
    ///
    /// let obj = Value::build_object()
    ///     .int("age", 30)
    ///     .build();
    /// ```
    #[must_use]
    pub fn int(self, key: impl Into<Key>, value: i64) -> Self {
        self.field(key, Value::Int(value))
    }

    /// Adds a float field.
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::core::Value;
    ///
    /// let obj = Value::build_object()
    ///     .float("price", 99.99)
    ///     .build();
    /// ```
    #[must_use]
    pub fn float(self, key: impl Into<Key>, value: f64) -> Self {
        self.field(key, Value::Float(value))
    }

    /// Adds a boolean field.
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::core::Value;
    ///
    /// let obj = Value::build_object()
    ///     .bool("active", true)
    ///     .build();
    /// ```
    #[must_use]
    pub fn bool(self, key: impl Into<Key>, value: bool) -> Self {
        self.field(key, Value::Bool(value))
    }

    /// Adds a nested object using a closure.
    ///
    /// The closure receives a new `ObjectBuilder` and must return it.
    /// This enables clean nested object construction.
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::core::Value;
    ///
    /// let config = Value::build_object()
    ///     .nested("database", |db| db
    ///         .text("host", "localhost")
    ///         .int("port", 5432)
    ///         .nested("credentials", |cred| cred
    ///             .text("username", "admin")
    ///             .text("password", "secret")
    ///         )
    ///     )
    ///     .build();
    /// ```
    #[must_use]
    pub fn nested<F>(mut self, key: impl Into<Key>, f: F) -> Self
    where
        F: FnOnce(ObjectBuilder) -> ObjectBuilder,
    {
        let nested = f(ObjectBuilder::new());
        self.map.insert(key.into(), nested.build());
        self
    }

    /// Adds multiple fields at once from an iterator.
    ///
    /// This is a convenience method for bulk field addition.
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::core::{Value, Key};
    /// use indexmap::IndexMap;
    ///
    /// let mut fields = IndexMap::<Key, Value>::new();
    /// fields.insert("x".into(), Value::Float(1.0));
    /// fields.insert("y".into(), Value::Float(2.0));
    ///
    /// let point = Value::build_object()
    ///     .fields(fields)
    ///     .build();
    /// ```
    #[must_use]
    pub fn fields<I, K>(mut self, fields: I) -> Self
    where
        I: IntoIterator<Item = (K, Value)>,
        K: Into<Key>,
    {
        for (key, value) in fields {
            self.map.insert(key.into(), value);
        }
        self
    }

    /// Conditionally adds a field if the condition is true.
    ///
    /// This is useful for building objects with optional fields based on runtime conditions.
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::core::{Value, Key};
    ///
    /// let include_age = true;
    /// let user = Value::build_object()
    ///     .text("name", "Charlie")
    ///     .field_if(include_age, "age", Value::Float(25.0))
    ///     .build();
    ///
    /// let age_key: Key = "age".into();
    /// assert!(user.as_object().unwrap().contains_key(&age_key));
    /// ```
    #[must_use]
    pub fn field_if(mut self, condition: bool, key: impl Into<Key>, value: Value) -> Self {
        if condition {
            self.map.insert(key.into(), value);
        }
        self
    }

    /// Builds the final `Value::Object`.
    ///
    /// Consumes the builder and returns the constructed object.
    #[must_use]
    pub fn build(self) -> Value {
        Value::Object(self.map.into())
    }
}

impl Value {
    /// Creates a new `ObjectBuilder` for fluent object construction.
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::core::Value;
    ///
    /// let obj = Value::build_object()
    ///     .text("name", "Alice")
    ///     .int("age", 30)
    ///     .build();
    /// ```
    #[must_use]
    pub fn build_object() -> ObjectBuilder {
        ObjectBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_builder_empty() {
        let obj = ObjectBuilder::new().build();
        assert!(obj.is_object());
        assert_eq!(obj.as_object().unwrap().len(), 0);
    }

    #[test]
    fn test_object_builder_flat() {
        let obj = Value::build_object()
            .text("host", "localhost")
            .int("port", 5432)
            .bool("ssl", true)
            .float("timeout", 30.0)
            .build();

        let map = obj.as_object().unwrap();
        assert_eq!(map.len(), 4);
        assert_eq!(map.get("host").and_then(|v| v.as_text()), Some("localhost"));
        assert_eq!(map.get("port").and_then(|v| v.as_int()), Some(5432));
        assert_eq!(map.get("ssl").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(map.get("timeout").and_then(|v| v.as_float()), Some(30.0));
    }

    #[test]
    fn test_object_builder_nested() {
        let obj = Value::build_object()
            .text("app", "myapp")
            .nested("database", |db| {
                db.text("host", "localhost")
                    .int("port", 5432)
                    .text("name", "mydb")
            })
            .build();

        let map = obj.as_object().unwrap();
        assert_eq!(map.len(), 2);

        let db = map.get("database").unwrap().as_object().unwrap();
        assert_eq!(db.get("host").and_then(|v| v.as_text()), Some("localhost"));
        assert_eq!(db.get("port").and_then(|v| v.as_int()), Some(5432));
    }

    #[test]
    fn test_object_builder_deeply_nested() {
        let obj = Value::build_object()
            .nested("level1", |l1| {
                l1.nested("level2", |l2| {
                    l2.nested("level3", |l3| l3.text("deep", "value"))
                })
            })
            .build();

        let l1 = obj.as_object().unwrap().get("level1").unwrap();
        let l2 = l1.as_object().unwrap().get("level2").unwrap();
        let l3 = l2.as_object().unwrap().get("level3").unwrap();
        assert_eq!(
            l3.as_object()
                .unwrap()
                .get("deep")
                .and_then(|v| v.as_text()),
            Some("value")
        );
    }

    #[test]
    fn test_object_builder_mixed_style() {
        let credentials = Value::build_object()
            .text("username", "admin")
            .text("password", "secret")
            .build();

        let config = Value::build_object()
            .text("host", "localhost")
            .field("credentials", credentials)
            .build();

        let map = config.as_object().unwrap();
        let creds = map.get("credentials").unwrap().as_object().unwrap();
        assert_eq!(
            creds.get("username").and_then(|v| v.as_text()),
            Some("admin")
        );
    }

    #[test]
    fn test_object_builder_with_capacity() {
        let obj = ObjectBuilder::with_capacity(10)
            .text("key1", "value1")
            .text("key2", "value2")
            .build();

        assert_eq!(obj.as_object().unwrap().len(), 2);
    }

    #[test]
    fn test_object_builder_field_generic() {
        let obj = ObjectBuilder::new()
            .field("array", Value::array([Value::Int(1), Value::Int(2)]))
            .field("null", Value::Null)
            .build();

        let map = obj.as_object().unwrap();
        assert_eq!(map.len(), 2);
        assert!(map.get("array").unwrap().is_array());
        assert!(map.get("null").unwrap().is_null());
    }
}
