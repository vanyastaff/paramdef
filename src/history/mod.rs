//! History system for undo/redo functionality using the Command pattern.
//!
//! The history system provides a flexible and memory-efficient way to implement undo/redo
//! functionality for parameter changes. It uses the Command pattern to capture state changes
//! and supports command merging for optimization.
//!
//! ## Design Principles
//!
//! 1. **Command Pattern**: Each change is represented as a `Command` object (~100 bytes)
//!    rather than storing full snapshots (~10KB each)
//! 2. **Command Merging**: Sequential similar commands can be merged to reduce memory usage
//! 3. **Extensibility**: Custom commands can be implemented for complex operations
//! 4. **Transaction Support**: Multiple commands can be grouped into a `MacroCommand`
//!
//! ## Industry Patterns
//!
//! | Source | Pattern Adopted |
//! |--------|-----------------|
//! | Qt Undo Framework | Command pattern, undo/redo stacks |
//! | Photoshop | Command merging for optimization |
//! | Text Editors | Transaction grouping (`MacroCommand`) |
//! | Game Engines | Memory-efficient command storage |
//!
//! ## Example
//!
//! ```
//! use paramdef::history::{HistoryManager, SetValueCommand};
//! use paramdef::context::Context;
//! use paramdef::core::Value;
//! # use paramdef::schema::Schema;
//! # use paramdef::types::leaf::Text;
//! # use std::sync::Arc;
//!
//! # let schema = Arc::new(Schema::builder()
//! #     .parameter(Text::builder("name").build())
//! #     .build());
//! let mut ctx = Context::new(schema);
//! let mut history = HistoryManager::new();
//!
//! // Execute a command (stores in undo stack)
//! let cmd = SetValueCommand::new("name", None, Value::text("Alice"));
//! history.execute(cmd, &mut ctx).unwrap();
//!
//! // Undo the change
//! history.undo(&mut ctx).unwrap();
//!
//! // Redo the change
//! history.redo(&mut ctx).unwrap();
//! ```

mod command;
mod commands;
mod manager;

pub use command::{Command, CommandResult};
pub use commands::{ClearValueCommand, MacroCommand, SetValueCommand, TouchCommand};
pub use manager::HistoryManager;
