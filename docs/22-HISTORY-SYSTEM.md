# History System

**Undo/redo functionality using the Command pattern**

Version: 1.0  
Feature: Always available (no feature flag)

---

## Overview

The history system provides memory-efficient undo/redo functionality for parameter changes. It uses the **Command pattern** to capture state changes, with support for command merging and transactions.

### Design Influences

| Source | Pattern Adopted |
|--------|-----------------|
| Qt Undo Framework | Command pattern, undo/redo stacks |
| Photoshop | Command merging for optimization |
| Text Editors | Transaction grouping (MacroCommand) |
| Game Engines | Memory-efficient command storage (~100 bytes vs ~10KB snapshots) |

---

## Quick Start

```rust
use paramdef::history::{HistoryManager, SetValueCommand};
use paramdef::context::Context;
use paramdef::core::Value;
use paramdef::schema::Schema;
use paramdef::types::leaf::Text;
use std::sync::Arc;

// Create schema and context
let schema = Arc::new(Schema::builder()
    .parameter(Text::builder("name").build())
    .build());
let mut ctx = Context::new(schema);

// Create history manager
let mut history = HistoryManager::new();

// Execute a command (stores in undo stack)
let cmd = SetValueCommand::new("name", None, Value::text("Alice"));
history.execute(cmd, &mut ctx).unwrap();

assert_eq!(ctx.get("name").unwrap().as_text(), Some("Alice"));

// Undo the change
history.undo(&mut ctx).unwrap();
assert_eq!(ctx.get("name"), None);

// Redo the change
history.redo(&mut ctx).unwrap();
assert_eq!(ctx.get("name").unwrap().as_text(), Some("Alice"));
```

---

## Command Trait

The `Command` trait represents a reversible operation.

```rust
pub trait Command: Send + Sync + Debug {
    fn as_any(&self) -> &dyn Any;
    fn execute(&mut self, ctx: &mut Context) -> CommandResult;
    fn undo(&mut self, ctx: &mut Context) -> CommandResult;
    fn redo(&mut self, ctx: &mut Context) -> CommandResult;
    fn merge(&mut self, other: &dyn Command) -> bool;
    fn can_merge_with(&self, other: &dyn Command) -> bool;
    fn description(&self) -> &str;
}
```

### Implementing Custom Commands

```rust
use paramdef::history::{Command, CommandResult};
use paramdef::context::Context;
use paramdef::core::{Key, Value};
use std::any::Any;

#[derive(Debug)]
struct CustomCommand {
    key: Key,
    old_value: Option<Value>,
    new_value: Value,
}

impl Command for CustomCommand {
    fn as_any(&self) -> &dyn Any {
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

    fn redo(&mut self, ctx: &mut Context) -> CommandResult {
        self.execute(ctx) // Default implementation
    }

    fn merge(&mut self, _other: &dyn Command) -> bool {
        false // No merging by default
    }

    fn can_merge_with(&self, _other: &dyn Command) -> bool {
        false
    }

    fn description(&self) -> &str {
        "Custom command"
    }
}
```

---

## Built-in Commands

### SetValueCommand

Sets a parameter value. Supports merging with other `SetValueCommand`s for the same key.

```rust
use paramdef::history::SetValueCommand;

let cmd = SetValueCommand::new("name", None, Value::text("Alice"));
```

**Merging behavior:**
```rust
// These two commands can be merged:
let cmd1 = SetValueCommand::new("name", None, Value::text("A"));
let cmd2 = SetValueCommand::new("name", Some(Value::text("A")), Value::text("Alice"));

// After merging: old_value = None, new_value = "Alice"
```

### ClearValueCommand

Clears a parameter value.

```rust
use paramdef::history::ClearValueCommand;

let cmd = ClearValueCommand::new("name", Some(Value::text("Alice")));
```

### TouchCommand

Marks a parameter as touched without changing its value.

```rust
use paramdef::history::TouchCommand;

let cmd = TouchCommand::new("name", false);
```

### MacroCommand

Groups multiple commands into a single transaction.

```rust
use paramdef::history::{MacroCommand, SetValueCommand};

let cmd = MacroCommand::new("Set full name")
    .with_command(SetValueCommand::new("first_name", None, Value::text("Alice")))
    .with_command(SetValueCommand::new("last_name", None, Value::text("Smith")));

// Undo will revert both changes together
```

---

## HistoryManager

Manages the undo/redo stacks and provides command execution.

### Configuration

```rust
use paramdef::history::HistoryManager;

// Default: 100 commands, merging enabled
let mut history = HistoryManager::new();

// Custom capacity
let mut history = HistoryManager::with_capacity(200);

// Disable command merging
history.set_merging_enabled(false);

// Adjust max history size
history.set_max_history(50);
```

### Core Operations

```rust
// Execute a command
history.execute(cmd, &mut ctx)?;

// Undo the most recent command
if history.can_undo() {
    history.undo(&mut ctx)?;
}

// Redo the most recently undone command
if history.can_redo() {
    history.redo(&mut ctx)?;
}

// Clear all history
history.clear();
```

### Inspection

```rust
// Check undo/redo availability
if history.can_undo() {
    println!("Can undo: {}", history.undo_description().unwrap());
}

if history.can_redo() {
    println!("Can redo: {}", history.redo_description().unwrap());
}

// Get stack sizes
println!("Undo: {}, Redo: {}", history.undo_count(), history.redo_count());
```

---

## Command Merging

Command merging is an optimization that reduces memory usage by combining sequential similar commands.

### When Merging Happens

1. A new command is executed
2. The manager checks if it can merge with the most recent command
3. If both commands agree to merge, they are combined into one

### Example: Text Typing

Without merging:
```
Command 1: Set "name" from None to "A"
Command 2: Set "name" from "A" to "Al"
Command 3: Set "name" from "Al" to "Ali"
Command 4: Set "name" from "Ali" to "Alic"
Command 5: Set "name" from "Alic" to "Alice"
```

With merging:
```
Merged: Set "name" from None to "Alice"
```

### Implementing Mergeable Commands

```rust
impl Command for MyCommand {
    fn can_merge_with(&self, other: &dyn Command) -> bool {
        // Fast check before attempting merge
        if let Some(other) = other.as_any().downcast_ref::<MyCommand>() {
            self.key == other.key
        } else {
            false
        }
    }

    fn merge(&mut self, other: &dyn Command) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<MyCommand>() {
            if self.key == other.key {
                // Update to final state, preserve original old_value
                self.new_value = other.new_value.clone();
                return true;
            }
        }
        false
    }
}
```

---

## Memory Efficiency

### Command Pattern vs Snapshots

| Approach | Memory per Change | 100 Changes |
|----------|------------------|-------------|
| **Snapshots** | ~10 KB (full context copy) | ~1 MB |
| **Commands** | ~100 bytes (delta only) | ~10 KB |

### Benefits

- **100x less memory**: Only store changes, not full state
- **Selective merging**: Similar commands combine automatically
- **Configurable history**: Limit memory usage with `max_history`
- **Efficient transactions**: `MacroCommand` groups operations

---

## Design Patterns

### 1. Command Pattern (GoF)

Each operation is encapsulated as an object with:
- State needed to execute
- State needed to undo
- Logic for both forward and reverse operations

### 2. Memento Pattern (Lightweight)

Commands store minimal state (deltas) rather than full snapshots.

### 3. Composite Pattern

`MacroCommand` allows composing multiple commands into one.

---

## Best Practices

### 1. Store Minimal State

```rust
// ✅ Good: Only store what's needed
struct SetValueCommand {
    key: Key,
    old_value: Option<Value>,
    new_value: Value,
}

// ❌ Bad: Storing entire context
struct SnapshotCommand {
    entire_context: Context, // Too much memory!
}
```

### 2. Use MacroCommand for Transactions

```rust
// Group related changes
let transaction = MacroCommand::new("Import user data")
    .with_command(SetValueCommand::new("name", None, Value::text("Alice")))
    .with_command(SetValueCommand::new("email", None, Value::text("alice@example.com")))
    .with_command(SetValueCommand::new("age", None, Value::Int(30)));

history.execute(transaction, &mut ctx)?;
// One undo reverts all three changes
```

### 3. Implement Merging for Frequent Operations

```rust
// For operations like typing, dragging, etc.
// Implement merge() to reduce memory usage
impl Command for TextEditCommand {
    fn merge(&mut self, other: &dyn Command) -> bool {
        // Combine sequential edits to same field
        // ...
    }
}
```

### 4. Set Appropriate History Limits

```rust
// For memory-constrained environments
let mut history = HistoryManager::with_capacity(20);

// For rich editing environments
let mut history = HistoryManager::with_capacity(500);
```

---

## Integration with Events

When used with the event system (feature `events`), commands can trigger events:

```rust
#[cfg(feature = "events")]
use paramdef::event::{Event, EventBus};

let bus = EventBus::new(64);
let mut ctx = Context::with_event_bus(schema, bus);
let mut history = HistoryManager::new();

// Commands executed through HistoryManager will emit events
let cmd = SetValueCommand::new("name", None, Value::text("Alice"));
history.execute(cmd, &mut ctx)?;
// Emits: ValueChanging, ValueChanged, Dirtied events
```

---

## Limitations

### 1. TouchCommand Undo

Currently, `TouchCommand::undo()` cannot fully restore the untouched state because `Context` does not support an `untouch()` operation. This is a known limitation.

### 2. External State

Commands only track changes made through the history manager. Direct modifications to `Context` bypass history tracking.

```rust
// ✅ Tracked
history.execute(SetValueCommand::new("name", None, Value::text("Alice")), &mut ctx)?;

// ❌ Not tracked
ctx.set("name", Value::text("Bob")); // Bypasses history!
```

### 3. Cross-Parameter Dependencies

Commands operate independently. Complex cross-parameter validations may require custom commands or `MacroCommand`.

---

## Performance Characteristics

| Operation | Time Complexity | Notes |
|-----------|----------------|-------|
| Execute | O(1) + merge check | Fast path if no merge |
| Undo | O(1) | Stack pop |
| Redo | O(1) | Stack pop |
| Merge check | O(1) | Only checks most recent |
| MacroCommand execute | O(n) | n = number of sub-commands |

---

## Thread Safety

- `Command` trait requires `Send + Sync`
- `HistoryManager` is **not** thread-safe (single-threaded use)
- For concurrent access, wrap in `Mutex` or use per-thread instances

```rust
use std::sync::Mutex;

let history = Arc::new(Mutex::new(HistoryManager::new()));

// Thread-safe access
let mut history = history.lock().unwrap();
history.execute(cmd, &mut ctx)?;
```

---

## Future Enhancements

Potential additions (not yet implemented):

1. **Command Groups**: Named groups for multi-level undo
2. **Branching History**: Support for undo tree (not just stack)
3. **Persistence**: Save/load command history
4. **Command Compression**: Archive old commands to save memory
5. **Context Integration**: `Context::with_history()` for automatic tracking

---

## Example: Rich Text Editor

```rust
use paramdef::history::{HistoryManager, MacroCommand, SetValueCommand};

let mut history = HistoryManager::with_capacity(100);

// User types "Hello"
for ch in ['H', 'e', 'l', 'l', 'o'] {
    let old = ctx.get("text").map(|v| v.clone());
    let new_text = format!("{}{}", old.as_ref().and_then(|v| v.as_text()).unwrap_or(""), ch);
    let cmd = SetValueCommand::new("text", old, Value::text(new_text));
    history.execute(cmd, &mut ctx)?;
}
// With merging enabled, this becomes a single command

// User applies formatting
let format_cmd = MacroCommand::new("Apply bold")
    .with_command(SetValueCommand::new("bold", Some(Value::Bool(false)), Value::Bool(true)))
    .with_command(SetValueCommand::new("selection_start", None, Value::Int(0)))
    .with_command(SetValueCommand::new("selection_end", None, Value::Int(5)));
history.execute(format_cmd, &mut ctx)?;

// Undo formatting (1 step)
history.undo(&mut ctx)?;

// Undo text typing (1 step, thanks to merging)
history.undo(&mut ctx)?;
```
