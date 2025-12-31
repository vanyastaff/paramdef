//! Runtime node wrapper combining schema with mutable state.

use std::sync::Arc;

use crate::core::Value;
use crate::node::Node;

use super::State;

/// Runtime wrapper for a node combining immutable schema with mutable state.
///
/// `RuntimeNode<T>` pairs an immutable node definition (shared via `Arc`) with
/// per-instance runtime state. This enables one schema definition to be used
/// across multiple contexts.
///
/// The generic parameter `T` allows specialized implementations:
/// - `impl<T: Leaf> RuntimeNode<T>` - methods specific to leaf nodes
/// - `impl<T: Container> ValueAccess for RuntimeNode<T>` - value access for containers
///
/// # Example
///
/// ```
/// use paramdef::runtime::RuntimeNode;
/// use paramdef::parameter::Text;
/// use paramdef::core::Value;
/// use std::sync::Arc;
///
/// let schema_node = Arc::new(Text::builder("username").build());
/// let mut runtime = RuntimeNode::new(schema_node);
///
/// // Initially clean
/// assert!(!runtime.state().is_dirty());
///
/// // Set a value
/// runtime.set_value(Value::text("alice"));
/// assert!(runtime.state().is_dirty());
/// assert_eq!(runtime.value().and_then(|v| v.as_text()), Some("alice"));
/// ```
#[derive(Debug)]
pub struct RuntimeNode<T: Node> {
    /// Immutable node definition (shared).
    node: Arc<T>,
    /// Mutable runtime state.
    state: State,
    /// Current value.
    value: Option<Value>,
}

impl<T: Node> Clone for RuntimeNode<T> {
    fn clone(&self) -> Self {
        Self {
            node: Arc::clone(&self.node),
            state: self.state.clone(),
            value: self.value.clone(),
        }
    }
}

impl<T: Node> RuntimeNode<T> {
    /// Creates a new runtime node wrapping the given schema node.
    #[must_use]
    pub fn new(node: Arc<T>) -> Self {
        Self {
            node,
            state: State::new(),
            value: None,
        }
    }

    /// Returns a reference to the underlying schema node.
    #[must_use]
    pub fn node(&self) -> &Arc<T> {
        &self.node
    }

    /// Returns a reference to the runtime state.
    #[must_use]
    pub fn state(&self) -> &State {
        &self.state
    }

    /// Returns a mutable reference to the runtime state.
    #[must_use]
    pub fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }

    /// Returns the current value.
    #[must_use]
    pub fn value(&self) -> Option<&Value> {
        self.value.as_ref()
    }

    /// Sets the value and marks the state as dirty.
    pub fn set_value(&mut self, value: Value) {
        self.value = Some(value);
        self.state.mark_dirty();
    }

    /// Clears the value.
    pub fn clear_value(&mut self) {
        self.value = None;
        self.state.mark_dirty();
    }

    /// Resets the runtime node to its initial state.
    pub fn reset(&mut self) {
        self.value = None;
        self.state.reset();
    }
}

// =============================================================================
// Type-erased wrapper for heterogeneous storage
// =============================================================================

/// Type-erased runtime node for storage in heterogeneous collections.
///
/// This wrapper allows storing `RuntimeNode<T>` for different `T` types
/// in a single collection (e.g., `HashMap<Key, ErasedRuntimeNode>`).
#[derive(Debug, Clone)]
pub struct ErasedRuntimeNode {
    /// Immutable node definition (type-erased).
    node: Arc<dyn Node>,
    /// Mutable runtime state.
    state: State,
    /// Current value.
    value: Option<Value>,
}

impl ErasedRuntimeNode {
    /// Creates a new erased runtime node from a typed runtime node.
    #[must_use]
    pub fn new<T: Node + 'static>(runtime: RuntimeNode<T>) -> Self {
        Self {
            node: runtime.node,
            state: runtime.state,
            value: runtime.value,
        }
    }

    /// Creates a new erased runtime node from a type-erased schema node.
    #[must_use]
    pub fn from_arc(node: Arc<dyn Node>) -> Self {
        Self {
            node,
            state: State::new(),
            value: None,
        }
    }

    /// Returns a reference to the underlying schema node.
    #[must_use]
    pub fn node(&self) -> &Arc<dyn Node> {
        &self.node
    }

    /// Returns a reference to the runtime state.
    #[must_use]
    pub fn state(&self) -> &State {
        &self.state
    }

    /// Returns a mutable reference to the runtime state.
    #[must_use]
    pub fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }

    /// Returns the current value.
    #[must_use]
    pub fn value(&self) -> Option<&Value> {
        self.value.as_ref()
    }

    /// Sets the value and marks the state as dirty.
    pub fn set_value(&mut self, value: Value) {
        self.value = Some(value);
        self.state.mark_dirty();
    }

    /// Clears the value.
    pub fn clear_value(&mut self) {
        self.value = None;
        self.state.mark_dirty();
    }

    /// Resets the runtime node to its initial state.
    pub fn reset(&mut self) {
        self.value = None;
        self.state.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameter::Text;

    #[test]
    fn test_runtime_node_create() {
        let schema = Arc::new(Text::builder("name").build());
        let runtime = RuntimeNode::new(schema);

        assert!(!runtime.state().is_dirty());
        assert!(!runtime.state().is_touched());
        assert!(runtime.state().is_valid());
        assert!(runtime.value().is_none());
    }

    #[test]
    fn test_runtime_node_set_value() {
        let schema = Arc::new(Text::builder("name").build());
        let mut runtime = RuntimeNode::new(schema);

        runtime.set_value(Value::text("hello"));

        assert!(runtime.state().is_dirty());
        assert_eq!(runtime.value().and_then(|v| v.as_text()), Some("hello"));
    }

    #[test]
    fn test_runtime_node_clear_value() {
        let schema = Arc::new(Text::builder("name").build());
        let mut runtime = RuntimeNode::new(schema);
        runtime.set_value(Value::text("hello"));
        runtime.state_mut().mark_clean();

        runtime.clear_value();

        assert!(runtime.state().is_dirty());
        assert!(runtime.value().is_none());
    }

    #[test]
    fn test_runtime_node_reset() {
        let schema = Arc::new(Text::builder("name").build());
        let mut runtime = RuntimeNode::new(schema);
        runtime.set_value(Value::text("hello"));
        runtime.state_mut().mark_touched();

        runtime.reset();

        assert!(!runtime.state().is_dirty());
        assert!(!runtime.state().is_touched());
        assert!(runtime.value().is_none());
    }

    #[test]
    fn test_runtime_node_access_schema() {
        let schema = Arc::new(Text::builder("username").label("Username").build());
        let runtime = RuntimeNode::new(schema);

        assert_eq!(runtime.node().key().as_str(), "username");
        assert_eq!(runtime.node().metadata().label(), Some("Username"));
    }

    #[test]
    fn test_runtime_node_clone() {
        let schema = Arc::new(Text::builder("name").build());
        let mut runtime = RuntimeNode::new(schema);
        runtime.set_value(Value::text("hello"));

        let cloned = runtime.clone();

        assert_eq!(cloned.value().and_then(|v| v.as_text()), Some("hello"));
        assert!(cloned.state().is_dirty());
        // Arc should be shared
        assert!(Arc::ptr_eq(runtime.node(), cloned.node()));
    }

    #[test]
    fn test_erased_runtime_node_from_typed() {
        let schema = Arc::new(Text::builder("name").build());
        let mut typed = RuntimeNode::new(schema);
        typed.set_value(Value::text("hello"));

        let erased = ErasedRuntimeNode::new(typed);

        assert_eq!(erased.value().and_then(|v| v.as_text()), Some("hello"));
        assert!(erased.state().is_dirty());
    }

    #[test]
    fn test_erased_runtime_node_from_arc() {
        let schema: Arc<dyn Node> = Arc::new(Text::builder("name").build());
        let erased = ErasedRuntimeNode::from_arc(schema);

        assert!(!erased.state().is_dirty());
        assert!(erased.value().is_none());
    }

    #[test]
    fn test_erased_runtime_node_set_value() {
        let schema: Arc<dyn Node> = Arc::new(Text::builder("name").build());
        let mut erased = ErasedRuntimeNode::from_arc(schema);

        erased.set_value(Value::text("world"));

        assert!(erased.state().is_dirty());
        assert_eq!(erased.value().and_then(|v| v.as_text()), Some("world"));
    }
}
