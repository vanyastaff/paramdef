//! Convenience re-exports for common usage patterns.
//!
//! This module provides a curated set of commonly used types for convenience.
//! Import everything you typically need with a single glob import:
//!
//! ```
//! use paramdef::prelude::*;
//! ```
//!
//! # What's Included
//!
//! - **Core types**: [`Value`], [`Flags`], [`Key`], [`Metadata`], [`Error`], [`Result`]
//! - **Node traits**: [`Node`], [`Leaf`], [`Container`], [`GroupNode`], [`Layout`], [`Decoration`]
//! - **Leaf types**: [`Text`], [`Number`], [`Boolean`], [`Vector`], [`Select`]
//! - **Container types**: [`Object`], [`List`], [`Mode`], [`Routing`], [`Expirable`], [`Reference`]
//! - **Group types**: [`Group`], [`Panel`]
//! - **Schema & Runtime**: [`Schema`], [`Context`], [`RuntimeNode`]
//! - **Subtype traits**: [`NumberSubtype`], [`TextSubtype`], [`VectorSubtype`], [`IntoBuilder`]
//!
//! # Example
//!
//! ```ignore
//! use paramdef::prelude::*;
//! use std::sync::Arc;
//!
//! // Build a schema with leaf parameters
//! let schema = Schema::builder()
//!     .parameter(Text::builder("username")
//!         .label("Username")
//!         .flags(Flags::REQUIRED)
//!         .build())
//!     .parameter(Number::builder("age")
//!         .label("Age")
//!         .range(0.0..=150.0)
//!         .build())
//!     .parameter(Boolean::builder("active")
//!         .label("Active")
//!         .default(true)
//!         .build())
//!     .build();
//!
//! // Create runtime context
//! let mut ctx = Context::new(Arc::new(schema));
//! ctx.set("username", Value::text("alice"));
//! ctx.set("age", Value::Int(30));
//! ```

// Core foundation types
pub use crate::core::{Error, Flags, Key, Metadata, Result, StateFlags, Value};

// Node trait system
pub use crate::types::traits::{Container, Decoration, GroupNode, Layout, Leaf, Node, ValueAccess};

// Feature-gated traits
#[cfg(feature = "visibility")]
pub use crate::types::traits::Visibility;

#[cfg(feature = "validation")]
pub use crate::types::traits::Validatable;

// Leaf parameter types (most commonly used)
pub use crate::types::leaf::{Boolean, Number, Select, Text, Vector};

// Container types
pub use crate::types::container::{Expirable, List, Mode, Object, Reference, Routing};

// Group types
pub use crate::types::group::{Group, Panel};

// Decoration types (less common, but useful)
pub use crate::types::decoration::{Code, Image, Link, Notice, Separator};

// Schema and runtime
pub use crate::context::Context;
pub use crate::runtime::{ErasedRuntimeNode, RuntimeNode, State};
pub use crate::schema::Schema;

// Subtype system
pub use crate::subtype::{IntoBuilder, NumberSubtype, NumberUnit, TextSubtype, VectorSubtype};
