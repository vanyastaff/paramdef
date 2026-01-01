//! Type-safe subtypes for parameter definitions.
//!
//! Subtypes provide compile-time constraints and semantic meaning to parameters:
//!
//! - **Number subtypes** - Constrained by numeric type (int/float/any)
//! - **Vector subtypes** - Constrained by size (2, 3, 4, etc.)
//! - **Text subtypes** - Semantic meaning (Email, URL, etc.)
//!
//! # Organization
//!
//! - [`number`] - Number subtypes and traits
//! - [`vector`] - Vector subtypes
//! - [`text`] - Text subtypes
//! - [`unit`] - Measurement units (Length, Mass, Time, etc.)
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

pub mod macros;
pub mod number;
pub mod text;
pub mod traits;
pub mod unit;
pub mod vector;

// Re-export commonly used items
pub use macros::{define_number_subtype, define_text_subtype, define_vector_subtype};
pub use traits::{IntoBuilder, NumberSubtype, Numeric, NumericKind, TextSubtype, VectorSubtype};
pub use unit::NumberUnit;

// Re-export all subtype type definitions for convenience
#[allow(clippy::wildcard_imports)]
pub use number::*;
#[allow(clippy::wildcard_imports)]
pub use text::*;
#[allow(clippy::wildcard_imports)]
pub use vector::*;
