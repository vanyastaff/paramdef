<!--
Sync Impact Report:
- Version change: 1.0.0 → 1.1.0
- Modified principles: II (updated from "23 types" to "14 types"), added VIII (Architectural Invariants)
- Added sections: Architectural Invariants with 10 core rules
- Removed sections: N/A
- Templates requiring updates:
  ✅ plan-template.md - Constitution Check section aligned
  ✅ spec-template.md - Requirements and testing aligned
  ✅ tasks-template.md - Task structure aligned with TDD and parallel execution
- Follow-up TODOs: None
- Rationale: MINOR bump - added new principle (VIII) with architectural invariants from docs/01-ARCHITECTURE.md
-->

# paramdef Constitution

## Core Principles

### I. Immutability-First Architecture (NON-NEGOTIABLE)

**Schema definitions are ALWAYS immutable after creation.** Runtime state lives exclusively in the `Context` layer, never in schema objects. This enables safe sharing via `Arc<T>` across threads and runtime instances without locks.

**Architectural Invariant:** Schema is ALWAYS immutable - runtime state lives in Context.

**Rationale:** Immutability eliminates data races, enables zero-cost sharing, and provides clear separation between "what can exist" (schema) and "what currently exists" (runtime state). This is fundamental to the three-layer architecture.

### II. Composition Over Proliferation (NON-NEGOTIABLE)

**NEVER create specialized node types for combinations of features.** The 14 core node types (1 Group + 1 Layout + 1 Decoration + 6 Container + 5 Leaf) plus subtypes, flags, and units provide thousands of valid combinations. Resist the urge to create `PasswordField`, `EmailInput`, or similar specialized types.

**Examples:**
- ❌ Bad: `struct Password { ... }`
- ✅ Good: `Text::builder().subtype(TextSubtype::Secret).flags(SENSITIVE | WRITE_ONLY)`

**Rationale:** Specialized types lead to exponential explosion. Instead of 100+ types, we maintain 14 types with rich composition. The subtype + unit pattern (Blender-style) provides 60 subtypes × 17 unit categories = 1000+ semantic combinations without new types.

### III. Strict Node Hierarchy (NON-NEGOTIABLE)

**Node containment rules MUST be enforced:**
- **Group** → Layout, Decoration, Container, Leaf (root, contains everything)
- **Layout** → Decoration, Container, Leaf (NOT Layout, NOT Group)
- **Container** → Decoration, Container, Leaf (NOT Layout, NOT Group)
- **Decoration** → nothing (terminal, display-only)
- **Leaf** → nothing (terminal, with value)

**Architectural Invariants:**
- Group and Layout have NO own Value (delegate via `ValueAccess`)
- Decoration has NO Value and NO `ValueAccess` (pure display)
- Container and Leaf HAVE own Value (Container also has `ValueAccess`)

**Rationale:** Strict hierarchy prevents architectural violations and ensures predictable behavior. This structure mirrors proven systems (Blender RNA, Qt Property System) while maintaining type safety.

### IV. Zero UI Dependencies (Headless-First)

**The core library MUST work without any UI dependencies.** All UI-specific features (theming, rendering, layout hints) are metadata only. The library must function perfectly in servers, CLI tools, and headless environments.

**Test:** Can paramdef run in a headless server validating API requests? If no, we've violated this principle.

**Rationale:** A form schema system that requires a UI framework is useless for backend validation, CLI wizards, or workflow engines. Headless-first ensures universal applicability across web, desktop, mobile, game engines, and servers.

### V. Feature Flags for Optional Dependencies (Zero-Cost Abstractions)

**Core library has ZERO dependencies beyond essentials.** All optional functionality MUST be feature-gated:

```toml
default = []                  # Core: smartstring, thiserror, bitflags only
visibility = []               # Visibility trait, Expr
validation = []               # Validatable trait, ValidationConfig
serde = ["dep:serde"]        # Serialization
events = ["dep:tokio"]       # Event system
i18n = ["dep:fluent"]        # Localization
chrono = ["dep:chrono"]      # Date/time support
full = ["visibility", "validation", "serde", "events", "i18n", "chrono"]
```

**Users pay only for what they use.** A simple CLI tool shouldn't pull in tokio. A game engine shouldn't pull in serde.

**Rationale:** Zero-cost abstractions mean zero dependencies for unused features. This enables paramdef to be "the serde of parameter schemas" - universally applicable because universally lightweight.

### VI. Test-Driven Development (NON-NEGOTIABLE)

**Tests MUST be written before implementation.** Every new feature, bug fix, or API addition follows Red-Green-Refactor:
1. Write failing test demonstrating desired behavior
2. Implement minimal code to pass the test  
3. Refactor while keeping tests green

**Coverage targets:**
- Core types: 95%+
- Parameter types: 90%+
- Overall: 90%+

**Rationale:** TDD ensures testability by design, provides executable specifications, prevents regressions, and maintains high code coverage naturally. This is critical for a library that will be used in production systems.

### VII. Rust 2024 Standards (Quality Gate)

**MSRV 1.92, Edition 2024, zero warnings policy.** All code MUST pass:
- `cargo fmt --all` - Code formatting
- `cargo clippy --workspace --all-features -- -D warnings` - Zero warnings
- `cargo test --workspace --all-features` - All tests pass
- `cargo nextest run --workspace --all-features` - Fast test runner
- `cargo doc --no-deps --all-features` - Documentation builds

**Additional requirements:**
- All public APIs documented (clippy enforced)
- No wildcard imports (clippy enforced)
- Cognitive complexity ≤ 25 (clippy enforced)
- Type complexity ≤ 250 (clippy enforced)

**Rationale:** Consistent standards prevent bikeshedding, catch bugs early, and ensure professional code quality. Edition 2024 enables modern Rust idioms. Zero warnings policy ensures code quality gates are met.

### VIII. Architectural Invariants (Design Constraints)

**The following architectural rules MUST NOT be broken:**

1. **Immutable Schema** - Schema is ALWAYS immutable; runtime state lives in Context
2. **Value Ownership** - Group/Layout: no own Value (delegate via `ValueAccess`); Decoration: no Value at all; Container/Leaf: have own Value
3. **Structural Integrity** - Mode produces `{mode, value}` discriminated union object, never scalar
4. **Validation Separation** - UI hints (soft constraints) NEVER affect validation (hard constraints)
5. **Clean Names** - Types use `Text`, `Number`, `Boolean` - NOT `TextParameter`, `NumberParameter`
6. **No God Objects** - Keep types focused and composable; resist kitchen-sink designs
7. **Type Count Discipline** - Maintain exactly 14 core node types; new types require constitution amendment
8. **Feature Trait Discipline** - Only Container/Leaf implement `Validatable` (10/14 types); All types implement `Visibility` (14/14 types with feature flag)
9. **Processing Order** - Transform → Validate Sync → Validate Async → Set Value → Notify (never reorder)
10. **Schema/Runtime Boundary** - Schema types NEVER contain runtime state; Runtime types NEVER define schema structure

**Rationale:** These invariants are derived from docs/01-ARCHITECTURE.md and represent hard-learned lessons from Blender, Unreal, Qt, and Houdini. Violating them creates architectural debt that compounds over time.

## Development Practices

### Three-Layer Architecture

**The separation MUST be preserved:**

1. **Schema Layer (Immutable)** - Parameter definitions, metadata, flags, validators, shared via `Arc<T>`
2. **Runtime Layer (Mutable)** - Current values, state flags (dirty, touched, valid), validation errors, per-instance
3. **Value Layer** - Runtime data representation, serialization target, `Value` enum

**Test:** If you need to store runtime state, ask "Does this belong in Schema or Context?" If Schema, you're probably wrong.

### Type Safety via Traits

**Compile-time constraints where possible, runtime validation where necessary:**
- Use trait bounds to enforce subtype constraints (`Number<Port>` is integer-only)
- Use const generics where valuable (`Vector<f64, 3>` for 3D positions)
- Use runtime checks only when compile-time is impossible (regex patterns, async validation)

**Trade-offs documented:** When we choose runtime over compile-time, we document why in code comments or design docs.

### Performance Discipline

**Design for efficiency, measure before optimizing:**

**Memory efficiency:**
- `SmartString<LazyCompact>` - Strings <23 bytes on stack
- `Arc<[Value]>` - Immutable arrays, cheap cloning
- `Arc<HashMap>` - Immutable objects, shared
- Const generics for fixed vectors - stack allocated
- Thread-local regex cache - avoid recompilation

**Fast paths:**
- Skip empty transformer/validator lists
- Lazy expression compilation
- Early exit on type mismatches

**Profiling required before optimization claims.** Benchmark with real workloads, not assumptions.

### Dependency Management

**Every dependency MUST be justified:**

**Core dependencies (always present):**
- `smartstring` - Stack strings <23 bytes
- `thiserror` - Error derive macros  
- `bitflags` - Type-safe flags

**Optional dependencies (feature-gated only):**
- `serde` + `serde_json` - Serialization
- `tokio` - Async event system
- `regex` - Pattern validation
- `fluent` - i18n (user-provided translations)
- `chrono` - Date/time support

**New dependency checklist:**
1. Clear problem statement - what gap does it fill?
2. Why existing tools insufficient - tried internal solution?
3. Cost-benefit analysis - compile time, binary size, maintenance burden
4. License compatibility - MIT/Apache-2.0 compatible
5. Feature gate plan - which feature flag will guard it?

**Rejected dependencies must be documented** in docs/17-DESIGN-DECISIONS.md with rationale.

### Soft vs Hard Constraints

**UI hints must never affect validation:**

```rust
Number {
    hard_min: Some(0.0),    // Validation enforced - MUST be in range
    hard_max: Some(100.0),  // Validation enforced - MUST be in range
    soft_min: Some(0.0),    // UI slider hint - user CAN type beyond
    soft_max: Some(10.0),   // UI slider hint - user CAN type beyond
}
```

**Architectural invariant:** UI hints (soft) NEVER affect validation (hard). This separation is non-negotiable.

## Documentation Standards

### Public API Documentation

**Every public API MUST have documentation** (clippy enforced):
- Purpose and use cases
- Example code that compiles (tested via doctests)
- Feature flags required (if any)
- Relationship to architectural principles
- Performance characteristics (if non-trivial)

**Doc comment structure:**
```rust
/// Brief one-line summary.
///
/// Longer description explaining purpose, behavior, and design rationale.
///
/// # Examples
///
/// ```
/// use paramdef::prelude::*;
///
/// let param = Text::builder()
///     .key("email")
///     .subtype(TextSubtype::Email)
///     .build();
/// ```
///
/// # Feature Flags
///
/// This feature requires the `validation` feature flag.
///
/// # Performance
///
/// This operation is O(1) with no allocations.
```

### Essential Documentation

**Start here for new contributors (reading order):**
1. `README.md` - Project overview
2. `docs/01-ARCHITECTURE.md` (30 min) - Three-layer architecture ← **THIS DOCUMENT**
3. `docs/02-TYPE-SYSTEM.md` (30 min) - 14 node types
4. `docs/17-DESIGN-DECISIONS.md` (20 min) - Why we made key choices
5. `docs/22-UNIFIED-EXPRESSIONS.md` (15 min) - Expression system
6. `docs/20-VALIDATION-SYSTEM.md` (15 min) - Validation
7. `docs/19-EVENT-SYSTEM.md` (15 min) - Reactive updates

**Full documentation:** `docs/` directory (23 comprehensive design documents)

### Design Decisions Document

**All rejected alternatives MUST be documented** in `docs/17-DESIGN-DECISIONS.md`:
- What was considered and rejected
- Why it was rejected
- What we chose instead
- Trade-offs accepted

**Examples of documented rejections:**
- ❌ NO `BooleanSubtype` - too simple, use naming conventions
- ❌ NO const generics for Vector - type erasure kills benefits
- ❌ NO subtype in Value enum - violates separation of concerns

## Governance

### Amendment Process

1. **Proposal**: Open GitHub issue with `constitution` label
2. **Discussion**: Community review (minimum 7 days for principles, 3 days for clarifications)
3. **Vote**: Maintainers vote (simple majority for PATCH, 2/3 majority for MINOR/MAJOR)
4. **Implementation**: Update constitution.md with version bump
5. **Propagation**: Update dependent templates and documentation
6. **Announcement**: Document changes in CHANGELOG.md and release notes

### Version Semantics

**MAJOR.MINOR.PATCH** following semantic versioning:
- **MAJOR**: Principle removed or fundamentally redefined (breaking governance change)
- **MINOR**: New principle added or section materially expanded (new architectural constraints)
- **PATCH**: Clarifications, wording improvements, typo fixes (no new constraints)

**Examples:**
- Removing principle II (Composition Over Proliferation) → MAJOR
- Adding principle IX (new constraint) → MINOR  
- Fixing typos, clarifying wording → PATCH

### Compliance Review

**All pull requests MUST verify compliance:**

**Architectural compliance:**
- [ ] Maintains immutable schema layer (no runtime state in schema types)
- [ ] Uses composition not proliferation (no specialized types when composition works)
- [ ] Follows strict node hierarchy (correct containment rules)
- [ ] Preserves three-layer architecture (schema/runtime/value separation)
- [ ] Respects architectural invariants (all 10 rules)

**Code quality compliance:**
- [ ] Tests exist and pass (TDD followed)
- [ ] Code formatted (`cargo fmt`)
- [ ] Zero warnings (`cargo clippy -- -D warnings`)
- [ ] Public APIs documented
- [ ] Feature flags correctly applied

**Dependency compliance:**
- [ ] New dependencies justified (if any)
- [ ] Feature-gated appropriately (not in core)
- [ ] License compatible (MIT/Apache-2.0)

**Constitution supersedes convenience.** If a feature cannot comply with core principles, the feature is rejected or the constitution must be amended through proper governance.

### Living Document

This constitution is a living document. As the project evolves, principles may be added, refined, or (rarely) removed. All changes follow the amendment process above.

**Constitution is not a substitute for detailed guidance.** For runtime development patterns, examples, and current project status, reference `CLAUDE.md`.

**Version**: 1.1.0 | **Ratified**: 2026-01-28 | **Last Amended**: 2026-01-28
