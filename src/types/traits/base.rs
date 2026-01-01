//! Core Node trait that all parameter types implement.

use std::any::Any;
use std::fmt::Debug;

use crate::core::{Key, Metadata};
use crate::types::kind::NodeKind;

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
/// - Decoration: Notice, Separator, Link, Code, Image (5)
/// - Container: Object, List, Mode, Routing, Expirable, Ref (6)
/// - Leaf: Text, Number, Boolean, Vector, Select (5)
///
/// # Example
///
/// ```
/// use paramdef::types::traits::Node;
/// use paramdef::types::leaf::Text;
/// use paramdef::types::kind::NodeKind;
///
/// let text = Text::builder("username").build();
///
/// assert_eq!(text.key().as_str(), "username");
/// assert_eq!(text.kind(), NodeKind::Leaf);
/// ```
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
