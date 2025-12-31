//! Type-safe parameter definition system.
//!
//! `paramdef` provides a comprehensive system for defining, validating, and managing
//! parameters in applications. Inspired by Blender RNA, Unreal Engine UPROPERTY,
//! and Qt Property System.
//!
//! # Features
//!
//! - **Type-safe**: Compile-time guarantees for parameter types
//! - **Flexible**: Support for primitives, collections, and complex types
//! - **Extensible**: Custom validators, transformers, and subtypes
//! - **UI-agnostic**: Core library works headless, UI is optional
//!
//! # Quick Start
//!
//! ```
//! use paramdef::core::{Value, Flags, Metadata};
//!
//! // Create metadata for a parameter
//! let meta = Metadata::builder("username")
//!     .label("Username")
//!     .description("Your unique identifier")
//!     .build();
//!
//! // Define flags
//! let flags = Flags::REQUIRED;
//!
//! // Work with values
//! let value = Value::text("alice");
//! assert_eq!(value.as_text(), Some("alice"));
//! ```
//!
//! # Feature Flags
//!
//! - `serde` - Enable serialization/deserialization support
//! - `events` - Enable event system with tokio channels
//! - `validation` - Enable validation system
//! - `display` - Enable display/visibility conditions
//! - `i18n` - Enable internationalization with Fluent
//! - `chrono` - Enable chrono type conversions
//! - `full` - Enable all features

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod container;
pub mod context;
pub mod core;
pub mod decoration;
pub mod group;
pub mod node;
pub mod parameter;
pub mod runtime;
pub mod schema;
pub mod subtypes;

// Re-export commonly used types at crate root
pub use core::{Error, Flags, Key, Metadata, Result, StateFlags, Value};

// Re-export subtype traits and common subtypes
pub use subtypes::{
    IntoBuilder, NumberSubtype, NumberUnit, Numeric, NumericKind, TextSubtype, VectorSubtype,
};
