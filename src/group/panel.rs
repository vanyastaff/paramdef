//! Panel type - UI organization layout.
//!
//! Panel organizes UI into sections or tabs. It can contain Container,
//! Leaf, and Decoration nodes, but NOT other Panels or Groups.
//! This is a schema-only type; runtime value access is provided by `Context`.

use std::any::Any;
use std::fmt;
use std::sync::Arc;

use crate::core::{Flags, Key, Metadata, SmartStr};
use crate::node::{Layout, Node, NodeKind};

/// Display type for a Panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PanelDisplayType {
    /// Standard section with header.
    #[default]
    Section,
    /// Collapsible section.
    Collapsible,
    /// Tab in a tabbed interface.
    Tab,
    /// Card-style container.
    Card,
    /// Inline group without visual boundaries.
    Inline,
}

impl PanelDisplayType {
    /// Returns the name of this display type.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Section => "section",
            Self::Collapsible => "collapsible",
            Self::Tab => "tab",
            Self::Card => "card",
            Self::Inline => "inline",
        }
    }
}

/// Layout for UI organization.
///
/// Panel organizes UI elements into sections, tabs, or cards.
/// It provides `ValueAccess` but has no own value.
///
/// # Restrictions
///
/// Panel can contain:
/// - Container nodes (Object, List, Mode, etc.)
/// - Leaf nodes (Text, Number, Boolean, etc.)
/// - Decoration nodes (Notice)
///
/// Panel CANNOT contain:
/// - Other Panel nodes
/// - Group nodes
///
/// # Example
///
/// ```ignore
/// use paramdef::group::{Panel, Notice};
/// use paramdef::parameter::{Text, Number};
///
/// let database = Panel::builder("database")
///     .label("Database Settings")
///     .display_type(PanelDisplayType::Collapsible)
///     .child(Text::builder("host").required().build())
///     .child(Number::int("port").default(5432).build())
///     .child(Text::builder("database").required().build())
///     .build();
/// ```
#[derive(Clone)]
pub struct Panel {
    metadata: Metadata,
    flags: Flags,
    children: Vec<Arc<dyn Node>>,
    display_type: PanelDisplayType,
    collapsed: bool,
}

impl fmt::Debug for Panel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Panel")
            .field("metadata", &self.metadata)
            .field("flags", &self.flags)
            .field("child_count", &self.children.len())
            .field("display_type", &self.display_type)
            .field("collapsed", &self.collapsed)
            .finish()
    }
}

impl Panel {
    /// Creates a new builder for a Panel.
    #[must_use]
    pub fn builder(key: impl Into<Key>) -> PanelBuilder {
        PanelBuilder::new(key)
    }

    /// Returns the flags for this panel.
    #[must_use]
    pub fn flags(&self) -> Flags {
        self.flags
    }

    /// Returns the display type.
    #[must_use]
    pub fn display_type(&self) -> PanelDisplayType {
        self.display_type
    }
}

impl Node for Panel {
    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn key(&self) -> &Key {
        self.metadata.key()
    }

    fn kind(&self) -> NodeKind {
        NodeKind::Layout
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Layout for Panel {
    fn children(&self) -> &[Arc<dyn Node>] {
        &self.children
    }

    fn is_collapsed(&self) -> bool {
        self.collapsed
    }

    fn set_collapsed(&mut self, collapsed: bool) {
        self.collapsed = collapsed;
    }
}

// =============================================================================
// Builder
// =============================================================================

/// Builder for [`Panel`].
pub struct PanelBuilder {
    key: Key,
    label: Option<SmartStr>,
    description: Option<SmartStr>,
    flags: Flags,
    children: Vec<Arc<dyn Node>>,
    display_type: PanelDisplayType,
    collapsed: bool,
}

impl fmt::Debug for PanelBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PanelBuilder")
            .field("key", &self.key)
            .field("label", &self.label)
            .field("description", &self.description)
            .field("flags", &self.flags)
            .field("child_count", &self.children.len())
            .field("display_type", &self.display_type)
            .field("collapsed", &self.collapsed)
            .finish()
    }
}

impl PanelBuilder {
    /// Creates a new builder with the given key.
    #[must_use]
    pub fn new(key: impl Into<Key>) -> Self {
        Self {
            key: key.into(),
            label: None,
            description: None,
            flags: Flags::empty(),
            children: Vec::new(),
            display_type: PanelDisplayType::default(),
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
    ///
    /// # Panics
    ///
    /// Panics if the child is a Panel (Layout) or Group node,
    /// as these cannot be nested inside a Panel.
    #[must_use]
    pub fn child(mut self, node: impl Node + 'static) -> Self {
        let arc_node: Arc<dyn Node> = Arc::new(node);
        Self::validate_child(&arc_node);
        self.children.push(arc_node);
        self
    }

    /// Adds a child node with an already-wrapped Arc.
    ///
    /// # Panics
    ///
    /// Panics if the child is a Panel (Layout) or Group node,
    /// as these cannot be nested inside a Panel.
    #[must_use]
    pub fn child_arc(mut self, node: Arc<dyn Node>) -> Self {
        Self::validate_child(&node);
        self.children.push(node);
        self
    }

    /// Validates that a child node is allowed inside a Panel.
    ///
    /// # Panics
    ///
    /// Panics if the node is a Layout (Panel) or Group.
    fn validate_child(node: &Arc<dyn Node>) {
        match node.kind() {
            NodeKind::Layout => {
                panic!(
                    "Panel cannot contain Layout (Panel) nodes: '{}'",
                    node.key()
                );
            }
            NodeKind::Group => {
                panic!("Panel cannot contain Group nodes: '{}'", node.key());
            }
            _ => {}
        }
    }

    /// Sets the display type.
    #[must_use]
    pub fn display_type(mut self, display_type: PanelDisplayType) -> Self {
        self.display_type = display_type;
        self
    }

    /// Sets whether the panel is initially collapsed.
    #[must_use]
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    /// Builds the Panel.
    #[must_use]
    pub fn build(self) -> Panel {
        let mut metadata = Metadata::new(self.key);
        if let Some(label) = self.label {
            metadata = metadata.with_label(label);
        }
        if let Some(description) = self.description {
            metadata = metadata.with_description(description);
        }

        Panel {
            metadata,
            flags: self.flags,
            children: self.children,
            display_type: self.display_type,
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
    fn test_panel_basic() {
        let panel = Panel::builder("database")
            .label("Database Settings")
            .build();

        assert_eq!(panel.key().as_str(), "database");
        assert_eq!(panel.metadata().label(), Some("Database Settings"));
        assert_eq!(panel.kind(), NodeKind::Layout);
    }

    #[test]
    fn test_panel_with_children() {
        let panel = Panel::builder("settings")
            .child(Text::builder("host").build())
            .child(Text::builder("port").build())
            .build();

        assert_eq!(panel.children().len(), 2);
    }

    #[test]
    fn test_panel_display_type() {
        let section = Panel::builder("s").build();
        assert_eq!(section.display_type(), PanelDisplayType::Section);

        let card = Panel::builder("c")
            .display_type(PanelDisplayType::Card)
            .build();
        assert_eq!(card.display_type(), PanelDisplayType::Card);
    }

    #[test]
    fn test_panel_collapsed() {
        let mut panel = Panel::builder("p")
            .display_type(PanelDisplayType::Collapsible)
            .collapsed(true)
            .build();

        assert!(panel.is_collapsed());

        panel.set_collapsed(false);
        assert!(!panel.is_collapsed());
    }

    #[test]
    fn test_panel_display_type_names() {
        assert_eq!(PanelDisplayType::Section.name(), "section");
        assert_eq!(PanelDisplayType::Collapsible.name(), "collapsible");
        assert_eq!(PanelDisplayType::Tab.name(), "tab");
        assert_eq!(PanelDisplayType::Card.name(), "card");
        assert_eq!(PanelDisplayType::Inline.name(), "inline");
    }

    #[test]
    fn test_panel_invariants() {
        let panel = Panel::builder("test").build();

        // Panel has NO own value
        assert!(!panel.kind().has_own_value());

        // Panel HAS ValueAccess
        assert!(panel.kind().has_value_access());

        // Panel CAN have children
        assert!(panel.kind().can_have_children());
    }

    #[test]
    #[should_panic(expected = "Panel cannot contain Layout (Panel) nodes")]
    fn test_panel_cannot_contain_panel() {
        let inner = Panel::builder("inner").build();
        let _ = Panel::builder("outer").child(inner).build();
    }
}
