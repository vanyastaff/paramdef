//! Object container for named fields.
//!
//! Object represents a structured collection of named child parameters,
//! similar to a JSON object or a struct. Each field has a unique key
//! and can be any Container or Leaf node type.
//!
//! # Extensible Objects
//!
//! Objects can be made extensible to allow additional key-value pairs
//! beyond the fixed fields defined at schema time. This is similar to
//! `additionalProperties` in JSON Schema.
//!
//! ```ignore
//! // HTTP headers - any string keys with string values
//! let headers = Object::builder("headers")
//!     .extensible(Text::builder("value").build())
//!     .build()?;
//!
//! // Environment variables with key pattern
//! let env = Object::builder("environment")
//!     .extensible_config(ExtensibleConfig::new(Text::builder("value").build())
//!         .key_pattern(r"^[A-Z][A-Z0-9_]*$")
//!         .max_properties(50))
//!     .build()?;
//! ```

use std::any::Any;
use std::fmt;
use std::sync::Arc;

use crate::core::{Flags, FxHashSet, Key, Metadata, SmartStr};
use crate::types::kind::NodeKind;
use crate::types::traits::{Container, Node};

// =============================================================================
// ExtensibleConfig
// =============================================================================

/// Configuration for extensible objects that allow additional properties.
///
/// When an object is extensible, it can accept key-value pairs beyond
/// the fixed fields defined in the schema. This is useful for:
/// - HTTP headers
/// - Environment variables
/// - Custom metadata
/// - User-defined properties
///
/// # Example
///
/// ```ignore
/// use paramdef::types::container::{Object, ExtensibleConfig};
/// use paramdef::types::leaf::Text;
///
/// let headers = Object::builder("headers")
///     .extensible_config(ExtensibleConfig::new(Text::builder("value").build())
///         .max_properties(100))
///     .build()?;
/// ```
#[derive(Clone)]
pub struct ExtensibleConfig {
    /// Template node defining the type of additional values.
    value_template: Arc<dyn Node>,
    /// Optional regex pattern for validating keys.
    key_pattern: Option<SmartStr>,
    /// Minimum number of additional properties.
    min_properties: Option<usize>,
    /// Maximum number of additional properties.
    max_properties: Option<usize>,
}

impl fmt::Debug for ExtensibleConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExtensibleConfig")
            .field("value_template", &self.value_template.key())
            .field("key_pattern", &self.key_pattern)
            .field("min_properties", &self.min_properties)
            .field("max_properties", &self.max_properties)
            .finish()
    }
}

impl ExtensibleConfig {
    /// Creates a new extensible configuration with the given value template.
    ///
    /// The value template defines what type of values can be added
    /// as additional properties.
    #[must_use]
    pub fn new(value_template: impl Node + 'static) -> Self {
        Self {
            value_template: Arc::new(value_template),
            key_pattern: None,
            min_properties: None,
            max_properties: None,
        }
    }

    /// Creates a new extensible configuration with an Arc-wrapped template.
    #[must_use]
    pub fn with_arc(value_template: Arc<dyn Node>) -> Self {
        Self {
            value_template,
            key_pattern: None,
            min_properties: None,
            max_properties: None,
        }
    }

    /// Sets a regex pattern for validating additional property keys.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Only allow SCREAMING_SNAKE_CASE keys
    /// ExtensibleConfig::new(Text::builder("value").build())
    ///     .key_pattern(r"^[A-Z][A-Z0-9_]*$")
    /// ```
    #[must_use]
    pub fn key_pattern(mut self, pattern: impl Into<SmartStr>) -> Self {
        self.key_pattern = Some(pattern.into());
        self
    }

    /// Sets the minimum number of additional properties required.
    #[must_use]
    pub fn min_properties(mut self, min: usize) -> Self {
        self.min_properties = Some(min);
        self
    }

    /// Sets the maximum number of additional properties allowed.
    #[must_use]
    pub fn max_properties(mut self, max: usize) -> Self {
        self.max_properties = Some(max);
        self
    }

    /// Returns the value template node.
    #[must_use]
    pub fn value_template(&self) -> &Arc<dyn Node> {
        &self.value_template
    }

    /// Returns the key pattern, if set.
    #[must_use]
    pub fn get_key_pattern(&self) -> Option<&str> {
        self.key_pattern.as_deref()
    }

    /// Returns the minimum properties constraint.
    #[must_use]
    pub fn get_min_properties(&self) -> Option<usize> {
        self.min_properties
    }

    /// Returns the maximum properties constraint.
    #[must_use]
    pub fn get_max_properties(&self) -> Option<usize> {
        self.max_properties
    }
}

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
    /// Configuration for additional properties beyond fixed fields.
    extensible: Option<ExtensibleConfig>,
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
            extensible: None,
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

    /// Returns whether this object is extensible (allows additional properties).
    #[inline]
    #[must_use]
    pub fn is_extensible(&self) -> bool {
        self.extensible.is_some()
    }

    /// Returns the extensible configuration, if set.
    #[must_use]
    pub fn extensible_config(&self) -> Option<&ExtensibleConfig> {
        self.extensible.as_ref()
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
    extensible: Option<ExtensibleConfig>,
}

impl fmt::Debug for ObjectBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ObjectBuilder")
            .field("key", &self.key)
            .field("label", &self.label)
            .field("description", &self.description)
            .field("flags", &self.flags)
            .field("field_count", &self.fields.len())
            .field("extensible", &self.extensible.is_some())
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
            extensible: None,
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

    /// Makes this object extensible, allowing additional properties.
    ///
    /// The value template defines what type of values can be added
    /// as additional properties beyond the fixed fields.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // HTTP headers - string keys with string values
    /// let headers = Object::builder("headers")
    ///     .extensible(Text::builder("value").build())
    ///     .build()?;
    /// ```
    #[must_use]
    pub fn extensible(mut self, value_template: impl Node + 'static) -> Self {
        self.extensible = Some(ExtensibleConfig::new(value_template));
        self
    }

    /// Makes this object extensible with full configuration.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Environment variables with constraints
    /// let env = Object::builder("environment")
    ///     .extensible_config(ExtensibleConfig::new(Text::builder("value").build())
    ///         .key_pattern(r"^[A-Z][A-Z0-9_]*$")
    ///         .max_properties(50))
    ///     .build()?;
    /// ```
    #[must_use]
    pub fn extensible_config(mut self, config: ExtensibleConfig) -> Self {
        self.extensible = Some(config);
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
            extensible: self.extensible,
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

    #[test]
    fn test_object_extensible_simple() {
        let obj = Object::builder("headers")
            .extensible(Text::builder("value").build())
            .build()
            .unwrap();

        assert!(obj.is_extensible());
        let config = obj.extensible_config().unwrap();
        assert!(config.get_key_pattern().is_none());
        assert!(config.get_min_properties().is_none());
        assert!(config.get_max_properties().is_none());
    }

    #[test]
    fn test_object_extensible_with_config() {
        let obj = Object::builder("environment")
            .extensible_config(
                ExtensibleConfig::new(Text::builder("value").build())
                    .key_pattern(r"^[A-Z][A-Z0-9_]*$")
                    .min_properties(1)
                    .max_properties(50),
            )
            .build()
            .unwrap();

        assert!(obj.is_extensible());
        let config = obj.extensible_config().unwrap();
        assert_eq!(config.get_key_pattern(), Some(r"^[A-Z][A-Z0-9_]*$"));
        assert_eq!(config.get_min_properties(), Some(1));
        assert_eq!(config.get_max_properties(), Some(50));
    }

    #[test]
    fn test_object_mixed_fixed_and_extensible() {
        let obj = Object::builder("config")
            .field("name", Text::builder("name").required().build())
            .field("version", Text::builder("version").build())
            .extensible(Text::builder("value").build())
            .build()
            .unwrap();

        assert_eq!(obj.field_count(), 2);
        assert!(obj.has_field("name"));
        assert!(obj.has_field("version"));
        assert!(obj.is_extensible());
    }

    #[test]
    fn test_object_not_extensible_by_default() {
        let obj = Object::builder("simple")
            .field("key", Text::builder("key").build())
            .build()
            .unwrap();

        assert!(!obj.is_extensible());
        assert!(obj.extensible_config().is_none());
    }

    #[test]
    fn test_extensible_config_value_template() {
        let config = ExtensibleConfig::new(Text::builder("value").build());
        assert_eq!(config.value_template().key().as_str(), "value");
    }
}
