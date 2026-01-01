//! All 14 node types organized by category.
//!
//! This module organizes the parameter type system into a clear hierarchy
//! matching the five categories: Group, Layout, Decoration, Container, and Leaf.
//!
//! # Organization
//!
//! - **[`group`]** - Root aggregator (Group, Panel)
//! - **[`leaf`]** - Terminal values (Text, Number, Boolean, Vector, Select)
//! - **[`container`]** - Structured data (Object, List, Mode, Routing, Expirable, Reference)
//! - **[`decoration`]** - Display-only (Notice, Separator, Link, Code, Image)
//! - **[`traits`]** - Node trait system (Node, Leaf, Container, etc.)
//! - **[`kind`]** - Node kind enumerations
//!
//! # Categories
//!
//! ## Group (1 type)
//!
//! Root aggregator that can contain Layout, Decoration, Container, and Leaf nodes.
//! Has NO own value, only delegates to children via `ValueAccess`.
//!
//! - [`group::Group`] - Root parameter group with layout
//! - [`group::Panel`] - UI organization panel (technically Layout, grouped here)
//!
//! ## Leaf (5 types)
//!
//! Terminal values with NO children. These are the actual data-bearing parameters.
//!
//! - [`leaf::Text`] - String values with validation
//! - [`leaf::Number`] - Numeric values (int/float) with units
//! - [`leaf::Boolean`] - True/false toggles
//! - [`leaf::Vector`] - Fixed-size numeric arrays
//! - [`leaf::Select`] - Single or multiple selection
//!
//! ## Container (6 types)
//!
//! Structured types that have BOTH own value AND children.
//!
//! - [`container::Object`] - Named field collection
//! - [`container::List`] - Dynamic array with item template
//! - [`container::Mode`] - Discriminated union (sum type)
//! - [`container::Routing`] - Connection/reference wrapper
//! - [`container::Expirable`] - TTL-based wrapper
//! - [`container::Reference`] - Template reference
//!
//! ## Decoration (5 types)
//!
//! Display-only elements with NO value and NO children.
//!
//! - [`decoration::Notice`] - Info/warning/error messages
//! - [`decoration::Separator`] - Visual dividers
//! - [`decoration::Link`] - Clickable references
//! - [`decoration::Code`] - Syntax-highlighted code
//! - [`decoration::Image`] - Static images
//!
//! # Example
//!
//! ```ignore
//! use paramdef::types::leaf::{Text, Number};
//! use paramdef::types::container::Object;
//! use paramdef::types::group::Group;
//! use paramdef::types::traits::Node;
//!
//! // Create a nested structure
//! let address = Object::builder("address")
//!     .field("street", Text::builder("street").required().build())
//!     .field("city", Text::builder("city").required().build())
//!     .field("zip", Text::builder("zip").build())
//!     .build();
//!
//! // All types implement the Node trait
//! assert_eq!(address.key().as_str(), "address");
//! ```

pub mod container;
pub mod decoration;
pub mod group;
pub mod kind;
pub mod leaf;
pub mod traits;

// Re-export all types at types:: level for convenience
pub use container::{Expirable, List, Mode, Object, Reference, Routing};
pub use decoration::{Code, Image, Link, Notice, Separator};
pub use group::{Group, Panel};
pub use kind::{LinkType, NodeKind, NoticeType, SeparatorStyle};
pub use leaf::{Boolean, Number, Select, Text, Vector};
pub use traits::{Container, Decoration, GroupNode, Layout, Leaf, Node, ValueAccess};

#[cfg(feature = "visibility")]
pub use traits::Visibility;

#[cfg(feature = "validation")]
pub use traits::Validatable;
