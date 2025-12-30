## Context
This is Phase 1 of implementing the paramdef library. The documentation defines a comprehensive type-safe parameter system inspired by Blender RNA, Unreal UPROPERTY, and Qt Property System. We start with core types that everything else builds upon.

**Constraints:**
- MSRV 1.85 (Rust Edition 2024)
- Zero UI dependencies in core
- All public APIs must have documentation
- 95%+ test coverage for core types

## Goals / Non-Goals

**Goals:**
- Implement Key, Metadata, Flags, StateFlags, Value types
- Establish module structure (`src/core/`)
- TDD approach with comprehensive tests
- Feature-gated serde support

**Non-Goals:**
- Node types (Phase 2)
- Validation system (Phase 3)
- Event system (Phase 4)
- UI hints (separate feature)

## Decisions

### Decision 1: Key uses SmartString
- **What:** `pub type Key = SmartString<LazyCompact>;`
- **Why:** Strings <23 bytes stay on stack, no heap allocation for typical keys
- **Alternatives:** `String` (always heap), `&'static str` (inflexible), `Cow<str>` (complexity)

### Decision 2: Value uses Arc for collections
- **What:** `Array(Arc<[Value]>)`, `Object(Arc<HashMap<SmartString, Value>>)`
- **Why:** Immutable sharing, cheap cloning, thread-safe
- **Alternatives:** `Vec`/`HashMap` (expensive clones), `Rc` (not thread-safe)

### Decision 3: Separate Flags vs StateFlags
- **What:** Two distinct bitflags types
- **Why:** Schema flags (immutable) vs runtime state (mutable) are different concerns
- **Alternatives:** Single combined flags (muddy semantics)

### Decision 4: Feature-gated serde
- **What:** `#[cfg(feature = "serde")]` for Serialize/Deserialize
- **Why:** Zero dependencies when not needed, optional JSON support
- **Alternatives:** Always include (bloat), separate crate (complexity)

## Risks / Trade-offs

- **Risk:** SmartString API changes → Mitigation: Pin version, wrap if needed
- **Risk:** Value enum grows too large → Mitigation: Keep it minimal (8 variants max)
- **Trade-off:** Arc overhead for small collections → Acceptable for correctness/sharing

## Migration Plan
N/A - This is greenfield implementation.

## Open Questions
None - design is fully specified in docs/02-TYPE-SYSTEM.md
