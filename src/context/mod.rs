//! Context for managing runtime parameter trees.
//!
//! Context combines a schema with runtime state for all parameters,
//! providing value storage, state tracking, and bulk operations.

use std::collections::HashMap;
use std::sync::Arc;

use crate::core::{FxHashMap, Key, Value};
use crate::runtime::ErasedRuntimeNode;
use crate::schema::Schema;
use rustc_hash::FxBuildHasher;

/// Runtime manager for a parameter tree.
///
/// Context instantiates runtime nodes for each parameter in a schema,
/// managing values and state. Multiple contexts can share the same schema.
///
/// # Example
///
/// ```
/// use paramdef::context::Context;
/// use paramdef::schema::Schema;
/// use paramdef::types::leaf::Text;
/// use paramdef::core::Value;
/// use std::sync::Arc;
///
/// let schema = Arc::new(Schema::builder()
///     .parameter(Text::builder("username").build())
///     .parameter(Text::builder("email").build())
///     .build());
///
/// let mut ctx = Context::new(schema);
///
/// ctx.set("username", Value::text("alice"));
/// assert_eq!(ctx.get("username").and_then(|v| v.as_text()), Some("alice"));
/// ```
#[derive(Debug)]
pub struct Context {
    /// Shared schema definition.
    schema: Arc<Schema>,
    /// Runtime nodes indexed by key.
    /// Uses `FxHashMap` for ~2x faster lookups with small keys.
    nodes: FxHashMap<Key, ErasedRuntimeNode>,
}

impl Context {
    /// Creates a new context from a schema.
    ///
    /// Instantiates a runtime node for each parameter in the schema.
    /// Pre-allocates the hash map with the exact capacity to avoid rehashing.
    #[must_use]
    pub fn new(schema: Arc<Schema>) -> Self {
        let mut nodes = FxHashMap::with_capacity_and_hasher(schema.len(), FxBuildHasher);

        for node in schema.iter() {
            let key = node.key().clone();
            nodes.insert(key, ErasedRuntimeNode::from_arc(Arc::clone(node)));
        }

        Self { schema, nodes }
    }

    /// Returns a reference to the schema.
    #[must_use]
    pub fn schema(&self) -> &Arc<Schema> {
        &self.schema
    }

    /// Returns the number of parameters.
    #[must_use]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns `true` if the context has no parameters.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Gets a value by key.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.nodes.get(key).and_then(|n| n.value())
    }

    /// Sets a value by key.
    ///
    /// Returns `true` if the parameter exists and was updated.
    pub fn set(&mut self, key: &str, value: Value) -> bool {
        if let Some(node) = self.nodes.get_mut(key) {
            node.set_value(value);
            true
        } else {
            false
        }
    }

    /// Clears a value by key.
    ///
    /// Returns `true` if the parameter exists.
    pub fn clear(&mut self, key: &str) -> bool {
        if let Some(node) = self.nodes.get_mut(key) {
            node.clear_value();
            true
        } else {
            false
        }
    }

    /// Returns a runtime node by key.
    #[must_use]
    pub fn node(&self, key: &str) -> Option<&ErasedRuntimeNode> {
        self.nodes.get(key)
    }

    /// Returns a mutable runtime node by key.
    #[must_use]
    pub fn node_mut(&mut self, key: &str) -> Option<&mut ErasedRuntimeNode> {
        self.nodes.get_mut(key)
    }

    /// Collects all values into a map.
    #[must_use]
    pub fn collect_values(&self) -> HashMap<Key, Value> {
        self.nodes
            .iter()
            .filter_map(|(k, n)| n.value().map(|v| (k.clone(), v.clone())))
            .collect()
    }

    /// Collects only dirty values into a map.
    #[must_use]
    pub fn collect_dirty_values(&self) -> HashMap<Key, Value> {
        self.nodes
            .iter()
            .filter(|(_, n)| n.state().is_dirty())
            .filter_map(|(k, n)| n.value().map(|v| (k.clone(), v.clone())))
            .collect()
    }

    /// Returns an iterator over dirty values without cloning.
    ///
    /// This is a zero-allocation alternative to [`collect_dirty_values()`](Self::collect_dirty_values)
    /// that returns references instead of owned values. Use this when you need to inspect
    /// dirty values without collecting them into a new map.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::context::Context;
    /// use paramdef::schema::Schema;
    /// use paramdef::types::leaf::Text;
    /// use paramdef::core::Value;
    /// use std::sync::Arc;
    ///
    /// let schema = Arc::new(Schema::builder()
    ///     .parameter(Text::builder("name").build())
    ///     .parameter(Text::builder("email").build())
    ///     .build());
    ///
    /// let mut ctx = Context::new(schema);
    /// ctx.set("name", Value::text("Alice"));
    ///
    /// for (key, value) in ctx.dirty_values() {
    ///     println!("{}: {:?}", key, value);
    /// }
    /// ```
    pub fn dirty_values(&self) -> impl Iterator<Item = (&Key, &Value)> + '_ {
        self.nodes
            .iter()
            .filter(|(_, n)| n.state().is_dirty())
            .filter_map(|(k, n)| n.value().map(|v| (k, v)))
    }

    /// Returns `true` if any parameter is dirty.
    #[must_use]
    pub fn is_dirty(&self) -> bool {
        self.nodes.values().any(|n| n.state().is_dirty())
    }

    /// Returns `true` if all parameters are valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.nodes.values().all(|n| n.state().is_valid())
    }

    /// Marks all parameters as clean.
    pub fn mark_all_clean(&mut self) {
        for node in self.nodes.values_mut() {
            node.state_mut().mark_clean();
        }
    }

    /// Resets all parameters to initial state.
    pub fn reset(&mut self) {
        for node in self.nodes.values_mut() {
            node.reset();
        }
    }

    /// Returns an iterator over all runtime nodes.
    pub fn iter(&self) -> impl Iterator<Item = (&Key, &ErasedRuntimeNode)> {
        self.nodes.iter()
    }

    /// Returns an iterator over all keys.
    pub fn keys(&self) -> impl Iterator<Item = &Key> {
        self.nodes.keys()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::leaf::{Number, Text};

    fn create_test_schema() -> Arc<Schema> {
        Arc::new(
            Schema::builder()
                .parameter(Text::builder("name").build())
                .parameter(Text::builder("email").build())
                .parameter(Number::builder("age").build())
                .build(),
        )
    }

    #[test]
    fn test_context_from_schema() {
        let schema = create_test_schema();
        let ctx = Context::new(schema);

        assert_eq!(ctx.len(), 3);
        assert!(!ctx.is_empty());
    }

    #[test]
    fn test_context_set_value() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);

        let result = ctx.set("name", Value::text("Alice"));

        assert!(result);
        assert_eq!(ctx.get("name").and_then(|v| v.as_text()), Some("Alice"));
    }

    #[test]
    fn test_context_set_unknown_key() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);

        let result = ctx.set("unknown", Value::text("test"));

        assert!(!result);
    }

    #[test]
    fn test_context_clear_value() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);
        ctx.set("name", Value::text("Alice"));

        ctx.clear("name");

        assert!(ctx.get("name").is_none());
    }

    #[test]
    fn test_context_collect_values() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);
        ctx.set("name", Value::text("Alice"));
        ctx.set("age", Value::Int(30));

        let values = ctx.collect_values();

        assert_eq!(values.len(), 2);
        assert_eq!(values.get("name").and_then(|v| v.as_text()), Some("Alice"));
        assert_eq!(values.get("age").and_then(|v| v.as_int()), Some(30));
    }

    #[test]
    fn test_context_collect_dirty_values() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);
        ctx.set("name", Value::text("Alice"));
        ctx.set("age", Value::Int(30));
        ctx.node_mut("name").unwrap().state_mut().mark_clean();

        let dirty = ctx.collect_dirty_values();

        assert_eq!(dirty.len(), 1);
        assert!(dirty.contains_key("age"));
    }

    #[test]
    fn test_context_is_dirty() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);

        assert!(!ctx.is_dirty());

        ctx.set("name", Value::text("Alice"));

        assert!(ctx.is_dirty());
    }

    #[test]
    fn test_context_mark_all_clean() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);
        ctx.set("name", Value::text("Alice"));
        ctx.set("age", Value::Int(30));

        ctx.mark_all_clean();

        assert!(!ctx.is_dirty());
    }

    #[test]
    fn test_context_reset() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);
        ctx.set("name", Value::text("Alice"));
        ctx.node_mut("name").unwrap().state_mut().mark_touched();

        ctx.reset();

        assert!(!ctx.is_dirty());
        assert!(ctx.get("name").is_none());
        assert!(!ctx.node("name").unwrap().state().is_touched());
    }

    #[test]
    fn test_context_node_access() {
        let schema = create_test_schema();
        let ctx = Context::new(schema);

        let node = ctx.node("name").unwrap();

        assert_eq!(node.node().key().as_str(), "name");
    }

    #[test]
    fn test_context_iter() {
        let schema = create_test_schema();
        let ctx = Context::new(schema);

        let keys: Vec<_> = ctx.keys().collect();

        assert_eq!(keys.len(), 3);
    }
}
