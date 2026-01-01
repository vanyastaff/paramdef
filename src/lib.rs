//! Type-safe parameter definition system.
//!
//! `paramdef` provides a comprehensive system for defining, validating, and managing
//! parameters in applications. Inspired by Blender RNA, Unreal Engine UPROPERTY,
//! and Qt Property System.
//!
//! # Quick Start
//!
//! The easiest way to get started is using the [`prelude`]:
//!
//! ```ignore
//! use paramdef::prelude::*;
//! use std::sync::Arc;
//!
//! // Create a schema with parameters
//! let schema = Schema::builder()
//!     .parameter(Text::builder("username")
//!         .label("Username")
//!         .required()
//!         .build())
//!     .parameter(Number::builder("age")
//!         .label("Age")
//!         .build())
//!     .parameter(Boolean::builder("active")
//!         .label("Active")
//!         .default(true)
//!         .build())
//!     .build();
//!
//! // Create a runtime context
//! let mut ctx = Context::new(Arc::new(schema));
//!
//! // Set values
//! ctx.set("username", Value::text("alice"));
//! ctx.set("age", Value::Int(30));
//! ctx.set("active", Value::Bool(true));
//!
//! // Get values
//! assert_eq!(ctx.get("username").and_then(|v| v.as_text()), Some("alice"));
//! ```
//!
//! # Architecture
//!
//! paramdef uses a three-layer architecture:
//!
//! ## 1. Schema Layer (Immutable)
//!
//! Parameter definitions shared via `Arc`. Contains metadata, flags, validators,
//! and transformers. Multiple contexts can share the same schema.
//!
//! ```text
//! Schema (Arc-shared, immutable)
//!   ├── Parameter definitions
//!   ├── Metadata (labels, descriptions)
//!   ├── Flags (REQUIRED, READONLY, etc.)
//!   └── Validation rules
//! ```
//!
//! ## 2. Runtime Layer (Mutable)
//!
//! Per-instance state management. Each [`Context`] has its own runtime state
//! for parameter values, dirty flags, and validation errors.
//!
//! ```text
//! Context (mutable, per-instance)
//!   ├── Current values
//!   ├── State flags (dirty, touched, valid)
//!   └── Validation errors
//! ```
//!
//! ## 3. Value Layer
//!
//! Runtime data representation. The [`Value`] enum is the serialization target
//! and runtime storage format for all parameter types.
//!
//! ```text
//! Value (runtime representation)
//!   ├── Primitives: Null, Bool, Int, Float, Text
//!   └── Collections: Array, Object, Binary
//! ```
//!
//! # Type System
//!
//! paramdef defines **14 node types** across **5 categories**:
//!
//! | Category   | Own Value | Children | Types | Module |
//! |------------|-----------|----------|-------|--------|
//! | **Group**      | ❌ | ✅ | 1 | [`types::group`] |
//! | **Layout**     | ❌ | ✅ | 1 | [`types::group`] |
//! | **Decoration** | ❌ | ❌ | 5 | [`types::decoration`] |
//! | **Container**  | ✅ | ✅ | 6 | [`types::container`] |
//! | **Leaf**       | ✅ | ❌ | 5 | [`types::leaf`] |
//!
//! ### Leaf Types (Terminal Values)
//!
//! - [`types::leaf::Text`] - String values with validation
//! - [`types::leaf::Number`] - Numeric values (int/float) with units
//! - [`types::leaf::Boolean`] - True/false toggles
//! - [`types::leaf::Vector`] - Fixed-size numeric arrays
//! - [`types::leaf::Select`] - Single or multiple selection
//!
//! ### Container Types (Structured Data)
//!
//! - [`types::container::Object`] - Named field collection
//! - [`types::container::List`] - Dynamic array with item template
//! - [`types::container::Mode`] - Discriminated union (sum type)
//! - [`types::container::Routing`] - Connection/reference wrapper
//! - [`types::container::Expirable`] - TTL-based wrapper
//! - [`types::container::Reference`] - Template reference
//!
//! # Module Organization
//!
//! - [`core`] - Foundation types (Key, Value, Metadata, Flags, Error)
//! - [`types`] - All 14 node types organized by category
//! - [`subtype`] - Type-safe subtypes and units
//! - [`schema`] - Schema and Context for managing parameters
//! - [`runtime`] - Runtime state management
//! - [`prelude`] - Common imports for convenience
//!
//! # Feature Flags
//!
//! | Feature | Description |
//! |---------|-------------|
//! | `serde` | Serialization/deserialization support |
//! | `validation` | Validation system with custom validators |
//! | `visibility` | Visibility conditions and expressions |
//! | `events` | Event system with tokio channels |
//! | `i18n` | Internationalization with Fluent |
//! | `chrono` | Chrono type conversions |
//! | `full` | Enable all features |
//!
//! # Examples
//!
//! ## Building Complex Schemas
//!
//! ```ignore
//! use paramdef::prelude::*;
//!
//! let address = Object::builder("address")
//!     .field("street", Text::builder("street").required().build())
//!     .field("city", Text::builder("city").required().build())
//!     .field("zip", Text::builder("zip").build())
//!     .build();
//!
//! let user = Object::builder("user")
//!     .field("name", Text::builder("name").required().build())
//!     .field("email", Text::builder("email").build())
//!     .field("address", address)
//!     .build();
//! ```
//!
//! ## Using Subtypes
//!
//! ```
//! use paramdef::prelude::*;
//! use paramdef::subtype::{Email, Port};
//!
//! let email = Text::email("contact");
//! let port = Number::port("http_port").default(8080.0).build();
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

// Core foundation
pub mod core;

// Type system (new organization)
pub mod types;

// Subtypes and units (renamed from 'subtypes')
pub mod subtype;

// Schema and runtime layers
pub mod context;
pub mod runtime;
pub mod schema;

// Feature-gated modules (TODO: implement these)
// #[cfg(feature = "validation")]
// pub mod validation;

// #[cfg(feature = "visibility")]
// pub mod visibility;

// #[cfg(feature = "events")]
// pub mod event;

// #[cfg(feature = "i18n")]
// pub mod i18n;

// Convenience re-exports
pub mod prelude;

// === Backward Compatibility (Deprecated) ===

// Root-level re-exports (most commonly used)
pub use context::Context;
pub use core::{Error, Flags, Key, Metadata, Result, StateFlags, Value};
pub use runtime::{ErasedRuntimeNode, RuntimeNode, State};
pub use schema::Schema;

// Re-export subtype traits at root for convenience
pub use subtype::{IntoBuilder, NumberSubtype, NumberUnit, TextSubtype, VectorSubtype};
