## ADDED Requirements

### Requirement: Event Enum
The system SHALL provide an `Event` enum covering all parameter lifecycle events.

#### Scenario: BeforeChange event
- **WHEN** a value is about to change
- **THEN** BeforeChange event contains key, old_value, new_value

#### Scenario: AfterChange event
- **WHEN** a value has changed
- **THEN** AfterChange event contains key, old_value, new_value

#### Scenario: ValidationStarted event
- **WHEN** validation begins
- **THEN** ValidationStarted event contains key

#### Scenario: ValidationPassed event
- **WHEN** validation succeeds
- **THEN** ValidationPassed event contains key

#### Scenario: ValidationFailed event
- **WHEN** validation fails
- **THEN** ValidationFailed event contains key and errors

#### Scenario: VisibilityChanged event
- **WHEN** a parameter's visibility changes
- **THEN** VisibilityChanged event contains key and visible flag

---

### Requirement: EventBus
The system SHALL provide an `EventBus` for central event dispatch using tokio::broadcast.

#### Scenario: EventBus creation
- **WHEN** EventBus::new(capacity) is called
- **THEN** it creates a bus with the specified channel capacity

#### Scenario: EventBus emit
- **WHEN** bus.emit(event) is called
- **THEN** the event is sent to all subscribers

#### Scenario: EventBus subscribe
- **WHEN** bus.subscribe() is called
- **THEN** it returns a broadcast Receiver

#### Scenario: Sync callbacks
- **WHEN** bus.on(callback) is registered
- **THEN** the callback is called synchronously on emit

---

### Requirement: Observer Trait
The system SHALL provide an `Observer` trait for handling events.

#### Scenario: Observer on_event
- **WHEN** an event occurs
- **THEN** observer.on_event(event) is called

#### Scenario: Observer filtering
- **WHEN** an observer is created with a filter
- **THEN** it only receives matching events

---

### Requirement: Built-in Observers
The system SHALL provide built-in observers for common patterns.

#### Scenario: LoggerObserver
- **WHEN** LoggerObserver receives events
- **THEN** it logs them for debugging

#### Scenario: ValidationObserver
- **WHEN** ValidationObserver receives AfterChange
- **THEN** it triggers re-validation

#### Scenario: DependencyObserver
- **WHEN** DependencyObserver receives changes
- **THEN** it updates dependent parameters

---

### Requirement: Command Trait
The system SHALL provide a `Command` trait for undo/redo operations.

#### Scenario: Command execute
- **WHEN** command.execute(context) is called
- **THEN** it applies the operation

#### Scenario: Command undo
- **WHEN** command.undo(context) is called
- **THEN** it reverts the operation

#### Scenario: Command redo
- **WHEN** command.redo(context) is called
- **THEN** it reapplies the operation

#### Scenario: Command merge
- **WHEN** command.merge(other) is called
- **THEN** it returns true if commands were merged (e.g., rapid typing)

---

### Requirement: SetValueCommand
The system SHALL provide a `SetValueCommand` for value changes.

#### Scenario: SetValueCommand stores old and new
- **WHEN** SetValueCommand is created
- **THEN** it stores key, old_value, new_value

#### Scenario: SetValueCommand undo restores old
- **WHEN** SetValueCommand.undo() is called
- **THEN** the old_value is restored

---

### Requirement: HistoryManager
The system SHALL provide a `HistoryManager` for undo/redo stacks.

#### Scenario: HistoryManager push
- **WHEN** history.push(command) is called
- **THEN** the command is added to undo stack

#### Scenario: HistoryManager undo
- **WHEN** history.undo(context) is called
- **THEN** the last command is undone and moved to redo stack

#### Scenario: HistoryManager redo
- **WHEN** history.redo(context) is called
- **THEN** the last undone command is redone

#### Scenario: HistoryManager max history
- **WHEN** max_history limit is reached
- **THEN** oldest commands are dropped

#### Scenario: Redo stack cleared on new action
- **WHEN** a new command is pushed after undo
- **THEN** the redo stack is cleared

---

### Requirement: MacroCommand
The system SHALL provide a `MacroCommand` for batching multiple commands.

#### Scenario: MacroCommand contains commands
- **WHEN** MacroCommand is created with multiple commands
- **THEN** all are stored together

#### Scenario: MacroCommand execute all
- **WHEN** MacroCommand.execute() is called
- **THEN** all contained commands execute in order

#### Scenario: MacroCommand undo all
- **WHEN** MacroCommand.undo() is called
- **THEN** all contained commands undo in reverse order

---

### Requirement: Transaction Support
The system SHALL provide transaction support for atomic operations.

#### Scenario: Begin transaction
- **WHEN** history.begin_transaction("description") is called
- **THEN** subsequent commands are collected

#### Scenario: Commit transaction
- **WHEN** history.commit_transaction() is called
- **THEN** collected commands become a single MacroCommand

#### Scenario: Rollback transaction
- **WHEN** history.rollback_transaction() is called
- **THEN** all transaction commands are undone and discarded

---

### Requirement: Event Batching
The system SHALL support batching multiple changes into single events.

#### Scenario: Begin batch
- **WHEN** context.begin_batch() is called
- **THEN** individual change events are suppressed

#### Scenario: End batch
- **WHEN** context.end_batch() is called
- **THEN** a single BatchUpdate event is emitted

---

### Requirement: Debouncing
The system SHALL provide debouncing utilities for expensive handlers.

#### Scenario: Debounced subscriber
- **WHEN** rapid events occur within debounce window
- **THEN** only the last event triggers the handler
