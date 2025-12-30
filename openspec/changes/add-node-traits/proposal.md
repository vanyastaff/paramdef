# Change: Add Node Trait System

## Why
Before implementing the 14 concrete node types, we need to establish the trait hierarchy that defines their behavior. The node trait system provides the foundation for the five-category architecture: Group, Layout, Decoration, Container, and Leaf.

## What Changes
- Add `Node` base trait (metadata, key, kind)
- Add `ValueAccess` trait for nodes that can access child values
- Add `GroupNode` trait for root aggregators
- Add `Layout` trait for UI organization (Panel)
- Add `Decoration` trait for display-only nodes (Notice)
- Add `Container` trait for data structures with children
- Add `Leaf` trait for terminal values
- Add `Visibility` trait (feature-gated)
- Add `Validatable` trait (feature-gated)
- Add `NodeKind` enum and supporting types

## Impact
- Affected specs: node-system (new capability)
- Affected code: `src/node/mod.rs`, `src/node/traits.rs`
- Required by: All 14 node types (Phase 2) depend on these traits
- Enables: Type-safe polymorphism via trait objects (Arc<dyn Node>)
