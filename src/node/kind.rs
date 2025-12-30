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
    #[must_use]
    pub const fn has_own_value(&self) -> bool {
        matches!(self, Self::Container | Self::Leaf)
    }

    /// Returns whether this kind can access child values.
    #[must_use]
    pub const fn has_value_access(&self) -> bool {
        matches!(self, Self::Group | Self::Layout | Self::Container)
    }

    /// Returns whether this kind can contain children.
    #[must_use]
    pub const fn can_have_children(&self) -> bool {
        matches!(self, Self::Group | Self::Layout | Self::Container)
    }

    /// Returns the name of this kind.
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

/// The type of a decoration node.
///
/// Used by Notice to indicate the semantic meaning of the message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum DecorationType {
    /// Informational message (blue).
    #[default]
    Info,

    /// Warning message (yellow/orange).
    Warning,

    /// Error message (red).
    Error,

    /// Success message (green).
    Success,
}

impl DecorationType {
    /// Returns the name of this decoration type.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Success => "success",
        }
    }
}

impl fmt::Display for DecorationType {
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
    fn test_decoration_type_variants() {
        assert_eq!(DecorationType::Info.name(), "info");
        assert_eq!(DecorationType::Warning.name(), "warning");
        assert_eq!(DecorationType::Error.name(), "error");
        assert_eq!(DecorationType::Success.name(), "success");
    }

    #[test]
    fn test_decoration_type_default() {
        assert_eq!(DecorationType::default(), DecorationType::Info);
    }

    #[test]
    fn test_decoration_type_display() {
        assert_eq!(format!("{}", DecorationType::Warning), "warning");
        assert_eq!(format!("{}", DecorationType::Error), "error");
    }
}
