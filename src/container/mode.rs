//! Mode container for discriminated unions.
//!
//! Mode represents a choice between different variants, where each variant
//! can have its own structure. The output is always `{ mode: "variant_key", value: {...} }`.

use std::any::Any;
use std::fmt;
use std::sync::Arc;

use crate::core::{Flags, Key, Metadata};
use crate::node::{Container, Node, NodeKind};

/// A variant in a Mode container.
///
/// Each variant has a key, label, and content node.
#[derive(Clone)]
pub struct ModeVariant {
    /// Unique key for this variant.
    pub key: Key,
    /// Display label for this variant.
    pub label: String,
    /// Optional description.
    pub description: Option<String>,
    /// The content node for this variant.
    pub content: Arc<dyn Node>,
}

impl fmt::Debug for ModeVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ModeVariant")
            .field("key", &self.key)
            .field("label", &self.label)
            .field("description", &self.description)
            .finish_non_exhaustive()
    }
}

impl ModeVariant {
    /// Creates a new variant.
    #[must_use]
    pub fn new(
        key: impl Into<Key>,
        label: impl Into<String>,
        content: impl Node + 'static,
    ) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            description: None,
            content: Arc::new(content),
        }
    }

    /// Creates a new variant with a description.
    #[must_use]
    pub fn with_description(
        key: impl Into<Key>,
        label: impl Into<String>,
        description: impl Into<String>,
        content: impl Node + 'static,
    ) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            description: Some(description.into()),
            content: Arc::new(content),
        }
    }
}

/// A container for discriminated unions.
///
/// Mode is one of the six container types. It allows selecting one variant
/// from a list, where each variant has its own content. The output is
/// always `{ mode: "variant_key", value: {...} }`.
///
/// # Example
///
/// ```ignore
/// use paramdef::container::{Mode, Object};
/// use paramdef::parameter::Text;
///
/// let auth = Mode::builder("auth")
///     .label("Authentication")
///     .variant("none", "No Auth", Object::empty("none_config"))
///     .variant("basic", "Basic Auth", Object::builder("basic_config")
///         .field("username", Text::builder("username").required().build())
///         .field("password", Text::builder("password").required().build())
///         .build())
///     .variant("bearer", "Bearer Token", Object::builder("bearer_config")
///         .field("token", Text::builder("token").required().build())
///         .build())
///     .default_variant("none")
///     .build()
///     .unwrap();
/// ```
#[derive(Clone)]
pub struct Mode {
    metadata: Metadata,
    flags: Flags,
    variants: Vec<ModeVariant>,
    default_variant: Option<Key>,
    /// Cached children for Container trait
    children_cache: Arc<[Arc<dyn Node>]>,
}

impl fmt::Debug for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Mode")
            .field("metadata", &self.metadata)
            .field("flags", &self.flags)
            .field("variant_count", &self.variants.len())
            .field("default_variant", &self.default_variant)
            .finish_non_exhaustive()
    }
}

impl Mode {
    /// Creates a new builder for a Mode.
    #[must_use]
    pub fn builder(key: impl Into<Key>) -> ModeBuilder {
        ModeBuilder::new(key)
    }

    /// Returns the flags for this mode.
    #[must_use]
    pub fn flags(&self) -> Flags {
        self.flags
    }

    /// Returns all variants.
    #[must_use]
    pub fn variants(&self) -> &[ModeVariant] {
        &self.variants
    }

    /// Returns the number of variants.
    #[must_use]
    pub fn variant_count(&self) -> usize {
        self.variants.len()
    }

    /// Gets a variant by key.
    #[must_use]
    pub fn get_variant(&self, key: &str) -> Option<&ModeVariant> {
        self.variants.iter().find(|v| v.key == key)
    }

    /// Returns the default variant key, if set.
    #[must_use]
    pub fn default_variant(&self) -> Option<&Key> {
        self.default_variant.as_ref()
    }

    /// Returns an iterator over variant keys.
    pub fn variant_keys(&self) -> impl Iterator<Item = &Key> {
        self.variants.iter().map(|v| &v.key)
    }
}

impl Node for Mode {
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
}

impl Container for Mode {
    fn children(&self) -> &[Arc<dyn Node>] {
        &self.children_cache
    }
}

// =============================================================================
// Builder
// =============================================================================

/// Builder for [`Mode`].
#[derive(Debug)]
pub struct ModeBuilder {
    key: Key,
    label: Option<String>,
    description: Option<String>,
    flags: Flags,
    variants: Vec<ModeVariant>,
    default_variant: Option<Key>,
}

impl ModeBuilder {
    /// Creates a new builder with the given key.
    #[must_use]
    pub fn new(key: impl Into<Key>) -> Self {
        Self {
            key: key.into(),
            label: None,
            description: None,
            flags: Flags::empty(),
            variants: Vec::new(),
            default_variant: None,
        }
    }

    /// Sets the label for this mode.
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the description for this mode.
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the flags for this mode.
    #[must_use]
    pub fn flags(mut self, flags: Flags) -> Self {
        self.flags = flags;
        self
    }

    /// Marks this mode as required.
    #[must_use]
    pub fn required(mut self) -> Self {
        self.flags |= Flags::REQUIRED;
        self
    }

    /// Adds a variant to the mode.
    #[must_use]
    pub fn variant(
        mut self,
        key: impl Into<Key>,
        label: impl Into<String>,
        content: impl Node + 'static,
    ) -> Self {
        self.variants.push(ModeVariant::new(key, label, content));
        self
    }

    /// Adds a variant with a description.
    #[must_use]
    pub fn variant_with_description(
        mut self,
        key: impl Into<Key>,
        label: impl Into<String>,
        description: impl Into<String>,
        content: impl Node + 'static,
    ) -> Self {
        self.variants.push(ModeVariant::with_description(
            key,
            label,
            description,
            content,
        ));
        self
    }

    /// Sets the default variant by key.
    #[must_use]
    pub fn default_variant(mut self, key: impl Into<Key>) -> Self {
        self.default_variant = Some(key.into());
        self
    }

    /// Builds the Mode.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No variants were added
    /// - Duplicate variant keys exist
    /// - `default_variant` references a non-existent variant key
    pub fn build(self) -> crate::core::Result<Mode> {
        if self.variants.is_empty() {
            return Err(crate::core::Error::missing_required("variant"));
        }

        // Check for duplicate variant keys
        let mut seen_keys = std::collections::HashSet::new();
        for variant in &self.variants {
            if !seen_keys.insert(&variant.key) {
                return Err(crate::core::Error::validation(
                    "duplicate_key",
                    format!("duplicate variant key: {}", variant.key),
                ));
            }
        }

        // Validate default_variant references an existing key
        if let Some(ref default) = self.default_variant
            && !self.variants.iter().any(|v| &v.key == default)
        {
            return Err(crate::core::Error::not_found(format!(
                "default_variant '{default}'"
            )));
        }

        let mut metadata = Metadata::new(self.key);
        if let Some(label) = self.label {
            metadata = metadata.with_label(label);
        }
        if let Some(description) = self.description {
            metadata = metadata.with_description(description);
        }

        // Build children cache from variant contents
        let children_cache: Arc<[Arc<dyn Node>]> = self
            .variants
            .iter()
            .map(|v| Arc::clone(&v.content))
            .collect();

        Ok(Mode {
            metadata,
            flags: self.flags,
            variants: self.variants,
            default_variant: self.default_variant,
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
    use crate::container::Object;
    use crate::parameter::Text;

    #[test]
    fn test_mode_basic() {
        let mode = Mode::builder("auth")
            .label("Authentication")
            .variant("none", "No Auth", Object::empty("none_config"))
            .build()
            .unwrap();

        assert_eq!(mode.key().as_str(), "auth");
        assert_eq!(mode.metadata().label(), Some("Authentication"));
        assert_eq!(mode.kind(), NodeKind::Container);
        assert_eq!(mode.variant_count(), 1);
    }

    #[test]
    fn test_mode_multiple_variants() {
        let mode = Mode::builder("auth")
            .variant("none", "No Auth", Object::empty("none"))
            .variant(
                "basic",
                "Basic Auth",
                Object::builder("basic")
                    .field("username", Text::builder("username").build())
                    .build()
                    .unwrap(),
            )
            .variant(
                "bearer",
                "Bearer Token",
                Object::builder("bearer")
                    .field("token", Text::builder("token").build())
                    .build()
                    .unwrap(),
            )
            .default_variant("none")
            .build()
            .unwrap();

        assert_eq!(mode.variant_count(), 3);
        assert_eq!(mode.default_variant(), Some(&Key::from("none")));
    }

    #[test]
    fn test_mode_get_variant() {
        let mode = Mode::builder("mode")
            .variant("a", "Option A", Object::empty("a"))
            .variant("b", "Option B", Object::empty("b"))
            .build()
            .unwrap();

        let variant_a = mode.get_variant("a");
        assert!(variant_a.is_some());
        assert_eq!(variant_a.unwrap().label, "Option A");

        assert!(mode.get_variant("c").is_none());
    }

    #[test]
    fn test_mode_variant_keys() {
        let mode = Mode::builder("mode")
            .variant("x", "X", Object::empty("x"))
            .variant("y", "Y", Object::empty("y"))
            .variant("z", "Z", Object::empty("z"))
            .build()
            .unwrap();

        let keys: Vec<&str> = mode.variant_keys().map(|k| k.as_str()).collect();
        assert_eq!(keys, vec!["x", "y", "z"]);
    }

    #[test]
    fn test_mode_variant_with_description() {
        let mode = Mode::builder("mode")
            .variant_with_description(
                "option",
                "Option",
                "This is an option",
                Object::empty("opt"),
            )
            .build()
            .unwrap();

        let variant = mode.get_variant("option").unwrap();
        assert_eq!(variant.description, Some("This is an option".to_string()));
    }

    #[test]
    fn test_mode_flags() {
        let mode = Mode::builder("required_mode")
            .variant("a", "A", Object::empty("a"))
            .required()
            .build()
            .unwrap();

        assert!(mode.flags().contains(Flags::REQUIRED));
    }

    #[test]
    fn test_mode_requires_variants() {
        let result = Mode::builder("empty_mode").build();
        assert!(result.is_err());
    }

    #[test]
    fn test_mode_duplicate_variant_keys() {
        let result = Mode::builder("mode")
            .variant("a", "First A", Object::empty("a1"))
            .variant("a", "Second A", Object::empty("a2"))
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_mode_invalid_default_variant() {
        let result = Mode::builder("mode")
            .variant("a", "A", Object::empty("a"))
            .default_variant("nonexistent")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_mode_children() {
        let mode = Mode::builder("mode")
            .variant("x", "X", Object::empty("x"))
            .variant("y", "Y", Object::empty("y"))
            .build()
            .unwrap();

        assert_eq!(mode.children().len(), 2);
    }
}
