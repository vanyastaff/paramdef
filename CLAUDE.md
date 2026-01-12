# AI Coding Assistant Rules - paramdef

> **Master rules file** for all AI coding assistants  
> Copies: `CLAUDE.md`, `.cursorrules`, `.clinerules`, `.windsurfrules`  
> **Important:** Edit this file and copy to others for consistency

---

<project_context>

## Project Identity

**paramdef** - Type-safe parameter definition system for Rust

- **Vision:** "The serde of parameter schemas"
- **Inspiration:** Blender RNA, Unreal UPROPERTY, Qt Property System
- **Use Cases:** Workflow engines, visual programming, no-code platforms, game engines
- **Status:** Active development - Phases 1-6 complete, Phase 7+ planned

</project_context>

---

<architecture>

## Three-Layer Architecture

### 1. Schema Layer (Immutable)
**Shared via `Arc<T>`** - Never mutated after creation

- Parameter definitions: metadata, flags, validators, transformers
- Shareable across multiple runtime contexts
- **Key trait:** All schema types are `Send + Sync`

### 2. Runtime Layer (Mutable)
**Per-instance state** - Lives in `Context`

- Current values, state flags (dirty, touched, valid)
- Validation errors
- Event subscriptions

### 3. Value Layer
**Runtime data representation**

- Unified `Value` enum for all parameter types
- Serialization/deserialization target
- Type-safe conversions

## Node Hierarchy (23 Types)

### Categories

| Category | Count | Behavior | Examples |
|----------|-------|----------|----------|
| **Group** | 2 | No own value, delegate via `ValueAccess` | Group, Panel |
| **Decoration** | 8 | Display-only, no value, no children | Notice, Separator, Link, Code, Image |
| **Container** | 7 | Own value + children | Object, List, Mode, Matrix, Routing |
| **Leaf** | 6 | Own value, no children | Text, Number, Boolean, Vector, Select, File |

### Key Invariants

```xml
<invariant name="immutable_schema">
Schema is ALWAYS immutable - runtime state lives in Context
</invariant>

<invariant name="value_access">
- Group/Panel: No own Value, delegate via ValueAccess
- Decoration: No Value, no ValueAccess  
- Container/Leaf: Have own Value
</invariant>

<invariant name="mode_structure">
Mode produces `{mode, value}` discriminated union object
</invariant>
```

## Separation of Concerns

### Trait-Based Subtypes (Compile-Time Safe)

```rust
// Type-constrained subtypes
NumberSubtype<T>   // int-only, float-only, any numeric
VectorSubtype<N>   // Size-constrained (2D, 3D, 4D)
TextSubtype        // String-based, runtime validated

// DRY macros for subtype definitions
define_number_subtype!(Distance, "Distance measurement")
define_vector_subtype!(Position3D, 3, "3D position")
```

### Subtype vs Unit Pattern (Blender-Inspired)

- **Subtype:** Semantic meaning (WHAT) - e.g., `Distance`, `Port`, `Position3D`
- **Unit:** Measurement system (HOW) - e.g., `NumberUnit::Length`
- **Benefit:** 60 subtypes × 17 unit categories = rich type system

### Soft vs Hard Constraints

- **Hard:** Validation enforced - value MUST be in range
- **Soft:** UI hints - user can type beyond slider bounds

</architecture>

---

<requirements>

## Technical Requirements

### Rust Version
- **MSRV:** 1.92 (enforced via `rust-toolchain.toml`)
- **Edition:** 2024
- **Toolchain:** `rustfmt`, `clippy` required

### Feature Flags

```toml
default = []                  # Core types only
visibility = []               # Visibility trait, Expr
validation = []               # Validators, ValidationConfig
serde = ["dep:serde"]        # Serialization + JSON
events = ["dep:tokio"]       # Event system with broadcast
i18n = ["dep:fluent"]        # Fluent localization
chrono = ["dep:chrono"]      # Chrono type conversions
full = ["visibility", "validation", "serde", "events", "i18n", "chrono"]
```

**Design Philosophy:** Core library has zero UI dependencies - works headless (servers, CLI)

### Dependencies Philosophy

**Core (always):**
- `smartstring` - Stack strings <23 bytes
- `thiserror` - Error derive macros
- `bitflags` - Type-safe flags

**Optional (feature-gated):**
- `serde` + `serde_json` - Serialization
- `tokio` - Async event system  
- `regex` - Pattern validation
- `fluent` - i18n (user-provided translations)
- `chrono` - Date/time support

</requirements>

---

<code_style>

## Naming Conventions

### Clean Names - No Redundant Suffixes

```rust
✅ Good
pub struct Text { ... }
pub struct Number { ... }

❌ Bad
pub struct TextParameter { ... }
pub struct NumberParameter { ... }
```

### Boolean Prefixes

No `BooleanSubtype` - use naming patterns:
- `show_`, `use_`, `is_`, `has_`, `enable_`, `hide_`

### Rust Conventions

- **Types:** `UpperCamelCase` (`WorkflowEngine`, `NodeId`)
- **Functions:** `snake_case` (`execute_node`, `get_value`)
- **Constants:** `SCREAMING_SNAKE_CASE` (`MAX_RETRIES`)
- **Crates:** `kebab-case` (`my-core`, `my-utils`)

## Code Quality Standards

**Configured via:**
- `rustfmt.toml` - Edition 2024, max_width 100, Unix newlines
- `clippy.toml` - MSRV 1.92, strict linting, missing-docs enforcement
- `deny.toml` - License checking, security advisories

**Requirements:**
- All public APIs must have documentation
- No wildcard imports (Clippy enforced)
- Cognitive complexity ≤ 25
- Type complexity ≤ 250  
- Zero warnings in CI (`RUSTFLAGS=-Dwarnings`)

</code_style>

---

<workflow>

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

# Fast testing with nextest (recommended)
cargo nextest run --workspace --all-features

# Check documentation
cargo doc --no-deps --all-features

# Verify MSRV
cargo +1.92 check --workspace
```

### CI Pipeline

GitHub Actions runs:
- `check` - workspace check with all targets
- `test` - default feature tests
- `test-features` - all feature combinations
- `fmt` - format checking
- `clippy` - linting with deny warnings
- `doc` - documentation generation
- `msrv` - Rust 1.92 compatibility

### Commit Message Format

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types:** `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `perf`

**Example:**
```
feat(validation): add email validator with regex caching

- Implement EmailValidator with thread-local regex cache
- Add tests for valid/invalid email formats
- Update validation documentation

Closes #123
```

</workflow>

---

<design_patterns>

## Core Design Patterns

### Composition Over Proliferation

**Principle:** 23 base types + subtypes + flags = thousands of combinations

```rust
// ❌ Don't create specialized types
struct Password { ... }

// ✅ Use composition
Text::builder()
    .subtype(TextSubtype::Secret)
    .flags(Flags::SENSITIVE)
    .build()
```

### Type-Safe Builders Without Const Generics

```rust
// Vector uses runtime size, but type-safe builders
let position = VectorParameter::vector3("position")
    .default_vec3([0.0, 0.0, 0.0])  // Enforces [f64; 3]
    .build();
```

### Generic RuntimeParameter Pattern

```rust
pub struct RuntimeParameter<T: Node> {
    node: Arc<T>,      // Immutable schema (shared)
    state: StateFlags, // Mutable state
    errors: Vec<ValidationError>,
}
```

### Transform System (Hybrid Approach)

**Declarative** (~80% of cases):
```rust
let transforms = Transforms::new()
    .trim()
    .lowercase()
    .capitalize();
```

**Programmatic** (complex cases):
```rust
let pipeline = Transforms::new()
    .push(Transform::Trim)
    .custom(PhoneFormatter)
    .func("custom", |v| v.clone());
```

### Unified Expression System

Single expression for validation + visibility:

```rust
use paramdef::expr::{Expr, Rule, ExprTarget};

// Validation: check current value
let email_rule = Rule::local(Expr::email());

// Visibility: check other field
let visible_when = when("mode").eq(Value::text("advanced"));
```

**40+ built-in expressions:** eq, ne, lt, gt, email, url, min_length, max_length, between, etc.

### Validation System (Hybrid)

**Declarative:**
```rust
let rules = Rules::from_rules([
    Rule::local(Expr::required()),
    Rule::local(Expr::min_length(3)),
    Rule::local(Expr::email()),
]);
```

**Programmatic:**
```rust
struct UniqueEmail;
impl Validator for UniqueEmail {
    fn validate(&self, value: &Value, ctx: &ValidationContext) -> ValidationResult {
        // Custom logic
    }
}
```

### Event System

**Uses `tokio::broadcast`:**

```rust
// Event types
pub enum Event {
    ValueChanging { key, old_value, new_value },
    ValueChanged { key, old_value, new_value },
    Validated { key, is_valid, errors },
    Touched { key },
    // ... more
}

// Usage
let bus = EventBus::new(64);
let mut sub = bus.subscribe();
let ctx = Context::with_event_bus(schema, bus);

ctx.set("name", Value::text("Alice")); // Emits events
```

### History System (Command Pattern)

**Memory-efficient delta storage:**

```rust
let mut history = HistoryManager::new();

// Execute (stores in undo stack)
let cmd = SetValueCommand::new("name", None, Value::text("Alice"));
history.execute(cmd, &mut ctx)?;

// Undo/Redo
history.undo(&mut ctx)?;
history.redo(&mut ctx)?;
```

**~100 bytes per command** vs ~10KB snapshots

</design_patterns>

---

<performance>

## Performance Optimization

### Memory Efficiency

- `SmartString<LazyCompact>` - Strings <23 bytes on stack
- `Arc<[Value]>` - Immutable arrays, cheap cloning
- `Arc<HashMap>` - Immutable objects, shared
- Const generics for fixed vectors - stack allocated
- Thread-local regex cache - avoid recompilation
- Lazy expression compilation - compile on first use
- Fast path checks - skip empty lists

### Profiling Tools

```bash
# CPU profiling
cargo install flamegraph
cargo flamegraph --bin my_app

# Memory profiling
cargo install cargo-bloat
cargo bloat --release --crates

# Benchmarking
cargo bench -p <crate>
```

</performance>

---

<testing>

## Testing Philosophy

### Coverage Targets
- Core types: 95%+
- Parameter types: 90%+
- Overall: 90%+

### Testing Tools

```bash
# Nextest (recommended - faster)
cargo nextest run --workspace --all-features
cargo nextest run --profile ci  # With retries

# Standard cargo test
cargo test --workspace --all-features

# Doctests (nextest doesn't run these)
cargo test --workspace --doc
```

### Test Organization

- **Unit tests:** Same file under `#[cfg(test)]`
- **Integration tests:** `tests/` directory
- **Doctests:** In doc comments

### Allow in Tests Only

Clippy enforces these only in tests:
- `expect`, `unwrap`, `dbg!`, `print!`

</testing>

---

<localization>

## i18n (Internationalization)

**User-managed approach:**
- Library provides Fluent keys
- User provides translations (zero library bloat)
- User controls all languages

```rust
// Library side
fluent_id("db-host")

// User side - locales/ru/app.ftl
db-host = База данных хост
```

</localization>

---

<rejected_alternatives>

## Architectural Decisions (Why Not)

Important constraints from `docs/17-DESIGN-DECISIONS.md`:

- ❌ NO `BooleanSubtype` - too simple, use naming conventions
- ❌ NO `ChoiceSubtype` - YAGNI, UI variations are presentation
- ❌ NO const generics for Vector - type erasure kills benefits
- ❌ NO subtype in Value enum - violates separation of concerns
- ❌ NO validation in Value - rules belong in schema
- ❌ NO UI coupling - must work headless

</rejected_alternatives>

---

<documentation>

## Essential Reading

**Start here (in order):**

1. `README.md` - Project overview
2. `docs/01-ARCHITECTURE.md` (30 min) - Three-layer architecture
3. `docs/02-TYPE-SYSTEM.md` (30 min) - 23 node types
4. `docs/17-DESIGN-DECISIONS.md` (20 min) - Why we made key choices
5. `docs/22-UNIFIED-EXPRESSIONS.md` (15 min) - Expression system
6. `docs/20-VALIDATION-SYSTEM.md` (15 min) - Validation
7. `docs/19-EVENT-SYSTEM.md` (15 min) - Reactive updates

**Full documentation:** `docs/` directory (20 comprehensive files)

</documentation>

---

<project_structure>

## Directory Layout

```
paramdef/
├── src/
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
├── tests/              # Integration tests
├── benches/            # Benchmarks
├── .cargo/
│   └── config.toml     # LLD linker, build config
├── .config/
│   └── nextest.toml    # Fast test runner config
├── .github/
│   ├── workflows/
│   │   └── ci.yml      # CI pipeline
│   └── dependabot.yml  # Dependency updates
├── clippy.toml         # Clippy linting config
├── rustfmt.toml        # Code formatting config
├── deny.toml           # License/security checks
├── rust-toolchain.toml # MSRV enforcement (1.92)
└── Cargo.toml          # Package manifest
```

</project_structure>

---

<common_tasks>

## Common Development Tasks

### Adding a New Node Type

1. Choose category: Group, Decoration, Container, or Leaf
2. Implement `Node` trait + category-specific trait
3. If has own Value: implement `Validatable` (if validation feature)
4. If can contain children: implement `ValueAccess`
5. All types implement `Visibility` (if visibility feature)
6. Add builder pattern with `::builder()`

### Adding a New Subtype

1. Choose enum: `TextSubtype`, `NumberSubtype`, `VectorSubtype`
2. Add variant (alphabetically within semantic group)
3. Add helper method if needed (e.g., `is_code()`)
4. Update tests
5. **No need for new node type** - use composition!

### Adding a New Flag

1. Add to `Flags` bitflags in order
2. Add convenience method if commonly used
3. Document behavior and use cases
4. Update `docs/11-FLAGS-REFERENCE.md`

### Adding a New Feature

1. Core: no dependencies, always available
2. Optional: gate with `#[cfg(feature = "name")]`
3. Update `Cargo.toml` features section
4. Test all feature combinations in CI

</common_tasks>

---

<examples>

## Code Examples

### Building a Form Field

```rust
use paramdef::prelude::*;

// Text field with validation
let email = Text::builder()
    .key("email")
    .label("Email Address")
    .subtype(TextSubtype::Email)
    .placeholder("user@example.com")
    .rules(Rules::from_rules([
        Rule::local(Expr::required()),
        Rule::local(Expr::email()),
    ]))
    .build();

// Number with soft/hard constraints
let age = Number::builder()
    .key("age")
    .label("Age")
    .subtype(NumberSubtype::Integer)
    .default(18.0)
    .soft_min(0.0)    // UI hint
    .soft_max(120.0)  // UI hint
    .hard_min(0.0)    // Validation enforced
    .build();

// Conditional visibility
let advanced = Text::builder()
    .key("advanced_config")
    .label("Advanced Configuration")
    .visible_when(when("show_advanced").is_true())
    .build();
```

### Using Runtime Context

```rust
// Create schema
let schema = Schema::builder()
    .node(email)
    .node(age)
    .build();

// Create runtime context
let mut ctx = Context::new(schema);

// Set values
ctx.set("email", Value::text("user@example.com"))?;
ctx.set("age", Value::number(25.0))?;

// Validate
ctx.validate_all()?;

// Check state
assert!(ctx.is_valid());
assert!(ctx.is_dirty());

// Get values
let email_value = ctx.get("email")?;
```

### Event Handling

```rust
#[cfg(feature = "events")]
{
    let bus = EventBus::new(64);
    let mut sub = bus.subscribe();
    let ctx = Context::with_event_bus(schema, bus);

    // Set value (emits ValueChanged)
    ctx.set("name", Value::text("Alice"))?;

    // Receive event
    match sub.recv().await {
        Ok(Event::ValueChanged { key, new_value, .. }) => {
            println!("{} = {:?}", key, new_value);
        }
        _ => {}
    }
}
```

</examples>

---

<industry_inspiration>

## Influences and Prior Art

**paramdef** combines best features from:

- **Blender RNA** - Property system architecture, subtype+unit pattern
- **Unreal Engine UPROPERTY** - Metadata and flags system
- **Qt Property System** - Signals and observers, reactive updates
- **Houdini Parameters** - Node-based workflows, soft/hard constraints
- **n8n** - Mode/branching for discriminated unions
- **JSON Schema** - Declarative validation
- **React Hook Form** - Cross-field validation patterns
- **Formik** - Touched/dirty state management

With Rust's type safety and zero-cost abstractions.

</industry_inspiration>

---

<guidelines>

## Guidelines for AI Assistants

### When Writing Code

1. **Read before modifying** - Always read files before editing
2. **Follow architecture** - Respect three-layer separation
3. **Feature gate correctly** - Use `#[cfg(feature = "...")]`
4. **Document public APIs** - All public items need docs
5. **Write tests first** - TDD approach recommended
6. **No unwrap in production** - Use `?` or proper error handling
7. **Prefer composition** - Don't create specialized types

### When Reviewing Code

1. **Check immutability** - Schema must never mutate
2. **Verify feature gates** - Optional features properly gated
3. **Validate tests** - All features tested
4. **Check docs** - Public APIs documented
5. **Run clippy** - Zero warnings policy
6. **Performance** - Consider allocations, use profiling

### When Explaining

1. **Use examples** - Code examples over theory
2. **Reference docs** - Point to specific doc files
3. **Explain trade-offs** - Why we chose this approach
4. **Show alternatives** - What was rejected and why

</guidelines>

---

**Last Updated:** 2026-01-11  
**Maintained By:** paramdef team  
**License:** MIT OR Apache-2.0
