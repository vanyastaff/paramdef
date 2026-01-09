//! History manager for undo/redo functionality.

use super::command::{Command, CommandResult};
use crate::context::Context;
use std::collections::VecDeque;

/// Manages command history for undo/redo operations.
///
/// The `HistoryManager` maintains two stacks:
/// - **Undo stack**: Commands that can be undone
/// - **Redo stack**: Commands that can be redone
///
/// ## Memory Management
///
/// The manager has a configurable maximum history size. When the limit is reached,
/// the oldest commands are removed from the undo stack.
///
/// ## Command Merging
///
/// When executing a new command, the manager attempts to merge it with the most
/// recent command if possible. This reduces memory usage for sequences of similar
/// operations (e.g., typing text character by character).
///
/// ## Example
///
/// ```
/// use paramdef::history::{HistoryManager, SetValueCommand};
/// use paramdef::context::Context;
/// use paramdef::core::Value;
/// # use paramdef::schema::Schema;
/// # use paramdef::types::leaf::Text;
/// # use std::sync::Arc;
///
/// # let schema = Arc::new(Schema::builder()
/// #     .parameter(Text::builder("name").build())
/// #     .build());
/// let mut ctx = Context::new(schema);
/// let mut history = HistoryManager::new();
///
/// // Execute commands
/// let cmd1 = SetValueCommand::new("name", None, Value::text("A"));
/// history.execute(cmd1, &mut ctx).unwrap();
///
/// let cmd2 = SetValueCommand::new("name", Some(Value::text("A")), Value::text("Alice"));
/// history.execute(cmd2, &mut ctx).unwrap();
///
/// // Undo
/// assert!(history.can_undo());
/// history.undo(&mut ctx).unwrap();
///
/// // Redo
/// assert!(history.can_redo());
/// history.redo(&mut ctx).unwrap();
/// ```
#[derive(Debug)]
pub struct HistoryManager {
    /// Stack of commands that can be undone
    undo_stack: VecDeque<Box<dyn Command>>,
    /// Stack of commands that can be redone
    redo_stack: VecDeque<Box<dyn Command>>,
    /// Maximum number of commands to keep in history
    max_history: usize,
    /// Whether to attempt command merging
    enable_merging: bool,
}

impl HistoryManager {
    /// Create a new history manager with default settings.
    ///
    /// Default settings:
    /// - Max history: 100 commands
    /// - Command merging: enabled
    #[must_use]
    pub fn new() -> Self {
        Self {
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
            max_history: 100,
            enable_merging: true,
        }
    }

    /// Create a new history manager with a custom maximum history size.
    #[must_use]
    pub fn with_capacity(max_history: usize) -> Self {
        Self {
            undo_stack: VecDeque::with_capacity(max_history.min(100)),
            redo_stack: VecDeque::with_capacity(max_history.min(100)),
            max_history,
            enable_merging: true,
        }
    }

    /// Set whether command merging is enabled.
    ///
    /// When enabled (default), the manager will attempt to merge new commands
    /// with the most recent command on the undo stack.
    pub fn set_merging_enabled(&mut self, enabled: bool) {
        self.enable_merging = enabled;
    }

    /// Execute a command and add it to the undo stack.
    ///
    /// This method:
    /// 1. Attempts to merge the command with the most recent command (if merging is enabled)
    /// 2. Executes the command on the context
    /// 3. Pushes the command onto the undo stack
    /// 4. Clears the redo stack (executing a new command invalidates the redo history)
    /// 5. Enforces the maximum history size
    ///
    /// # Errors
    ///
    /// Returns an error if the command execution fails.
    pub fn execute<C: Command + 'static>(
        &mut self,
        mut cmd: C,
        ctx: &mut Context,
    ) -> CommandResult {
        // Try to merge with the most recent command
        if self.enable_merging {
            if let Some(last_cmd) = self.undo_stack.back_mut() {
                if last_cmd.can_merge_with(&cmd) && last_cmd.merge(&cmd) {
                    // Merge successful, no need to add a new command
                    return Ok(());
                }
            }
        }

        // Execute the command
        cmd.execute(ctx)?;

        // Add to undo stack
        self.undo_stack.push_back(Box::new(cmd));

        // Clear redo stack (new action invalidates redo history)
        self.redo_stack.clear();

        // Enforce maximum history size
        while self.undo_stack.len() > self.max_history {
            self.undo_stack.pop_front();
        }

        Ok(())
    }

    /// Undo the most recent command.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The undo stack is empty
    /// - The command's undo operation failed
    pub fn undo(&mut self, ctx: &mut Context) -> CommandResult {
        if let Some(mut cmd) = self.undo_stack.pop_back() {
            cmd.undo(ctx)?;
            self.redo_stack.push_back(cmd);
            Ok(())
        } else {
            Err(crate::Error::not_found("undo"))
        }
    }

    /// Redo the most recently undone command.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The redo stack is empty
    /// - The command's redo operation failed
    pub fn redo(&mut self, ctx: &mut Context) -> CommandResult {
        if let Some(mut cmd) = self.redo_stack.pop_back() {
            cmd.redo(ctx)?;
            self.undo_stack.push_back(cmd);
            Ok(())
        } else {
            Err(crate::Error::not_found("redo"))
        }
    }

    /// Check if there are commands that can be undone.
    #[must_use]
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if there are commands that can be redone.
    #[must_use]
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Get the number of commands in the undo stack.
    #[must_use]
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get the number of commands in the redo stack.
    #[must_use]
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }

    /// Clear all history (both undo and redo stacks).
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Get a description of the command that would be undone.
    ///
    /// Returns `None` if the undo stack is empty.
    #[must_use]
    pub fn undo_description(&self) -> Option<&str> {
        self.undo_stack.back().map(|cmd| cmd.description())
    }

    /// Get a description of the command that would be redone.
    ///
    /// Returns `None` if the redo stack is empty.
    #[must_use]
    pub fn redo_description(&self) -> Option<&str> {
        self.redo_stack.back().map(|cmd| cmd.description())
    }

    /// Get the maximum history size.
    #[must_use]
    pub fn max_history(&self) -> usize {
        self.max_history
    }

    /// Set the maximum history size.
    ///
    /// If the new size is smaller than the current undo stack size,
    /// the oldest commands will be removed.
    pub fn set_max_history(&mut self, max_history: usize) {
        self.max_history = max_history;
        while self.undo_stack.len() > max_history {
            self.undo_stack.pop_front();
        }
    }
}

impl Default for HistoryManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Key, Value};
    use crate::schema::Schema;
    use crate::types::leaf::Text;
    use std::sync::Arc;

    // Simple test command
    #[derive(Debug)]
    struct TestCommand {
        key: Key,
        old_value: Option<Value>,
        new_value: Value,
        description: String,
    }

    impl TestCommand {
        fn new(key: &str, old: Option<Value>, new: Value) -> Self {
            Self {
                key: Key::from(key),
                old_value: old,
                new_value: new,
                description: format!("Set {}", key),
            }
        }
    }

    impl Command for TestCommand {
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }

        fn execute(&mut self, ctx: &mut Context) -> CommandResult {
            ctx.set(self.key.as_str(), self.new_value.clone());
            Ok(())
        }

        fn undo(&mut self, ctx: &mut Context) -> CommandResult {
            if let Some(old) = &self.old_value {
                ctx.set(self.key.as_str(), old.clone());
            } else {
                ctx.clear(self.key.as_str());
            }
            Ok(())
        }

        fn merge(&mut self, _other: &dyn Command) -> bool {
            false
        }

        fn description(&self) -> &str {
            &self.description
        }
    }

    #[test]
    fn test_basic_undo_redo() {
        let schema = Arc::new(
            Schema::builder()
                .parameter(Text::builder("name").build())
                .build(),
        );
        let mut ctx = Context::new(schema);
        let mut history = HistoryManager::new();

        // Execute command
        let cmd = TestCommand::new("name", None, Value::text("Alice"));
        history.execute(cmd, &mut ctx).unwrap();
        assert_eq!(ctx.get("name").unwrap().as_text(), Some("Alice"));
        assert!(history.can_undo());
        assert!(!history.can_redo());

        // Undo
        history.undo(&mut ctx).unwrap();
        assert_eq!(ctx.get("name"), None);
        assert!(!history.can_undo());
        assert!(history.can_redo());

        // Redo
        history.redo(&mut ctx).unwrap();
        assert_eq!(ctx.get("name").unwrap().as_text(), Some("Alice"));
        assert!(history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_max_history() {
        let schema = Arc::new(
            Schema::builder()
                .parameter(Text::builder("name").build())
                .build(),
        );
        let mut ctx = Context::new(schema);
        let mut history = HistoryManager::with_capacity(3);

        // Add 5 commands
        for i in 0..5 {
            let cmd = TestCommand::new("name", None, Value::text(format!("Value {}", i)));
            history.execute(cmd, &mut ctx).unwrap();
        }

        // Should only keep the last 3
        assert_eq!(history.undo_count(), 3);
    }

    #[test]
    fn test_clear() {
        let schema = Arc::new(
            Schema::builder()
                .parameter(Text::builder("name").build())
                .build(),
        );
        let mut ctx = Context::new(schema);
        let mut history = HistoryManager::new();

        let cmd = TestCommand::new("name", None, Value::text("Alice"));
        history.execute(cmd, &mut ctx).unwrap();
        history.undo(&mut ctx).unwrap();

        assert!(history.can_redo());
        history.clear();
        assert!(!history.can_undo());
        assert!(!history.can_redo());
    }
}
