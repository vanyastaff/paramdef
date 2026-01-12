//! Typed getter methods for Context.
//!
//! This module provides type-safe value accessors that reduce boilerplate
//! from 3 lines to 1 while providing clear error messages.
//!
//! # Examples
//!
//! ```
//! use paramdef::context::Context;
//! use paramdef::schema::Schema;
//! use paramdef::types::leaf::Text;
//! use paramdef::core::Value;
//! use std::sync::Arc;
//! # fn main() -> Result<(), paramdef::core::Error> {
//!
//! let schema = Arc::new(Schema::builder()
//!     .parameter(Text::builder("email").build())
//!     .build());
//!
//! let mut ctx = Context::new(schema);
//! ctx.set("email", Value::text("user@example.com"))?;
//!
//! // Old way (3 lines)
//! // let email = ctx.get("email")
//! //     .and_then(|v| v.as_text())
//! //     .ok_or_else(|| Error::not_found("email"))?;
//!
//! // New way (1 line)
//! let email = ctx.get_text("email")?;
//! assert_eq!(email, "user@example.com");
//! # Ok(())
//! # }
//! ```

use crate::context::Context;
use crate::core::{Error, IndexMap, Key, Result, Value, ValueKind};

impl Context {
    // === Primitive typed getters ===

    /// Gets a text value by key.
    ///
    /// # Errors
    ///
    /// Returns:
    /// - `Error::NotFound` if the key doesn't exist in the schema
    /// - `Error::NullValue` if the value is `Value::Null`
    /// - `Error::TypeMismatch` if the value is not a `Text` variant
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::context::Context;
    /// use paramdef::schema::Schema;
    /// use paramdef::types::leaf::Text;
    /// use paramdef::core::Value;
    /// use std::sync::Arc;
    ///
    /// let schema = Arc::new(Schema::builder()
    ///     .parameter(Text::builder("name").build())
    ///     .build());
    ///
    /// let mut ctx = Context::new(schema);
    /// ctx.set("name", Value::text("Alice")).unwrap();
    ///
    /// assert_eq!(ctx.get_text("name").unwrap(), "Alice");
    /// ```
    pub fn get_text(&self, key: &str) -> Result<&str> {
        let node = self.node(key).ok_or_else(|| Error::not_found(key))?;

        match node.value() {
            None => Err(Error::null_value(key)),
            Some(v) => v
                .as_text()
                .ok_or_else(|| Error::type_mismatch(key, ValueKind::Text, v.kind())),
        }
    }

    /// Gets an integer value by key.
    ///
    /// # Errors
    ///
    /// Returns:
    /// - `Error::NotFound` if the key doesn't exist in the schema
    /// - `Error::NullValue` if the value is `Value::Null`
    /// - `Error::TypeMismatch` if the value is not an `Int` variant
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::context::Context;
    /// use paramdef::schema::Schema;
    /// use paramdef::types::leaf::Number;
    /// use paramdef::core::Value;
    /// use std::sync::Arc;
    ///
    /// let schema = Arc::new(Schema::builder()
    ///     .parameter(Number::builder("age").build())
    ///     .build());
    ///
    /// let mut ctx = Context::new(schema);
    /// ctx.set("age", Value::Int(30)).unwrap();
    ///
    /// assert_eq!(ctx.get_int("age").unwrap(), 30);
    /// ```
    pub fn get_int(&self, key: &str) -> Result<i64> {
        let node = self.node(key).ok_or_else(|| Error::not_found(key))?;

        match node.value() {
            None => Err(Error::null_value(key)),
            Some(v) => v
                .as_int()
                .ok_or_else(|| Error::type_mismatch(key, ValueKind::Int, v.kind())),
        }
    }

    /// Gets a float value by key.
    ///
    /// # Errors
    ///
    /// Returns:
    /// - `Error::NotFound` if the key doesn't exist in the schema
    /// - `Error::NullValue` if the value is `Value::Null`
    /// - `Error::TypeMismatch` if the value is not a `Float` variant
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::context::Context;
    /// use paramdef::schema::Schema;
    /// use paramdef::types::leaf::Number;
    /// use paramdef::core::Value;
    /// use std::sync::Arc;
    ///
    /// let schema = Arc::new(Schema::builder()
    ///     .parameter(Number::builder("price").build())
    ///     .build());
    ///
    /// let mut ctx = Context::new(schema);
    /// ctx.set("price", Value::Float(99.99)).unwrap();
    ///
    /// assert_eq!(ctx.get_float("price").unwrap(), 99.99);
    /// ```
    pub fn get_float(&self, key: &str) -> Result<f64> {
        let node = self.node(key).ok_or_else(|| Error::not_found(key))?;

        match node.value() {
            None => Err(Error::null_value(key)),
            Some(v) => v
                .as_float()
                .ok_or_else(|| Error::type_mismatch(key, ValueKind::Float, v.kind())),
        }
    }

    /// Gets a boolean value by key.
    ///
    /// # Errors
    ///
    /// Returns:
    /// - `Error::NotFound` if the key doesn't exist in the schema
    /// - `Error::NullValue` if the value is `Value::Null`
    /// - `Error::TypeMismatch` if the value is not a `Bool` variant
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::context::Context;
    /// use paramdef::schema::Schema;
    /// use paramdef::types::leaf::Boolean;
    /// use paramdef::core::Value;
    /// use std::sync::Arc;
    ///
    /// let schema = Arc::new(Schema::builder()
    ///     .parameter(Boolean::builder("active").build())
    ///     .build());
    ///
    /// let mut ctx = Context::new(schema);
    /// ctx.set("active", Value::Bool(true)).unwrap();
    ///
    /// assert_eq!(ctx.get_bool("active").unwrap(), true);
    /// ```
    pub fn get_bool(&self, key: &str) -> Result<bool> {
        let node = self.node(key).ok_or_else(|| Error::not_found(key))?;

        match node.value() {
            None => Err(Error::null_value(key)),
            Some(v) => v
                .as_bool()
                .ok_or_else(|| Error::type_mismatch(key, ValueKind::Bool, v.kind())),
        }
    }

    // === Collection typed getters ===

    /// Gets an array value by key.
    ///
    /// # Errors
    ///
    /// Returns:
    /// - `Error::NotFound` if the key doesn't exist in the schema
    /// - `Error::NullValue` if the value is `Value::Null`
    /// - `Error::TypeMismatch` if the value is not an `Array` variant
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::context::Context;
    /// use paramdef::schema::Schema;
    /// use paramdef::types::leaf::Text;
    /// use paramdef::core::Value;
    /// use std::sync::Arc;
    ///
    /// let schema = Arc::new(Schema::builder()
    ///     .parameter(Text::builder("tags").build())
    ///     .build());
    ///
    /// let mut ctx = Context::new(schema);
    /// ctx.set("tags", Value::array([Value::text("rust"), Value::text("paramdef")])).unwrap();
    ///
    /// assert_eq!(ctx.get_array("tags").unwrap().len(), 2);
    /// ```
    pub fn get_array(&self, key: &str) -> Result<&[Value]> {
        let node = self.node(key).ok_or_else(|| Error::not_found(key))?;

        match node.value() {
            None => Err(Error::null_value(key)),
            Some(v) => v
                .as_array()
                .ok_or_else(|| Error::type_mismatch(key, ValueKind::Array, v.kind())),
        }
    }

    /// Gets an object value by key.
    ///
    /// # Errors
    ///
    /// Returns:
    /// - `Error::NotFound` if the key doesn't exist in the schema
    /// - `Error::NullValue` if the value is `Value::Null`
    /// - `Error::TypeMismatch` if the value is not an `Object` variant
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::context::Context;
    /// use paramdef::schema::Schema;
    /// use paramdef::types::container::Object;
    /// use paramdef::core::Value;
    /// use std::sync::Arc;
    ///
    /// let schema = Arc::new(Schema::builder()
    ///     .parameter(Object::builder("config").build().expect("valid object"))
    ///     .build());
    ///
    /// let mut ctx = Context::new(schema);
    /// ctx.set("config", Value::object([("key", Value::text("value"))])).unwrap();
    ///
    /// assert_eq!(ctx.get_object("config").unwrap().len(), 1);
    /// ```
    pub fn get_object(&self, key: &str) -> Result<&IndexMap<Key, Value>> {
        let node = self.node(key).ok_or_else(|| Error::not_found(key))?;

        match node.value() {
            None => Err(Error::null_value(key)),
            Some(v) => v
                .as_object()
                .ok_or_else(|| Error::type_mismatch(key, ValueKind::Object, v.kind())),
        }
    }

    /// Gets a binary value by key.
    ///
    /// # Errors
    ///
    /// Returns:
    /// - `Error::NotFound` if the key doesn't exist in the schema
    /// - `Error::NullValue` if the value is `Value::Null`
    /// - `Error::TypeMismatch` if the value is not a `Binary` variant
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::context::Context;
    /// use paramdef::schema::Schema;
    /// use paramdef::types::leaf::File;
    /// use paramdef::core::Value;
    /// use std::sync::Arc;
    ///
    /// let schema = Arc::new(Schema::builder()
    ///     .parameter(File::builder("data").build())
    ///     .build());
    ///
    /// let mut ctx = Context::new(schema);
    /// ctx.set("data", Value::binary([0x00, 0x01, 0x02])).unwrap();
    ///
    /// assert_eq!(ctx.get_binary("data").unwrap(), &[0x00, 0x01, 0x02]);
    /// ```
    pub fn get_binary(&self, key: &str) -> Result<&[u8]> {
        let node = self.node(key).ok_or_else(|| Error::not_found(key))?;

        match node.value() {
            None => Err(Error::null_value(key)),
            Some(v) => v
                .as_binary()
                .ok_or_else(|| Error::type_mismatch(key, ValueKind::Binary, v.kind())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::Schema;
    use crate::types::leaf::{Boolean, Number, Text};
    use std::sync::Arc;

    fn create_test_schema() -> Arc<Schema> {
        Arc::new(
            Schema::builder()
                .parameter(Text::builder("name").build())
                .parameter(Text::builder("email").build())
                .parameter(Number::builder("age").build())
                .parameter(Number::builder("price").build())
                .parameter(Boolean::builder("active").build())
                .build(),
        )
    }

    // === Primitive getters tests ===

    #[test]
    fn test_get_text_success() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);
        ctx.set("name", Value::text("Alice")).unwrap();

        assert_eq!(ctx.get_text("name").unwrap(), "Alice");
    }

    #[test]
    fn test_get_text_not_found() {
        let schema = create_test_schema();
        let ctx = Context::new(schema);

        let result = ctx.get_text("unknown");
        assert!(matches!(result, Err(Error::NotFound { .. })));
    }

    #[test]
    fn test_get_text_null_value() {
        let schema = create_test_schema();
        let ctx = Context::new(schema);

        let result = ctx.get_text("name");
        assert!(matches!(result, Err(Error::NullValue { .. })));
    }

    #[test]
    fn test_get_text_type_mismatch() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);
        ctx.set("name", Value::Int(42)).unwrap();

        let result = ctx.get_text("name");
        assert!(matches!(result, Err(Error::TypeMismatch { .. })));
    }

    #[test]
    fn test_get_int_success() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);
        ctx.set("age", Value::Int(30)).unwrap();

        assert_eq!(ctx.get_int("age").unwrap(), 30);
    }

    #[test]
    fn test_get_int_null_value() {
        let schema = create_test_schema();
        let ctx = Context::new(schema);

        let result = ctx.get_int("age");
        assert!(matches!(result, Err(Error::NullValue { .. })));
    }

    #[test]
    fn test_get_int_type_mismatch() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);
        ctx.set("age", Value::text("thirty")).unwrap();

        let result = ctx.get_int("age");
        assert!(matches!(result, Err(Error::TypeMismatch { .. })));
    }

    #[test]
    fn test_get_float_success() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);
        ctx.set("price", Value::Float(99.99)).unwrap();

        assert_eq!(ctx.get_float("price").unwrap(), 99.99);
    }

    #[test]
    fn test_get_float_null_value() {
        let schema = create_test_schema();
        let ctx = Context::new(schema);

        let result = ctx.get_float("price");
        assert!(matches!(result, Err(Error::NullValue { .. })));
    }

    #[test]
    fn test_get_float_type_mismatch() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);
        ctx.set("price", Value::Bool(true)).unwrap();

        let result = ctx.get_float("price");
        assert!(matches!(result, Err(Error::TypeMismatch { .. })));
    }

    #[test]
    fn test_get_bool_success() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);
        ctx.set("active", Value::Bool(true)).unwrap();

        assert_eq!(ctx.get_bool("active").unwrap(), true);
    }

    #[test]
    fn test_get_bool_null_value() {
        let schema = create_test_schema();
        let ctx = Context::new(schema);

        let result = ctx.get_bool("active");
        assert!(matches!(result, Err(Error::NullValue { .. })));
    }

    #[test]
    fn test_get_bool_type_mismatch() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);
        ctx.set("active", Value::Int(1)).unwrap();

        let result = ctx.get_bool("active");
        assert!(matches!(result, Err(Error::TypeMismatch { .. })));
    }
}
