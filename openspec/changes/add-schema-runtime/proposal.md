# Change: Add Schema and Runtime Layer

## Why
The paramdef library needs to separate immutable schema definitions from mutable runtime state. Schema defines the structure (shareable via Arc), while Context holds per-instance values and state. This enables one schema with multiple contexts (e.g., multiple form instances).

## What Changes
- Add `Schema` struct with SchemaBuilder for parameter registration
- Add `RuntimeNode<T>` wrapper combining immutable node with mutable state
- Add `Context` struct managing values, states, and event bus
- Add `State` struct tracking per-parameter state

## Impact
- Affected specs: schema-runtime (new capability)
- Affected code: new `src/schema/` and `src/runtime/` modules
- Depends on: add-core-types (Phase 1), add-node-types (Phase 3)
