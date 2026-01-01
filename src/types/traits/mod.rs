//! Node trait system defining the parameter hierarchy.
//!
//! This module contains all traits that define the behavior and capabilities
//! of the 14 node types in the parameter system.
//!
//! # Trait Hierarchy
//!
//! ```text
//! Node (base trait - all 14 types implement this)
//! ├── GroupNode: Node       (1 type:  Group)
//! ├── Layout: Node          (1 type:  Panel)
//! ├── Decoration: Node      (5 types: Notice, Separator, Link, Code, Image)
//! ├── Container: Node       (6 types: Object, List, Mode, Routing, Expirable, Ref)
//! └── Leaf: Node            (5 types: Text, Number, Boolean, Vector, Select)
//! ```
//!
//! # Runtime Trait
//!
//! - [`ValueAccess`] - NOT implemented by schema types, only by runtime wrappers
//!
//! # Feature-Gated Traits
//!
//! - [`Visibility`] - Requires `visibility` feature (all 14 types)
//! - [`Validatable`] - Requires `validation` feature (Container + Leaf = 11 types)
//!
//! # Core Design Principles
//!
//! ## Separation of Schema and Runtime
//!
//! Schema types (Group, Layout, Container, Leaf, Decoration) are **immutable**
//! and define the structure. They do NOT store runtime values or state.
//!
//! Runtime wrappers (`RuntimeNode<T>`, `Context`) implement [`ValueAccess`]
//! to provide mutable value storage and state management.
//!
//! ## Node Capabilities Matrix
//!
//! | Category   | Own Value | Children | `ValueAccess` | Count |
//! |------------|-----------|----------|---------------|-------|
//! | Group      | ❌        | ✅       | ✅ (runtime)| 1     |
//! | Layout     | ❌        | ✅       | ✅ (runtime)| 1     |
//! | Decoration | ❌        | ❌       | ❌          | 5     |
//! | Container  | ✅        | ✅       | ✅ (runtime)| 6     |
//! | Leaf       | ✅        | ❌       | ❌          | 5     |
//!
//! # Example
//!
//! ```
//! use paramdef::types::traits::{Node, Leaf};
//! use paramdef::types::leaf::Text;
//! use paramdef::core::Value;
//!
//! let text = Text::builder("username").default("guest").build();
//!
//! // All types implement Node
//! assert_eq!(text.key().as_str(), "username");
//!
//! // Leaf types have default values
//! assert_eq!(text.default_value(), Some(Value::text("guest")));
//! ```

mod access;
mod base;
mod category;

#[cfg(feature = "validation")]
mod validatable;

#[cfg(feature = "visibility")]
mod visibility;

// Re-export all traits
pub use access::ValueAccess;
pub use base::Node;
pub use category::{Container, Decoration, GroupNode, Layout, Leaf};

#[cfg(feature = "validation")]
pub use validatable::Validatable;

#[cfg(feature = "visibility")]
pub use visibility::Visibility;
