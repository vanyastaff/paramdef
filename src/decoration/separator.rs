//! Separator decoration for visual dividers.
//!
//! Separator creates visual boundaries between sections.

use std::any::Any;

use crate::core::{Flags, Key, Metadata};
use crate::node::{Decoration, Node, NodeKind, SeparatorStyle};

/// A visual separator decoration.
///
/// Separator creates visual boundaries between form sections. It can have
/// an optional label and configurable spacing.
///
/// # Example
///
/// ```ignore
/// use paramdef::decoration::Separator;
/// use paramdef::node::SeparatorStyle;
///
/// // Simple thin separator
/// let thin = Separator::thin("sep1");
///
/// // Thick separator with label
/// let section = Separator::thick("advanced")
///     .label("Advanced Settings")
///     .build();
///
/// // Dashed separator
/// let dashed = Separator::builder("sep2")
///     .style(SeparatorStyle::Dashed)
///     .build();
///
/// // Just whitespace (no visible line)
/// let space = Separator::space("gap", 24.0);
/// ```
#[derive(Debug, Clone)]
pub struct Separator {
    metadata: Metadata,
    flags: Flags,
    style: SeparatorStyle,
    label: Option<String>,
    spacing: Option<f32>,
}

impl Separator {
    /// Creates a new builder for a Separator.
    #[must_use]
    pub fn builder(key: impl Into<Key>) -> SeparatorBuilder {
        SeparatorBuilder::new(key)
    }

    /// Creates a thin separator.
    #[must_use]
    pub fn thin(key: impl Into<Key>) -> Self {
        Self::builder(key).style(SeparatorStyle::Thin).build()
    }

    /// Creates a thick separator builder (for chaining with label).
    #[must_use]
    pub fn thick(key: impl Into<Key>) -> SeparatorBuilder {
        Self::builder(key).style(SeparatorStyle::Thick)
    }

    /// Creates a whitespace separator with specified spacing.
    #[must_use]
    pub fn space(key: impl Into<Key>, spacing: f32) -> Self {
        Self::builder(key)
            .style(SeparatorStyle::Space)
            .spacing(spacing)
            .build()
    }

    /// Returns the flags for this separator.
    #[must_use]
    pub fn flags(&self) -> Flags {
        self.flags
    }

    /// Returns the separator style.
    #[must_use]
    pub fn style(&self) -> SeparatorStyle {
        self.style
    }

    /// Returns the optional label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Returns the spacing in pixels.
    #[must_use]
    pub fn spacing(&self) -> Option<f32> {
        self.spacing
    }
}

impl Node for Separator {
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

impl Decoration for Separator {}

// =============================================================================
// Builder
// =============================================================================

/// Builder for [`Separator`].
#[derive(Debug)]
pub struct SeparatorBuilder {
    key: Key,
    flags: Flags,
    style: SeparatorStyle,
    label: Option<String>,
    spacing: Option<f32>,
}

impl SeparatorBuilder {
    /// Creates a new builder with the given key.
    #[must_use]
    pub fn new(key: impl Into<Key>) -> Self {
        Self {
            key: key.into(),
            flags: Flags::empty(),
            style: SeparatorStyle::Thin,
            label: None,
            spacing: None,
        }
    }

    /// Sets the flags.
    #[must_use]
    pub fn flags(mut self, flags: Flags) -> Self {
        self.flags = flags;
        self
    }

    /// Sets the separator style.
    #[must_use]
    pub fn style(mut self, style: SeparatorStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets the section label.
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the vertical spacing in pixels.
    #[must_use]
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(spacing);
        self
    }

    /// Builds the Separator.
    #[must_use]
    pub fn build(self) -> Separator {
        Separator {
            metadata: Metadata::new(self.key),
            flags: self.flags,
            style: self.style,
            label: self.label,
            spacing: self.spacing,
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
    fn test_separator_thin() {
        let sep = Separator::thin("sep");

        assert_eq!(sep.key().as_str(), "sep");
        assert_eq!(sep.style(), SeparatorStyle::Thin);
        assert!(sep.label().is_none());
        assert!(sep.spacing().is_none());
    }

    #[test]
    fn test_separator_thick_with_label() {
        let sep = Separator::thick("advanced")
            .label("Advanced Settings")
            .spacing(20.0)
            .build();

        assert_eq!(sep.style(), SeparatorStyle::Thick);
        assert_eq!(sep.label(), Some("Advanced Settings"));
        assert_eq!(sep.spacing(), Some(20.0));
    }

    #[test]
    fn test_separator_space() {
        let sep = Separator::space("gap", 32.0);

        assert_eq!(sep.style(), SeparatorStyle::Space);
        assert_eq!(sep.spacing(), Some(32.0));
    }

    #[test]
    fn test_separator_styles() {
        let dashed = Separator::builder("d")
            .style(SeparatorStyle::Dashed)
            .build();
        assert_eq!(dashed.style(), SeparatorStyle::Dashed);

        let dotted = Separator::builder("dt")
            .style(SeparatorStyle::Dotted)
            .build();
        assert_eq!(dotted.style(), SeparatorStyle::Dotted);
    }

    #[test]
    fn test_separator_kind() {
        let sep = Separator::thin("test");

        assert_eq!(sep.kind(), NodeKind::Decoration);
    }

    #[test]
    fn test_separator_invariants() {
        let sep = Separator::thin("test");

        assert!(!sep.kind().has_own_value());
        assert!(!sep.kind().has_value_access());
        assert!(!sep.kind().can_have_children());
    }
}
