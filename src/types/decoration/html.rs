//! Html decoration for displaying rich HTML content.
//!
//! Html displays arbitrary HTML content in the UI, useful for
//! complex formatting, embedded widgets, or custom rendering.

use std::any::Any;

use crate::core::{Flags, Key, Metadata, SmartStr};
use crate::types::kind::NodeKind;
use crate::types::traits::{Decoration, Node};

/// HTML sanitization level for security.
///
/// Controls what HTML tags and attributes are allowed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SanitizeLevel {
    /// No sanitization - trust content completely.
    /// Only use for trusted, internal content.
    None,

    /// Basic sanitization - allow common formatting tags.
    /// Removes scripts, iframes, and dangerous attributes.
    #[default]
    Basic,

    /// Strict sanitization - only allow safe inline formatting.
    /// Removes all block elements, links, and images.
    Strict,

    /// Custom sanitization - defer to UI implementation.
    /// The UI layer decides what to allow based on context.
    Custom,
}

/// A display-only HTML content decoration.
///
/// Html displays rich HTML content in the UI. It has no value and
/// cannot contain children. Used for complex formatting, embedded
/// content, or custom HTML widgets.
///
/// # Security
///
/// By default, HTML is sanitized to prevent XSS attacks. Use
/// [`SanitizeLevel::None`] only for trusted internal content.
///
/// # Example
///
/// ```ignore
/// use paramdef::decoration::Html;
/// use paramdef::decoration::SanitizeLevel;
///
/// // Simple formatted content
/// let formatted = Html::new("intro", "<p>Welcome to <strong>MyApp</strong>!</p>");
///
/// // Content with custom CSS class
/// let styled = Html::builder("custom")
///     .content("<div class='highlight'>Important notice</div>")
///     .css_class("notice-box")
///     .build();
///
/// // Trusted content (no sanitization)
/// let trusted = Html::builder("internal")
///     .content("<script>...</script>")
///     .sanitize(SanitizeLevel::None)
///     .build();
///
/// // Inline HTML snippet
/// let inline = Html::inline("badge", "<span class='badge'>NEW</span>");
/// ```
#[derive(Debug, Clone)]
pub struct Html {
    metadata: Metadata,
    flags: Flags,
    content: SmartStr,
    sanitize: SanitizeLevel,
    css_class: Option<SmartStr>,
    inline: bool,
}

impl Html {
    /// Creates a new builder for an Html decoration.
    #[must_use]
    pub fn builder(key: impl Into<Key>) -> HtmlBuilder {
        HtmlBuilder::new(key)
    }

    /// Creates a simple Html decoration with content.
    #[must_use]
    pub fn new(key: impl Into<Key>, content: impl Into<SmartStr>) -> Self {
        Self::builder(key).content(content).build()
    }

    /// Creates an inline Html decoration.
    ///
    /// Inline HTML is rendered without block-level wrapping,
    /// suitable for badges, icons, or inline formatting.
    #[must_use]
    pub fn inline(key: impl Into<Key>, content: impl Into<SmartStr>) -> Self {
        Self::builder(key).content(content).inline(true).build()
    }

    /// Returns the flags for this Html decoration.
    #[must_use]
    pub fn flags(&self) -> Flags {
        self.flags
    }

    /// Returns the HTML content.
    #[must_use]
    pub fn content(&self) -> &str {
        self.content.as_str()
    }

    /// Returns the sanitization level.
    #[must_use]
    pub fn sanitize(&self) -> SanitizeLevel {
        self.sanitize
    }

    /// Returns the optional CSS class.
    #[must_use]
    pub fn css_class(&self) -> Option<&str> {
        self.css_class.as_deref()
    }

    /// Returns whether this is inline HTML.
    #[must_use]
    pub fn is_inline(&self) -> bool {
        self.inline
    }
}

impl Node for Html {
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

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Decoration for Html {}

// =============================================================================
// Builder
// =============================================================================

/// Builder for [`Html`].
#[derive(Debug)]
pub struct HtmlBuilder {
    key: Key,
    label: Option<SmartStr>,
    description: Option<SmartStr>,
    flags: Flags,
    content: SmartStr,
    sanitize: SanitizeLevel,
    css_class: Option<SmartStr>,
    inline: bool,
}

impl HtmlBuilder {
    /// Creates a new builder with the given key.
    #[must_use]
    pub fn new(key: impl Into<Key>) -> Self {
        Self {
            key: key.into(),
            label: None,
            description: None,
            flags: Flags::empty(),
            content: SmartStr::new(),
            sanitize: SanitizeLevel::default(),
            css_class: None,
            inline: false,
        }
    }

    /// Sets the label.
    #[must_use]
    pub fn label(mut self, label: impl Into<SmartStr>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the description.
    #[must_use]
    pub fn description(mut self, description: impl Into<SmartStr>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the flags.
    #[must_use]
    pub fn flags(mut self, flags: Flags) -> Self {
        self.flags = flags;
        self
    }

    /// Sets the HTML content.
    #[must_use]
    pub fn content(mut self, content: impl Into<SmartStr>) -> Self {
        self.content = content.into();
        self
    }

    /// Sets the sanitization level.
    #[must_use]
    pub fn sanitize(mut self, level: SanitizeLevel) -> Self {
        self.sanitize = level;
        self
    }

    /// Sets the CSS class for the wrapper element.
    #[must_use]
    pub fn css_class(mut self, class: impl Into<SmartStr>) -> Self {
        self.css_class = Some(class.into());
        self
    }

    /// Sets whether this is inline HTML.
    #[must_use]
    pub fn inline(mut self, inline: bool) -> Self {
        self.inline = inline;
        self
    }

    /// Builds the Html decoration.
    #[must_use]
    pub fn build(self) -> Html {
        let mut metadata = Metadata::new(self.key);
        if let Some(label) = self.label {
            metadata = metadata.with_label(label);
        }
        if let Some(description) = self.description {
            metadata = metadata.with_description(description);
        }

        Html {
            metadata,
            flags: self.flags,
            content: self.content,
            sanitize: self.sanitize,
            css_class: self.css_class,
            inline: self.inline,
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_new() {
        let html = Html::new("intro", "<p>Hello</p>");

        assert_eq!(html.key().as_str(), "intro");
        assert_eq!(html.content(), "<p>Hello</p>");
        assert_eq!(html.sanitize(), SanitizeLevel::Basic);
        assert!(!html.is_inline());
        assert!(html.css_class().is_none());
    }

    #[test]
    fn test_html_inline() {
        let html = Html::inline("badge", "<span>NEW</span>");

        assert_eq!(html.key().as_str(), "badge");
        assert_eq!(html.content(), "<span>NEW</span>");
        assert!(html.is_inline());
    }

    #[test]
    fn test_html_builder() {
        let html = Html::builder("custom")
            .label("Custom HTML")
            .content("<div>Content</div>")
            .sanitize(SanitizeLevel::Strict)
            .css_class("highlight")
            .inline(false)
            .build();

        assert_eq!(html.metadata().label(), Some("Custom HTML"));
        assert_eq!(html.content(), "<div>Content</div>");
        assert_eq!(html.sanitize(), SanitizeLevel::Strict);
        assert_eq!(html.css_class(), Some("highlight"));
        assert!(!html.is_inline());
    }

    #[test]
    fn test_html_no_sanitization() {
        let html = Html::builder("trusted")
            .content("<script>alert('hi')</script>")
            .sanitize(SanitizeLevel::None)
            .build();

        assert_eq!(html.sanitize(), SanitizeLevel::None);
        assert!(html.content().contains("<script>"));
    }

    #[test]
    fn test_html_kind() {
        let html = Html::new("test", "");

        assert_eq!(html.kind(), NodeKind::Decoration);
    }

    #[test]
    fn test_html_invariants() {
        let html = Html::new("test", "");

        // Decoration has NO own value
        assert!(!html.kind().has_own_value());

        // Decoration has NO ValueAccess
        assert!(!html.kind().has_value_access());

        // Decoration CANNOT have children
        assert!(!html.kind().can_have_children());
    }

    #[test]
    fn test_sanitize_level_default() {
        assert_eq!(SanitizeLevel::default(), SanitizeLevel::Basic);
    }
}
