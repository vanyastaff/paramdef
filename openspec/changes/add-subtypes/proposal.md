# Change: Add Subtypes and Units System

## Why
The paramdef library needs type-safe subtypes that provide compile-time guarantees. NumberSubtype<T> ensures integer-only subtypes (Port) can't be used with floats. VectorSubtype<N> ensures size constraints (Quaternion needs 4 components). TextSubtype provides semantic meaning for string values.

## What Changes
- Add `NumberSubtype<T>` trait with macros for int-only, float-only, and universal subtypes
- Add `VectorSubtype<const N: usize>` trait with size-constrained implementations
- Add `TextSubtype` trait with pattern and placeholder support
- Add `NumberUnit` enum with unit conversion support
- Add `IntoBuilder` trait for ergonomic subtype-first API
- Add `define_number_subtype!`, `define_vector_subtype!`, `define_text_subtype!` macros

## Impact
- Affected specs: subtypes (new capability)
- Affected code: new `src/subtypes/` module
- Depends on: add-core-types (Phase 1)
