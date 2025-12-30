# Project Context

## Purpose
`paramdef` is a type-safe parameter definition system for Rust, inspired by Blender RNA, Unreal Engine UPROPERTY, and Qt Property System. The goal is to create the "serde of parameter schemas" - a production-ready library for workflow engines, visual programming tools, no-code platforms, and game engines.

**Current Status:** Early development - core structure and documentation complete, implementation in progress.

## Tech Stack
- **Language:** Rust (Edition 2024, MSRV 1.85)
- **Core Dependencies:** smartstring, thiserror, bitflags
- **Optional Features:** serde, tokio (events), fluent (i18n), chrono
- **Build Tools:** cargo, LLD linker, cargo-audit, cargo-deny
- **CI:** GitHub Actions (check, test, clippy, fmt, doc, msrv)

## Project Conventions

### Code Style
- **Formatting:** rustfmt.toml (Edition 2024, max_width 100, Unix newlines)
- **Linting:** clippy.toml (MSRV 1.85, deny warnings, missing-docs enforcement)
- **Cognitive complexity threshold:** 25
- **Type complexity threshold:** 250
- **Naming:** Clean names (e.g., `Text` not `TextParameter`)
- **Boolean prefixes:** `show_`, `use_`, `is_`, `has_`, `enable_`, `hide_`
- **Zero warnings in CI:** RUSTFLAGS=-Dwarnings

### Architecture Patterns
**Three-Layer Architecture:**
1. **Schema Layer (Immutable):** Parameter definitions shared via Arc - metadata, flags, validators
2. **Runtime Layer (Mutable):** Per-instance state - values, state flags (dirty, touched, valid)
3. **Value Layer:** Unified Value enum for all parameter types

**Node Hierarchy (14 Core Types):**
- Group (1): Root aggregator, no own value, has ValueAccess
- Layout (1): UI organization (Panel), no own value, has ValueAccess
- Decoration (1): Display-only (Notice), no value, no children
- Container (6): Object, List, Mode, Routing, Expirable, Ref - own value + children
- Leaf (5): Text, Number, Boolean, Vector, Select - own value, no children

**Design Principles:**
- Composition over proliferation (14 base types + subtypes + flags = thousands of combinations)
- Separation of concerns (subtype = semantic meaning, unit = measurement system)
- Trait-based subtypes for compile-time safety
- Type-safe API without const generics
- No UI coupling in core (must work headless)

### Testing Strategy
- **Target Coverage:** Core types 95%+, Parameter types 90%+, Overall 90%+
- **Test-only allowances:** `expect`, `unwrap`, `dbg!`, `print!` (enforced by Clippy)
- **Feature combinations:** Test default, visibility, validation, serde, full features
- **CI testing:** All feature combinations + MSRV compatibility

### Git Workflow
- **Main branch:** `main`
- **Commit style:** Conventional commits preferred
- **Before committing:**
  - `cargo fmt --all`
  - `cargo clippy --workspace --all-features -- -D warnings`
  - Test all feature combinations
  - `cargo doc --no-deps --all-features`
  - `cargo +1.85 check --workspace` (MSRV)

## Domain Context
**Parameter System Design:**
- **Soft vs Hard Constraints:** Hard = validated (MUST be in range), Soft = UI hints (can type beyond)
- **Subtype vs Unit Pattern (Blender-style):** 60 subtypes × 17 unit categories
- **No BooleanSubtype:** Too simple, use naming conventions instead
- **No validation in Value enum:** Validation rules belong in schema layer
- **User-managed localization:** Library provides Fluent keys, user provides translations

**Industry Inspiration:**
- Blender RNA: Property system architecture, subtype+unit pattern
- Unreal Engine UPROPERTY: Metadata and flags system
- Qt Property System: Signals and observers, reactive updates
- Houdini Parameters: Node-based workflows, soft/hard constraints
- n8n: Mode/branching for discriminated unions

## Important Constraints
- **MSRV:** Rust 1.85 (must maintain compatibility)
- **Zero UI dependencies:** Core library must work headless (servers, CLI)
- **Zero validation library dependencies:** Provide trait, users choose implementation
- **No embedded translations:** i18n feature provides keys only, users manage translations
- **All public APIs must have documentation:** Enforced by Clippy
- **No wildcard imports:** Enforced by Clippy
- **Performance targets:** SmallString <23 bytes on stack, Arc for cheap cloning, lazy compilation

## External Dependencies
**Production Dependencies:**
- `smartstring` - Stack-allocated short strings
- `thiserror` - Error derive macros
- `bitflags` - Type-safe bitfield flags
- `serde` + `serde_json` (optional) - Serialization
- `tokio` (optional) - Event system with broadcast channels
- `fluent` (optional) - Mozilla Fluent localization
- `chrono` (optional) - Date/time conversions

**No external validation libraries** - users bring their own (garde, validator, custom)
