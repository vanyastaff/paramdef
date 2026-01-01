# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-01-01

### BREAKING CHANGES

- **Removed all deprecated module re-exports** from `lib.rs`
  - `paramdef::parameter` → Use `paramdef::types::leaf` instead
  - `paramdef::node` → Use `paramdef::types::traits` instead
  - `paramdef::container` → Use `paramdef::types::container` instead
  - `paramdef::decoration` → Use `paramdef::types::decoration` instead
  - `paramdef::group` → Use `paramdef::types::group` instead
  - `paramdef::subtypes` → Use `paramdef::subtype` instead (renamed module)

- **Deleted legacy directory structures**
  - Removed `src/parameter/` (moved to `src/types/leaf/`)
  - Removed `src/node/` (moved to `src/types/traits/`)
  - Removed `src/container/` (moved to `src/types/container/`)
  - Removed `src/decoration/` (moved to `src/types/decoration/`)
  - Removed `src/group/` (moved to `src/types/group/`)

### Migration Guide

Update your imports as follows:

```rust
// Before (v0.1.x)
use paramdef::parameter::{Text, Number, Boolean};
use paramdef::subtypes::NumberUnit;
use paramdef::node::Node;

// After (v0.2.0+)
use paramdef::types::leaf::{Text, Number, Boolean};
use paramdef::subtype::NumberUnit;
use paramdef::types::traits::Node;
```

### Performance

- Benchmarks show excellent performance characteristics:
  - Schema creation: ~100-500ns per parameter
  - Context with 100 parameters: ~50µs initialization
  - Runtime node creation: ~200ns per node
  - Container operations: ~2-10µs for nested structures

## [0.1.1] - 2025-12-31

### Added

- Initial project setup
- Core type system with 14 node types
- Three-layer architecture (Schema, Runtime, Value)
- Comprehensive documentation (18 design documents)
- Benchmark suite for performance testing
