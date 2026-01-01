//! Leaf parameter types for terminal values.
//!
//! This module provides the five leaf parameter types that represent
//! user-facing data inputs:
//!
//! - [`Text`] - String values with optional constraints
//! - [`Number`] - Numeric values (integer or float) with units
//! - [`Boolean`] - Simple true/false toggles
//! - [`Vector`] - Fixed-size numeric arrays
//! - [`Select`] - Single or multiple selection from options
//!
//! # Design Philosophy
//!
//! Leaf parameters are the terminal nodes in the parameter hierarchy.
//! They have their own value but cannot contain children. All leaf types
//! implement the [`Leaf`](crate::node::Leaf) trait.
//!
//! # Example
//!
//! ```ignore
//! use paramdef::types::leaf::{Text, Number, Boolean};
//!
//! // Create a text parameter with constraints
//! let username = Text::builder("username")
//!     .label("Username")
//!     .min_length(3)
//!     .max_length(32)
//!     .build();
//!
//! // Create a number with unit
//! let temperature = Number::float("temperature")
//!     .label("Temperature")
//!     .unit(NumberUnit::Temperature(Celsius))
//!     .min(-273.15)
//!     .build();
//!
//! // Create a simple boolean
//! let enabled = Boolean::builder("enabled")
//!     .default(true)
//!     .build();
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
