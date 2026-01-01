//! Object container for named fields.
//!
//! Object represents a structured collection of named child parameters,
//! similar to a JSON object or a struct. Each field has a unique key
//! and can be any Container or Leaf node type.

use std::any::Any;
use std::fmt;
use std::sync::Arc;

use crate::core::{Flags, FxHashSet, Key, Metadata, SmartStr};
use crate::types::kind::NodeKind;
use crate::types::traits::{Container, Node};

/// A container with named fields.
///
/// Object is one of the six container types. It holds a collection of
/// named child nodes and produces a `Value::Object` when serialized.
///
/// # Example
///
/// ```ignore
/// use paramdef::container::Object;
/// use paramdef::types::leaf::Text;
///
/// let address = Object::builder("address")
///     .label("Address")
///     .field("street", Text::builder("street").required().build())
///     .field("city", Text::builder("city").required().build())
///     .field("zip", Text::builder("zip").build())
///     .build();
/// ```
#[derive(Clone)]
pub struct Object {
    metadata: Metadata,
    flags: Flags,
    fields: Vec<(Key, Arc<dyn Node>)>,
    /// Cached children for Container trait
    children_cache: Arc<[Arc<dyn Node>]>,
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Object")
            .field("metadata", &self.metadata)
            .field("flags", &self.flags)
            .field("field_count", &self.fields.len())
            .finish_non_exhaustive()
    }
}

impl Object {
    /// Creates a new builder for an Object.
    #[must_use]
    pub fn builder(key: impl Into<Key>) -> ObjectBuilder {
        ObjectBuilder::new(key)
    }

    /// Creates an empty Object with no fields.
    #[must_use]
    pub fn empty(key: impl Into<Key>) -> Self {
        Self {
            metadata: Metadata::new(key),
            flags: Flags::empty(),
            fields: Vec::new(),
            children_cache: Arc::from([]),
        }
    }

    /// Returns the flags for this object.
    #[inline]
    #[must_use]
    pub fn flags(&self) -> Flags {
        self.flags
    }

    /// Returns a slice of all fields.
    #[inline]
    #[must_use]
    pub fn fields(&self) -> &[(Key, Arc<dyn Node>)] {
        &self.fields
    }

    /// Returns the number of fields.
    #[inline]
    #[must_use]
    pub fn field_count(&self) -> usize {
        self.fields.len()
    }

    /// Gets a field by key.
    #[must_use]
    pub fn get_field(&self, key: &str) -> Option<&Arc<dyn Node>> {
        self.fields.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    /// Returns whether the object has a field with the given key.
    #[must_use]
    pub fn has_field(&self, key: &str) -> bool {
        self.fields.iter().any(|(k, _)| k == key)
    }

    /// Returns an iterator over field keys.
    pub fn field_keys(&self) -> impl Iterator<Item = &Key> {
        self.fields.iter().map(|(k, _)| k)
    }
}

impl Node for Object {
    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn key(&self) -> &Key {
        self.metadata.key()
    }

    fn kind(&self) -> NodeKind {
        NodeKind::Container
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Container for Object {
    fn children(&self) -> &[Arc<dyn Node>] {
        &self.children_cache
    }
}

// =============================================================================
// Builder
// =============================================================================

/// Builder for [`Object`].
#[derive(Clone)]
pub struct ObjectBuilder {
    key: Key,
    label: Option<SmartStr>,
    description: Option<SmartStr>,
    flags: Flags,
    fields: Vec<(Key, Arc<dyn Node>)>,
}

impl fmt::Debug for ObjectBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ObjectBuilder")
            .field("key", &self.key)
            .field("label", &self.label)
            .field("description", &self.description)
            .field("flags", &self.flags)
            .field("field_count", &self.fields.len())
            .finish()
    }
}

impl ObjectBuilder {
    /// Creates a new builder with the given key.
    #[must_use]
    pub fn new(key: impl Into<Key>) -> Self {
        Self {
            key: key.into(),
            label: None,
            description: None,
            flags: Flags::empty(),
            fields: Vec::new(),
        }
    }

    /// Sets the label for this object.
    #[must_use]
    pub fn label(mut self, label: impl Into<SmartStr>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the description for this object.
    #[must_use]
    pub fn description(mut self, description: impl Into<SmartStr>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the flags for this object.
    #[must_use]
    pub fn flags(mut self, flags: Flags) -> Self {
        self.flags = flags;
        self
    }

    /// Marks this object as required.
    #[must_use]
    pub fn required(mut self) -> Self {
        self.flags |= Flags::REQUIRED;
        self
    }

    /// Adds a field to the object.
    ///
    /// Duplicate keys are detected at build time and will return an error.
    #[must_use]
    pub fn field(mut self, key: impl Into<Key>, node: impl Node + 'static) -> Self {
        self.fields.push((key.into(), Arc::new(node)));
        self
    }

    /// Adds a field with an already-wrapped Arc.
    ///
    /// Duplicate keys are detected at build time and will return an error.
    #[must_use]
    pub fn field_arc(mut self, key: impl Into<Key>, node: Arc<dyn Node>) -> Self {
        self.fields.push((key.into(), node));
        self
    }

    /// Builds the Object.
    ///
    /// # Errors
    ///
    /// Returns an error if duplicate field keys exist.
    pub fn build(self) -> crate::core::Result<Object> {
        // Check for duplicate field keys
        let mut seen_keys = FxHashSet::default();
        for (key, _) in &self.fields {
            if !seen_keys.insert(key) {
                return Err(crate::core::Error::validation(
                    "duplicate_key",
                    format!("duplicate field key: {key}"),
                ));
            }
        }

        let mut metadata = Metadata::new(self.key);
        if let Some(label) = self.label {
            metadata = metadata.with_label(label);
        }
        if let Some(description) = self.description {
            metadata = metadata.with_description(description);
        }

        // Build children cache
        let children_cache: Arc<[Arc<dyn Node>]> = self
            .fields
            .iter()
            .map(|(_, node)| Arc::clone(node))
            .collect();

        Ok(Object {
            metadata,
            flags: self.flags,
            fields: self.fields,
            children_cache,
        })
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::leaf::Text;

    #[test]
    fn test_object_empty() {
        let obj = Object::empty("empty_obj");
        assert_eq!(obj.key().as_str(), "empty_obj");
        assert_eq!(obj.field_count(), 0);
        assert!(obj.fields().is_empty());
        assert!(obj.children().is_empty());
    }

    #[test]
    fn test_object_builder_basic() {
        let obj = Object::builder("address")
            .label("Address")
            .description("Mailing address")
            .build()
            .unwrap();

        assert_eq!(obj.key().as_str(), "address");
        assert_eq!(obj.metadata().label(), Some("Address"));
        assert_eq!(obj.metadata().description(), Some("Mailing address"));
    }

    #[test]
    fn test_object_with_fields() {
        let obj = Object::builder("person")
            .field("name", Text::builder("name").build())
            .field("email", Text::builder("email").build())
            .build()
            .unwrap();

        assert_eq!(obj.field_count(), 2);
        assert!(obj.has_field("name"));
        assert!(obj.has_field("email"));
        assert!(!obj.has_field("phone"));
    }

    #[test]
    fn test_object_get_field() {
        let obj = Object::builder("config")
            .field("host", Text::builder("host").build())
            .build()
            .unwrap();

        assert!(obj.get_field("host").is_some());
        assert!(obj.get_field("port").is_none());
    }

    #[test]
    fn test_object_field_keys() {
        let obj = Object::builder("config")
            .field("a", Text::builder("a").build())
            .field("b", Text::builder("b").build())
            .field("c", Text::builder("c").build())
            .build()
            .unwrap();

        let keys: Vec<&str> = obj.field_keys().map(|k| k.as_str()).collect();
        assert_eq!(keys, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_object_node_trait() {
        let obj = Object::builder("test").build().unwrap();

        assert_eq!(obj.kind(), NodeKind::Container);
        assert_eq!(obj.key().as_str(), "test");
    }

    #[test]
    fn test_object_flags() {
        let obj = Object::builder("required_obj").required().build().unwrap();

        assert!(obj.flags().contains(Flags::REQUIRED));
    }

    #[test]
    fn test_nested_object() {
        let inner = Object::builder("inner")
            .field("value", Text::builder("value").build())
            .build()
            .unwrap();

        let outer = Object::builder("outer")
            .field("nested", inner)
            .build()
            .unwrap();

        assert_eq!(outer.field_count(), 1);
        assert!(outer.has_field("nested"));
    }

    #[test]
    fn test_object_children() {
        let obj = Object::builder("config")
            .field("a", Text::builder("a").build())
            .field("b", Text::builder("b").build())
            .build()
            .unwrap();

        assert_eq!(obj.children().len(), 2);
    }

    #[test]
    fn test_object_duplicate_key_error() {
        let result = Object::builder("config")
            .field("host", Text::builder("host").build())
            .field("host", Text::builder("host2").build()) // duplicate key
            .build();

        assert!(result.is_err());
    }
}
