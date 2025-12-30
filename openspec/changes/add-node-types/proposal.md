# Change: Add Node Types (14 Core Types)

## Why
The paramdef library's core feature is its 14-node type hierarchy. These nodes represent all possible parameter structures: from simple leaves (Text, Number, Boolean) to complex containers (Object, List, Mode) and organizational elements (Group, Panel, Notice).

## What Changes
- Add `Node` base trait
- Add `GroupNode` trait and `Group` type (root aggregator)
- Add `Layout` trait and `Panel` type (UI organization)
- Add `Decoration` trait and `Notice` type (display-only)
- Add `Container` trait and types: `Object`, `List`, `Mode`, `Routing`, `Expirable`, `Ref`
- Add `Leaf` trait and types: `Text`, `Number`, `Boolean`, `Vector`, `Select`
- Add `ValueAccess` trait for nodes with children
- Add builder pattern for all node types

## Impact
- Affected specs: node-types (new capability)
- Affected code: new `src/nodes/` module
- Depends on: add-core-types (Phase 1), add-subtypes (Phase 2)
- **BREAKING**: This defines the entire API surface for schema definition
