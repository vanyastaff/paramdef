//! Type-safe subtypes for parameter definitions.
//!
//! This module provides compile-time safe subtypes that constrain parameter values:
//!
//! - [`NumberSubtype`] - Constrained by numeric type (int-only, float-only, any)
//! - [`VectorSubtype`] - Constrained by vector size (2, 3, 4, etc.)
//! - [`TextSubtype`] - Semantic meaning for string values
//! - [`NumberUnit`] - Measurement units with conversion support
//!
//! # Design Philosophy
//!
//! Subtypes provide compile-time guarantees:
//! - `Port` (integer-only) cannot be used with floats
//! - `Quaternion` (size 4) ensures exactly 4 components
//! - `Email` provides pattern validation hints
//!
//! # Example
//!
//! ```ignore
//! use paramdef::subtypes::{Port, Percentage, Position3D, Email};
//!
//! // Integer-only subtype
//! let port = Port::into_builder("server_port")
//!     .default(8080)
//!     .range(1, 65535)
//!     .build();
//!
//! // Float-only subtype
//! let opacity = Percentage::into_builder("opacity")
//!     .default(100.0)
//!     .build();
//!
//! // Vector subtype with size constraint
//! let position = Position3D::into_builder("spawn_point")
//!     .default([0.0, 0.0, 0.0])
//!     .build();
//!
//! // Text subtype with pattern
//! let email = Email::into_builder("contact")
//!     .placeholder("user@example.com")
//!     .build();
//! ```

mod macros;
mod number;
mod text;
mod traits;
mod unit;
mod vector;

pub use macros::{define_number_subtype, define_text_subtype, define_vector_subtype};
pub use number::*;
pub use text::*;
pub use traits::{IntoBuilder, NumberSubtype, Numeric, TextSubtype, VectorSubtype};
pub use unit::NumberUnit;
pub use vector::*;
