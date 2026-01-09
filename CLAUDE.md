<!-- OPENSPEC:START -->
# OpenSpec Instructions

These instructions are for AI assistants working in this project.

Always open `@/openspec/AGENTS.md` when the request:
- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security work
- Sounds ambiguous and you need the authoritative spec before coding

Use `@/openspec/AGENTS.md` to learn:
- How to create and apply change proposals
- Spec format and conventions
- Project structure and guidelines

Keep this managed block so 'openspec update' can refresh the instructions.

<!-- OPENSPEC:END -->

# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`paramdef` is a type-safe parameter definition system for Rust, inspired by Blender RNA, Unreal Engine UPROPERTY, and Qt Property System. The goal is to create the "serde of parameter schemas" - a production-ready library for workflow engines, visual programming tools, no-code platforms, and game engines.

**Current Status:** Active development - Phase 1-5 complete (Event System, Validation, Transformers, History, Visibility), Phase 6+ in progress.

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

### Node Hierarchy (23 Types)

The system defines 23 node types across five categories:

- **Group (2)**: Group, Panel - root aggregators, no own value, have ValueAccess
- **Decoration (8)**: Notice, Separator, Link, Code, Image, Html, Video, Progress - display-only, no value, no children
- **Container (7)**: Object, List, Mode, Matrix, Routing, Expirable, Reference - have own value + children
- **Leaf (6)**: Text, Number, Boolean, Vector, Select, File - have own value, no children

**Key Invariants:**
- Schema is ALWAYS immutable - runtime state lives in Context
- Group and Layout have no own Value - only delegate via ValueAccess API
- Decoration has no Value and no ValueAccess - pure display element
- Container and Leaf have own Value - Container also has ValueAccess
- Mode is structural, produces `{mode, value}` object (discriminated union)

### Separation of Concerns

**Trait-Based Subtypes (compile-time safe):**
- **NumberSubtype<T>**: Constrained by numeric type (int-only, float-only, any)
- **VectorSubtype<N>**: Constrained by vector size (2, 3, 4, etc.)
- **TextSubtype**: All work with String (runtime validation)
- Macros: `define_number_subtype!`, `define_vector_subtype!` for DRY definitions

**Subtype vs Unit Pattern (Blender-style):**
- **Subtype**: Semantic meaning (WHAT it is) - e.g., `Distance`, `Port`, `Position3D`
- **Unit**: Measurement system (HOW to measure) - e.g., `NumberUnit::Length`
- Benefits: Compile-time safety + 60 subtypes × 17 unit categories

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
- 23 base types + subtypes + flags = thousands of combinations
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

### Transform System (Implemented)

Hybrid transformation combining declarative expressions with programmatic transformers (see `docs/21-TRANSFORM-SYSTEM.md`):

```rust
// Declarative transforms (~80% of cases)
let transforms = Transforms::new()
    .trim()
    .lowercase()
    .capitalize();

// Programmatic transformation (complex cases)
let pipeline = Transforms::new()
    .push(Transform::Trim)
    .custom(PhoneFormatter)
    .func("custom", |v| v.clone());

// Apply transformations
let result = transforms.apply(&value);
```

**Built-in:** `Trim`, `Lowercase`, `Uppercase`, `Capitalize`, `Clamp`, `Round`, `Truncate`, `Replace`, `Default`.

**Industry patterns:**
- `parse`/`format` pipeline (React Final Form)
- Method chaining (Express Validator)
- Normalize before validate (OWASP)

### Validation System (Implemented)

Hybrid validation combining declarative expressions with programmatic validators (see `docs/20-VALIDATION-SYSTEM.md`):

```rust
// Declarative validation (~80% of cases)
let rules = Rules::from_rules([
    Rule::required(),
    Rule::min_length(3),
    Rule::max_length(50),
    Rule::email(),
]);

// Programmatic validation (complex cases)
let custom = Rule::custom("password_match", |value, ctx| {
    let confirm = ctx.get("confirm_password");
    if value != confirm {
        return Err(Error::custom("mismatch", "Passwords must match").into());
    }
    Ok(())
});

// Cross-field validation via ValidationContext
pub trait Validator: Send + Sync + Debug {
    fn validate(&self, value: &Value, ctx: &ValidationContext<'_>) -> ValidationResult;
}
```

**Built-in validators:** `Required`, `Length`, `Range`, `Match`, `PasswordStrength`, `When` (conditional).

**Industry patterns:**
- Declarative expressions (JSON Schema, Zod)
- Cross-field validation (Yup `.when()`)
- Resolver pattern (React Hook Form)
- Thread-local regex cache (performance)

### Event System (Implemented)

Uses `tokio::broadcast` for EventBus (see `docs/19-EVENT-SYSTEM.md`):

```rust
// Event types
pub enum Event {
    ValueChanging { key, old_value, new_value },
    ValueChanged { key, old_value, new_value },
    ValueCleared { key, old_value },
    Validated { key, is_valid, errors },
    Touched { key },
    Dirtied { key },
    Cleaned { key },
    Reset { key },
    BatchBegin { id, description },
    BatchEnd { id },
    ContextReset,
    AllCleaned,
}

// Usage
let bus = EventBus::new(64);
let mut sub = bus.subscribe();
let ctx = Context::with_event_bus(schema, bus);

ctx.set("name", Value::text("Alice")); // Emits events

// Async receive
while let Ok(event) = sub.recv().await {
    match event {
        Event::ValueChanged { key, .. } => println!("{} changed", key),
        _ => {}
    }
}
```

**Industry patterns implemented:**
- `ValueChanging`/`ValueChanged` pair (SurveyJS)
- Batching with `BatchBegin`/`BatchEnd` (MobX transactions)
- `Touched` state (Formik)
- RAII Subscription cleanup (MobX disposer)

### History System (Implemented)

Command pattern for undo/redo (see `docs/22-HISTORY-SYSTEM.md`):

```rust
use paramdef::history::{HistoryManager, SetValueCommand};

let mut history = HistoryManager::new();

// Execute a command (stores in undo stack)
let cmd = SetValueCommand::new("name", None, Value::text("Alice"));
history.execute(cmd, &mut ctx).unwrap();

// Undo the change
history.undo(&mut ctx).unwrap();

// Redo the change
history.redo(&mut ctx).unwrap();
```

**Built-in commands:** `SetValueCommand`, `ClearValueCommand`, `TouchCommand`, `MacroCommand` (transactions).

**Industry patterns implemented:**
- Command pattern (Qt Undo Framework)
- Command merging (Photoshop) - ~100 bytes per command vs ~10KB per snapshot
- Transaction grouping (Text Editors)
- Memory-efficient delta storage (Game Engines)

### Visibility System (Implemented)

Conditional display with declarative expressions (see `docs/23-VISIBILITY-SYSTEM.md`):

```rust
use paramdef::visibility::Expr;

// Simple condition
let expr = Expr::is_true("show_advanced");
assert_eq!(expr.eval(&ctx), true);

// Compound logic: (premium AND age >= 18) OR admin
let expr = Expr::or(vec![
    Expr::and(vec![
        Expr::is_true("premium"),
        Expr::gte("age", 18.0),
    ]),
    Expr::is_true("admin"),
]);

// Get dependencies for reactive updates
let deps = expr.dependencies(); // ["premium", "age", "admin"]
```

**Expression types:** `Eq`, `Ne`, `Lt`, `Gt`, `Lte`, `Gte`, `IsSet`, `IsEmpty`, `IsTrue`, `IsFalse`, `IsValid`, `OneOf`, `Contains`, `And`, `Or`, `Not`.

**Industry patterns implemented:**
- Conditional schemas (JSON Schema)
- Field dependencies (React Hook Form)
- Dynamic form controls (Angular Forms)
- Type-safe evaluation with graceful fallbacks

## Key Documentation Files

Essential reading in `docs/`:
- `01-ARCHITECTURE.md` - Core design decisions and philosophy
- `02-TYPE-SYSTEM.md` - Complete reference for all node types
- `17-DESIGN-DECISIONS.md` - Rationale for major architectural choices
- `18-ROADMAP.md` - Implementation plan and milestones
- `19-EVENT-SYSTEM.md` - Event system documentation
- `20-VALIDATION-SYSTEM.md` - Hybrid validation system documentation
- `21-TRANSFORM-SYSTEM.md` - Transformation system documentation
- `22-HISTORY-SYSTEM.md` - Undo/redo system documentation
- `23-VISIBILITY-SYSTEM.md` - Conditional display documentation

**Reading Guide for Full Understanding:**
1. README.md (this overview)
2. docs/01-ARCHITECTURE.md (30 min)
3. docs/02-TYPE-SYSTEM.md (30 min)
4. docs/17-DESIGN-DECISIONS.md (20 min)
5. docs/19-EVENT-SYSTEM.md (15 min) - for reactive features

## Common Patterns

### Adding a New Node Type

1. Must fit into one of 5 categories (Group, Layout, Decoration, Container, Leaf)
2. Implement the `Node` trait + category-specific trait
3. If has own Value: implement `Validatable` trait (if validation feature enabled)
4. If can contain children: implement `ValueAccess` trait
5. All types implement `Visibility` trait (if visibility feature enabled)
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
│   ├── lib.rs          # Main library entry
│   ├── core/           # Key, Value, Metadata, Flags, Error
│   ├── types/          # 23 node types (leaf, container, group, decoration)
│   ├── subtype/        # TextSubtype, NumberSubtype, VectorSubtype, Unit
│   ├── schema/         # Schema builder and storage
│   ├── context/        # Runtime context with event integration
│   ├── runtime/        # RuntimeNode, State
│   ├── event/          # Event, EventBus, Subscription (events feature)
│   ├── validation/     # Expr, Rule, Validator, validators (validation feature)
│   └── prelude.rs      # Common imports
├── docs/               # 20 comprehensive design documents
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
- `regex` - Pattern validation (validation feature)
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
