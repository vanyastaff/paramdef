# Change: Add Leaf Parameter Types

## Why
Leaf parameters (Text, Number, Boolean, Vector, Select) are the core terminal value types that users interact with. They represent 5 of the 14 node types and are the foundation for all user-facing data input in the system.

## What Changes
- Add `Text` parameter type with TextSubtype, min/max length, pattern validation, transformers
- Add `Number` parameter type with NumberSubtype, range constraints, unit support, step
- Add `Boolean` parameter type (simple toggle, no subtype needed)
- Add `Vector` parameter type with VectorSubtype, component-wise constraints
- Add `Select` parameter type with unified single/multiple selection, static/dynamic options
- Add builder patterns for all leaf types
- Add convenience constructors (e.g., Text::email(), Number::percentage(), Vector::vector3())

## Impact
- Affected specs: leaf-parameters (new capability)
- Affected code: new `src/parameter/` module with text.rs, number.rs, boolean.rs, vector.rs, select.rs
- Depends on: add-core-types, add-subtypes, add-node-traits
- Required by: add-container-parameters, add-schema-runtime
- Enables: 90%+ of use cases (most parameters are leaf types)
