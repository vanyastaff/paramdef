//! Panel type - UI organization layout.
//!
//! Panel organizes UI into sections or tabs. It can contain Container,
//! Leaf, and Decoration nodes, but NOT other Panels or Groups.
//! This is a schema-only type; runtime value access is provided by `Context`.

use std::any::Any;
use std::fmt;
use std::sync::Arc;

use crate::core::{Flags, Key, Metadata, SmartStr};
use crate::types::kind::NodeKind;
use crate::types::traits::{Layout, Node};

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
/// use paramdef::types::leaf::{Text, Number};
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
    #[cfg(feature = "visibility")]
    visibility: Option<crate::expr::Rule>,
}

impl fmt::Debug for Panel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("Panel");
        debug
            .field("metadata", &self.metadata)
            .field("flags", &self.flags)
            .field("child_count", &self.children.len())
            .field("display_type", &self.display_type);

        #[cfg(feature = "visibility")]
        debug.field("visibility", &self.visibility);

        debug.finish()
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

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Layout for Panel {
    fn children(&self) -> &[Arc<dyn Node>] {
        &self.children
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
    #[cfg(feature = "visibility")]
    visibility: Option<crate::expr::Rule>,
}

impl fmt::Debug for PanelBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("PanelBuilder");
        debug
            .field("key", &self.key)
            .field("label", &self.label)
            .field("description", &self.description)
            .field("flags", &self.flags)
            .field("child_count", &self.children.len())
            .field("display_type", &self.display_type)
            .field("collapsed", &self.collapsed);

        #[cfg(feature = "visibility")]
        debug.field("visibility", &self.visibility);

        debug.finish()
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
            #[cfg(feature = "visibility")]
            visibility: None,
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
    /// # Panics (Debug Mode Only)
    ///
    /// In debug builds, panics if the child is a Panel (Layout) or Group node,
    /// as these cannot be nested inside a Panel. This check is compiled away in release builds.
    #[must_use]
    pub fn child_arc(mut self, node: Arc<dyn Node>) -> Self {
        Self::validate_child(&node);
        self.children.push(node);
        self
    }

    /// Validates that a child node is allowed inside a Panel.
    ///
    /// # Panics (Debug Mode Only)
    ///
    /// In debug builds, panics if the node is a Layout (Panel) or Group.
    /// This is a design-time invariant check that is compiled away in release builds.
    fn validate_child(node: &Arc<dyn Node>) {
        debug_assert!(
            !matches!(node.kind(), NodeKind::Layout),
            "Panel cannot contain Layout (Panel) nodes: '{}'",
            node.key()
        );
        debug_assert!(
            !matches!(node.kind(), NodeKind::Group),
            "Panel cannot contain Group nodes: '{}'",
            node.key()
        );
    }

    /// Sets the display type.
    #[must_use]
    pub fn display_type(mut self, display_type: PanelDisplayType) -> Self {
        self.display_type = display_type;
        self
    }

    /// Sets the initial collapsed state hint for this panel.
    ///
    /// **Important**: This is an initial UI state hint that should be used
    /// during Context initialization. The actual runtime collapsed state
    /// is managed by `Context::ui_state()`, not in the Panel schema itself.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::types::group::Panel;
    /// use paramdef::schema::Schema;
    /// use paramdef::context::Context;
    /// use std::sync::Arc;
    ///
    /// // Set initial state hint during panel construction
    /// let panel = Panel::builder("settings")
    ///     .collapsed(true)  // Hint: initially collapsed
    ///     .build();
    ///
    /// let schema = Arc::new(Schema::builder().parameter(panel).build());
    /// let mut ctx = Context::new(schema);
    ///
    /// // Runtime state is managed through Context
    /// ctx.set_panel_collapsed("settings", false);  // Now expanded
    /// assert!(!ctx.is_panel_collapsed(&"settings".into()));
    /// ```
    #[must_use]
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    /// Sets a visibility condition.
    ///
    /// The parameter will only be visible when the expression evaluates to true.
    #[cfg(feature = "visibility")]
    #[must_use]
    pub fn visible_when(mut self, rule: crate::expr::Rule) -> Self {
        self.visibility = Some(rule);
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
            #[cfg(feature = "visibility")]
            visibility: self.visibility,
        }
    }
}

// Visibility trait implementation
#[cfg(feature = "visibility")]
impl crate::types::traits::Visibility for Panel {
    fn visibility_rule(&self) -> Option<&crate::expr::Rule> {
        self.visibility.as_ref()
    }

    fn set_visibility_rule(&mut self, expr: Option<crate::expr::Rule>) {
        self.visibility = expr;
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
    fn test_panel_collapsed_moved_to_context() {
        // Collapsed state is now managed by Context/UiStateManager
        // This test verifies Panel no longer has mutable state
        use crate::context::Context;
        use crate::schema::Schema;
        use std::sync::Arc;

        let panel = Panel::builder("p")
            .display_type(PanelDisplayType::Collapsible)
            .collapsed(true) // This is now just an initial state hint
            .build();

        // Panel itself has no collapsed state
        // State is managed through Context
        let schema = Arc::new(Schema::builder().parameter(panel).build());
        let mut ctx = Context::new(schema);

        // Set collapsed state via Context
        ctx.set_panel_collapsed("p", true);
        assert!(ctx.is_panel_collapsed(&"p".into()));

        ctx.set_panel_collapsed("p", false);
        assert!(!ctx.is_panel_collapsed(&"p".into()));
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
