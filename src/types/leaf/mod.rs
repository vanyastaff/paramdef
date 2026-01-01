//! Leaf parameter types representing terminal values.
//!
//! Leaf types have their own value but cannot contain children.
//! These are the actual data-bearing parameters in the system.
//!
//! # Types
//!
//! - [`Text`] - String values with optional constraints and subtypes
//! - [`Number`] - Numeric values (integer or float) with units and ranges
//! - [`Boolean`] - Simple true/false toggles
//! - [`Vector`] - Fixed-size numeric arrays (Position, Color, etc.)
//! - [`Select`] - Single or multiple selection from options
//!
//! # Example
//!
//! ```ignore
//! use paramdef::types::leaf::{Text, Number, Boolean};
//! use paramdef::core::Flags;
//!
//! // Text with validation
//! let username = Text::builder("username")
//!     .label("Username")
//!     .flags(Flags::REQUIRED)
//!     .min_length(3)
//!     .max_length(20)
//!     .build();
//!
//! // Number with range and unit
//! let temperature = Number::builder("temp")
//!     .label("Temperature")
//!     .range(-273.15..=1000.0)
//!     .build();
//!
//! // Boolean with default
//! let enabled = Boolean::builder("enabled")
//!     .label("Enabled")
//!     .default(true)
//!     .build();
//! ```
//!
//! # Subtypes
//!
//! Leaf types support type-safe subtypes for semantic meaning:
//!
//! ```
//! use paramdef::types::leaf::Text;
//!
//! // Using subtype for email validation
//! let email = Text::email("contact_email");
//! ```

mod boolean;
mod number;
mod select;
mod text;
mod vector;

pub use boolean::{Boolean, BooleanBuilder};
pub use number::{Number, NumberBuilder};
pub use select::{OptionSource, Select, SelectBuilder, SelectOption, SelectionMode};
pub use text::{Text, TextBuilder};
pub use vector::{Vector, VectorBuilder};
