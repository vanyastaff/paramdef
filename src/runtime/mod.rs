//! Runtime layer for mutable parameter state.
//!
//! The runtime module provides per-instance state management:
//! - [`State`] - Tracks dirty, touched, valid flags and validation errors
//! - [`RuntimeNode`] - Generic wrapper for schema node with runtime state and value
//! - [`ErasedRuntimeNode`] - Type-erased wrapper for heterogeneous collections

mod node;
mod state;

pub use node::{ErasedRuntimeNode, RuntimeNode};
pub use state::State;
