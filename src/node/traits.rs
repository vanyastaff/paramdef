//! Core traits for the node system.

use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use crate::core::{Key, Metadata, Value};

use super::kind::NodeKind;

// =============================================================================
// Base Node Trait
// =============================================================================

/// Base trait for all node types.
///
/// Every node in the system implements this trait, which provides access to
/// common properties like metadata, key, and kind.
///
/// # Implementors
///
/// All 14 node types implement this trait:
/// - Group (1)
/// - Layout: Panel (1)
/// - Decoration: Notice (1)
/// - Container: Object, List, Mode, Routing, Expirable, Ref (6)
/// - Leaf: Text, Number, Boolean, Vector, Select (5)
pub trait Node: Send + Sync + Debug {
    /// Returns the node's metadata.
    fn metadata(&self) -> &Metadata;

    /// Returns the node's unique key.
    fn key(&self) -> &Key;

    /// Returns the node's kind (category).
    fn kind(&self) -> NodeKind;

    /// Returns a reference to the underlying type for downcasting.
    fn as_any(&self) -> &dyn Any;

    /// Returns a mutable reference to the underlying type for downcasting.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

// =============================================================================
// ValueAccess Trait (Runtime Only)
// =============================================================================

/// Trait for runtime nodes that can access child values.
///
/// This trait is for **runtime** use only, not for schema definitions.
/// Schema types (Group, Layout, Container) do NOT implement this trait.
/// It will be implemented by `RuntimeParameter<T>` and `Context`.
///
/// # Invariants
///
/// - Group and Layout have NO own value, only delegate to children
/// - Container has BOTH own value AND children
pub trait ValueAccess: Node {
    /// Collects all values from this node and its descendants.
    ///
    /// Returns a flat map of all key-value pairs in the subtree.
    fn collect_values(&self) -> HashMap<String, Value>;

    /// Gets a value by key from this node or its children.
    ///
    /// Returns `None` if the key is not found.
    fn get_value(&self, key: &str) -> Option<Value>;

    /// Sets a value by key in this node or its children.
    ///
    /// # Errors
    ///
    /// Returns an error if the key is not found or the value type is incompatible.
    fn set_value(&mut self, key: &str, value: Value) -> crate::core::Result<()>;
}

// =============================================================================
// GroupNode Trait
// =============================================================================

/// Trait for the Group node type (schema definition).
///
/// Group is the root aggregator that can contain Layout, Decoration,
/// Container, and Leaf nodes. This is a schema-only trait; runtime
/// value access is provided by `RuntimeParameter<T>` or `Context`.
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
/// use paramdef::node::{Node, Decoration, NodeKind};
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
pub trait Leaf: Node {
    /// Returns the default value for this parameter, if set.
    fn default_value(&self) -> Option<Value>;
}

// =============================================================================
// Visibility Trait (Feature-Gated)
// =============================================================================

/// Trait for visibility control.
///
/// All 14 node types implement this trait when the `visibility` feature is
/// enabled. Provides methods to evaluate conditional visibility based on
/// other parameter values.
#[cfg(feature = "visibility")]
pub trait Visibility: Node {
    /// Returns the visibility expression, if any.
    fn visibility_expr(&self) -> Option<&crate::core::Value>;

    /// Sets the visibility expression.
    fn set_visibility_expr(&mut self, expr: Option<crate::core::Value>);

    /// Returns whether the node is currently visible.
    ///
    /// If no visibility expression is set, returns `true`.
    fn is_visible(&self) -> bool {
        true
    }

    /// Returns the keys that this node's visibility depends on.
    fn dependencies(&self) -> Vec<String> {
        Vec::new()
    }
}

// =============================================================================
// Validatable Trait (Feature-Gated)
// =============================================================================

/// Trait for nodes that can be validated.
///
/// Implemented by Container and Leaf nodes (10 out of 14 types) when the
/// `validation` feature is enabled. Group, Layout, and Decoration do not
/// have values to validate.
///
/// # Future Extensions
///
/// - `validate_async`: Async validation will be added when `ValidationConfig`
///   is implemented. Requires either `async_trait` crate or RPITIT (Rust 1.75+).
/// - `validation()`: Returns `Option<&ValidationConfig>` - deferred until
///   `ValidationConfig` type is implemented in the validation feature phase.
#[cfg(feature = "validation")]
pub trait Validatable: Node {
    /// Validates a value synchronously.
    ///
    /// Runs all synchronous validators and returns the first error, if any.
    ///
    /// # Errors
    ///
    /// Returns a validation error if the value fails any validator.
    fn validate_sync(&self, value: &Value) -> crate::core::Result<()>;

    /// Returns the expected `NodeKind` for values.
    fn expected_kind(&self) -> NodeKind {
        self.kind()
    }

    /// Returns whether the given value is considered empty.
    ///
    /// Used for required field validation.
    fn is_empty(&self, value: &Value) -> bool {
        match value {
            Value::Null => true,
            Value::Text(s) => s.is_empty(),
            Value::Array(arr) => arr.is_empty(),
            Value::Object(obj) => obj.is_empty(),
            _ => false,
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Metadata;

    // Test implementation of Node for testing
    #[derive(Debug)]
    struct TestNode {
        metadata: Metadata,
        kind: NodeKind,
    }

    impl TestNode {
        fn new(key: &str, kind: NodeKind) -> Self {
            Self {
                metadata: Metadata::new(key),
                kind,
            }
        }
    }

    impl Node for TestNode {
        fn metadata(&self) -> &Metadata {
            &self.metadata
        }

        fn key(&self) -> &Key {
            self.metadata.key()
        }

        fn kind(&self) -> NodeKind {
            self.kind
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[test]
    fn test_node_trait_methods() {
        let node = TestNode::new("test_key", NodeKind::Leaf);

        assert_eq!(node.key().as_str(), "test_key");
        assert_eq!(node.kind(), NodeKind::Leaf);
        assert_eq!(node.metadata().key().as_str(), "test_key");
    }

    #[test]
    fn test_node_kind_group() {
        let node = TestNode::new("group", NodeKind::Group);
        assert_eq!(node.kind(), NodeKind::Group);
        assert!(!node.kind().has_own_value());
        assert!(node.kind().has_value_access());
    }

    #[test]
    fn test_node_kind_leaf() {
        let node = TestNode::new("leaf", NodeKind::Leaf);
        assert_eq!(node.kind(), NodeKind::Leaf);
        assert!(node.kind().has_own_value());
        assert!(!node.kind().has_value_access());
    }

    #[test]
    fn test_node_kind_container() {
        let node = TestNode::new("container", NodeKind::Container);
        assert_eq!(node.kind(), NodeKind::Container);
        assert!(node.kind().has_own_value());
        assert!(node.kind().has_value_access());
    }

    #[test]
    fn test_node_kind_decoration() {
        let node = TestNode::new("notice", NodeKind::Decoration);
        assert_eq!(node.kind(), NodeKind::Decoration);
        assert!(!node.kind().has_own_value());
        assert!(!node.kind().has_value_access());
    }

    // Test implementation of Leaf for testing
    #[derive(Debug)]
    struct TestLeaf {
        metadata: Metadata,
        default: Option<Value>,
    }

    impl TestLeaf {
        fn new(key: &str) -> Self {
            Self {
                metadata: Metadata::new(key),
                default: None,
            }
        }

        fn with_default(key: &str, default: Value) -> Self {
            Self {
                metadata: Metadata::new(key),
                default: Some(default),
            }
        }
    }

    impl Node for TestLeaf {
        fn metadata(&self) -> &Metadata {
            &self.metadata
        }

        fn key(&self) -> &Key {
            self.metadata.key()
        }

        fn kind(&self) -> NodeKind {
            NodeKind::Leaf
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    impl Leaf for TestLeaf {
        fn default_value(&self) -> Option<Value> {
            self.default.clone()
        }
    }

    #[test]
    fn test_leaf_trait() {
        let leaf = TestLeaf::new("test");
        assert!(leaf.default_value().is_none());

        let leaf_with_default = TestLeaf::with_default("test", Value::text("hello"));
        assert_eq!(
            leaf_with_default.default_value(),
            Some(Value::text("hello"))
        );
    }

    // Test implementation of Decoration for testing
    #[derive(Debug)]
    struct TestDecoration {
        metadata: Metadata,
        message: String,
    }

    impl TestDecoration {
        fn new(key: &str, message: &str) -> Self {
            Self {
                metadata: Metadata::new(key),
                message: message.to_string(),
            }
        }

        fn message(&self) -> &str {
            &self.message
        }
    }

    impl Node for TestDecoration {
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

    // Decoration is now a marker trait - no required methods
    impl Decoration for TestDecoration {}

    #[test]
    fn test_decoration_trait() {
        let decoration = TestDecoration::new("notice", "Hello world");

        // Decoration has correct NodeKind
        assert_eq!(decoration.kind(), NodeKind::Decoration);

        // Decoration invariants
        assert!(!decoration.kind().has_own_value());
        assert!(!decoration.kind().has_value_access());
        assert!(!decoration.kind().can_have_children());

        // Custom methods on concrete type
        assert_eq!(decoration.message(), "Hello world");
    }

    // Test ValueAccess with a simple implementation
    #[derive(Debug)]
    struct TestContainer {
        metadata: Metadata,
        values: HashMap<String, Value>,
    }

    impl TestContainer {
        fn new(key: &str) -> Self {
            Self {
                metadata: Metadata::new(key),
                values: HashMap::new(),
            }
        }
    }

    impl Node for TestContainer {
        fn metadata(&self) -> &Metadata {
            &self.metadata
        }

        fn key(&self) -> &Key {
            self.metadata.key()
        }

        fn kind(&self) -> NodeKind {
            NodeKind::Container
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    impl ValueAccess for TestContainer {
        fn collect_values(&self) -> HashMap<String, Value> {
            self.values.clone()
        }

        fn get_value(&self, key: &str) -> Option<Value> {
            self.values.get(key).cloned()
        }

        fn set_value(&mut self, key: &str, value: Value) -> crate::core::Result<()> {
            self.values.insert(key.to_string(), value);
            Ok(())
        }
    }

    #[test]
    fn test_value_access_collect() {
        let mut container = TestContainer::new("container");
        container.set_value("a", Value::Int(1)).unwrap();
        container.set_value("b", Value::Int(2)).unwrap();

        let values = container.collect_values();
        assert_eq!(values.len(), 2);
        assert_eq!(values.get("a"), Some(&Value::Int(1)));
        assert_eq!(values.get("b"), Some(&Value::Int(2)));
    }

    #[test]
    fn test_value_access_set_value() {
        let mut container = TestContainer::new("container");

        assert!(container.set_value("key", Value::text("value")).is_ok());
        assert_eq!(container.get_value("key"), Some(Value::text("value")));
    }
}

#[cfg(all(test, feature = "validation"))]
mod validation_tests {
    use super::*;
    use crate::core::Metadata;

    #[derive(Debug)]
    struct ValidatableLeaf {
        metadata: Metadata,
    }

    impl ValidatableLeaf {
        fn new(key: &str) -> Self {
            Self {
                metadata: Metadata::new(key),
            }
        }
    }

    impl Node for ValidatableLeaf {
        fn metadata(&self) -> &Metadata {
            &self.metadata
        }

        fn key(&self) -> &Key {
            self.metadata.key()
        }

        fn kind(&self) -> NodeKind {
            NodeKind::Leaf
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    impl Validatable for ValidatableLeaf {
        fn validate_sync(&self, _value: &Value) -> crate::core::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_validatable_trait() {
        let leaf = ValidatableLeaf::new("test");
        assert!(leaf.validate_sync(&Value::text("hello")).is_ok());
    }

    #[test]
    fn test_validatable_is_empty() {
        let leaf = ValidatableLeaf::new("test");

        assert!(leaf.is_empty(&Value::Null));
        assert!(leaf.is_empty(&Value::text("")));
        assert!(leaf.is_empty(&Value::array(vec![])));

        assert!(!leaf.is_empty(&Value::text("hello")));
        assert!(!leaf.is_empty(&Value::Int(42)));
        assert!(!leaf.is_empty(&Value::Bool(false)));
    }

    #[test]
    fn test_validatable_expected_kind() {
        let leaf = ValidatableLeaf::new("test");
        assert_eq!(leaf.expected_kind(), NodeKind::Leaf);
    }
}

#[cfg(all(test, feature = "visibility"))]
mod visibility_tests {
    use super::*;
    use crate::core::Metadata;

    #[derive(Debug)]
    struct VisibleNode {
        metadata: Metadata,
        visibility_expr: Option<Value>,
    }

    impl VisibleNode {
        fn new(key: &str) -> Self {
            Self {
                metadata: Metadata::new(key),
                visibility_expr: None,
            }
        }
    }

    impl Node for VisibleNode {
        fn metadata(&self) -> &Metadata {
            &self.metadata
        }

        fn key(&self) -> &Key {
            self.metadata.key()
        }

        fn kind(&self) -> NodeKind {
            NodeKind::Leaf
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    impl Visibility for VisibleNode {
        fn visibility_expr(&self) -> Option<&Value> {
            self.visibility_expr.as_ref()
        }

        fn set_visibility_expr(&mut self, expr: Option<Value>) {
            self.visibility_expr = expr;
        }
    }

    #[test]
    fn test_visibility_trait() {
        let mut node = VisibleNode::new("test");

        assert!(node.visibility_expr().is_none());
        assert!(node.is_visible());
        assert!(node.dependencies().is_empty());

        node.set_visibility_expr(Some(Value::Bool(true)));
        assert!(node.visibility_expr().is_some());
    }
}
