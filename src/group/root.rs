//! Group type - root aggregator.
//!
//! Group can contain Layout (Panel), Decoration (Notice), Container, and Leaf nodes.
//! This is a schema-only type; runtime value access is provided by `Context`.

use std::any::Any;
use std::fmt;
use std::sync::Arc;

use crate::core::{Flags, Key, Metadata, SmartStr};
use crate::node::{GroupNode, Node, NodeKind};

/// Layout style for a Group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GroupLayout {
    /// Vertical layout (default).
    #[default]
    Vertical,
    /// Horizontal layout.
    Horizontal,
    /// Grid layout.
    Grid,
    /// Tabbed layout (children are shown as tabs).
    Tabs,
}

impl GroupLayout {
    /// Returns the name of this layout.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
            Self::Grid => "grid",
            Self::Tabs => "tabs",
        }
    }
}

/// Root aggregator that can contain all node types.
///
/// Group is the only type that can contain Layout nodes (Panel).
/// It provides `ValueAccess` to collect values from all descendants,
/// but has no own value.
///
/// # Example
///
/// ```ignore
/// use paramdef::group::{Group, Panel};
/// use paramdef::parameter::{Text, Number};
///
/// let config = Group::builder("settings")
///     .label("Settings")
///     .child(Panel::builder("general")
///         .child(Text::builder("name").build())
///         .child(Number::int("port").build())
///         .build())
///     .build();
///
/// // Collect all values
/// let values = config.collect_values();
/// ```
#[derive(Clone)]
pub struct Group {
    metadata: Metadata,
    flags: Flags,
    children: Vec<Arc<dyn Node>>,
    layout: GroupLayout,
    collapsed: bool,
}

impl fmt::Debug for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Group")
            .field("metadata", &self.metadata)
            .field("flags", &self.flags)
            .field("child_count", &self.children.len())
            .field("layout", &self.layout)
            .field("collapsed", &self.collapsed)
            .finish()
    }
}

impl Group {
    /// Creates a new builder for a Group.
    #[must_use]
    pub fn builder(key: impl Into<Key>) -> GroupBuilder {
        GroupBuilder::new(key)
    }

    /// Returns the flags for this group.
    #[must_use]
    pub fn flags(&self) -> Flags {
        self.flags
    }

    /// Returns the layout style.
    #[must_use]
    pub fn layout(&self) -> GroupLayout {
        self.layout
    }

    /// Returns whether the group is collapsed.
    #[must_use]
    pub fn is_collapsed(&self) -> bool {
        self.collapsed
    }
}

impl Node for Group {
    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn key(&self) -> &Key {
        self.metadata.key()
    }

    fn kind(&self) -> NodeKind {
        NodeKind::Group
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl GroupNode for Group {
    fn children(&self) -> &[Arc<dyn Node>] {
        &self.children
    }
}

// =============================================================================
// Builder
// =============================================================================

/// Builder for [`Group`].
pub struct GroupBuilder {
    key: Key,
    label: Option<SmartStr>,
    description: Option<SmartStr>,
    flags: Flags,
    children: Vec<Arc<dyn Node>>,
    layout: GroupLayout,
    collapsed: bool,
}

impl fmt::Debug for GroupBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GroupBuilder")
            .field("key", &self.key)
            .field("label", &self.label)
            .field("description", &self.description)
            .field("flags", &self.flags)
            .field("child_count", &self.children.len())
            .field("layout", &self.layout)
            .field("collapsed", &self.collapsed)
            .finish()
    }
}

impl GroupBuilder {
    /// Creates a new builder with the given key.
    #[must_use]
    pub fn new(key: impl Into<Key>) -> Self {
        Self {
            key: key.into(),
            label: None,
            description: None,
            flags: Flags::empty(),
            children: Vec::new(),
            layout: GroupLayout::default(),
            collapsed: false,
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

    /// Adds a child node.
    #[must_use]
    pub fn child(mut self, node: impl Node + 'static) -> Self {
        self.children.push(Arc::new(node));
        self
    }

    /// Adds a child node with an already-wrapped Arc.
    #[must_use]
    pub fn child_arc(mut self, node: Arc<dyn Node>) -> Self {
        self.children.push(node);
        self
    }

    /// Sets the layout style.
    #[must_use]
    pub fn layout(mut self, layout: GroupLayout) -> Self {
        self.layout = layout;
        self
    }

    /// Sets whether the group is initially collapsed.
    #[must_use]
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    /// Builds the Group.
    #[must_use]
    pub fn build(self) -> Group {
        let mut metadata = Metadata::new(self.key);
        if let Some(label) = self.label {
            metadata = metadata.with_label(label);
        }
        if let Some(description) = self.description {
            metadata = metadata.with_description(description);
        }

        Group {
            metadata,
            flags: self.flags,
            children: self.children,
            layout: self.layout,
            collapsed: self.collapsed,
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameter::Text;

    #[test]
    fn test_group_basic() {
        let group = Group::builder("settings")
            .label("Settings")
            .description("Application settings")
            .build();

        assert_eq!(group.key().as_str(), "settings");
        assert_eq!(group.metadata().label(), Some("Settings"));
        assert_eq!(group.kind(), NodeKind::Group);
    }

    #[test]
    fn test_group_with_children() {
        let group = Group::builder("config")
            .child(Text::builder("name").build())
            .child(Text::builder("email").build())
            .build();

        assert_eq!(group.children().len(), 2);
    }

    #[test]
    fn test_group_layout() {
        let vertical = Group::builder("v").build();
        assert_eq!(vertical.layout(), GroupLayout::Vertical);

        let tabs = Group::builder("t").layout(GroupLayout::Tabs).build();
        assert_eq!(tabs.layout(), GroupLayout::Tabs);
    }

    #[test]
    fn test_group_collapsed() {
        let collapsed = Group::builder("c").collapsed(true).build();
        assert!(collapsed.is_collapsed());

        let expanded = Group::builder("e").build();
        assert!(!expanded.is_collapsed());
    }

    #[test]
    fn test_group_node_trait() {
        let group = Group::builder("g")
            .child(Text::builder("a").build())
            .build();

        assert!(!group.is_empty());
        assert_eq!(group.len(), 1);
    }

    #[test]
    fn test_group_layout_names() {
        assert_eq!(GroupLayout::Vertical.name(), "vertical");
        assert_eq!(GroupLayout::Horizontal.name(), "horizontal");
        assert_eq!(GroupLayout::Grid.name(), "grid");
        assert_eq!(GroupLayout::Tabs.name(), "tabs");
    }

    #[test]
    fn test_group_invariants() {
        let group = Group::builder("test").build();

        // Group has NO own value
        assert!(!group.kind().has_own_value());

        // Group HAS ValueAccess
        assert!(group.kind().has_value_access());

        // Group CAN have children
        assert!(group.kind().can_have_children());
    }
}
