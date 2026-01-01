//! All 23 node types organized by category.
//!
//! This module organizes the parameter type system into a clear hierarchy
//! matching the four categories: Group, Decoration, Container, and Leaf.
//!
//! # Organization
//!
//! - **[`group`]** - Root aggregators (Group, Panel)
//! - **[`leaf`]** - Terminal values (Text, Number, Boolean, Vector, Select, File)
//! - **[`container`]** - Structured data (Object, List, Mode, Matrix, Routing, Expirable, Reference)
//! - **[`decoration`]** - Display-only (Notice, Separator, Link, Code, Image, Html, Video, Progress)
//! - **[`traits`]** - Node trait system (Node, Leaf, Container, etc.)
//! - **[`kind`]** - Node kind enumerations
//!
//! # Categories
//!
//! ## Group (2 types)
//!
//! Root aggregators that can contain Decoration, Container, and Leaf nodes.
//! Have NO own value, only delegate to children via `ValueAccess`.
//!
//! - [`group::Group`] - Root parameter group with layout
//! - [`group::Panel`] - UI organization panel
//!
//! ## Leaf (6 types)
//!
//! Terminal values with NO children. These are the actual data-bearing parameters.
//!
//! - [`leaf::Text`] - String values with validation
//! - [`leaf::Number`] - Numeric values (int/float) with units
//! - [`leaf::Boolean`] - True/false toggles
//! - [`leaf::Vector`] - Fixed-size numeric arrays
//! - [`leaf::Select`] - Single or multiple selection
//! - [`leaf::File`] - File uploads and references
//!
//! ## Container (7 types)
//!
//! Structured types that have BOTH own value AND children.
//!
//! - [`container::Object`] - Named field collection
//! - [`container::List`] - Dynamic array with item template
//! - [`container::Mode`] - Discriminated union (sum type)
//! - [`container::Matrix`] - Table-based data entry
//! - [`container::Routing`] - Connection/reference wrapper
//! - [`container::Expirable`] - TTL-based wrapper
//! - [`container::Reference`] - Template reference
//!
//! ## Decoration (8 types)
//!
//! Display-only elements with NO value and NO children.
//!
//! - [`decoration::Notice`] - Info/warning/error messages
//! - [`decoration::Separator`] - Visual dividers
//! - [`decoration::Link`] - Clickable references
//! - [`decoration::Code`] - Syntax-highlighted code
//! - [`decoration::Image`] - Static images
//! - [`decoration::Html`] - Rich HTML content
//! - [`decoration::Video`] - Embedded video
//! - [`decoration::Progress`] - Progress indicators
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
