//! Validation context for cross-field validation.

use std::sync::Arc;

use crate::core::{Key, Value};
use crate::schema::Schema;

/// Context provided to validators during validation.
///
/// Enables cross-field validation by providing access to:
/// - The key of the parameter being validated
/// - The schema for metadata lookup
/// - A value accessor for sibling values
///
/// # Example
///
/// ```ignore
/// use paramdef::validation::{ValidationContext, Validator, ValidationResult};
///
/// struct PasswordMatch;
///
/// impl Validator for PasswordMatch {
///     fn validate(&self, value: &Value, ctx: &ValidationContext<'_>) -> ValidationResult {
///         let password = value.as_text().unwrap_or("");
///         let confirm = ctx.get("password_confirm")
///             .and_then(|v| v.as_text())
///             .unwrap_or("");
///
///         if password != confirm {
///             return Err(Error::custom("password_mismatch", "Passwords do not match").into());
///         }
///         Ok(())
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ValidationContext<'a> {
    /// Key of the parameter being validated.
    key: &'a Key,
    /// Schema reference for metadata access.
    schema: &'a Arc<Schema>,
    /// Value accessor for cross-field validation.
    values: &'a dyn ValueAccess,
}

impl<'a> ValidationContext<'a> {
    /// Creates a new validation context.
    #[must_use]
    pub fn new(key: &'a Key, schema: &'a Arc<Schema>, values: &'a dyn ValueAccess) -> Self {
        Self { key, schema, values }
    }

    /// Returns the key of the parameter being validated.
    #[must_use]
    pub fn key(&self) -> &Key {
        self.key
    }

    /// Returns the schema reference.
    #[must_use]
    pub fn schema(&self) -> &Arc<Schema> {
        self.schema
    }

    /// Gets a sibling value by key.
    ///
    /// Use this for cross-field validation (e.g., password confirmation).
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.values.get_value(key)
    }

    /// Returns `true` if a sibling value exists.
    #[must_use]
    pub fn has(&self, key: &str) -> bool {
        self.values.get_value(key).is_some()
    }

    /// Gets the parameter metadata for a key.
    #[must_use]
    pub fn get_metadata(&self, key: &str) -> Option<&crate::core::Metadata> {
        self.schema.get(key).map(|n| n.metadata())
    }
}

/// Trait for accessing values during validation.
///
/// This trait abstracts over different value sources (`Context`, `HashMap`, etc.)
/// to enable validation in various scenarios.
pub trait ValueAccess: std::fmt::Debug {
    /// Gets a value by key.
    fn get_value(&self, key: &str) -> Option<&Value>;
}

// Implement for Context (main use case)
impl ValueAccess for crate::context::Context {
    fn get_value(&self, key: &str) -> Option<&Value> {
        self.get(key)
    }
}

// Implement for HashMap (useful for testing)
impl<S: std::hash::BuildHasher> ValueAccess for std::collections::HashMap<Key, Value, S> {
    fn get_value(&self, key: &str) -> Option<&Value> {
        self.get(key)
    }
}

// Implement for IndexMap
impl ValueAccess for crate::core::IndexMap<Key, Value> {
    fn get_value(&self, key: &str) -> Option<&Value> {
        self.get(key)
    }
}

/// Empty value access for standalone validation.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoValues;

impl ValueAccess for NoValues {
    fn get_value(&self, _key: &str) -> Option<&Value> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::Schema;
    use crate::types::leaf::Text;
    use std::collections::HashMap;

    fn create_test_schema() -> Arc<Schema> {
        Arc::new(
            Schema::builder()
                .parameter(Text::builder("password").build())
                .parameter(Text::builder("password_confirm").build())
                .build(),
        )
    }

    #[test]
    fn test_validation_context_key() {
        let schema = create_test_schema();
        let values = NoValues;
        let key: Key = "password".into();

        let ctx = ValidationContext::new(&key, &schema, &values);

        assert_eq!(ctx.key().as_str(), "password");
    }

    #[test]
    fn test_validation_context_get_sibling() {
        let schema = create_test_schema();
        let mut values: HashMap<Key, Value> = HashMap::new();
        values.insert("password_confirm".into(), Value::text("secret123"));

        let key: Key = "password".into();
        let ctx = ValidationContext::new(&key, &schema, &values);

        assert_eq!(
            ctx.get("password_confirm").and_then(|v| v.as_text()),
            Some("secret123")
        );
    }

    #[test]
    fn test_validation_context_has() {
        let schema = create_test_schema();
        let mut values: HashMap<Key, Value> = HashMap::new();
        values.insert("password_confirm".into(), Value::text("secret123"));

        let key: Key = "password".into();
        let ctx = ValidationContext::new(&key, &schema, &values);

        assert!(ctx.has("password_confirm"));
        assert!(!ctx.has("unknown"));
    }

    #[test]
    fn test_no_values() {
        let values = NoValues;
        assert!(values.get_value("any").is_none());
    }
}
