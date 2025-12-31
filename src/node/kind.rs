//! Node kind enumeration and supporting types.

use std::fmt;

/// The five categories of node types.
///
/// Every node in the system falls into one of these five categories,
/// which determines its capabilities and constraints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum NodeKind {
    /// Root aggregator (Group).
    ///
    /// - NO own value
    /// - HAS `ValueAccess`
    /// - Can contain: Layout, Decoration, Container, Leaf
    Group,

    /// UI organization (Panel).
    ///
    /// - NO own value
    /// - HAS `ValueAccess`
    /// - Can contain: Decoration, Container, Leaf
    Layout,

    /// Display-only (Notice).
    ///
    /// - NO value
    /// - NO children
    Decoration,

    /// Data structures (Object, List, Mode, Routing, Expirable, Ref).
    ///
    /// - HAS own value
    /// - HAS `ValueAccess`
    /// - Can contain: Decoration, Container, Leaf
    Container,

    /// Terminal values (Text, Number, Boolean, Vector, Select).
    ///
    /// - HAS own value
    /// - NO children
    Leaf,
}

impl NodeKind {
    /// Returns whether this kind has its own value.
    #[inline]
    #[must_use]
    pub const fn has_own_value(&self) -> bool {
        matches!(self, Self::Container | Self::Leaf)
    }

    /// Returns whether this kind can access child values.
    #[inline]
    #[must_use]
    pub const fn has_value_access(&self) -> bool {
        matches!(self, Self::Group | Self::Layout | Self::Container)
    }

    /// Returns whether this kind can contain children.
    #[inline]
    #[must_use]
    pub const fn can_have_children(&self) -> bool {
        matches!(self, Self::Group | Self::Layout | Self::Container)
    }

    /// Returns the name of this kind.
    #[inline]
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Group => "group",
            Self::Layout => "layout",
            Self::Decoration => "decoration",
            Self::Container => "container",
            Self::Leaf => "leaf",
        }
    }
}

impl fmt::Display for NodeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// The semantic type of a Notice decoration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum NoticeType {
    /// Informational message (blue).
    #[default]
    Info,

    /// Warning message (yellow/orange).
    Warning,

    /// Error message (red).
    Error,

    /// Success message (green).
    Success,

    /// Tip or hint message (purple).
    Tip,
}

impl NoticeType {
    /// Returns the name of this notice type.
    #[inline]
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Success => "success",
            Self::Tip => "tip",
        }
    }
}

impl fmt::Display for NoticeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// The visual style of a Separator decoration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum SeparatorStyle {
    /// Thin line (default).
    #[default]
    Thin,
    /// Thick/bold line.
    Thick,
    /// Dashed line.
    Dashed,
    /// Dotted line.
    Dotted,
    /// Just whitespace, no visible line.
    Space,
}

impl SeparatorStyle {
    /// Returns the name of this separator style.
    #[inline]
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Thin => "thin",
            Self::Thick => "thick",
            Self::Dashed => "dashed",
            Self::Dotted => "dotted",
            Self::Space => "space",
        }
    }
}

impl fmt::Display for SeparatorStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// The type of a Link decoration.
///
/// This enum categorizes links by their **content type** for UI purposes
/// (e.g., showing appropriate icons). The `External` variant is a catch-all
/// for links that don't fit other semantic categories.
///
/// Note: Use the `open_in_new_tab` field on [`Link`](crate::decoration::Link)
/// to control whether links open in a new tab, regardless of their type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum LinkType {
    /// Documentation link (e.g., API docs, user guides).
    #[default]
    Documentation,
    /// Tutorial or how-to guide link.
    Tutorial,
    /// Video content link (for video hosting platforms).
    Video,
    /// General external link that doesn't fit other categories.
    ///
    /// Use this for links that aren't documentation, tutorials, videos, or API references.
    /// The `open_in_new_tab` field on the Link struct controls external behavior.
    External,
    /// API reference link (e.g., REST API docs, SDK reference).
    Api,
}

impl LinkType {
    /// Returns the name of this link type.
    #[inline]
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Documentation => "documentation",
            Self::Tutorial => "tutorial",
            Self::Video => "video",
            Self::External => "external",
            Self::Api => "api",
        }
    }
}

impl fmt::Display for LinkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_kind_variants() {
        assert_eq!(NodeKind::Group.name(), "group");
        assert_eq!(NodeKind::Layout.name(), "layout");
        assert_eq!(NodeKind::Decoration.name(), "decoration");
        assert_eq!(NodeKind::Container.name(), "container");
        assert_eq!(NodeKind::Leaf.name(), "leaf");
    }

    #[test]
    fn test_node_kind_has_own_value() {
        assert!(!NodeKind::Group.has_own_value());
        assert!(!NodeKind::Layout.has_own_value());
        assert!(!NodeKind::Decoration.has_own_value());
        assert!(NodeKind::Container.has_own_value());
        assert!(NodeKind::Leaf.has_own_value());
    }

    #[test]
    fn test_node_kind_has_value_access() {
        assert!(NodeKind::Group.has_value_access());
        assert!(NodeKind::Layout.has_value_access());
        assert!(!NodeKind::Decoration.has_value_access());
        assert!(NodeKind::Container.has_value_access());
        assert!(!NodeKind::Leaf.has_value_access());
    }

    #[test]
    fn test_node_kind_can_have_children() {
        assert!(NodeKind::Group.can_have_children());
        assert!(NodeKind::Layout.can_have_children());
        assert!(!NodeKind::Decoration.can_have_children());
        assert!(NodeKind::Container.can_have_children());
        assert!(!NodeKind::Leaf.can_have_children());
    }

    #[test]
    fn test_node_kind_display() {
        assert_eq!(format!("{}", NodeKind::Group), "group");
        assert_eq!(format!("{}", NodeKind::Container), "container");
    }

    #[test]
    fn test_notice_type_variants() {
        assert_eq!(NoticeType::Info.name(), "info");
        assert_eq!(NoticeType::Warning.name(), "warning");
        assert_eq!(NoticeType::Error.name(), "error");
        assert_eq!(NoticeType::Success.name(), "success");
        assert_eq!(NoticeType::Tip.name(), "tip");
    }

    #[test]
    fn test_notice_type_default() {
        assert_eq!(NoticeType::default(), NoticeType::Info);
    }

    #[test]
    fn test_separator_style_variants() {
        assert_eq!(SeparatorStyle::Thin.name(), "thin");
        assert_eq!(SeparatorStyle::Thick.name(), "thick");
        assert_eq!(SeparatorStyle::Dashed.name(), "dashed");
        assert_eq!(SeparatorStyle::Dotted.name(), "dotted");
        assert_eq!(SeparatorStyle::Space.name(), "space");
    }

    #[test]
    fn test_separator_style_default() {
        assert_eq!(SeparatorStyle::default(), SeparatorStyle::Thin);
    }

    #[test]
    fn test_link_type_variants() {
        assert_eq!(LinkType::Documentation.name(), "documentation");
        assert_eq!(LinkType::Tutorial.name(), "tutorial");
        assert_eq!(LinkType::Video.name(), "video");
        assert_eq!(LinkType::External.name(), "external");
        assert_eq!(LinkType::Api.name(), "api");
    }

    #[test]
    fn test_link_type_default() {
        assert_eq!(LinkType::default(), LinkType::Documentation);
    }
}
