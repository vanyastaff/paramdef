//! Core traits for the node system.

use std::collections::HashMap;
use std::sync::Arc;

use crate::core::{Key, Metadata, Value};

use super::kind::{DecorationType, NodeKind};

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
pub trait Node: Send + Sync {
    /// Returns the node's metadata.
    fn metadata(&self) -> &Metadata;

    /// Returns the node's unique key.
    fn key(&self) -> &Key;

    /// Returns the node's kind (category).
    fn kind(&self) -> NodeKind;
}

// =============================================================================
// ValueAccess Trait
// =============================================================================

/// Trait for nodes that can access child values.
///
/// Implemented by Group, Layout, and Container nodes. Provides methods to
/// collect values from children and set values by key path.
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
    /// Returns `true` if the value was set, `false` if key not found.
    fn set_value(&mut self, key: &str, value: Value) -> bool;

    /// Sets multiple values at once.
    ///
    /// Returns the number of values successfully set.
    fn set_values(&mut self, values: HashMap<String, Value>) -> usize {
        let mut count = 0;
        for (key, value) in values {
            if self.set_value(&key, value) {
                count += 1;
            }
        }
        count
    }
}

// =============================================================================
// GroupNode Trait
// =============================================================================

/// Trait for the Group node type.
///
/// Group is the root aggregator that can contain Layout, Decoration,
/// Container, and Leaf nodes. It provides `ValueAccess` but has no own value.
pub trait GroupNode: Node + ValueAccess {
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

/// Trait for the Layout node type (Panel).
///
/// Layout organizes UI elements without its own value. Can contain
/// Decoration, Container, and Leaf nodes (but NOT Group or other Layout).
pub trait Layout: Node + ValueAccess {
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

/// Trait for the Decoration node type (Notice).
///
/// Decoration is display-only with no value and no children.
/// Used for informational messages, warnings, and notices.
pub trait Decoration: Node {
    /// Returns the decoration type (Info, Warning, Error, Success).
    fn decoration_type(&self) -> DecorationType;

    /// Returns whether the decoration can be dismissed by the user.
    fn is_dismissible(&self) -> bool;

    /// Returns the message content.
    fn message(&self) -> &str;
}

// =============================================================================
// Container Trait
// =============================================================================

/// Trait for Container node types.
///
/// Container nodes have both their own value AND children. This includes:
/// - Object: Named fields
/// - List: Dynamic array
/// - Mode: Discriminated union
/// - Routing: Connection wrapper
/// - Expirable: TTL wrapper
/// - Ref: Reference to template
pub trait Container: Node + ValueAccess {
    /// Converts the container's data to a Value.
    fn to_value(&self) -> Value;

    /// Populates the container from a Value.
    ///
    /// # Errors
    ///
    /// Returns an error if the value structure doesn't match the expected format.
    fn set_from_value(&mut self, value: Value) -> crate::core::Result<()>;

    /// Returns all child nodes.
    fn children(&self) -> &[Arc<dyn Node>];
}

// =============================================================================
// Leaf Trait
// =============================================================================

/// Trait for Leaf node types.
///
/// Leaf nodes have a value but no children. This includes:
/// - Text: String values
/// - Number: Integer or float values
/// - Boolean: True/false values
/// - Vector: Fixed-size numeric arrays
/// - Select: Single or multiple selection
pub trait Leaf: Node {
    /// Converts the leaf's data to a Value.
    fn to_value(&self) -> Value;

    /// Sets the leaf's value from a Value.
    ///
    /// # Errors
    ///
    /// Returns an error if the value type doesn't match the expected type.
    fn set_from_value(&mut self, value: Value) -> crate::core::Result<()>;

    /// Returns whether the leaf has a value set.
    fn has_value(&self) -> bool;

    /// Clears the leaf's value.
    fn clear(&mut self);
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
    struct TestLeaf {
        metadata: Metadata,
        value: Option<Value>,
    }

    impl TestLeaf {
        fn new(key: &str) -> Self {
            Self {
                metadata: Metadata::new(key),
                value: None,
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
    }

    impl Leaf for TestLeaf {
        fn to_value(&self) -> Value {
            self.value.clone().unwrap_or(Value::Null)
        }

        fn set_from_value(&mut self, value: Value) -> crate::core::Result<()> {
            self.value = Some(value);
            Ok(())
        }

        fn has_value(&self) -> bool {
            self.value.is_some()
        }

        fn clear(&mut self) {
            self.value = None;
        }
    }

    #[test]
    fn test_leaf_trait() {
        let mut leaf = TestLeaf::new("test");

        assert!(!leaf.has_value());
        assert_eq!(leaf.to_value(), Value::Null);

        leaf.set_from_value(Value::text("hello")).unwrap();
        assert!(leaf.has_value());
        assert_eq!(leaf.to_value(), Value::text("hello"));

        leaf.clear();
        assert!(!leaf.has_value());
    }

    // Test implementation of Decoration for testing
    struct TestNotice {
        metadata: Metadata,
        decoration_type: DecorationType,
        dismissible: bool,
        message: String,
    }

    impl TestNotice {
        fn new(key: &str, message: &str) -> Self {
            Self {
                metadata: Metadata::new(key),
                decoration_type: DecorationType::Info,
                dismissible: false,
                message: message.to_string(),
            }
        }
    }

    impl Node for TestNotice {
        fn metadata(&self) -> &Metadata {
            &self.metadata
        }

        fn key(&self) -> &Key {
            self.metadata.key()
        }

        fn kind(&self) -> NodeKind {
            NodeKind::Decoration
        }
    }

    impl Decoration for TestNotice {
        fn decoration_type(&self) -> DecorationType {
            self.decoration_type
        }

        fn is_dismissible(&self) -> bool {
            self.dismissible
        }

        fn message(&self) -> &str {
            &self.message
        }
    }

    #[test]
    fn test_decoration_trait() {
        let notice = TestNotice::new("notice", "Hello world");

        assert_eq!(notice.kind(), NodeKind::Decoration);
        assert_eq!(notice.decoration_type(), DecorationType::Info);
        assert!(!notice.is_dismissible());
        assert_eq!(notice.message(), "Hello world");
    }

    // Test ValueAccess with a simple implementation
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
    }

    impl ValueAccess for TestContainer {
        fn collect_values(&self) -> HashMap<String, Value> {
            self.values.clone()
        }

        fn get_value(&self, key: &str) -> Option<Value> {
            self.values.get(key).cloned()
        }

        fn set_value(&mut self, key: &str, value: Value) -> bool {
            self.values.insert(key.to_string(), value);
            true
        }
    }

    #[test]
    fn test_value_access_collect() {
        let mut container = TestContainer::new("container");
        container.set_value("a", Value::Int(1));
        container.set_value("b", Value::Int(2));

        let values = container.collect_values();
        assert_eq!(values.len(), 2);
        assert_eq!(values.get("a"), Some(&Value::Int(1)));
        assert_eq!(values.get("b"), Some(&Value::Int(2)));
    }

    #[test]
    fn test_value_access_set_value() {
        let mut container = TestContainer::new("container");

        assert!(container.set_value("key", Value::text("value")));
        assert_eq!(container.get_value("key"), Some(Value::text("value")));
    }

    #[test]
    fn test_value_access_set_values() {
        let mut container = TestContainer::new("container");
        let mut values = HashMap::new();
        values.insert("a".to_string(), Value::Int(1));
        values.insert("b".to_string(), Value::Int(2));
        values.insert("c".to_string(), Value::Int(3));

        let count = container.set_values(values);
        assert_eq!(count, 3);
        assert_eq!(container.collect_values().len(), 3);
    }
}

#[cfg(all(test, feature = "validation"))]
mod validation_tests {
    use super::*;
    use crate::core::Metadata;

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
