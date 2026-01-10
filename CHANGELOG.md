# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-01-09

### Added

- **Expression Parser** - Parse validation rules from strings
  - Lexer with comprehensive tokenization (12 tests)
  - Recursive descent parser with proper precedence (17 tests)
  - Support for comparisons: `==`, `!=`, `<`, `<=`, `>`, `>=`
  - Support for function calls: `email()`, `min_length(5)`, `starts_with("foo")`
  - Support for logical operators: `AND`, `OR`, `NOT`
  - Support for parentheses and nested expressions
  - Case-insensitive keywords (AND/and/And all work)
  - String escape sequences: `\n`, `\t`, `\r`, `\"`, `\\`
  - Error handling with clear error messages
  - Public API: `Expr::parse()` and `Rule::parse()`
  - Example: `examples/15_expression_parser.rs`

- **Parser Capabilities Documentation**
  - Comprehensive analysis of parser features (docs/24-PARSER-CAPABILITIES.md)
  - Coverage analysis: ~80% of common validation use cases
  - Real-world usage examples (config files, UI builders, database-driven validation)
  - Future enhancement roadmap with priorities

### Changed

- **Feature Gates for Examples**
  - Added `required-features` to Cargo.toml for examples 11-15
  - Examples now compile correctly with `--no-default-features`
  - Prevents build errors when optional features are disabled

### Fixed

- **Clippy Warnings** - Zero warnings with `--all-features`
  - Added documentation backticks for token types
  - Made helper methods static (removed unused `self`)
  - Changed arguments to pass-by-reference (avoid unnecessary clones)
  - Modernized format strings with inline variables (`format!("{op:?}")`)
  - Reduced nesting using let-else pattern
  - Simplified function return types
  - Removed unused imports and mut bindings

- **Documentation**
  - Fixed module-level doctest to use `no_run` for feature-gated code
  - All doctests now pass (71 doctests passing)

### Performance

- Parser performance (preliminary):
  - Tokenization: ~1-5µs for typical rule strings
  - Parsing: ~2-10µs for simple rules, ~20-50µs for complex nested rules
  - Zero allocation for small string literals (<23 bytes via SmartString)

### Documentation

- Updated README.md to v0.3.0 with parser examples
- Updated ROADMAP.md to Phase 7 complete (v1.3)
- Added docs/23-EXPRESSION-PARSER.md with technical details
- Added docs/24-PARSER-CAPABILITIES.md with coverage analysis

### Testing

- **645 unit tests** passing (was 598)
- **71 doctests** passing
- **29 new parser tests** (lexer + parser + integration)
- All feature combinations tested (default, visibility, validation, serde, events, full)

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
