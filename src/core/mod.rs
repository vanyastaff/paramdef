//! Core types for the paramdef library.
//!
//! This module contains the foundational types that all other components depend on:
//! - [`Key`] - Parameter identifier using stack-optimized strings
//! - [`SmartStr`] - Stack-optimized string for display text (labels, descriptions)
//! - [`Metadata`] - Parameter display information (label, description, group, tags)
//! - [`Flags`] - Schema-level parameter attributes (REQUIRED, READONLY, etc.)
//! - [`StateFlags`] - Runtime parameter state (DIRTY, TOUCHED, VALID, etc.)
//! - [`Value`] - Unified runtime representation for all parameter values
//! - [`Error`] - Error types for parameter operations
//! - [`FxHashMap`] / [`FxHashSet`] - Fast hash collections using `FxHash` algorithm
//! - [`IndexMap`] - Insertion-ordered hash map

mod error;
mod flags;
mod key;
mod metadata;
mod value;

pub use error::{Error, Result};
pub use flags::{Flags, StateFlags};
pub use key::Key;
pub use metadata::{Metadata, MetadataBuilder};
pub use value::Value;

/// Stack-optimized string for display text (labels, descriptions, messages).
///
/// Strings shorter than 23 bytes are stored inline on the stack,
/// avoiding heap allocation. Use [`Key`] for parameter identifiers.
pub type SmartStr = smartstring::SmartString<smartstring::LazyCompact>;

// Re-export fast hash collections.
// FxHash is ~2x faster than std HashMap for small keys like Key/SmartStr.
pub use rustc_hash::{FxHashMap, FxHashSet};

// Re-export IndexMap for ordered collections.
// Preserves insertion order while maintaining O(1) lookup.
pub use indexmap::IndexMap;
