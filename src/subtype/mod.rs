//! Type-safe subtypes for parameter definitions.
//!
//! Subtypes provide compile-time constraints and semantic meaning to parameters:
//!
//! - **Number subtypes** - Constrained by numeric type (int/float/any)
//! - **Vector subtypes** - Constrained by size (2, 3, 4, etc.)
//! - **Text subtypes** - Semantic meaning (Email, URL, etc.)
//! - **File subtypes** - MIME type and size constraints (Image, Pdf, etc.)
//!
//! # Organization
//!
//! - [`number`] - Number subtypes and traits
//! - [`vector`] - Vector subtypes
//! - [`text`] - Text subtypes
//! - [`mod@file`] - File subtypes
//! - [`mod@unit`] - Measurement units (Length, Mass, Time, etc.)
//! - [`macros`] - Macros for defining custom subtypes
//!
//! # Example
//!
//! ```
//! use paramdef::types::leaf::{Text, Number};
//!
//! // Text with Email subtype
//! let email = Text::email("contact");
//!
//! // Number with Port subtype (integer-only, range 1-65535)
//! let port = Number::port("http_port")
//!     .default(8080.0)
//!     .build();
//! ```
//!
//! # Safety & Validation
//!
//! Subtypes provide semantic hints and compile-time constraints, but **do not**
//! enforce runtime validation automatically. Use the `validation` feature and
//! [`Rules`](crate::validation::Rules) to enforce constraints at runtime:
//!
//! ```ignore
//! // Requires "validation" feature
//! use paramdef::expr::{Expr, Rule};
//! use paramdef::types::leaf::Number;
//!
//! let port = Number::port("http_port")
//!     .rules(vec![
//!         Rule::local(Expr::min(1.0)),
//!         Rule::local(Expr::max(65535.0)),
//!     ])
//!     .build();
//! ```
//!
//! # Design Philosophy
//!
//! Subtypes follow the **separation of concerns** principle:
//!
//! - **Compile-time**: Type constraints via traits ([`Integer`], [`Float`], [`VectorSubtype`])
//! - **Semantic**: Hints for UI/presentation (e.g., Port suggests range 1-65535)
//! - **Runtime**: Validation rules enforce actual constraints
//!
//! This design enables:
//! - Flexible parameter definitions without rigid validation
//! - Soft constraints for UI (sliders) vs hard constraints (validation)
//! - Composition of 23 node types × 60+ subtypes × flags = thousands of combinations

pub mod file;
pub mod macros;
pub mod number;
pub mod text;
pub mod traits;
pub mod unit;
pub mod vector;

// Re-export commonly used items
pub use macros::{
    define_file_subtype, define_number_subtype, define_text_subtype, define_vector_subtype,
};
pub use traits::{
    FileSubtype, Float, Integer, IntoBuilder, NumberSubtype, Numeric, NumericKind, TextSubtype,
    VectorSubtype,
};
pub use unit::NumberUnit;

// Re-export all subtype type definitions for convenience
#[allow(clippy::wildcard_imports)]
pub use file::*;
#[allow(clippy::wildcard_imports)]
pub use number::*;
#[allow(clippy::wildcard_imports)]
pub use text::*;
#[allow(clippy::wildcard_imports)]
pub use vector::*;
