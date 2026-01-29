# Feature Specification: Code Quality Improvements

**Feature Branch**: `002-code-quality-improvements`  
**Created**: 2026-01-28  
**Status**: Draft  
**Based on**: Comprehensive codebase analysis (CODEBASE_ANALYSIS_2026-01-28.md)

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Fix Immutability Violations (Priority: P1) 🎯 CRITICAL

As a library architect maintaining paramdef, I need to eliminate all schema mutation after construction so that the immutability-first architecture is properly enforced and schemas can be safely shared via Arc across threads without unexpected state changes.

**Why this priority**: This is a critical architectural violation that breaks the fundamental "Schema is ALWAYS immutable" invariant documented in the constitution and architecture docs. Must be fixed before 1.0.0 release.

**Independent Test**: Can be tested by attempting to mutate a schema after Arc-wrapping it and verifying compilation errors or runtime panics. All schema types must be truly immutable after construction.

**Acceptance Scenarios**:

1. **Given** a Panel schema with default collapsed state, **When** I share it via Arc across multiple contexts, **Then** each context independently manages collapsed state without affecting the shared schema
2. **Given** any node type with visibility rules, **When** I attempt to call `set_visibility_rule()` after construction, **Then** the compilation fails or method is marked as deprecated/removed
3. **Given** a schema shared between 10 contexts, **When** context A changes Panel collapsed state, **Then** contexts B-J see no schema changes (state is per-context)
4. **Given** Panel runtime state (collapsed/expanded), **When** stored in Context or RuntimeNode, **Then** schema struct contains no mutable fields
5. **Given** multi-threaded access to shared schema, **When** threads read schema concurrently, **Then** no data races occur and no synchronization primitives are needed

---

### User Story 2 - Improve API Ergonomics for Common Patterns (Priority: P1) 🎯 MVP

As a paramdef user building forms and workflows, I need concise, intuitive APIs for common tasks (creating required fields, setting values, handling errors) so that I can be productive quickly without excessive boilerplate code.

**Why this priority**: Ergonomics directly impact adoption. Current verbosity (Arc wrapping, 4-line required fields, 5-line validation setup) creates friction that prevents users from seeing paramdef's value proposition.

**Independent Test**: Can be tested by comparing line count and readability of common patterns before/after improvements. Target: 40-50% reduction in boilerplate for typical use cases.

**Acceptance Scenarios**:

1. **Given** I want to create a Context from a Schema, **When** I use the new API, **Then** I don't need to manually wrap in Arc::new() - one method call suffices
2. **Given** I want to create a required email field, **When** I use shorthand constructor, **Then** it takes 1 line instead of 4 (builder + label + required + build)
3. **Given** I want to add email validation, **When** I use builder shortcuts, **Then** I write `.validate_email()` instead of `Rules::from_rules([Rule::local(Expr::email())])`
4. **Given** I get a validation error on nested object field, **When** I read the error message, **Then** I see the full path (e.g., "user.address.email") not just "email"
5. **Given** I want a value with fallback default, **When** I use `get_text_or()`, **Then** I don't need manual unwrap_or chains

---

### User Story 3 - Optimize Performance Hot Paths (Priority: P2)

As a paramdef user building high-performance applications, I need efficient value operations (setting values with events, bulk updates, collecting values) so that my forms and workflows don't become performance bottlenecks.

**Why this priority**: Performance impacts production use at scale. While not architecturally critical, excessive cloning (3x per set with events) and allocations affect real-world applications processing thousands of parameter updates.

**Independent Test**: Can be tested with benchmarks measuring throughput (updates/sec) and memory allocations (bytes allocated per operation) before/after optimizations.

**Acceptance Scenarios**:

1. **Given** Context with event bus enabled, **When** I set 1000 values, **Then** Value cloning is reduced by 66% (1 clone instead of 3 per operation)
2. **Given** I perform transactional bulk update with 5 fields, **When** the update succeeds, **Then** no HashMap allocation occurs (uses stack buffer)
3. **Given** I need to read 10 values, **When** I use bulk getter API, **Then** I perform 1 batch lookup instead of 10 individual hash lookups
4. **Given** I collect all context values, **When** I use zero-copy iterator, **Then** no Value cloning occurs (returns references)
5. **Given** large Value objects (1KB+), **When** emitting events, **Then** values are shared via Arc instead of cloned

---

### User Story 4 - Enhance Documentation and Developer Experience (Priority: P2)

As a new paramdef user, I need clear documentation, working examples, and helpful error messages so that I can quickly understand how to use the library effectively without trial-and-error or digging through source code.

**Why this priority**: Documentation is the first impression. Working examples and helpful errors dramatically reduce time-to-productivity and support burden.

**Independent Test**: Can be tested by onboarding new developers and measuring time-to-first-working-code, plus counting doc example compilation rate.

**Acceptance Scenarios**:

1. **Given** I read any doc comment example, **When** I copy-paste the code, **Then** it compiles successfully (95%+ examples compile)
2. **Given** I get a TypeMismatch error, **When** I read the error message, **Then** it suggests the correct getter method to use
3. **Given** I want to implement a common pattern, **When** I check COOKBOOK.md, **Then** I find a ready-to-use recipe with explanation
4. **Given** I encounter a validation error, **When** I read the error documentation, **Then** I understand whether it's recoverable and how to handle it
5. **Given** I'm learning paramdef, **When** I follow the recommended reading order, **Then** I understand the three-layer architecture and 23 node types within 2 hours

---

### Edge Cases

- **Arc wrapping confusion**: Users mix `Arc::new(schema)`, `Arc::clone(&schema)`, and `schema.clone()` - need clear guidance
- **Nullable vs Optional confusion**: Users don't understand difference between `Value::Null` (present but null) vs field not in schema
- **Nested validation errors**: Users can't locate error when path is missing (e.g., "email" error in deeply nested object)
- **Thread safety misunderstandings**: Users worry about Arc<Schema> safety - need clear documentation on Send+Sync guarantees
- **Builder explosion**: Adding too many shortcuts (`.required()`, `.optional()`, `.required_email()`, etc.) creates method overload
- **Backward compatibility**: Fixing immutability violations may break existing code that relies on `set_collapsed()` or `set_visibility_rule()`
- **Performance regressions**: Optimizations must not break existing behavior or introduce new bugs
- **Documentation rot**: Examples must stay synchronized with code as API evolves

## Requirements *(mandatory)*

### Functional Requirements

#### Immutability Fixes (P1)

- **FR-001**: Panel type MUST NOT contain mutable `collapsed` field in schema struct
- **FR-002**: Layout trait MUST NOT define `set_collapsed(&mut self)` method
- **FR-003**: Context or RuntimeNode MUST provide API for managing per-instance Panel UI state (collapsed/expanded)
- **FR-004**: All 23 node types MUST NOT provide `set_visibility_rule(&mut self)` method post-construction
- **FR-005**: Visibility rules MUST be set only during construction via builder pattern
- **FR-006**: Documentation MUST clearly state that schemas are immutable and runtime state lives in Context

#### API Ergonomics (P1)

- **FR-007**: Context MUST provide `from_schema(schema: Schema)` constructor that auto-wraps in Arc
- **FR-008**: Text type MUST provide `required(key, label)` shorthand constructor
- **FR-009**: Number type MUST provide `required(key, label)` shorthand constructor
- **FR-010**: Boolean type MUST provide `required(key, label)` shorthand constructor
- **FR-011**: Text builder MUST provide `.validate_required()`, `.validate_email()`, `.validate_min_length()` shortcuts
- **FR-012**: Number builder MUST provide `.validate_required()`, `.validate_min()`, `.validate_max()` shortcuts
- **FR-013**: Context MUST provide `get_text_or()`, `get_int_or()`, `get_bool_or()` error recovery methods
- **FR-014**: ValidationError MUST include full field path (e.g., "user.address.email") for nested objects
- **FR-015**: Object builder MUST provide `.fields()` bulk method for adding multiple fields at once

#### Performance Optimizations (P2)

- **FR-016**: Event system MUST use Arc<Value> instead of cloning Value when emitting events
- **FR-017**: Context::set_many_transactional MUST use stack buffer for <8 items to avoid HashMap allocation
- **FR-018**: Context MUST provide `get_many()` bulk getter returning iterator over requested keys
- **FR-019**: Context MUST provide zero-copy `values()` iterator (already exists - must be documented)
- **FR-020**: Value type SHOULD provide ValueBuilder for ergonomic construction of complex objects

#### Documentation Improvements (P2)

- **FR-021**: All doc comment examples MUST compile successfully (remove `ignore`, use `no_run` if needed)
- **FR-022**: Error types MUST include usage examples showing how to handle each error variant
- **FR-023**: COOKBOOK.md MUST be created with recipes for 10+ common patterns
- **FR-024**: Error messages MUST include actionable hints (e.g., "Use get_text() for Text values")
- **FR-025**: Documentation MUST clarify nullable vs optional field semantics
- **FR-026**: Type count documentation MUST be corrected from "14 types" to "23 types" everywhere

### Key Entities

- **PanelState**: New runtime struct holding UI state (collapsed, last_interaction) separate from Panel schema
- **ValidationError**: Enhanced with `path: String` field for nested object errors
- **ValueBuilder**: New builder for ergonomic Value::Object construction
- **BulkFieldSet**: Internal optimization struct for Object builder's `.fields()` method
- **ErrorHint**: New struct attached to Error types with actionable suggestions
- **RollbackStorage**: Enum for efficient transactional updates (Small stack buffer vs Large HashMap)

## Success Criteria *(mandatory)*

### Measurable Outcomes

#### Architecture & Quality (P1)

- **SC-001**: Zero mutable fields in schema structs after immutability fixes (currently 1: Panel::collapsed)
- **SC-002**: Zero `&mut self` methods on schema types except builders (currently 2: set_collapsed, set_visibility_rule)
- **SC-003**: All schema types remain Send + Sync with no additional synchronization primitives required
- **SC-004**: Backward compatibility: existing code compiles with deprecation warnings, not hard breaks

#### Developer Experience (P1)

- **SC-005**: Boilerplate code reduction: 40-50% fewer lines for common patterns (required fields, validation, Context creation)
- **SC-006**: Doc example success rate: 95%+ of examples compile successfully (currently ~80%)
- **SC-007**: Error message improvement: 100% of error variants include actionable hints
- **SC-008**: Nested validation errors: 100% include full field path (e.g., "user.address.email")
- **SC-009**: Time to first working code: New users achieve productive use within 30 minutes (vs 1-2 hours currently)

#### Performance (P2)

- **SC-010**: Value cloning reduction: 66% fewer clones in event-enabled contexts (1 clone vs 3 per set operation)
- **SC-011**: Allocation reduction: Zero heap allocations for transactional updates <8 fields
- **SC-012**: Bulk operations: get_many() performs 1 batch lookup instead of N individual lookups
- **SC-013**: Benchmark improvement: 20-30% throughput increase in high-frequency update scenarios
- **SC-014**: Memory efficiency: No performance regression in existing benchmarks

#### Documentation (P2)

- **SC-015**: COOKBOOK.md created with 10+ recipes covering common use cases
- **SC-016**: Error handling guide added to Error module documentation
- **SC-017**: API complexity reduction: 20% fewer public methods via consolidation (e.g., remove duplicate `*_arc()` methods)
- **SC-018**: Type system documentation: Corrected from "14 types" to "23 types" in all locations
- **SC-019**: Architecture documentation: All immutability invariants clearly stated and verifiable

## Assumptions

- **Breaking changes acceptable**: This is pre-1.0, so fixing architectural violations via breaking changes is acceptable with proper deprecation warnings
- **User migration support**: We will provide migration guide and deprecation warnings for 1-2 minor versions before hard removal
- **Performance targets**: Benchmarks target typical use cases (10-100 parameters, 100-1000 updates/sec)
- **Documentation completeness**: We assume English documentation is primary; i18n of docs is out of scope
- **Test coverage maintenance**: Existing 90%+ test coverage must be maintained; new code follows same standard
- **MSRV stability**: Rust 1.92 MSRV is maintained; no new MSRV-breaking features required
- **Backward compatibility priorities**: Critical architectural fixes (immutability) take precedence over perfect backward compatibility
- **Implementation timeline**: Fixes are prioritized P1 (critical) → P2 (important) → P3 (nice-to-have) over 4 weeks
- **Dependencies remain minimal**: No new required dependencies; optional dependencies only if feature-gated
- **Performance measurement**: Benchmarks run on CI with consistent hardware; accept 5% variance as noise

## Implementation Phases

### Phase 1: Immutability Fixes (Week 1) - CRITICAL

**Goal**: Eliminate all schema mutation, establish proper runtime state management

**Tasks**:
1. Remove `collapsed: bool` from Panel struct
2. Add `PanelState` to Context or create `UiStateManager`
3. Remove `set_collapsed(&mut self)` from Layout trait
4. Deprecate `set_visibility_rule(&mut self)` on all types
5. Update tests to use new runtime state API
6. Add migration guide for breaking changes

**Deliverable**: Schema types are truly immutable; runtime state properly separated

---

### Phase 2: API Ergonomics (Week 2) - HIGH PRIORITY

**Goal**: Reduce boilerplate, improve developer experience

**Tasks**:
1. Add `Context::from_schema()` convenience constructor
2. Add `Text::required()`, `Number::required()`, `Boolean::required()` shortcuts
3. Add validation builder shortcuts (`.validate_email()`, `.validate_required()`, etc.)
4. Add error recovery methods (`get_text_or()`, `get_int_or()`, etc.)
5. Enhance ValidationError with field paths
6. Add Object builder `.fields()` bulk method

**Deliverable**: Common patterns are 40-50% less verbose

---

### Phase 3: Performance Optimizations (Week 3) - IMPORTANT

**Goal**: Eliminate unnecessary cloning and allocations

**Tasks**:
1. Change Event to use Arc<Value> for value sharing
2. Add RollbackStorage enum for efficient transactions
3. Add Context::get_many() bulk getter
4. Document zero-copy .values() iterator
5. Run benchmarks and verify improvements
6. Profile for any regressions

**Deliverable**: 20-30% throughput improvement in high-frequency scenarios

---

### Phase 4: Documentation (Week 4) - POLISH

**Goal**: Improve onboarding and discoverability

**Tasks**:
1. Fix all doc examples to compile (remove `ignore` markers)
2. Create COOKBOOK.md with 10+ recipes
3. Add error handling guide to Error module
4. Correct type count documentation (14→23)
5. Add error hints to all Error variants
6. Review and polish all public API documentation

**Deliverable**: 95%+ examples compile; clear cookbook for common patterns

---

## Non-Functional Requirements

### Code Quality
- All code follows Rust 2024 Edition best practices
- Zero clippy warnings with `-- -D warnings` flag
- Code formatted with rustfmt
- All public APIs documented with examples
- Test coverage maintained at 90%+

### Performance
- No regression in existing benchmarks (within 5% variance)
- Memory usage does not increase for common operations
- Hot path optimizations validated via micro-benchmarks

### Compatibility
- Deprecation warnings for breaking changes
- Migration guide provided
- Graceful degradation where possible
- MSRV 1.92 maintained

### Documentation
- All changes documented in CHANGELOG.md
- Architecture docs updated to reflect fixes
- API docs include migration examples
- COOKBOOK.md provides practical recipes

### Testing
- Unit tests for all new functionality
- Integration tests for API changes
- Performance benchmarks for optimizations
- Backward compatibility tests (with deprecations)

## Migration Guide

### For Users of Panel::set_collapsed()

**Before:**
```rust
let mut panel = Panel::builder("settings").build();
panel.set_collapsed(true);  // ❌ Will be deprecated
```

**After:**
```rust
let panel = Panel::builder("settings")
    .collapsed(true)  // ✅ Set during construction
    .build();

// Runtime state management
ctx.set_panel_collapsed("settings", true);  // ✅ Per-context state
```

### For Users of set_visibility_rule()

**Before:**
```rust
let mut text = Text::builder("field").build();
text.set_visibility_rule(Some(rule));  // ❌ Will be deprecated
```

**After:**
```rust
let text = Text::builder("field")
    .visible_when(Expr::eq("mode", "advanced"))  // ✅ Set during construction
    .build();
```

### For Arc Wrapping

**Before:**
```rust
let schema = Schema::builder()...build();
let ctx = Context::new(Arc::new(schema));  // ❌ Verbose
```

**After:**
```rust
let schema = Schema::builder()...build();
let ctx = Context::from_schema(schema);  // ✅ Simplified
```

## Related Documentation

- **CODEBASE_ANALYSIS_2026-01-28.md** - Comprehensive analysis report
- **docs/01-ARCHITECTURE.md** - Three-layer architecture
- **.specify/memory/constitution.md** - Architectural invariants
- **CLAUDE.md** - Development guide

## Acceptance Criteria Summary

This feature is complete when:

1. ✅ All schema types are truly immutable (no mutable fields, no `&mut self` methods)
2. ✅ Runtime state (Panel collapsed, etc.) managed in Context/RuntimeNode
3. ✅ Common patterns require 40-50% less code
4. ✅ All validation errors include full field paths
5. ✅ 95%+ of doc examples compile successfully
6. ✅ Performance improves by 20-30% in update-heavy scenarios
7. ✅ COOKBOOK.md exists with 10+ practical recipes
8. ✅ Migration guide provided for all breaking changes
9. ✅ All tests pass with maintained 90%+ coverage
10. ✅ Documentation corrected (23 types, not 14)
