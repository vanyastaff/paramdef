# Change: Add Reactive System (Events, Observers, Undo/Redo)

## Why
The paramdef library needs reactive capabilities for real-time UI updates, validation feedback, and undo/redo support. This includes an event bus using tokio::broadcast, observer patterns for specific parameters, and a command-based history system.

## What Changes
- Add `Event` enum with all parameter event types
- Add `EventBus` struct using tokio::broadcast (feature = "events")
- Add `Observer` trait and built-in observers (Logger, Validation, Dependency, UI)
- Add `Command` trait for undo/redo operations
- Add `HistoryManager` with undo/redo stacks
- Add `MacroCommand` for transaction batching
- Add event batching and debouncing utilities

## Impact
- Affected specs: reactive-system (new capability)
- Affected code: new `src/events/` and `src/history/` modules
- Depends on: add-schema-runtime (Phase 4)
- Feature-gated: `events` feature required for async capabilities
