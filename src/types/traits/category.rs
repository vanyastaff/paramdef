//! Category-specific traits for node types.

use std::sync::Arc;

use crate::core::Value;
use crate::types::traits::Node;

// =============================================================================
// GroupNode Trait
// =============================================================================

/// Trait for the Group node type (schema definition).
///
/// Group is the root aggregator that can contain Layout, Decoration,
/// Container, and Leaf nodes. This is a schema-only trait; runtime
/// value access is provided by `RuntimeParameter<T>` or `Context`.
///
/// # Example
///
/// ```
/// use paramdef::types::traits::GroupNode;
/// use paramdef::types::group::Group;
///
/// let group = Group::builder("settings").build();
/// assert_eq!(group.len(), 0);
/// ```
pub trait GroupNode: Node {
    /// Returns all child nodes.
    fn children(&self) -> &[Arc<dyn Node>];

    /// Returns the number of children.
    fn len(&self) -> usize {
        self.children().len()
    }

    /// Returns whether the group has no children.
    fn is_empty(&self) -> bool {
        self.children().is_empty()
    }
}

// =============================================================================
// Layout Trait
// =============================================================================

/// Trait for the Layout node type (Panel) - schema definition.
///
/// Layout organizes UI elements without its own value. Can contain
/// Decoration, Container, and Leaf nodes (but NOT Group or other Layout).
/// This is a schema-only trait; runtime value access is provided by
/// `RuntimeParameter<T>` or `Context`.
///
/// # Example
///
/// ```
/// use paramdef::types::traits::Layout;
/// use paramdef::types::group::Panel;
///
/// let panel = Panel::builder("advanced").build();
/// assert!(!panel.is_collapsed());
/// ```
pub trait Layout: Node {
    /// Returns all child nodes.
    fn children(&self) -> &[Arc<dyn Node>];

    /// Returns the layout's UI state (collapsed, expanded, etc.).
    fn is_collapsed(&self) -> bool;

    /// Sets the collapsed state.
    fn set_collapsed(&mut self, collapsed: bool);
}

// =============================================================================
// Decoration Trait
// =============================================================================

/// Marker trait for Decoration node types.
///
/// Decorations are display-only with no value and no children.
/// This is an open trait - anyone can implement their own decoration types.
///
/// # Invariants
///
/// All types implementing `Decoration` MUST satisfy these invariants:
///
/// 1. **No own value**: Decorations have no runtime value and don't participate
///    in value serialization. They exist purely for UI/display purposes.
///
/// 2. **No children**: Decorations cannot contain other nodes. They are always
///    leaf elements in the node hierarchy (though distinct from `Leaf` nodes
///    which do have values).
///
/// 3. **`NodeKind::Decoration`**: The `kind()` method MUST return
///    `NodeKind::Decoration`. This is verified by the test suite.
///
/// 4. **Immutable content**: Once constructed, decoration content should be
///    treated as immutable. UI state (like collapsed panels) is separate.
///
/// # Built-in Decorations
///
/// - `Notice` - Info, warning, error, success messages
/// - `Separator` - Visual dividers between sections
/// - `Link` - Clickable references to docs/external resources
/// - `Code` - Syntax-highlighted code snippets
/// - `Image` - Static image display
///
/// # Custom Decorations
///
/// You can create your own decoration types by implementing this trait:
///
/// ```ignore
/// use paramdef::types::traits::{Node, Decoration};
/// use paramdef::types::kind::NodeKind;
/// use paramdef::core::Metadata;
///
/// pub struct MyBadge {
///     metadata: Metadata,
///     text: String,
///     color: String,
/// }
///
/// impl Node for MyBadge {
///     fn kind(&self) -> NodeKind { NodeKind::Decoration }
///     // ... other Node methods
/// }
///
/// impl Decoration for MyBadge {
///     // Marker trait - no required methods
/// }
/// ```
pub trait Decoration: Node {
    // Marker trait - no required methods.
    // Each decoration type defines its own specific interface.
}

// =============================================================================
// Container Trait
// =============================================================================

/// Trait for Container node types (schema definition).
///
/// Container nodes have both their own value AND children. This includes:
/// - Object: Named fields
/// - List: Dynamic array
/// - Mode: Discriminated union
/// - Routing: Connection wrapper
/// - Expirable: TTL wrapper
/// - Ref: Reference to template
///
/// This trait defines the **schema** for container parameters. Runtime values
/// are managed separately in `RuntimeParameter<T>` or `Context`.
///
/// # Example
///
/// ```ignore
/// use paramdef::types::traits::Container;
/// use paramdef::types::container::Object;
/// use paramdef::types::leaf::Text;
///
/// let obj = Object::builder("user")
///     .field("name", Text::builder("name").build())
///     .field("email", Text::builder("email").build())
///     .build();
///
/// assert_eq!(obj.children().len(), 2);
/// ```
pub trait Container: Node {
    /// Returns all child nodes.
    fn children(&self) -> &[Arc<dyn Node>];
}

// =============================================================================
// Leaf Trait
// =============================================================================

/// Trait for Leaf node types (schema definition).
///
/// Leaf nodes represent terminal values with no children. This includes:
/// - Text: String values
/// - Number: Integer or float values
/// - Boolean: True/false values
/// - Vector: Fixed-size numeric arrays
/// - Select: Single or multiple selection
///
/// This trait defines the **schema** for leaf parameters. Runtime values
/// are managed separately in `RuntimeParameter<T>` or `Context`.
///
/// # Example
///
/// ```
/// use paramdef::types::traits::Leaf;
/// use paramdef::types::leaf::Text;
/// use paramdef::core::Value;
///
/// let text = Text::builder("username").default("guest").build();
/// assert_eq!(text.default_value(), Some(Value::text("guest")));
/// ```
pub trait Leaf: Node {
    /// Returns the default value for this parameter, if set.
    fn default_value(&self) -> Option<Value>;
}
