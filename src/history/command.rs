//! Command trait and related types for the undo/redo system.

use crate::Error;
use crate::context::Context;
use std::any::Any;
use std::fmt::Debug;

/// Result type for command operations.
pub type CommandResult = Result<(), Error>;

/// A command that can be executed, undone, and redone.
///
/// Commands represent discrete changes to the parameter context. They follow the
/// Command pattern, encapsulating both the action and the information needed to
/// reverse it.
///
/// ## Design Requirements
///
/// 1. **Idempotent undo/redo**: Multiple undo/redo cycles should produce consistent results
/// 2. **Small memory footprint**: Commands should be ~100 bytes, not full snapshots
/// 3. **Mergeable**: Sequential similar commands should support merging
/// 4. **Thread-safe**: Commands must be `Send + Sync` for concurrent contexts
///
/// ## Example
///
/// ```
/// use paramdef::history::{Command, CommandResult};
/// use paramdef::context::Context;
/// use paramdef::core::{Key, Value};
/// use std::any::Any;
///
/// #[derive(Debug)]
/// struct MyCommand {
///     key: Key,
///     old_value: Option<Value>,
///     new_value: Value,
/// }
///
/// impl Command for MyCommand {
///     fn as_any(&self) -> &dyn Any {
///         self
///     }
///
///     fn execute(&mut self, ctx: &mut Context) -> CommandResult {
///         ctx.set(self.key.as_str(), self.new_value.clone());
///         Ok(())
///     }
///
///     fn undo(&mut self, ctx: &mut Context) -> CommandResult {
///         if let Some(old) = &self.old_value {
///             ctx.set(self.key.as_str(), old.clone());
///         } else {
///             ctx.clear(self.key.as_str());
///         }
///         Ok(())
///     }
///
///     fn redo(&mut self, ctx: &mut Context) -> CommandResult {
///         self.execute(ctx)
///     }
///
///     fn merge(&mut self, _other: &dyn Command) -> bool {
///         false // No merging by default
///     }
///
///     fn description(&self) -> &str {
///         "Set value"
///     }
/// }
/// ```
pub trait Command: Send + Sync + Debug {
    /// Downcast to `Any` for type checking and conversion.
    ///
    /// This is used internally for command merging.
    fn as_any(&self) -> &dyn Any;

    /// Execute the command, applying its changes to the context.
    ///
    /// This method is called when:
    /// - The command is first executed via `HistoryManager::execute()`
    /// - The command is redone via `HistoryManager::redo()`
    ///
    /// # Errors
    ///
    /// Returns an error if the command cannot be executed (e.g., invalid context state).
    fn execute(&mut self, ctx: &mut Context) -> CommandResult;

    /// Undo the command, reverting its changes to the context.
    ///
    /// This method should restore the state that existed before `execute()` was called.
    /// It must be idempotent - calling it multiple times should produce the same result.
    ///
    /// # Errors
    ///
    /// Returns an error if the command cannot be undone (e.g., invalid context state).
    fn undo(&mut self, ctx: &mut Context) -> CommandResult;

    /// Redo the command after it has been undone.
    ///
    /// The default implementation calls `execute()`, which is correct for most commands.
    /// Override if your command needs different behavior for redo vs initial execution.
    ///
    /// # Errors
    ///
    /// Returns an error if the command cannot be redone (e.g., invalid context state).
    fn redo(&mut self, ctx: &mut Context) -> CommandResult {
        self.execute(ctx)
    }

    /// Attempt to merge this command with another command.
    ///
    /// Returns `true` if the merge was successful, `false` otherwise.
    ///
    /// Merging is an optimization that reduces memory usage by combining sequential
    /// similar commands. For example, multiple consecutive `SetValueCommand`s for the
    /// same key can be merged into a single command.
    ///
    /// ## Requirements
    ///
    /// - Only merge commands of the same type
    /// - Only merge commands that affect the same parameter
    /// - Preserve the final state after merging
    ///
    /// ## Example
    ///
    /// ```text
    /// Command 1: Set "name" from None to "A"
    /// Command 2: Set "name" from "A" to "Alice"
    /// Merged:    Set "name" from None to "Alice"
    /// ```
    fn merge(&mut self, other: &dyn Command) -> bool;

    /// Get a human-readable description of this command.
    ///
    /// Used for debugging and potentially for UI display (e.g., "Undo: Set name").
    fn description(&self) -> &str;

    /// Check if this command can be merged with another command.
    ///
    /// This is a fast check before attempting the actual merge. The default
    /// implementation returns `false`, which is safe and correct.
    fn can_merge_with(&self, _other: &dyn Command) -> bool {
        false
    }
}
