//! Built-in command implementations.

use super::command::{Command, CommandResult};
use crate::context::Context;
use crate::core::{Key, Value};
use std::any::Any;
use std::fmt;

/// Command to set a parameter value.
///
/// This is the most common command type, used for any value change operation.
///
/// ## Command Merging
///
/// `SetValueCommand` supports merging with other `SetValueCommand`s for the same key.
/// This is useful for operations like typing, where each character creates a new command.
///
/// ## Example
///
/// ```
/// use paramdef::history::{SetValueCommand, HistoryManager};
/// use paramdef::core::Value;
/// # use paramdef::context::Context;
/// # use paramdef::schema::Schema;
/// # use paramdef::types::leaf::Text;
/// # use std::sync::Arc;
///
/// # let schema = Arc::new(Schema::builder()
/// #     .parameter(Text::builder("name").build())
/// #     .build());
/// # let mut ctx = Context::new(schema);
/// # let mut history = HistoryManager::new();
/// let cmd = SetValueCommand::new("name", None, Value::text("Alice"));
/// history.execute(cmd, &mut ctx).unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct SetValueCommand {
    key: Key,
    old_value: Option<Value>,
    new_value: Value,
}

impl SetValueCommand {
    /// Create a new set value command.
    ///
    /// # Arguments
    ///
    /// * `key` - The parameter key to modify
    /// * `old_value` - The current value (before the change)
    /// * `new_value` - The new value to set
    pub fn new(key: impl Into<Key>, old_value: Option<Value>, new_value: Value) -> Self {
        Self {
            key: key.into(),
            old_value,
            new_value,
        }
    }

    /// Get the key being modified.
    #[must_use]
    pub fn key(&self) -> &Key {
        &self.key
    }

    /// Get the old value.
    #[must_use]
    pub fn old_value(&self) -> Option<&Value> {
        self.old_value.as_ref()
    }

    /// Get the new value.
    #[must_use]
    pub fn new_value(&self) -> &Value {
        &self.new_value
    }
}

impl Command for SetValueCommand {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn execute(&mut self, ctx: &mut Context) -> CommandResult {
        ctx.set(self.key.as_str(), self.new_value.clone())?;
        Ok(())
    }

    fn undo(&mut self, ctx: &mut Context) -> CommandResult {
        if let Some(old) = &self.old_value {
            ctx.set(self.key.as_str(), old.clone())?;
        } else {
            ctx.clear(self.key.as_str())?;
        }
        Ok(())
    }

    fn merge(&mut self, other: &dyn Command) -> bool {
        // Try to downcast to SetValueCommand
        if let Some(other) = other.as_any().downcast_ref::<SetValueCommand>() {
            // Only merge if same key
            if self.key == other.key {
                // Update new_value, keep original old_value
                self.new_value = other.new_value.clone();
                return true;
            }
        }
        false
    }

    fn can_merge_with(&self, other: &dyn Command) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<SetValueCommand>() {
            self.key == other.key
        } else {
            false
        }
    }

    fn description(&self) -> &'static str {
        "Set value"
    }
}

/// Command to clear a parameter value.
///
/// ## Example
///
/// ```
/// use paramdef::history::{ClearValueCommand, HistoryManager};
/// use paramdef::core::Value;
/// # use paramdef::context::Context;
/// # use paramdef::schema::Schema;
/// # use paramdef::types::leaf::Text;
/// # use std::sync::Arc;
///
/// # let schema = Arc::new(Schema::builder()
/// #     .parameter(Text::builder("name").build())
/// #     .build());
/// # let mut ctx = Context::new(schema);
/// # ctx.set("name", Value::text("Alice"));
/// # let mut history = HistoryManager::new();
/// let cmd = ClearValueCommand::new("name", Some(Value::text("Alice")));
/// history.execute(cmd, &mut ctx).unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct ClearValueCommand {
    key: Key,
    old_value: Option<Value>,
}

impl ClearValueCommand {
    /// Create a new clear value command.
    ///
    /// # Arguments
    ///
    /// * `key` - The parameter key to clear
    /// * `old_value` - The current value (before clearing)
    pub fn new(key: impl Into<Key>, old_value: Option<Value>) -> Self {
        Self {
            key: key.into(),
            old_value,
        }
    }

    /// Get the key being cleared.
    #[must_use]
    pub fn key(&self) -> &Key {
        &self.key
    }

    /// Get the old value.
    #[must_use]
    pub fn old_value(&self) -> Option<&Value> {
        self.old_value.as_ref()
    }
}

impl Command for ClearValueCommand {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn execute(&mut self, ctx: &mut Context) -> CommandResult {
        ctx.clear(self.key.as_str())?;
        Ok(())
    }

    fn undo(&mut self, ctx: &mut Context) -> CommandResult {
        if let Some(old) = &self.old_value {
            ctx.set(self.key.as_str(), old.clone())?;
        }
        Ok(())
    }

    fn merge(&mut self, _other: &dyn Command) -> bool {
        false // Clear commands don't merge
    }

    fn description(&self) -> &'static str {
        "Clear value"
    }
}

/// Command to touch a parameter (mark as touched without changing value).
///
/// ## Example
///
/// ```
/// use paramdef::history::{TouchCommand, HistoryManager};
/// # use paramdef::context::Context;
/// # use paramdef::schema::Schema;
/// # use paramdef::types::leaf::Text;
/// # use std::sync::Arc;
///
/// # let schema = Arc::new(Schema::builder()
/// #     .parameter(Text::builder("name").build())
/// #     .build());
/// # let mut ctx = Context::new(schema);
/// # let mut history = HistoryManager::new();
/// let cmd = TouchCommand::new("name", false);
/// history.execute(cmd, &mut ctx).unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct TouchCommand {
    key: Key,
    was_touched: bool,
}

impl TouchCommand {
    /// Create a new touch command.
    ///
    /// # Arguments
    ///
    /// * `key` - The parameter key to touch
    /// * `was_touched` - Whether the parameter was already touched before this command
    pub fn new(key: impl Into<Key>, was_touched: bool) -> Self {
        Self {
            key: key.into(),
            was_touched,
        }
    }

    /// Get the key being touched.
    #[must_use]
    pub fn key(&self) -> &Key {
        &self.key
    }
}

impl Command for TouchCommand {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn execute(&mut self, ctx: &mut Context) -> CommandResult {
        ctx.touch(self.key.as_str())?;
        Ok(())
    }

    fn undo(&mut self, _ctx: &mut Context) -> CommandResult {
        if !self.was_touched {
            // If it wasn't touched before, we should untouch it
            // This requires Context to support untouch (not currently implemented)
            // For now, we accept that undo won't fully restore the touched state
        }
        Ok(())
    }

    fn merge(&mut self, _other: &dyn Command) -> bool {
        false // Touch commands don't merge
    }

    fn description(&self) -> &'static str {
        "Touch parameter"
    }
}

/// A macro command that groups multiple commands into a single transaction.
///
/// This is useful for operations that logically belong together, such as:
/// - Updating multiple related fields
/// - Complex transformations that require multiple steps
/// - Batch operations
///
/// ## Example
///
/// ```
/// use paramdef::history::{MacroCommand, SetValueCommand, HistoryManager};
/// use paramdef::core::Value;
/// # use paramdef::context::Context;
/// # use paramdef::schema::Schema;
/// # use paramdef::types::leaf::Text;
/// # use std::sync::Arc;
///
/// # let schema = Arc::new(Schema::builder()
/// #     .parameter(Text::builder("first_name").build())
/// #     .parameter(Text::builder("last_name").build())
/// #     .build());
/// # let mut ctx = Context::new(schema);
/// # let mut history = HistoryManager::new();
/// let cmd = MacroCommand::new("Set full name")
///     .with_command(SetValueCommand::new("first_name", None, Value::text("Alice")))
///     .with_command(SetValueCommand::new("last_name", None, Value::text("Smith")));
///
/// history.execute(cmd, &mut ctx).unwrap();
/// // Undo will revert both changes
/// ```
pub struct MacroCommand {
    commands: Vec<Box<dyn Command>>,
    description: String,
}

impl MacroCommand {
    /// Create a new macro command with a description.
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            commands: Vec::new(),
            description: description.into(),
        }
    }

    /// Add a command to this macro (builder pattern).
    #[must_use]
    pub fn with_command<C: Command + 'static>(mut self, cmd: C) -> Self {
        self.commands.push(Box::new(cmd));
        self
    }

    /// Add a command to this macro (builder style).
    pub fn push<C: Command + 'static>(&mut self, cmd: C) {
        self.commands.push(Box::new(cmd));
    }

    /// Get the number of commands in this macro.
    #[must_use]
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    /// Check if this macro is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
}

impl fmt::Debug for MacroCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MacroCommand")
            .field("description", &self.description)
            .field("command_count", &self.commands.len())
            .finish()
    }
}

impl Command for MacroCommand {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn execute(&mut self, ctx: &mut Context) -> CommandResult {
        for cmd in &mut self.commands {
            cmd.execute(ctx)?;
        }
        Ok(())
    }

    fn undo(&mut self, ctx: &mut Context) -> CommandResult {
        // Undo in reverse order
        for cmd in self.commands.iter_mut().rev() {
            cmd.undo(ctx)?;
        }
        Ok(())
    }

    fn redo(&mut self, ctx: &mut Context) -> CommandResult {
        for cmd in &mut self.commands {
            cmd.redo(ctx)?;
        }
        Ok(())
    }

    fn merge(&mut self, _other: &dyn Command) -> bool {
        false // Macro commands don't merge
    }

    fn description(&self) -> &str {
        &self.description
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::Schema;
    use crate::types::leaf::Text;
    use std::sync::Arc;

    #[test]
    fn test_set_value_command() {
        let schema = Arc::new(
            Schema::builder()
                .parameter(Text::builder("name").build())
                .build(),
        );
        let mut ctx = Context::new(schema);

        let mut cmd = SetValueCommand::new("name", None, Value::text("Alice"));
        cmd.execute(&mut ctx).unwrap();
        assert_eq!(ctx.get("name").unwrap().as_text(), Some("Alice"));

        cmd.undo(&mut ctx).unwrap();
        assert_eq!(ctx.get("name"), None);

        cmd.redo(&mut ctx).unwrap();
        assert_eq!(ctx.get("name").unwrap().as_text(), Some("Alice"));
    }

    #[test]
    fn test_set_value_merge() {
        let mut cmd1 = SetValueCommand::new("name", None, Value::text("A"));
        let cmd2 = SetValueCommand::new("name", Some(Value::text("A")), Value::text("Alice"));

        assert!(cmd1.can_merge_with(&cmd2));
        assert!(cmd1.merge(&cmd2));

        // After merge, old_value should be None (from cmd1), new_value should be "Alice" (from cmd2)
        assert_eq!(cmd1.old_value, None);
        assert_eq!(cmd1.new_value.as_text(), Some("Alice"));
    }

    #[test]
    fn test_clear_value_command() {
        let schema = Arc::new(
            Schema::builder()
                .parameter(Text::builder("name").build())
                .build(),
        );
        let mut ctx = Context::new(schema);
        ctx.set("name", Value::text("Alice")).unwrap();

        let mut cmd = ClearValueCommand::new("name", Some(Value::text("Alice")));
        cmd.execute(&mut ctx).unwrap();
        assert_eq!(ctx.get("name"), None);

        cmd.undo(&mut ctx).unwrap();
        assert_eq!(ctx.get("name").unwrap().as_text(), Some("Alice"));
    }

    #[test]
    fn test_macro_command() {
        let schema = Arc::new(
            Schema::builder()
                .parameter(Text::builder("first").build())
                .parameter(Text::builder("last").build())
                .build(),
        );
        let mut ctx = Context::new(schema);

        let mut cmd = MacroCommand::new("Set names")
            .with_command(SetValueCommand::new("first", None, Value::text("Alice")))
            .with_command(SetValueCommand::new("last", None, Value::text("Smith")));

        cmd.execute(&mut ctx).unwrap();
        assert_eq!(ctx.get("first").unwrap().as_text(), Some("Alice"));
        assert_eq!(ctx.get("last").unwrap().as_text(), Some("Smith"));

        cmd.undo(&mut ctx).unwrap();
        assert_eq!(ctx.get("first"), None);
        assert_eq!(ctx.get("last"), None);

        cmd.redo(&mut ctx).unwrap();
        assert_eq!(ctx.get("first").unwrap().as_text(), Some("Alice"));
        assert_eq!(ctx.get("last").unwrap().as_text(), Some("Smith"));
    }
}
