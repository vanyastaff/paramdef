//! Context for managing runtime parameter trees.
//!
//! Context combines a schema with runtime state for all parameters,
//! providing value storage, state tracking, and bulk operations.
//!
//! # Event System
//!
//! When the `events` feature is enabled, Context emits events for all
//! value and state changes. Subscribe to these events to build reactive
//! applications.
//!
//! ```ignore
//! use paramdef::context::Context;
//! use paramdef::event::{Event, EventBus};
//!
//! let bus = EventBus::new(64);
//! let mut ctx = Context::with_event_bus(schema, bus.clone());
//!
//! let mut sub = bus.subscribe();
//! ctx.set("name", Value::text("Alice"));
//!
//! // Subscriber receives ValueChanging and ValueChanged events
//! ```

use std::collections::HashMap;
use std::sync::Arc;

use crate::core::{FxHashMap, Key, Value};
use crate::runtime::ErasedRuntimeNode;
use crate::schema::Schema;
use rustc_hash::FxBuildHasher;

#[cfg(feature = "events")]
use crate::event::{Event, EventBus};

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
    /// Event bus for broadcasting changes (when `events` feature is enabled).
    #[cfg(feature = "events")]
    event_bus: Option<EventBus>,
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

        Self {
            schema,
            nodes,
            #[cfg(feature = "events")]
            event_bus: None,
        }
    }

    /// Creates a new context with an event bus.
    ///
    /// All value and state changes will be broadcast to subscribers.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use paramdef::context::Context;
    /// use paramdef::event::EventBus;
    ///
    /// let bus = EventBus::new(64);
    /// let ctx = Context::with_event_bus(schema, bus);
    /// ```
    #[cfg(feature = "events")]
    #[must_use]
    pub fn with_event_bus(schema: Arc<Schema>, event_bus: EventBus) -> Self {
        let mut ctx = Self::new(schema);
        ctx.event_bus = Some(event_bus);
        ctx
    }

    /// Returns a reference to the event bus, if configured.
    #[cfg(feature = "events")]
    #[must_use]
    pub fn event_bus(&self) -> Option<&EventBus> {
        self.event_bus.as_ref()
    }

    /// Sets the event bus for this context.
    ///
    /// Replaces any existing event bus.
    #[cfg(feature = "events")]
    pub fn set_event_bus(&mut self, event_bus: EventBus) {
        self.event_bus = Some(event_bus);
    }

    /// Removes the event bus from this context.
    ///
    /// Returns the removed event bus, if any.
    #[cfg(feature = "events")]
    pub fn take_event_bus(&mut self) -> Option<EventBus> {
        self.event_bus.take()
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
    /// # Errors
    ///
    /// Returns `Err(Error::NotFound)` if the parameter doesn't exist in the schema.
    ///
    /// When `events` feature is enabled, emits `ValueChanging` before
    /// and `ValueChanged` after the update.
    #[allow(clippy::needless_pass_by_value)]
    pub fn set(&mut self, key: &str, value: Value) -> crate::core::Result<()> {
        let node = self
            .nodes
            .get_mut(key)
            .ok_or_else(|| crate::core::Error::not_found(key))?;

        #[cfg(feature = "events")]
        let old_value = node.value().cloned();

        #[cfg(feature = "events")]
        if let Some(ref bus) = self.event_bus {
            bus.emit(Event::value_changing(key, old_value.clone(), value.clone()));
        }

        let was_dirty = node.state().is_dirty();
        node.set_value(value.clone());

        #[cfg(feature = "events")]
        if let Some(ref bus) = self.event_bus {
            bus.emit(Event::value_changed(key, old_value, value));

            // Emit Dirtied if this is the first dirty state
            if !was_dirty && node.state().is_dirty() {
                bus.emit(Event::dirtied(key));
            }
        }

        #[cfg(not(feature = "events"))]
        let _ = was_dirty; // Suppress unused warning

        Ok(())
    }

    /// Clears a value by key.
    ///
    /// # Errors
    ///
    /// Returns `Err(Error::NotFound)` if the parameter doesn't exist in the schema.
    ///
    /// When `events` feature is enabled, emits `ValueCleared`.
    pub fn clear(&mut self, key: &str) -> crate::core::Result<()> {
        let node = self
            .nodes
            .get_mut(key)
            .ok_or_else(|| crate::core::Error::not_found(key))?;

        #[cfg(feature = "events")]
        let old_value = node.value().cloned();

        node.clear_value();

        #[cfg(feature = "events")]
        if let (Some(bus), Some(old)) = (&self.event_bus, old_value) {
            bus.emit(Event::value_cleared(key, old));
        }

        Ok(())
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
    ///
    /// When `events` feature is enabled, emits `AllCleaned`.
    pub fn mark_all_clean(&mut self) {
        for node in self.nodes.values_mut() {
            node.state_mut().mark_clean();
        }

        #[cfg(feature = "events")]
        if let Some(ref bus) = self.event_bus {
            bus.emit(Event::AllCleaned);
        }
    }

    /// Resets all parameters to initial state.
    ///
    /// When `events` feature is enabled, emits `ContextReset`.
    pub fn reset(&mut self) {
        for node in self.nodes.values_mut() {
            node.reset();
        }

        #[cfg(feature = "events")]
        if let Some(ref bus) = self.event_bus {
            bus.emit(Event::ContextReset);
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

    // === Batch Operations (events feature) ===

    /// Begins a batch operation.
    ///
    /// Returns a batch ID that must be passed to [`end_batch`](Self::end_batch).
    /// Events emitted during a batch are grouped together.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let batch_id = ctx.begin_batch(Some("Update user"));
    /// ctx.set("name", Value::text("Alice"));
    /// ctx.set("email", Value::text("alice@example.com"));
    /// ctx.end_batch(batch_id);
    /// ```
    #[cfg(feature = "events")]
    pub fn begin_batch(
        &self,
        description: Option<impl Into<crate::core::SmartStr>>,
    ) -> Option<u64> {
        self.event_bus
            .as_ref()
            .map(|bus| bus.begin_batch(description))
    }

    /// Ends a batch operation.
    #[cfg(feature = "events")]
    pub fn end_batch(&self, id: u64) {
        if let Some(ref bus) = self.event_bus {
            bus.end_batch(id);
        }
    }

    /// Executes a closure within a batch.
    ///
    /// Automatically emits `BatchBegin` and `BatchEnd` events.
    ///
    /// # Example
    ///
    /// ```ignore
    /// ctx.batch(Some("Update profile"), |ctx| {
    ///     ctx.set("name", Value::text("Alice"));
    ///     ctx.set("age", Value::Int(30));
    /// });
    /// ```
    #[cfg(feature = "events")]
    pub fn batch<F, R>(&mut self, description: Option<impl Into<crate::core::SmartStr>>, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        let batch_id = self.begin_batch(description);
        let result = f(self);
        if let Some(id) = batch_id {
            self.end_batch(id);
        }
        result
    }

    /// Marks a parameter as touched.
    ///
    /// When `events` feature is enabled, emits `Touched`.
    /// # Errors
    ///
    /// Returns `Err(Error::NotFound)` if the parameter doesn't exist in the schema.
    pub fn touch(&mut self, key: &str) -> crate::core::Result<()> {
        let node = self
            .nodes
            .get_mut(key)
            .ok_or_else(|| crate::core::Error::not_found(key))?;

        #[cfg(feature = "events")]
        let was_touched = node.state().is_touched();

        node.state_mut().mark_touched();

        #[cfg(feature = "events")]
        if !was_touched && let Some(ref bus) = self.event_bus {
            bus.emit(Event::touched(key));
        }

        Ok(())
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

        assert!(result.is_ok());
        assert_eq!(ctx.get("name").and_then(|v| v.as_text()), Some("Alice"));
    }

    #[test]
    fn test_context_set_unknown_key() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);

        let result = ctx.set("unknown", Value::text("test"));

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            crate::core::Error::NotFound { .. }
        ));
    }

    #[test]
    fn test_context_clear_value() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);
        ctx.set("name", Value::text("Alice")).unwrap();

        ctx.clear("name").unwrap();

        assert!(ctx.get("name").is_none());
    }

    #[test]
    fn test_context_collect_values() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);
        ctx.set("name", Value::text("Alice")).unwrap();
        ctx.set("age", Value::Int(30)).unwrap();

        let values = ctx.collect_values();

        assert_eq!(values.len(), 2);
        assert_eq!(values.get("name").and_then(|v| v.as_text()), Some("Alice"));
        assert_eq!(values.get("age").and_then(|v| v.as_int()), Some(30));
    }

    #[test]
    fn test_context_collect_dirty_values() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);
        ctx.set("name", Value::text("Alice")).unwrap();
        ctx.set("age", Value::Int(30)).unwrap();
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

        ctx.set("name", Value::text("Alice")).unwrap();

        assert!(ctx.is_dirty());
    }

    #[test]
    fn test_context_mark_all_clean() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);
        ctx.set("name", Value::text("Alice")).unwrap();
        ctx.set("age", Value::Int(30)).unwrap();

        ctx.mark_all_clean();

        assert!(!ctx.is_dirty());
    }

    #[test]
    fn test_context_reset() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);
        ctx.set("name", Value::text("Alice")).unwrap();
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

    #[test]
    fn test_context_touch() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);

        assert!(!ctx.node("name").unwrap().state().is_touched());

        ctx.touch("name").unwrap();

        assert!(ctx.node("name").unwrap().state().is_touched());
    }

    #[test]
    fn test_context_touch_unknown_key() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);

        let result = ctx.touch("unknown");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            crate::core::Error::NotFound { .. }
        ));
    }
}

#[cfg(all(test, feature = "events"))]
mod event_tests {
    use super::*;
    use crate::event::{Event, EventBus};
    use crate::types::leaf::Text;

    fn create_test_schema() -> Arc<Schema> {
        Arc::new(
            Schema::builder()
                .parameter(Text::builder("name").build())
                .parameter(Text::builder("email").build())
                .build(),
        )
    }

    #[tokio::test]
    async fn test_context_with_event_bus() {
        let schema = create_test_schema();
        let bus = EventBus::new(64);
        let mut sub = bus.subscribe();

        let mut ctx = Context::with_event_bus(schema, bus);

        ctx.set("name", Value::text("Alice")).unwrap();

        // Should receive ValueChanging
        let event = sub.recv().await.unwrap();
        assert!(matches!(event, Event::ValueChanging { .. }));

        // Should receive ValueChanged
        let event = sub.recv().await.unwrap();
        assert!(matches!(event, Event::ValueChanged { .. }));

        // Should receive Dirtied
        let event = sub.recv().await.unwrap();
        assert!(matches!(event, Event::Dirtied { .. }));
    }

    #[tokio::test]
    async fn test_context_clear_emits_event() {
        let schema = create_test_schema();
        let bus = EventBus::new(64);
        let mut sub = bus.subscribe();

        let mut ctx = Context::with_event_bus(schema, bus);
        ctx.set("name", Value::text("Alice")).unwrap();

        // Drain set events
        while sub.try_recv().unwrap().is_some() {}

        ctx.clear("name").unwrap();

        let event = sub.recv().await.unwrap();
        assert!(matches!(event, Event::ValueCleared { .. }));
    }

    #[tokio::test]
    async fn test_context_reset_emits_event() {
        let schema = create_test_schema();
        let bus = EventBus::new(64);
        let mut sub = bus.subscribe();

        let mut ctx = Context::with_event_bus(schema, bus);

        ctx.reset();

        let event = sub.recv().await.unwrap();
        assert!(matches!(event, Event::ContextReset));
    }

    #[tokio::test]
    async fn test_context_mark_all_clean_emits_event() {
        let schema = create_test_schema();
        let bus = EventBus::new(64);
        let mut sub = bus.subscribe();

        let mut ctx = Context::with_event_bus(schema, bus);
        ctx.set("name", Value::text("Alice")).unwrap();

        // Drain set events
        while sub.try_recv().unwrap().is_some() {}

        ctx.mark_all_clean();

        let event = sub.recv().await.unwrap();
        assert!(matches!(event, Event::AllCleaned));
    }

    #[tokio::test]
    async fn test_context_touch_emits_event() {
        let schema = create_test_schema();
        let bus = EventBus::new(64);
        let mut sub = bus.subscribe();

        let mut ctx = Context::with_event_bus(schema, bus);

        ctx.touch("name").unwrap();

        let event = sub.recv().await.unwrap();
        assert!(matches!(event, Event::Touched { .. }));

        // Second touch should not emit (already touched)
        ctx.touch("name").unwrap();
        assert!(sub.try_recv().unwrap().is_none());
    }

    #[tokio::test]
    async fn test_context_batch() {
        let schema = create_test_schema();
        let bus = EventBus::new(64);
        let mut sub = bus.subscribe();

        let mut ctx = Context::with_event_bus(schema, bus);

        ctx.batch(Some("Update profile"), |ctx| {
            ctx.set("name", Value::text("Alice")).unwrap();
            ctx.set("email", Value::text("alice@example.com")).unwrap();
        });

        // Should receive: BatchBegin, ValueChanging, ValueChanged, Dirtied,
        // ValueChanging, ValueChanged, Dirtied, BatchEnd
        let mut events = Vec::new();
        while let Ok(Some(event)) = sub.try_recv() {
            events.push(event);
        }

        assert!(matches!(events.first(), Some(Event::BatchBegin { .. })));
        assert!(matches!(events.last(), Some(Event::BatchEnd { .. })));
    }

    #[test]
    fn test_context_event_bus_accessors() {
        let schema = create_test_schema();
        let bus = EventBus::new(64);

        let mut ctx = Context::with_event_bus(schema, bus);

        assert!(ctx.event_bus().is_some());

        let taken = ctx.take_event_bus();
        assert!(taken.is_some());
        assert!(ctx.event_bus().is_none());

        ctx.set_event_bus(EventBus::new(32));
        assert!(ctx.event_bus().is_some());
    }
}
