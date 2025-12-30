# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`paramdef` is a type-safe parameter definition system for Rust, inspired by Blender RNA, Unreal Engine UPROPERTY, and Qt Property System. The goal is to create the "serde of parameter schemas" - a production-ready library for workflow engines, visual programming tools, no-code platforms, and game engines.

**Current Status:** Early development - core structure and documentation complete, implementation in progress.

## Build and Test Commands

### Basic Operations
```bash
# Check project
cargo check --workspace --all-targets

# Run tests (default features)
cargo test --workspace

# Run tests with specific features
cargo test --workspace --no-default-features
cargo test --workspace --features visibility
cargo test --workspace --features validation
cargo test --workspace --features serde
cargo test --workspace --features full

# Build documentation
cargo doc --no-deps --all-features
```

### Code Quality
```bash
# Format code
cargo fmt --all

# Lint with Clippy
cargo clippy --workspace --all-features -- -D warnings

# Check MSRV (1.85)
cargo +1.85 check --workspace
```

### Development Tools
```bash
# Run with LLD linker (faster builds)
cargo build  # Uses .cargo/config.toml settings

# Run security audit
cargo audit

# Check licenses
cargo deny check licenses
```

## Architecture Overview

### Three-Layer Architecture

1. **Schema Layer (Immutable)** - Parameter definitions shared via `Arc`
   - Metadata, flags, validators, transformers
   - Shareable across multiple contexts

2. **Runtime Layer (Mutable)** - Per-instance state
   - Current values, state flags (dirty, touched, valid)
   - Validation errors

3. **Value Layer** - Runtime data representation
   - Unified `Value` enum for all parameter types
   - Serialization target

### Node Hierarchy (14 Core Types)

The system defines exactly 14 node types across five categories:

- **Group (1)**: Root aggregator, no own value, has ValueAccess
- **Layout (1)**: UI organization (Panel), no own value, has ValueAccess
- **Decoration (1)**: Display-only (Notice), no value, no children
- **Container (6)**: Object, List, Mode, Routing, Expirable, Ref - have own value + children
- **Leaf (5)**: Text, Number, Boolean, Vector, Select - have own value, no children

**Key Invariants:**
- Schema is ALWAYS immutable - runtime state lives in Context
- Group and Layout have no own Value - only delegate via ValueAccess API
- Decoration has no Value and no ValueAccess - pure display element
- Container and Leaf have own Value - Container also has ValueAccess
- Mode is structural, produces `{mode, value}` object (discriminated union)

### Separation of Concerns

**Subtype vs Unit Pattern (Blender-style):**
- **Subtype**: Semantic meaning (WHAT it is) - e.g., `NumberSubtype::Distance`
- **Unit**: Measurement system (HOW to measure) - e.g., `NumberUnit::Length`
- Benefits: 60 subtypes × 17 unit categories = thousands of combinations with minimal API

**Soft vs Hard Constraints:**
- **Hard constraints**: Validation enforced (value MUST be in range)
- **Soft constraints**: UI slider hints (user can type beyond)

### Feature Flags

```toml
default = []                  # Core types only
visibility = []               # Visibility trait, Expr
validation = []               # Validators, ValidationConfig
serde = ["dep:serde"]        # Serialization + JSON conversions
events = ["dep:tokio"]       # Event system with broadcast channels
i18n = ["dep:fluent"]        # Fluent localization
chrono = ["dep:chrono"]      # Chrono type conversions
full = ["visibility", "validation", "serde", "events", "i18n", "chrono"]
```

**Design Philosophy:** Core library has zero UI dependencies - works headless (servers, CLI).

## Implementation Guidance

### Code Style and Standards

**Configured via:**
- `rustfmt.toml`: Edition 2024, max_width 100, Unix newlines
- `clippy.toml`: MSRV 1.85, strict linting with missing-docs enforcement
- `deny.toml`: License checking, security advisories

**Requirements:**
- All public APIs must have documentation
- No wildcard imports (enforced by Clippy)
- Cognitive complexity threshold: 25
- Type complexity threshold: 250
- Zero warnings in CI (`RUSTFLAGS=-Dwarnings`)

### Naming Conventions

**Clean names** - Types use `Text`, not `TextParameter`:
```rust
// ✅ Good
pub struct Text { ... }
pub struct Number { ... }

// ❌ Bad
pub struct TextParameter { ... }
pub struct NumberParameter { ... }
```

**Boolean naming** (no BooleanSubtype - use naming conventions):
- Prefixes: `show_`, `use_`, `is_`, `has_`, `enable_`, `hide_`

### Design Patterns

**Composition over proliferation:**
- 14 base types + subtypes + flags = thousands of combinations
- No specialized types like `Password` - use `Text` + `subtype: Secret` + `flags: SENSITIVE`

**Type-safe API without const generics:**
```rust
// Vector uses runtime size (VectorSubtype encodes size)
// Type-safe builders and getters, but flexible schema storage
let position = VectorParameter::vector3("position")
    .default_vec3([0.0, 0.0, 0.0])  // Enforces [f64; 3]
    .build();
```

**Generic RuntimeParameter pattern:**
```rust
pub struct RuntimeParameter<T: Node> {
    node: Arc<T>,      // Immutable schema (shared)
    state: StateFlags, // Mutable state
    errors: Vec<ValidationError>,
}
```

### Validation Integration

The library provides the `Validator` trait but NO built-in validation library dependencies:
```rust
pub trait Validator: Send + Sync {
    fn validate(&self, value: &Value) -> Result<(), ValidationError>;
}
```

**Rationale:** No version conflicts, user controls dependencies, works with any library (garde, validator, custom).

### Event System

Uses `tokio::broadcast` for EventBus:
- Async + sync support
- Multiple subscribers built-in
- Battle-tested by Tokio team
- Already in dependencies (events feature)

**Command Pattern for Undo/Redo:**
- ~100 bytes per command vs ~10KB per snapshot
- Supports command merging (optimization)
- Extensible (custom commands)
- Enables transactions (MacroCommand)

## Key Documentation Files

Essential reading in `docs/`:
- `01-ARCHITECTURE.md` - Core design decisions and philosophy
- `02-TYPE-SYSTEM.md` - Complete reference for all 14 node types
- `17-DESIGN-DECISIONS.md` - Rationale for major architectural choices
- `18-ROADMAP.md` - Implementation plan and milestones

**Reading Guide for Full Understanding:**
1. README.md (this overview)
2. docs/01-ARCHITECTURE.md (30 min)
3. docs/02-TYPE-SYSTEM.md (30 min)
4. docs/17-DESIGN-DECISIONS.md (20 min)

## Common Patterns

### Adding a New Node Type

1. Must fit into one of 5 categories (Group, Layout, Decoration, Container, Leaf)
2. Implement the `Node` trait + category-specific trait
3. If has own Value: implement `Validatable` trait (if validation feature enabled)
4. If can contain children: implement `ValueAccess` trait
5. All 14 types implement `Visibility` trait (if visibility feature enabled)
6. Add builder pattern with `::builder()` constructor

### Adding a New Subtype

1. Choose appropriate enum: `TextSubtype`, `NumberSubtype`, or `VectorSubtype`
2. Add variant to enum (keep alphabetically sorted within semantic groups)
3. Add helper method if needed (e.g., `is_code()`, `is_sensitive()`)
4. Update tests to cover new variant
5. NO need for new node type - use composition!

### Adding a New Flag

1. Add to `Flags` bitflags in order
2. Add convenience method if commonly used together
3. Document behavior and use cases
4. Update `11-FLAGS-REFERENCE.md`

## Development Workflow

### Before Committing

```bash
# Format and lint
cargo fmt --all
cargo clippy --workspace --all-features -- -D warnings

# Test all feature combinations
cargo test --workspace --no-default-features
cargo test --workspace --features visibility
cargo test --workspace --features validation
cargo test --workspace --features full

# Check documentation
cargo doc --no-deps --all-features

# Verify MSRV
cargo +1.85 check --workspace
```

### CI Pipeline

GitHub Actions runs on push/PR:
- `check`: workspace check with all targets
- `test`: default feature tests
- `test-features`: tests for each feature combination
- `fmt`: format checking
- `clippy`: linting with deny warnings
- `doc`: documentation generation with deny warnings
- `msrv`: Rust 1.85 compatibility check

## Project Structure

```
paramdef/
├── src/
│   └── lib.rs          # Main library entry (currently stub)
├── docs/               # 18 comprehensive design documents
├── .cargo/
│   ├── config.toml     # LLD linker configuration
│   └── audit.toml      # Security audit config
├── .github/
│   └── workflows/
│       └── ci.yml      # CI pipeline
├── .claude/
│   └── skills/         # Claude Code skills (Rust development aids)
├── clippy.toml         # Clippy linting configuration
├── deny.toml           # cargo-deny license/security config
├── rustfmt.toml        # rustfmt style configuration
└── Cargo.toml          # Package manifest (Edition 2024, MSRV 1.85)
```

## Dependencies

**Core:**
- `smartstring` - Stack-allocated short strings (<23 bytes)
- `thiserror` - Error derive macros
- `bitflags` - Type-safe bitfield flags

**Optional:**
- `serde` + `serde_json` - Serialization (serde feature)
- `tokio` - Event system with broadcast channels (events feature)
- `fluent` - Mozilla Fluent localization (i18n feature)
- `chrono` - Date/time conversions (chrono feature)

## Performance Considerations

**Optimization techniques:**
- `SmartString<LazyCompact>` - Strings <23 bytes on stack
- `Arc<[Value]>` - Immutable arrays, cheap cloning
- `Arc<HashMap>` - Immutable objects, shared
- Const generics for fixed-size vectors - on stack, no heap
- Thread-local regex cache - avoid recompilation
- Lazy expression compilation - compile on first use
- Fast path checks - skip empty transformer/validator lists

## Testing Philosophy

**Target Coverage:**
- Core types: 95%+
- Parameter types: 90%+
- Overall: 90%+

**Allow in tests only** (enforced by Clippy):
- `expect`, `unwrap`, `dbg!`, `print!`
- Outside tests these trigger warnings

## Localization (i18n Feature)

**User-managed approach:**
- Library provides Fluent keys, user provides translations
- No embedded translations in library (zero binary bloat)
- User controls all languages
- Example: `fluent_id("db-host")` → user's `locales/ru/app.ftl`

## Rejected Alternatives

Important architectural decisions from `17-DESIGN-DECISIONS.md`:

- ❌ NO BooleanSubtype - too simple, Blender doesn't use
- ❌ NO ChoiceSubtype - YAGNI, UI variations are presentation not semantics
- ❌ NO const generics for Vector - type erasure kills benefits, VectorSubtype encodes size
- ❌ NO subtype in Value enum - violates separation of concerns
- ❌ NO validation in Value - validation rules belong in schema
- ❌ NO UI coupling in core - must work headless

## Industry Inspiration

paramdef combines best features from:
- **Blender RNA** - Property system architecture, subtype+unit pattern
- **Unreal Engine UPROPERTY** - Metadata and flags system
- **Qt Property System** - Signals and observers, reactive updates
- **Houdini Parameters** - Node-based workflows, soft/hard constraints
- **n8n** - Mode/branching for discriminated unions

With Rust's type safety and zero-cost abstractions.
