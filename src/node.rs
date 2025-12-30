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
//! ├── GroupNode: Node + ValueAccess
//! ├── Layout: Node + ValueAccess
//! ├── Decoration: Node (no ValueAccess, no own value)
//! ├── Container: Node + ValueAccess (has own value)
//! └── Leaf: Node (has own value, no ValueAccess)
//! ```
//!
//! # Key Invariants
//!
//! | Category | Own Value | `ValueAccess` | Can Contain |
//! |----------|-----------|-------------|-------------|
//! | Group | NO | YES | Layout, Decoration, Container, Leaf |
//! | Layout | NO | YES | Decoration, Container, Leaf |
//! | Decoration | NO | NO | nothing |
//! | Container | YES | YES | Decoration, Container, Leaf |
//! | Leaf | YES | NO | nothing |
//!
//! # Feature Gates
//!
//! - `visibility` - Enables the [`Visibility`] trait for all nodes
//! - `validation` - Enables the [`Validatable`] trait for Container and Leaf nodes

mod kind;
mod traits;

pub use kind::{DecorationType, NodeKind};
pub use traits::{Container, Decoration, GroupNode, Layout, Leaf, Node, ValueAccess};

#[cfg(feature = "visibility")]
pub use traits::Visibility;

#[cfg(feature = "validation")]
pub use traits::Validatable;
