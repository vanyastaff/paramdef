//! Runtime value access trait.

use std::collections::HashMap;

use crate::core::Value;
use crate::types::traits::Node;

/// Trait for runtime nodes that can access child values.
///
/// This trait is for **runtime** use only, not for schema definitions.
/// Schema types (Group, Layout, Container) do NOT implement this trait.
/// It will be implemented by `RuntimeParameter<T>` and `Context`.
///
/// # Important
///
/// This is NOT part of the schema layer. Schema nodes are immutable definitions.
/// Only runtime wrappers that manage state and values implement this trait.
///
/// # Invariants
///
/// - Group and Layout have NO own value, only delegate to children
/// - Container has BOTH own value AND children
///
/// # Example
///
/// ```
/// use paramdef::context::Context;
/// use paramdef::types::traits::ValueAccess;
/// use paramdef::core::Value;
/// // Context implements ValueAccess at runtime
/// ```
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
