//! Link decoration for clickable references.
//!
//! Link provides clickable references to documentation, tutorials, or external resources.

use std::any::Any;

use crate::core::{Flags, Key, Metadata};
use crate::node::{Decoration, LinkType, Node, NodeKind};

/// A clickable link decoration.
///
/// Link displays a clickable reference to documentation, tutorials, videos,
/// or external resources. It has no value and cannot contain children.
///
/// # Example
///
/// ```ignore
/// use paramdef::decoration::Link;
/// use paramdef::node::LinkType;
///
/// // Documentation link
/// let docs = Link::documentation("api_docs", "API Reference")
///     .url("https://docs.example.com/api")
///     .build();
///
/// // Tutorial link
/// let tutorial = Link::tutorial("getting_started", "Getting Started Guide")
///     .url("https://example.com/tutorial")
///     .build();
///
/// // External link that opens in new tab
/// let external = Link::builder("github")
///     .text("View on GitHub")
///     .url("https://github.com/example/repo")
///     .link_type(LinkType::External)
///     .open_in_new_tab(true)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct Link {
    metadata: Metadata,
    flags: Flags,
    text: String,
    url: String,
    kind: LinkType,
    open_in_new_tab: bool,
}

impl Link {
    /// Creates a new builder for a Link.
    #[must_use]
    pub fn builder(key: impl Into<Key>) -> LinkBuilder {
        LinkBuilder::new(key)
    }

    /// Creates a documentation link builder.
    #[must_use]
    pub fn documentation(key: impl Into<Key>, text: impl Into<String>) -> LinkBuilder {
        Self::builder(key)
            .text(text)
            .link_type(LinkType::Documentation)
    }

    /// Creates a tutorial link builder.
    #[must_use]
    pub fn tutorial(key: impl Into<Key>, text: impl Into<String>) -> LinkBuilder {
        Self::builder(key).text(text).link_type(LinkType::Tutorial)
    }

    /// Creates a video link builder.
    #[must_use]
    pub fn video(key: impl Into<Key>, text: impl Into<String>) -> LinkBuilder {
        Self::builder(key).text(text).link_type(LinkType::Video)
    }

    /// Creates an external link builder.
    #[must_use]
    pub fn external(key: impl Into<Key>, text: impl Into<String>) -> LinkBuilder {
        Self::builder(key)
            .text(text)
            .link_type(LinkType::External)
            .open_in_new_tab(true)
    }

    /// Creates an API reference link builder.
    #[must_use]
    pub fn api(key: impl Into<Key>, text: impl Into<String>) -> LinkBuilder {
        Self::builder(key).text(text).link_type(LinkType::Api)
    }

    /// Returns the flags for this link.
    #[must_use]
    pub fn flags(&self) -> Flags {
        self.flags
    }

    /// Returns the link text.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns the URL.
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Returns the link type.
    #[must_use]
    pub fn link_type(&self) -> LinkType {
        self.kind
    }

    /// Returns whether the link opens in a new tab.
    #[must_use]
    pub fn open_in_new_tab(&self) -> bool {
        self.open_in_new_tab
    }
}

impl Node for Link {
    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn key(&self) -> &Key {
        self.metadata.key()
    }

    fn kind(&self) -> NodeKind {
        NodeKind::Decoration
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Decoration for Link {}

// =============================================================================
// Builder
// =============================================================================

/// Builder for [`Link`].
#[derive(Debug)]
pub struct LinkBuilder {
    key: Key,
    flags: Flags,
    text: String,
    url: Option<String>,
    kind: LinkType,
    open_in_new_tab: bool,
}

impl LinkBuilder {
    /// Creates a new builder with the given key.
    #[must_use]
    pub fn new(key: impl Into<Key>) -> Self {
        Self {
            key: key.into(),
            flags: Flags::empty(),
            text: String::new(),
            url: None,
            kind: LinkType::Documentation,
            open_in_new_tab: false,
        }
    }

    /// Sets the flags.
    #[must_use]
    pub fn flags(mut self, flags: Flags) -> Self {
        self.flags = flags;
        self
    }

    /// Sets the link text.
    #[must_use]
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    /// Sets the URL (required).
    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Sets the link type.
    #[must_use]
    pub fn link_type(mut self, link_type: LinkType) -> Self {
        self.kind = link_type;
        self
    }

    /// Sets whether the link opens in a new tab.
    #[must_use]
    pub fn open_in_new_tab(mut self, open_in_new_tab: bool) -> Self {
        self.open_in_new_tab = open_in_new_tab;
        self
    }

    /// Builds the Link.
    ///
    /// # Errors
    ///
    /// Returns an error if the URL was not specified.
    pub fn build(self) -> crate::core::Result<Link> {
        let url = self
            .url
            .ok_or_else(|| crate::core::Error::missing_required("url"))?;

        Ok(Link {
            metadata: Metadata::new(self.key),
            flags: self.flags,
            text: self.text,
            url,
            kind: self.kind,
            open_in_new_tab: self.open_in_new_tab,
        })
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_documentation() {
        let link = Link::documentation("docs", "API Reference")
            .url("https://docs.example.com")
            .build()
            .unwrap();

        assert_eq!(link.key().as_str(), "docs");
        assert_eq!(link.text(), "API Reference");
        assert_eq!(link.url(), "https://docs.example.com");
        assert_eq!(link.link_type(), LinkType::Documentation);
        assert!(!link.open_in_new_tab());
    }

    #[test]
    fn test_link_tutorial() {
        let link = Link::tutorial("guide", "Getting Started")
            .url("https://example.com/guide")
            .build()
            .unwrap();

        assert_eq!(link.link_type(), LinkType::Tutorial);
        assert_eq!(link.text(), "Getting Started");
    }

    #[test]
    fn test_link_video() {
        let link = Link::video("demo", "Watch Demo")
            .url("https://youtube.com/watch?v=123")
            .build()
            .unwrap();

        assert_eq!(link.link_type(), LinkType::Video);
    }

    #[test]
    fn test_link_external() {
        let link = Link::external("github", "View on GitHub")
            .url("https://github.com/example")
            .build()
            .unwrap();

        assert_eq!(link.link_type(), LinkType::External);
        assert!(link.open_in_new_tab()); // External links open in new tab by default
    }

    #[test]
    fn test_link_api() {
        let link = Link::api("api_ref", "API Docs")
            .url("https://api.example.com/docs")
            .build()
            .unwrap();

        assert_eq!(link.link_type(), LinkType::Api);
    }

    #[test]
    fn test_link_builder() {
        let link = Link::builder("custom")
            .text("Custom Link")
            .url("https://example.com")
            .link_type(LinkType::External)
            .open_in_new_tab(true)
            .build()
            .unwrap();

        assert_eq!(link.text(), "Custom Link");
        assert!(link.open_in_new_tab());
    }

    #[test]
    fn test_link_kind() {
        let link = Link::documentation("test", "Test")
            .url("#")
            .build()
            .unwrap();

        assert_eq!(link.kind(), NodeKind::Decoration);
    }

    #[test]
    fn test_link_invariants() {
        let link = Link::documentation("test", "Test")
            .url("#")
            .build()
            .unwrap();

        assert!(!link.kind().has_own_value());
        assert!(!link.kind().has_value_access());
        assert!(!link.kind().can_have_children());
    }

    #[test]
    fn test_link_requires_url() {
        let result = Link::builder("no_url").text("Missing URL").build();
        assert!(result.is_err());
    }
}
