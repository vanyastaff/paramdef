//! Parameter metadata for display and organization.
//!
//! Metadata contains display information for parameters like labels, descriptions,
//! grouping, and tags. It uses the builder pattern for ergonomic construction.

use super::Key;
use smallvec::SmallVec;

/// Display and organizational metadata for a parameter.
///
/// Metadata is immutable once created and contains information used for
/// UI display and parameter organization.
///
/// # Examples
///
/// ```
/// use paramdef::core::Metadata;
///
/// // Minimal metadata with just a key
/// let meta = Metadata::new("username");
///
/// // Full metadata using builder
/// let meta = Metadata::builder("email")
///     .label("Email Address")
///     .description("Your primary email for notifications")
///     .group("contact")
///     .tag("required")
///     .tag("validated")
///     .build();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metadata {
    /// Unique identifier for the parameter.
    key: Key,

    /// Human-readable label for display.
    label: Option<Key>,

    /// Longer description or help text.
    description: Option<Key>,

    /// Grouping category for organization.
    group: Option<Key>,

    /// Tags for filtering and categorization.
    /// Uses `SmallVec` to avoid heap allocation for small tag counts.
    tags: SmallVec<[Key; 4]>,
}

impl Metadata {
    /// Creates new metadata with just a key.
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::core::Metadata;
    ///
    /// let meta = Metadata::new("my_param");
    /// assert_eq!(meta.key(), "my_param");
    /// ```
    pub fn new(key: impl Into<Key>) -> Self {
        Self {
            key: key.into(),
            label: None,
            description: None,
            group: None,
            tags: SmallVec::new(),
        }
    }

    /// Creates a builder for constructing metadata.
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::core::Metadata;
    ///
    /// let meta = Metadata::builder("config")
    ///     .label("Configuration")
    ///     .build();
    /// ```
    pub fn builder(key: impl Into<Key>) -> MetadataBuilder {
        MetadataBuilder::new(key)
    }

    /// Returns the parameter key.
    #[inline]
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the display label, if set.
    #[inline]
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Returns the description, if set.
    #[inline]
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the group, if set.
    #[inline]
    #[must_use]
    pub fn group(&self) -> Option<&str> {
        self.group.as_deref()
    }

    /// Returns the tags slice.
    #[inline]
    #[must_use]
    pub fn tags(&self) -> &[Key] {
        &self.tags
    }

    /// Returns `true` if the parameter has a specific tag.
    #[must_use]
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t.as_str() == tag)
    }

    /// Returns the display label or falls back to the key.
    #[inline]
    #[must_use]
    pub fn display_label(&self) -> &str {
        self.label.as_deref().unwrap_or(&self.key)
    }
}

/// Builder for constructing [`Metadata`].
///
/// # Examples
///
/// ```
/// use paramdef::core::Metadata;
///
/// let meta = Metadata::builder("port")
///     .label("Port Number")
///     .description("Network port to listen on")
///     .group("network")
///     .tag("advanced")
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct MetadataBuilder {
    key: Key,
    label: Option<Key>,
    description: Option<Key>,
    group: Option<Key>,
    tags: SmallVec<[Key; 4]>,
}

impl MetadataBuilder {
    /// Creates a new builder with the given key.
    pub fn new(key: impl Into<Key>) -> Self {
        Self {
            key: key.into(),
            label: None,
            description: None,
            group: None,
            tags: SmallVec::new(),
        }
    }

    /// Sets the display label.
    #[must_use]
    pub fn label(mut self, label: impl Into<Key>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the description.
    #[must_use]
    pub fn description(mut self, description: impl Into<Key>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the group.
    #[must_use]
    pub fn group(mut self, group: impl Into<Key>) -> Self {
        self.group = Some(group.into());
        self
    }

    /// Adds a tag.
    #[must_use]
    pub fn tag(mut self, tag: impl Into<Key>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Adds multiple tags.
    #[must_use]
    pub fn tags(mut self, tags: impl IntoIterator<Item = impl Into<Key>>) -> Self {
        self.tags.extend(tags.into_iter().map(Into::into));
        self
    }

    /// Builds the metadata.
    #[must_use]
    pub fn build(self) -> Metadata {
        Metadata {
            key: self.key,
            label: self.label,
            description: self.description,
            group: self.group,
            tags: self.tags,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_minimal() {
        let meta = Metadata::new("test_key");
        assert_eq!(meta.key(), "test_key");
        assert!(meta.label().is_none());
        assert!(meta.description().is_none());
        assert!(meta.group().is_none());
        assert!(meta.tags().is_empty());
    }

    #[test]
    fn test_metadata_with_label() {
        let meta = Metadata::builder("key").label("My Label").build();
        assert_eq!(meta.label(), Some("My Label"));
    }

    #[test]
    fn test_metadata_with_description() {
        let meta = Metadata::builder("key")
            .description("A helpful description")
            .build();
        assert_eq!(meta.description(), Some("A helpful description"));
    }

    #[test]
    fn test_metadata_with_group() {
        let meta = Metadata::builder("key").group("settings").build();
        assert_eq!(meta.group(), Some("settings"));
    }

    #[test]
    fn test_metadata_with_tags() {
        let meta = Metadata::builder("key")
            .tag("important")
            .tag("validated")
            .build();

        assert_eq!(meta.tags().len(), 2);
        assert!(meta.has_tag("important"));
        assert!(meta.has_tag("validated"));
        assert!(!meta.has_tag("unknown"));
    }

    #[test]
    fn test_metadata_tags_batch() {
        let meta = Metadata::builder("key")
            .tags(["tag1", "tag2", "tag3"])
            .build();

        assert_eq!(meta.tags().len(), 3);
    }

    #[test]
    fn test_metadata_builder() {
        let meta = Metadata::builder("full_example")
            .label("Full Example")
            .description("A complete example with all fields")
            .group("examples")
            .tag("demo")
            .tag("complete")
            .build();

        assert_eq!(meta.key(), "full_example");
        assert_eq!(meta.label(), Some("Full Example"));
        assert_eq!(
            meta.description(),
            Some("A complete example with all fields")
        );
        assert_eq!(meta.group(), Some("examples"));
        assert_eq!(meta.tags().len(), 2);
    }

    #[test]
    fn test_metadata_display_label() {
        // With label
        let meta = Metadata::builder("key").label("Display").build();
        assert_eq!(meta.display_label(), "Display");

        // Without label - falls back to key
        let meta = Metadata::new("fallback_key");
        assert_eq!(meta.display_label(), "fallback_key");
    }

    #[test]
    fn test_metadata_clone() {
        let meta1 = Metadata::builder("key").label("Label").tag("tag").build();
        let meta2 = meta1.clone();

        assert_eq!(meta1, meta2);
    }
}
