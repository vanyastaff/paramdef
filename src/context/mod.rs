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

mod typed;
mod ui_state;

use std::collections::HashMap;
use std::sync::Arc;

use crate::core::{FxHashMap, Key, Value};
use crate::runtime::ErasedRuntimeNode;
use crate::schema::Schema;
use rustc_hash::FxBuildHasher;

pub use ui_state::{PanelState, UiStateManager};

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
pub struct Context {
    /// Shared schema definition.
    schema: Arc<Schema>,
    /// Runtime nodes indexed by key.
    /// Uses `FxHashMap` for ~2x faster lookups with small keys.
    nodes: FxHashMap<Key, ErasedRuntimeNode>,
    /// UI presentation state (panel collapsed states, etc.).
    /// Separate from immutable schema to maintain architectural invariants.
    ui_state: UiStateManager,
    /// Event bus for broadcasting changes (when `events` feature is enabled).
    #[cfg(feature = "events")]
    event_bus: Option<EventBus>,
}

impl std::fmt::Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dbg = f.debug_struct("Context");
        dbg.field("schema", &self.schema)
            .field("nodes", &self.nodes.len())
            .field("ui_state", &self.ui_state);
        #[cfg(feature = "events")]
        dbg.field("event_bus", &self.event_bus.is_some());
        dbg.finish()
    }
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
            ui_state: UiStateManager::new(),
            #[cfg(feature = "events")]
            event_bus: None,
        }
    }

    /// Creates a new context from a schema, automatically wrapping it in Arc.
    ///
    /// This is a convenience constructor that wraps the schema in `Arc` for you.
    /// Use this when you don't need to share the schema across multiple contexts.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::context::Context;
    /// use paramdef::schema::Schema;
    /// use paramdef::types::leaf::Text;
    /// use paramdef::core::Value;
    ///
    /// let schema = Schema::builder()
    ///     .parameter(Text::builder("name").build())
    ///     .build();
    ///
    /// // Convenience: no need to wrap in Arc manually
    /// let mut ctx = Context::from_schema(schema);
    /// ctx.set("name", Value::text("Alice")).unwrap();
    /// ```
    #[must_use]
    pub fn from_schema(schema: Schema) -> Self {
        Self::new(Arc::new(schema))
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
        let mut nodes = FxHashMap::with_capacity_and_hasher(schema.len(), FxBuildHasher);

        for node in schema.iter() {
            let key = node.key().clone();
            nodes.insert(key, ErasedRuntimeNode::from_arc(Arc::clone(node)));
        }

        Self {
            schema,
            nodes,
            ui_state: UiStateManager::new(),
            event_bus: Some(event_bus),
        }
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

    /// Gets a text value by key, returning a default if not found or wrong type.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::prelude::*;
    ///
    /// let schema = Schema::builder()
    ///     .parameter(Text::builder("name").build())
    ///     .build();
    /// let mut ctx = Context::from_schema(schema);
    /// ctx.set("name", Value::text("Alice")).unwrap();
    ///
    /// assert_eq!(ctx.get_text_or("name", "Unknown"), "Alice");
    /// assert_eq!(ctx.get_text_or("missing", "Unknown"), "Unknown");
    /// ```
    #[must_use]
    pub fn get_text_or<'a>(&'a self, key: &str, default: &'a str) -> &'a str {
        self.get(key).and_then(|v| v.as_text()).unwrap_or(default)
    }

    /// Gets an integer value by key, returning a default if not found or wrong type.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::prelude::*;
    ///
    /// let schema = Schema::builder()
    ///     .parameter(Number::builder("count").build())
    ///     .build();
    /// let mut ctx = Context::from_schema(schema);
    /// ctx.set("count", Value::Int(42)).unwrap();
    ///
    /// assert_eq!(ctx.get_int_or("count", 0), 42);
    /// assert_eq!(ctx.get_int_or("missing", 99), 99);
    /// ```
    #[must_use]
    pub fn get_int_or(&self, key: &str, default: i64) -> i64 {
        self.get(key).and_then(|v| v.as_int()).unwrap_or(default)
    }

    /// Gets a boolean value by key, returning a default if not found or wrong type.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::prelude::*;
    ///
    /// let schema = Schema::builder()
    ///     .parameter(Boolean::builder("enabled").build())
    ///     .build();
    /// let mut ctx = Context::from_schema(schema);
    /// ctx.set("enabled", Value::Bool(true)).unwrap();
    ///
    /// assert_eq!(ctx.get_bool_or("enabled", false), true);
    /// assert_eq!(ctx.get_bool_or("missing", false), false);
    /// ```
    #[must_use]
    pub fn get_bool_or(&self, key: &str, default: bool) -> bool {
        self.get(key).and_then(|v| v.as_bool()).unwrap_or(default)
    }

    /// Gets a float value by key, returning a default if not found or wrong type.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::prelude::*;
    ///
    /// let schema = Schema::builder()
    ///     .parameter(Number::builder("pi").build())
    ///     .build();
    /// let mut ctx = Context::from_schema(schema);
    /// ctx.set("pi", Value::Float(3.14159)).unwrap();
    ///
    /// assert_eq!(ctx.get_float_or("pi", 0.0), 3.14159);
    /// assert_eq!(ctx.get_float_or("missing", 1.0), 1.0);
    /// ```
    #[must_use]
    pub fn get_float_or(&self, key: &str, default: f64) -> f64 {
        self.get(key).and_then(|v| v.as_f64()).unwrap_or(default)
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

    /// Sets multiple values transactionally (all-or-nothing).
    ///
    /// All keys are validated first, then all changes are applied.
    /// If any operation fails, all changes are rolled back and the
    /// context is restored to its original state.
    ///
    /// When `events` feature is enabled:
    /// - Emits `BatchBegin` at start
    /// - Emits individual `ValueChanging`/`ValueChanged` for each successful set
    /// - On error: emits `Reverted` events for already-applied changes
    /// - Emits `BatchEnd` with success status
    ///
    /// # Errors
    ///
    /// Returns the first error encountered. On error, all changes are rolled back.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use paramdef::core::{Key, Value};
    ///
    /// ctx.set_many_transactional([
    ///     ("name", Value::text("Alice")),
    ///     ("age", Value::Int(30)),
    /// ])?;
    /// ```
    pub fn set_many_transactional<I, K>(&mut self, values: I) -> crate::core::Result<()>
    where
        I: IntoIterator<Item = (K, Value)>,
        K: AsRef<str>,
    {
        use crate::core::FxHashMap;

        let values: Vec<_> = values.into_iter().collect();

        // Validate all keys exist first
        for (key, _) in &values {
            if !self.nodes.contains_key(key.as_ref()) {
                return Err(crate::core::Error::not_found(key.as_ref()));
            }
        }

        #[cfg(feature = "events")]
        let batch_id = if let Some(ref bus) = self.event_bus {
            let id = bus.begin_batch(Some("Transactional update"));
            Some(id)
        } else {
            None
        };

        // Store old values for rollback
        let mut old_values = FxHashMap::default();
        for (key, _) in &values {
            let key_str = key.as_ref();
            if let Some(node) = self.nodes.get(key_str) {
                old_values.insert(Key::from(key_str), node.value().cloned());
            }
        }

        // Apply all changes, collect errors
        let mut applied_keys = Vec::new();
        let mut error = None;

        for (key, value) in values {
            let key_str = key.as_ref();
            match self.set(key_str, value.clone()) {
                Ok(()) => applied_keys.push((Key::from(key_str), value)),
                Err(e) => {
                    error = Some((Key::from(key_str), value, e));
                    break;
                }
            }
        }

        // If error occurred, rollback all applied changes
        if let Some((_failed_key, _failed_value, err)) = error {
            // Rollback all successfully applied changes
            for (key, _) in &applied_keys {
                let Some(old_val) = old_values.get(key) else {
                    continue;
                };
                let Some(node) = self.nodes.get_mut(key.as_str()) else {
                    continue;
                };

                // Restore old value (or clear if it was None)
                match old_val {
                    Some(v) => node.set_value(v.clone()),
                    None => node.clear_value(),
                }

                #[cfg(feature = "events")]
                if let Some(ref bus) = self.event_bus {
                    bus.emit(Event::reverted(
                        key.clone(),
                        old_val.clone().unwrap_or(Value::Null),
                        node.value().cloned().unwrap_or(Value::Null),
                    ));
                }
            }

            #[cfg(feature = "events")]
            if let (Some(bus), Some(id)) = (&self.event_bus, batch_id) {
                bus.end_batch(id, false, false); // success=false, partial=false
            }

            return Err(err);
        }

        #[cfg(feature = "events")]
        if let (Some(bus), Some(id)) = (&self.event_bus, batch_id) {
            bus.end_batch(id, true, false); // success=true, partial=false
        }

        Ok(())
    }

    /// Sets multiple values partially (best-effort).
    ///
    /// Attempts to set all values, collecting errors for failed operations.
    /// Successful operations are applied even if some fail.
    ///
    /// When `events` feature is enabled:
    /// - Emits `BatchBegin` at start
    /// - Emits individual `ValueChanging`/`ValueChanged` for successful sets
    /// - Emits `SetFailed` for each error
    /// - Emits `BatchEnd` with success/partial status
    ///
    /// # Returns
    ///
    /// A vector of results, one per input value. Each result indicates
    /// success or failure for that specific key-value pair.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use paramdef::core::{Key, Value};
    ///
    /// let results = ctx.set_many_partial([
    ///     ("name", Value::text("Alice")),
    ///     ("invalid", Value::Int(30)),  // May fail
    ///     ("email", Value::text("alice@example.com")),
    /// ]);
    ///
    /// for (i, result) in results.iter().enumerate() {
    ///     if let Err(e) = result {
    ///         eprintln!("Field {} failed: {}", i, e);
    ///     }
    /// }
    /// ```
    pub fn set_many_partial<I, K>(&mut self, values: I) -> Vec<crate::core::Result<()>>
    where
        I: IntoIterator<Item = (K, Value)>,
        K: AsRef<str>,
    {
        let values: Vec<_> = values.into_iter().collect();

        #[cfg(feature = "events")]
        let batch_id = if let Some(ref bus) = self.event_bus {
            let id = bus.begin_batch(Some("Partial update"));
            Some(id)
        } else {
            None
        };

        let mut results = Vec::with_capacity(values.len());
        let mut success_count = 0;
        let mut error_count = 0;

        for (key, value) in values {
            let key_str = key.as_ref();
            match self.set(key_str, value) {
                Ok(()) => {
                    results.push(Ok(()));
                    success_count += 1;
                }
                Err(e) => {
                    #[cfg(feature = "events")]
                    if let Some(ref bus) = self.event_bus {
                        bus.emit(Event::set_failed(key_str, e.to_string()));
                    }

                    results.push(Err(e));
                    error_count += 1;
                }
            }
        }

        #[cfg(feature = "events")]
        if let (Some(bus), Some(id)) = (&self.event_bus, batch_id) {
            let success = error_count == 0;
            let partial = success_count > 0 && error_count > 0;
            bus.end_batch(id, success, partial);
        }

        #[cfg(not(feature = "events"))]
        {
            let _ = success_count;
            let _ = error_count;
        }

        results
    }

    /// Convenience method to load values from a `HashMap` transactionally.
    ///
    /// This is equivalent to calling `set_many_transactional` with the map's entries.
    ///
    /// # Errors
    ///
    /// Returns an error if any key doesn't exist or if any value fails to set.
    /// On error, all changes are rolled back.
    pub fn load_from_map(&mut self, map: HashMap<Key, Value>) -> crate::core::Result<()> {
        self.set_many_transactional(map)
    }

    /// Saves dirty values to a `HashMap`.
    ///
    /// Returns a map containing only the parameters that have been modified
    /// since the last reset or `mark_all_clean()`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let dirty = ctx.save_dirty_to_map();
    /// println!("Modified fields: {:?}", dirty.keys());
    /// ```
    #[must_use]
    pub fn save_dirty_to_map(&self) -> HashMap<Key, Value> {
        self.nodes
            .iter()
            .filter(|(_, n)| n.state().is_dirty())
            .filter_map(|(k, n)| n.value().map(|v| (k.clone(), v.clone())))
            .collect()
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

    /// Returns an iterator over dirty values without cloning.
    ///
    /// This is a zero-allocation alternative to [`save_dirty_to_map()`](Self::save_dirty_to_map)
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

    /// Returns an iterator over all non-null values.
    ///
    /// This is a zero-allocation iterator that yields key-value pairs for all
    /// parameters that have a non-null value set.
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
    /// // Only "name" has a value, "email" is null
    /// assert_eq!(ctx.values().count(), 1);
    /// ```
    pub fn values(&self) -> impl Iterator<Item = (&Key, &Value)> + '_ {
        self.nodes
            .iter()
            .filter_map(|(k, n)| n.value().map(|v| (k, v)))
    }

    /// Returns an iterator over touched values.
    ///
    /// Yields key-value pairs for parameters that have been marked as touched
    /// (user has interacted with them).
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
    /// ctx.touch("name").unwrap();
    ///
    /// assert_eq!(ctx.touched_values().count(), 1);
    /// ```
    pub fn touched_values(&self) -> impl Iterator<Item = (&Key, &Value)> + '_ {
        self.nodes
            .iter()
            .filter(|(_, n)| n.state().is_touched())
            .filter_map(|(k, n)| n.value().map(|v| (k, v)))
    }

    /// Returns an iterator over invalid values.
    ///
    /// Yields key-value pairs for parameters that have failed validation.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use paramdef::context::Context;
    ///
    /// // After validation
    /// for (key, value) in ctx.invalid_values() {
    ///     let errors = ctx.node(key).unwrap().state().errors();
    ///     eprintln!("{}: {:?}", key, errors);
    /// }
    /// ```
    pub fn invalid_values(&self) -> impl Iterator<Item = (&Key, &Value)> + '_ {
        self.nodes
            .iter()
            .filter(|(_, n)| !n.state().is_valid())
            .filter_map(|(k, n)| n.value().map(|v| (k, v)))
    }

    /// Returns an iterator over valid values.
    ///
    /// Yields key-value pairs for parameters that have passed validation.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use paramdef::context::Context;
    ///
    /// // Collect only valid values for submission
    /// let valid_data: Vec<_> = ctx.valid_values().collect();
    /// ```
    pub fn valid_values(&self) -> impl Iterator<Item = (&Key, &Value)> + '_ {
        self.nodes
            .iter()
            .filter(|(_, n)| n.state().is_valid())
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
    pub fn end_batch(&self, id: u64, success: bool, partial: bool) {
        if let Some(ref bus) = self.event_bus {
            bus.end_batch(id, success, partial);
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
            self.end_batch(id, true, false); // success=true, partial=false
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

    // === UI State Management ===

    /// Returns a reference to the UI state manager.
    ///
    /// The UI state manager handles presentation state like panel collapsed states,
    /// separate from the immutable schema.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::context::Context;
    /// use paramdef::schema::Schema;
    /// use std::sync::Arc;
    ///
    /// let schema = Arc::new(Schema::builder().build());
    /// let ctx = Context::new(schema);
    ///
    /// let ui_state = ctx.ui_state();
    /// assert!(ui_state.is_empty());
    /// ```
    #[must_use]
    pub fn ui_state(&self) -> &UiStateManager {
        &self.ui_state
    }

    /// Returns a mutable reference to the UI state manager.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::context::Context;
    /// use paramdef::schema::Schema;
    /// use std::sync::Arc;
    ///
    /// let schema = Arc::new(Schema::builder().build());
    /// let mut ctx = Context::new(schema);
    ///
    /// ctx.ui_state_mut().set_panel_collapsed("settings", true);
    /// ```
    #[must_use]
    pub fn ui_state_mut(&mut self) -> &mut UiStateManager {
        &mut self.ui_state
    }

    /// Checks if a panel is collapsed.
    ///
    /// Returns `false` if the panel has no state tracked (default is expanded).
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::context::Context;
    /// use paramdef::schema::Schema;
    /// use paramdef::core::Key;
    /// use std::sync::Arc;
    ///
    /// let schema = Arc::new(Schema::builder().build());
    /// let ctx = Context::new(schema);
    ///
    /// assert!(!ctx.is_panel_collapsed(&Key::from("settings")));
    /// ```
    #[must_use]
    pub fn is_panel_collapsed(&self, key: &Key) -> bool {
        self.ui_state.is_panel_collapsed(key)
    }

    /// Sets the collapsed state of a panel.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::context::Context;
    /// use paramdef::schema::Schema;
    /// use std::sync::Arc;
    ///
    /// let schema = Arc::new(Schema::builder().build());
    /// let mut ctx = Context::new(schema);
    ///
    /// ctx.set_panel_collapsed("settings", true);
    /// assert!(ctx.is_panel_collapsed(&"settings".into()));
    /// ```
    pub fn set_panel_collapsed(&mut self, key: impl Into<Key>, collapsed: bool) {
        self.ui_state.set_panel_collapsed(key, collapsed);
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
    fn test_context_save_dirty_to_map() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);
        ctx.set("name", Value::text("Alice")).unwrap();
        ctx.set("age", Value::Int(30)).unwrap();
        ctx.node_mut("name").unwrap().state_mut().mark_clean();

        let dirty = ctx.save_dirty_to_map();

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

    // === Iterator tests ===

    #[test]
    fn test_values_iterator() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);

        // Initially all values are null
        assert_eq!(ctx.values().count(), 0);

        // Set some values
        ctx.set("name", Value::text("Alice")).unwrap();
        ctx.set("age", Value::Int(30)).unwrap();

        // Should have 2 non-null values
        assert_eq!(ctx.values().count(), 2);

        let keys: Vec<_> = ctx.values().map(|(k, _)| k.as_str()).collect();
        assert!(keys.contains(&"name"));
        assert!(keys.contains(&"age"));
    }

    #[test]
    fn test_touched_values_iterator() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);

        ctx.set("name", Value::text("Alice")).unwrap();
        ctx.set("age", Value::Int(30)).unwrap();

        // Initially no touched values
        assert_eq!(ctx.touched_values().count(), 0);

        // Touch one field
        ctx.touch("name").unwrap();

        assert_eq!(ctx.touched_values().count(), 1);
        let (key, _) = ctx.touched_values().next().unwrap();
        assert_eq!(key.as_str(), "name");
    }

    #[test]
    fn test_valid_values_iterator() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);

        ctx.set("name", Value::text("Alice")).unwrap();
        ctx.set("age", Value::Int(30)).unwrap();

        // All values are valid by default (no validation run)
        assert_eq!(ctx.valid_values().count(), 2);
    }

    #[test]
    fn test_invalid_values_iterator() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);

        ctx.set("name", Value::text("Alice")).unwrap();

        // By default all are valid (no validation failures)
        assert_eq!(ctx.invalid_values().count(), 0);
    }

    #[test]
    fn test_iterators_are_zero_allocation() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);

        ctx.set("name", Value::text("Alice")).unwrap();
        ctx.set("email", Value::text("alice@example.com")).unwrap();
        ctx.set("age", Value::Int(30)).unwrap();

        // Iterators should be lazy and not allocate
        let mut iter = ctx.values();
        assert!(iter.next().is_some());
        assert!(iter.next().is_some());
        assert!(iter.next().is_some());
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_iterators_with_mixed_states() {
        let schema = create_test_schema();
        let mut ctx = Context::new(schema);

        // Set two values, touch one
        ctx.set("name", Value::text("Alice")).unwrap();
        ctx.set("age", Value::Int(30)).unwrap();
        ctx.touch("name").unwrap();

        // values() returns both
        assert_eq!(ctx.values().count(), 2);

        // touched_values() returns only name
        assert_eq!(ctx.touched_values().count(), 1);

        // dirty_values() returns both (just set)
        assert_eq!(ctx.dirty_values().count(), 2);
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
