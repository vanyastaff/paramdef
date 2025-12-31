//! Node trait system for the parameter hierarchy.
//!
//! This module defines the trait hierarchy that all 14 node types implement.
//! The traits are organized into five categories:
//!
//! - **Group**: Root aggregator ([`GroupNode`])
//! - **Layout**: UI organization ([`Layout`])
//! - **Decoration**: Display-only ([`Decoration`])
//! - **Container**: Data structures with children ([`Container`])
//! - **Leaf**: Terminal values ([`Leaf`])
//!
//! # Trait Hierarchy
//!
//! ```text
//! Node (base trait)
//! ├── GroupNode: Node (schema definition, no own value)
//! ├── Layout: Node (schema definition, no own value)
//! ├── Decoration: Node (display-only, no value, no children)
//! ├── Container: Node (schema definition, has own value)
//! └── Leaf: Node (terminal values, has own value)
//! ```
//!
//! Note: `ValueAccess` is a **runtime-only** trait implemented by `RuntimeParameter<T>`
//! and `Context`, not by schema types. Schema types are immutable.
//!
//! # Key Invariants
//!
//! | Category | Own Value | Children | Runtime `ValueAccess` |
//! |----------|-----------|----------|----------------------|
//! | Group | NO | YES | via Context |
//! | Layout | NO | YES | via Context |
//! | Decoration | NO | NO | N/A |
//! | Container | YES | YES | via `RuntimeParameter` |
//! | Leaf | YES | NO | via `RuntimeParameter` |
//!
//! # Feature Gates
//!
//! - `visibility` - Enables the [`Visibility`] trait for all nodes
//! - `validation` - Enables the [`Validatable`] trait for Container and Leaf nodes

mod kind;
mod traits;

pub use kind::{LinkType, NodeKind, NoticeType, SeparatorStyle};
pub use traits::{Container, Decoration, GroupNode, Layout, Leaf, Node, ValueAccess};

#[cfg(feature = "visibility")]
pub use traits::Visibility;

#[cfg(feature = "validation")]
pub use traits::Validatable;
