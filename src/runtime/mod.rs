//! Runtime layer for mutable parameter state.
//!
//! The runtime module provides per-instance state management:
//! - [`State`] - Tracks dirty, touched, valid flags and validation errors
//! - [`RuntimeNode`] - Wraps schema node with runtime state and value

mod node;
mod state;

pub use node::RuntimeNode;
pub use state::State;
