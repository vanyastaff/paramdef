//! Runtime node wrapper combining schema with mutable state.

use std::sync::Arc;

use crate::core::Value;
use crate::node::Node;

use super::State;

/// Runtime wrapper for a node combining immutable schema with mutable state.
///
/// `RuntimeNode` pairs an immutable node definition (shared via `Arc`) with
/// per-instance runtime state. This enables one schema definition to be used
/// across multiple contexts.
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
#[derive(Debug, Clone)]
pub struct RuntimeNode {
    /// Immutable node definition (shared).
    node: Arc<dyn Node>,
    /// Mutable runtime state.
    state: State,
    /// Current value.
    value: Option<Value>,
}

impl RuntimeNode {
    /// Creates a new runtime node wrapping the given schema node.
    #[must_use]
    pub fn new(node: Arc<dyn Node>) -> Self {
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
}
