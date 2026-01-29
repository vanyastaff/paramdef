# Changelog

All notable changes to paramdef will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - Code Quality Improvements (Branch: 002-code-quality-improvements)

### Added

#### Phase 2: Foundational Infrastructure
- **UiStateManager**: Separate UI presentation state from immutable schema
  - Panel collapsed states managed in Context instead of schema
  - Per-panel `last_interaction` timestamp tracking
  - Serde support (feature-gated) for state persistence
  - Zero-copy iteration over panel states

#### Phase 3: Immutability Fixes
- **RuntimeParameter<T>**: Proper separation of schema (Arc<T>) and runtime state
  - Immutable schema shared via Arc across contexts
  - Mutable state (StateFlags) in Context
  - Thread-safe schema sharing (Send + Sync)

#### Phase 4: API Ergonomics
- **Convenience constructors** for common parameter types:
  - `Text::email(key)`, `Text::password(key)`, `Text::phone(key)`, `Text::url(key)`
  - `Text::slug(key)`, `Text::uuid(key)`, `Text::textarea(key)`
  - `Number::port(key)`, `Number::percentage(key)`, `Number::year(key)`
  - `Number::opacity(key)`, `Number::count(key)`, `Number::rating_max(key)`
  - `Boolean::required(key)` for mandatory boolean flags
- **Typed getters** on Context:
  - `get_text(key)`, `get_int(key)`, `get_float(key)`, `get_bool(key)`
  - `get_array(key)`, `get_object(key)`, `get_binary(key)`
  - Type-safe access with `Option<T>` returns
- **Fallback getters** with defaults:
  - `get_text_or(key, default)`, `get_int_or(key, default)`
  - `get_bool_or(key, default)`, `get_float_or(key, default)`

#### Phase 5: Performance Optimizations
- **RollbackStorage**: Stack-optimized transactional updates
  - Zero heap allocations for ≤8 field transactions
  - Automatic upgrade to heap HashMap for larger transactions
  - 100% stack storage for 80% of use cases
- **Bulk operations**:
  - `Context::get_many(keys)` - zero-copy bulk value retrieval
  - `Context::set_many_transactional(values)` - atomic multi-field updates with rollback
  - `Context::set_many_partial(values)` - best-effort bulk updates
- **Event system optimization**:
  - Arc<Value> in Event types reduces clones from 3× to 1× per update
  - 66% clone reduction in event-heavy scenarios
  - Batch event emission for transactional updates
- **Zero-copy iterators**:
  - `Context::values()` - all non-null values
  - `Context::dirty_values()` - modified values only
  - `Context::touched_values()` - user-interacted values
  - `Context::valid_values()` / `Context::invalid_values()` - validation state
- **Performance benchmarks**:
  - Transactional updates: 1.77µs for 8 fields, 22.5µs for 100 fields
  - Event overhead: ~86ns per update
  - Throughput: 1.71M fields/sec with events, 4.02M/sec without

#### Phase 6: Documentation
- **COOKBOOK.md**: 12 practical recipes with production-ready examples
  - Simple forms, validation, error handling
  - Complex objects, transactional updates, reactive patterns
  - Performance optimization tips, subtypes and units
- **Improved doc examples**: 71% passing rate (up from 65.8%)
  - Fixed all decoration type examples (9 types)
  - Fixed group type examples (Panel, Group)
  - Corrected import paths and API usage
- **DOC_AUDIT.md**: Comprehensive documentation audit report

### Changed

#### Phase 3: Breaking Changes
- **Panel.collapsed removed**: UI state moved to Context
  - Old: `panel.collapsed()` getter on schema
  - New: `ctx.is_panel_collapsed(key)` on runtime context
  - Migration: Use `ctx.set_panel_collapsed(key, bool)` for UI state
- **PanelBuilder.collapsed()** now sets initial hint only (not schema state)
- **StateFlags refactored**: Separated into persistent flags and transient state

#### Phase 4: API Improvements
- **ValueBuilder**: Enhanced fluent API for complex object construction
  - `.field_if(condition, key, value)` - conditional field addition
  - `.nested(key, builder_fn)` - inline sub-object construction
  - Improved ergonomics for JSON-like structures

#### Phase 5: Performance
- **Context::set_many_transactional**: Now uses RollbackStorage
  - 100% faster for small transactions (zero allocations)
  - Better error messages on rollback
- **Event types**: Use `Arc<Value>` instead of `Value`
  - Reduces memory allocations in event-heavy scenarios
  - Breaking: Event subscribers receive `Arc<Value>`

### Deprecated

- **Panel.collapsed field**: Use `Context::is_panel_collapsed()` instead
- **Direct schema mutation**: All mutations should go through Context

### Removed

None (deprecations provided for smooth migration)

### Fixed

- **Schema immutability**: Eliminated all mutable schema fields
- **Thread safety**: All schema types now properly Send + Sync
- **Doc examples**: Fixed 10+ examples with wrong import paths or API usage
- **Type safety**: Removed unsafe patterns in favor of Arc-based sharing

### Performance

#### Improvements
- **66% reduction** in Value clones for event-enabled contexts
- **Zero allocations** for transactional updates ≤8 fields
- **18% faster** batch updates vs sequential for 50+ fields
- **Linear scaling**: ~221-225ns per field regardless of transaction size

#### Benchmarks (Release mode)
```
Transaction (1 field):    897 ns   (stack)
Transaction (8 fields):  1,769 ns  (stack, 221ns/field)
Transaction (100 fields): 22,520 ns (heap, 225ns/field)
Event overhead:           ~86 ns/update
Throughput (with events): 1.71M fields/second
Throughput (no events):   4.02M fields/second
```

### Documentation

- **Architecture**: Updated to reflect immutability principles
- **Type System**: Documented all 23 node types with examples
- **Performance**: Baseline measurements and optimization guide
- **Cookbook**: 12 practical recipes for common use cases
- **Migration Guide**: Breaking changes and upgrade paths

### Technical Details

#### MSRV
- Minimum Supported Rust Version: **1.92**
- Edition: **2024**
- Enforced via `rust-toolchain.toml`

#### Feature Flags
- `default`: Core types only (zero dependencies)
- `visibility`: Visibility trait and expression system
- `validation`: Validators and validation configuration
- `serde`: Serialization support (with serde 1.0)
- `events`: Event system with tokio broadcast
- `i18n`: Fluent localization support
- `chrono`: Chrono type conversions
- `full`: All features enabled

#### Dependencies
- **Core**: smartstring 1.0, thiserror 2.0, bitflags 2.6, rustc-hash 2.1
- **Optional**: serde 1.0, tokio 1.0, regex 1.11, fluent 0.16, chrono 0.4

---

## [0.3.1] - 2026-01-11

### Fixed
- Minor documentation improvements
- Clippy warnings resolved

---

## [0.3.0] - 2025-12-15

### Added
- Initial public release
- 23 parameter node types
- Schema and Context system
- Runtime state management
- Validation framework (optional)
- Event system (optional)
- Comprehensive examples

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
