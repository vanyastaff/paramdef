//! Schema module for immutable parameter definitions.
//!
//! Schema holds the structure of parameters shared via `Arc`.
//! Multiple [`Context`](crate::context::Context) instances can share the same schema.

use std::sync::Arc;

use crate::core::{IndexMap, Key};
use crate::types::traits::Node;

/// Immutable parameter definitions shared across contexts.
///
/// Schema defines the structure of parameters and is designed to be shared
/// via `Arc`. Create contexts from a schema to work with runtime values.
///
/// # Example
///
/// ```
/// use paramdef::schema::Schema;
/// use paramdef::types::leaf::Text;
///
/// let schema = Schema::builder()
///     .parameter(Text::builder("username").required().build())
///     .parameter(Text::builder("email").build())
///     .build();
///
/// assert_eq!(schema.len(), 2);
/// assert!(schema.get("username").is_some());
/// ```
#[derive(Debug, Clone)]
pub struct Schema {
    /// Root parameters indexed by key, preserving insertion order.
    /// `IndexMap` provides O(1) lookup while maintaining order.
    parameters: IndexMap<Key, Arc<dyn Node>>,
}

impl Schema {
    /// Creates a new builder for constructing a schema.
    #[must_use]
    pub fn builder() -> SchemaBuilder {
        SchemaBuilder::new()
    }

    /// Returns the number of root parameters.
    #[must_use]
    pub fn len(&self) -> usize {
        self.parameters.len()
    }

    /// Returns `true` if the schema has no parameters.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.parameters.is_empty()
    }

    /// Returns a parameter by key.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Arc<dyn Node>> {
        self.parameters.get(key)
    }

    /// Returns an iterator over all parameters in insertion order.
    pub fn iter(&self) -> impl Iterator<Item = &Arc<dyn Node>> {
        self.parameters.values()
    }

    /// Returns an iterator over parameter keys in insertion order.
    pub fn keys(&self) -> impl Iterator<Item = &Key> {
        self.parameters.keys()
    }
}

/// Builder for constructing a [`Schema`].
#[derive(Debug, Default)]
pub struct SchemaBuilder {
    parameters: IndexMap<Key, Arc<dyn Node>>,
}

impl SchemaBuilder {
    /// Creates a new schema builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a parameter to the schema.
    ///
    /// If a parameter with the same key already exists, it will be replaced.
    #[must_use]
    pub fn parameter(mut self, node: impl Node + 'static) -> Self {
        let key = node.key().clone();
        self.parameters.insert(key, Arc::new(node));
        self
    }

    /// Adds a parameter wrapped in Arc.
    #[must_use]
    pub fn parameter_arc(mut self, node: Arc<dyn Node>) -> Self {
        let key = node.key().clone();
        self.parameters.insert(key, node);
        self
    }

    /// Builds the schema.
    #[must_use]
    pub fn build(self) -> Schema {
        Schema {
            parameters: self.parameters,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::leaf::{Boolean, Number, Text};

    #[test]
    fn test_schema_builder() {
        let schema = Schema::builder()
            .parameter(Text::builder("username").build())
            .parameter(Number::builder("age").build())
            .build();

        assert_eq!(schema.len(), 2);
    }

    #[test]
    fn test_schema_get_parameter() {
        let schema = Schema::builder()
            .parameter(Text::builder("name").build())
            .build();

        assert!(schema.get("name").is_some());
        assert!(schema.get("unknown").is_none());
    }

    #[test]
    fn test_schema_iter() {
        let schema = Schema::builder()
            .parameter(Text::builder("a").build())
            .parameter(Text::builder("b").build())
            .parameter(Text::builder("c").build())
            .build();

        let keys: Vec<_> = schema.keys().map(|k| k.as_str()).collect();
        assert_eq!(keys, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_schema_empty() {
        let schema = Schema::builder().build();

        assert!(schema.is_empty());
        assert_eq!(schema.len(), 0);
    }

    #[test]
    fn test_schema_duplicate_key() {
        let schema = Schema::builder()
            .parameter(Text::builder("name").label("First").build())
            .parameter(Text::builder("name").label("Second").build())
            .build();

        // Should have only one parameter (replaced)
        assert_eq!(schema.len(), 1);
        // The label should be from the second one
        let param = schema.get("name").unwrap();
        assert_eq!(param.metadata().label(), Some("Second"));
    }

    #[test]
    fn test_schema_multiple_types() {
        let schema = Schema::builder()
            .parameter(Text::builder("name").build())
            .parameter(Number::builder("count").build())
            .parameter(Boolean::builder("enabled").build())
            .build();

        assert_eq!(schema.len(), 3);
    }
}
