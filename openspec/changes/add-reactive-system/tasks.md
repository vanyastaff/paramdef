## 1. Module Structure Setup

- [ ] 1.1 Create `src/events/mod.rs`
- [ ] 1.2 Create `src/events/event.rs` (Event enum)
- [ ] 1.3 Create `src/events/bus.rs` (EventBus)
- [ ] 1.4 Create `src/events/observer.rs` (Observer trait)
- [ ] 1.5 Create `src/events/observers/mod.rs` (built-in observers)
- [ ] 1.6 Create `src/history/mod.rs`
- [ ] 1.7 Create `src/history/command.rs` (Command trait)
- [ ] 1.8 Create `src/history/commands.rs` (SetValueCommand, etc.)
- [ ] 1.9 Create `src/history/manager.rs` (HistoryManager)
- [ ] 1.10 Create `src/history/transaction.rs` (MacroCommand)
- [ ] 1.11 Update `src/lib.rs` to export events and history modules
- [ ] 1.12 Run `cargo check` to verify structure compiles

## 2. Event Enum Implementation

- [ ] 2.1 Write failing test: `test_event_before_change`
- [ ] 2.2 Implement Event enum with BeforeChange variant
- [ ] 2.3 Run test to verify it passes
- [ ] 2.4 Add AfterChange, ValidationStarted, ValidationPassed, ValidationFailed
- [ ] 2.5 Add Touched, Reset, VisibilityChanged, EnabledChanged
- [ ] 2.6 Add ActionTriggered, BatchUpdate
- [ ] 2.7 Write tests for all variants
- [ ] 2.8 Implement Clone for Event
- [ ] 2.9 Add documentation
- [ ] 2.10 Commit: `feat(events): add Event enum`

## 3. EventBus Implementation

- [ ] 3.1 Write failing test: `test_event_bus_creation`
- [ ] 3.2 Implement EventBus struct with tokio::broadcast
- [ ] 3.3 Run test to verify it passes
- [ ] 3.4 Write failing test: `test_event_bus_emit`
- [ ] 3.5 Implement emit() method
- [ ] 3.6 Run test to verify it passes
- [ ] 3.7 Write failing test: `test_event_bus_subscribe`
- [ ] 3.8 Implement subscribe() method
- [ ] 3.9 Run test to verify it passes
- [ ] 3.10 Write failing test: `test_event_bus_sync_callback`
- [ ] 3.11 Implement on() for sync callbacks
- [ ] 3.12 Run test to verify it passes
- [ ] 3.13 Add documentation
- [ ] 3.14 Commit: `feat(events): add EventBus with broadcast`

## 4. Observer Trait

- [ ] 4.1 Write failing test: `test_observer_trait`
- [ ] 4.2 Define Observer trait with on_event method
- [ ] 4.3 Run test to verify it passes
- [ ] 4.4 Add documentation
- [ ] 4.5 Commit: `feat(events): add Observer trait`

## 5. Built-in Observers

- [ ] 5.1 Write failing test: `test_logger_observer`
- [ ] 5.2 Implement LoggerObserver
- [ ] 5.3 Run test to verify it passes
- [ ] 5.4 Write failing test: `test_validation_observer`
- [ ] 5.5 Implement ValidationObserver
- [ ] 5.6 Run test to verify it passes
- [ ] 5.7 Write failing test: `test_dependency_observer`
- [ ] 5.8 Implement DependencyObserver
- [ ] 5.9 Run test to verify it passes
- [ ] 5.10 Add UiObserver for mpsc-based UI updates
- [ ] 5.11 Add documentation
- [ ] 5.12 Commit: `feat(events): add built-in observers`

## 6. Command Trait

- [ ] 6.1 Write failing test: `test_command_trait`
- [ ] 6.2 Define Command trait with execute, undo, redo, merge
- [ ] 6.3 Run test to verify it passes
- [ ] 6.4 Add Send + Sync bounds
- [ ] 6.5 Add documentation
- [ ] 6.6 Commit: `feat(history): add Command trait`

## 7. SetValueCommand

- [ ] 7.1 Write failing test: `test_set_value_command_execute`
- [ ] 7.2 Implement SetValueCommand struct
- [ ] 7.3 Run test to verify it passes
- [ ] 7.4 Write failing test: `test_set_value_command_undo`
- [ ] 7.5 Implement undo() restoring old_value
- [ ] 7.6 Run test to verify it passes
- [ ] 7.7 Write failing test: `test_set_value_command_redo`
- [ ] 7.8 Implement redo()
- [ ] 7.9 Run test to verify it passes
- [ ] 7.10 Write failing test: `test_set_value_command_merge`
- [ ] 7.11 Implement merge() for rapid edits
- [ ] 7.12 Run test to verify it passes
- [ ] 7.13 Add documentation
- [ ] 7.14 Commit: `feat(history): add SetValueCommand`

## 8. HistoryManager

- [ ] 8.1 Write failing test: `test_history_manager_push`
- [ ] 8.2 Implement HistoryManager with undo/redo stacks
- [ ] 8.3 Run test to verify it passes
- [ ] 8.4 Write failing test: `test_history_manager_undo`
- [ ] 8.5 Implement undo() method
- [ ] 8.6 Run test to verify it passes
- [ ] 8.7 Write failing test: `test_history_manager_redo`
- [ ] 8.8 Implement redo() method
- [ ] 8.9 Run test to verify it passes
- [ ] 8.10 Write failing test: `test_history_max_limit`
- [ ] 8.11 Implement max_history limit
- [ ] 8.12 Run test to verify it passes
- [ ] 8.13 Write failing test: `test_redo_cleared_on_new_action`
- [ ] 8.14 Verify redo stack clears on push
- [ ] 8.15 Run test to verify it passes
- [ ] 8.16 Add can_undo(), can_redo() methods
- [ ] 8.17 Add documentation
- [ ] 8.18 Commit: `feat(history): add HistoryManager`

## 9. MacroCommand and Transactions

- [ ] 9.1 Write failing test: `test_macro_command_execute`
- [ ] 9.2 Implement MacroCommand struct
- [ ] 9.3 Run test to verify it passes
- [ ] 9.4 Write failing test: `test_macro_command_undo`
- [ ] 9.5 Implement undo() in reverse order
- [ ] 9.6 Run test to verify it passes
- [ ] 9.7 Write failing test: `test_begin_transaction`
- [ ] 9.8 Implement begin_transaction() on HistoryManager
- [ ] 9.9 Run test to verify it passes
- [ ] 9.10 Write failing test: `test_commit_transaction`
- [ ] 9.11 Implement commit_transaction()
- [ ] 9.12 Run test to verify it passes
- [ ] 9.13 Write failing test: `test_rollback_transaction`
- [ ] 9.14 Implement rollback_transaction()
- [ ] 9.15 Run test to verify it passes
- [ ] 9.16 Add documentation
- [ ] 9.17 Commit: `feat(history): add MacroCommand and transactions`

## 10. Event Batching

- [ ] 10.1 Write failing test: `test_begin_batch`
- [ ] 10.2 Implement begin_batch() on Context
- [ ] 10.3 Run test to verify it passes
- [ ] 10.4 Write failing test: `test_end_batch`
- [ ] 10.5 Implement end_batch() emitting BatchUpdate
- [ ] 10.6 Run test to verify it passes
- [ ] 10.7 Write failing test: `test_batch_suppresses_individual_events`
- [ ] 10.8 Verify individual events are suppressed during batch
- [ ] 10.9 Run test to verify it passes
- [ ] 10.10 Add documentation
- [ ] 10.11 Commit: `feat(events): add event batching`

## 11. Debouncing

- [ ] 11.1 Write failing test: `test_debounced_subscriber`
- [ ] 11.2 Implement DebouncedSubscriber
- [ ] 11.3 Run test to verify it passes
- [ ] 11.4 Write failing test: `test_debounce_rapid_events`
- [ ] 11.5 Verify only last event fires after delay
- [ ] 11.6 Run test to verify it passes
- [ ] 11.7 Add documentation
- [ ] 11.8 Commit: `feat(events): add debouncing utilities`

## 12. Final Verification

- [ ] 12.1 Run `cargo fmt --all`
- [ ] 12.2 Run `cargo clippy --workspace --all-features -- -D warnings`
- [ ] 12.3 Run `cargo test --workspace`
- [ ] 12.4 Run `cargo test --workspace --features events`
- [ ] 12.5 Run `cargo doc --no-deps --all-features`
- [ ] 12.6 Run `cargo +1.85 check --workspace` (MSRV)
- [ ] 12.7 Verify test coverage is 85%+
- [ ] 12.8 Commit: `chore: verify Phase 5 complete`
