# Change: Add Core Types Foundation

## Why
The paramdef library currently has no implementation - only documentation. We need to implement the foundational types (Key, Metadata, Flags, StateFlags, Value) that all other components depend on. These are the building blocks for the entire parameter system.

## What Changes
- Add `Key` type (SmartString-based parameter identifier)
- Add `Metadata` struct (label, description, group, tags)
- Add `Flags` bitflags (schema-level: REQUIRED, READONLY, HIDDEN, etc.)
- Add `StateFlags` bitflags (runtime-level: DIRTY, TOUCHED, VALID, etc.)
- Add `Value` enum (unified runtime representation)
- Add `Error` types with thiserror

## Impact
- Affected specs: core-types (new capability)
- Affected code: `src/lib.rs`, new `src/core/` module
- This is the foundation - all subsequent phases depend on these types
