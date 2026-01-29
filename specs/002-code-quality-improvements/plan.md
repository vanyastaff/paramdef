# Implementation Plan: Code Quality Improvements

**Branch**: `002-code-quality-improvements` | **Date**: 2026-01-28 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `specs/002-code-quality-improvements/spec.md`

## Summary

Fix critical architectural violations (Panel::collapsed, visibility mutations) and improve API ergonomics (reduce boilerplate by 40-50%, add convenience methods). Optimize performance hot paths (66% fewer clones, 20-30% throughput gain). Enhance documentation (95%+ examples compile, COOKBOOK.md).

**Based on**: Comprehensive codebase analysis identifying 20 problems across architecture, ergonomics, performance, and documentation.

## Technical Context

**Language/Version**: Rust 1.92 (MSRV enforced via rust-toolchain.toml)  
**Primary Dependencies**: smartstring 1.0, thiserror 2.0, bitflags 2.6, rustc-hash 2.1, indexmap 2.7  
**Optional Dependencies**: serde 1.0, tokio 1.0, regex 1.11, fluent 0.16, chrono 0.4  
**Storage**: In-memory (Context with FxHashMap), no external storage  
**Testing**: cargo test, cargo nextest (preferred), criterion for benchmarks  
**Target Platform**: Cross-platform (Windows, Linux, macOS), WASM-compatible core  
**Project Type**: Single Rust library crate (src/) with comprehensive tests/  
**Performance Goals**: 
- 66% reduction in Value clones (3 → 1 per set with events)
- 20-30% throughput improvement in update-heavy scenarios
- Zero allocations for transactional updates <8 fields  
**Constraints**:
- MSRV 1.92 cannot be increased
- Breaking changes acceptable (pre-1.0) with deprecation warnings
- Existing test coverage (90%+) must be maintained
- Feature flags discipline must be preserved  
**Scale/Scope**: 104 Rust files, ~32K LOC, 23 node types, 20 identified problems

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### ✅ Passes

- **III. Strict Node Hierarchy**: No new node types added (fixes only)
- **IV. Zero UI Dependencies**: All changes headless-compatible
- **V. Feature Flags**: No new required dependencies
- **VI. TDD**: Tests written before implementation (Red-Green-Refactor)
- **VII. Rust 2024 Standards**: Zero warnings policy maintained

### ⚠️ Requires Justification

- **I. Immutability-First Architecture**: Currently VIOLATED by Panel::collapsed and set_visibility_rule() - this feature FIXES the violation
- **II. Composition Over Proliferation**: Adding convenience constructors (Text::required()) but NOT new types - composition preserved

### 🔴 Critical Gates

1. **Immutability Violation Fix (FR-001, FR-002)**: 
   - **Current violation**: Panel::collapsed field (mutable schema state)
   - **Fix required**: Move to Context/RuntimeNode
   - **Complexity justification**: Required to comply with Constitution Principle I
   - **Alternative rejected**: Keeping mutable state in schema - violates core architecture

2. **Breaking Changes (FR-004, FR-005)**:
   - **Change**: Remove set_visibility_rule(&mut self) from all 23 types
   - **Impact**: Breaks existing code calling this method
   - **Justification**: Pre-1.0, fixing architectural violation takes precedence
   - **Mitigation**: Deprecation warnings for 1-2 versions, migration guide provided

3. **API Surface Changes (FR-007 to FR-015)**:
   - **Changes**: Add 15+ new convenience methods
   - **Risk**: API bloat if not carefully designed
   - **Justification**: Reduces boilerplate 40-50%, improves adoption
   - **Mitigation**: Follow builder pattern, mark advanced methods clearly

## Project Structure

### Documentation (this feature)

```text
specs/002-code-quality-improvements/
├── plan.md              # This file
├── spec.md              # Feature specification
├── PROBLEMS_SUMMARY.md  # 20 problems identified
├── research.md          # Phase 0 output (runtime state patterns, migration strategies)
├── data-model.md        # Phase 1 output (PanelState, ValidationError enhancements)
├── quickstart.md        # Phase 1 output (updated examples)
├── contracts/           # Phase 1 output (API contracts for new methods)
│   ├── context-api.md
│   ├── builder-api.md
│   ├── error-api.md
│   └── performance-api.md
└── checklists/
    └── requirements.md  # Quality validation (already complete)
```

### Source Code (repository root)

```text
src/
├── core/                # Error enhancements (field paths, hints)
│   ├── error.rs         # Modified: Add path to ValidationError, hints to Error
│   └── value/           # New: ValueBuilder for ergonomic construction
│       └── builder.rs   # New file
├── types/
│   ├── group/
│   │   └── panel.rs     # Modified: Remove collapsed field, deprecate set_collapsed
│   ├── leaf/
│   │   ├── text.rs      # Modified: Add required(), validate_*() shortcuts
│   │   ├── number.rs    # Modified: Add required(), validate_*() shortcuts
│   │   └── boolean.rs   # Modified: Add required() shortcut
│   └── container/
│       └── object.rs    # Modified: Add fields() bulk method
├── context/
│   ├── mod.rs           # Modified: Add from_schema(), get_*_or(), get_many()
│   └── ui_state.rs      # New: PanelState and UI state management
├── event/
│   └── types.rs         # Modified: Change Event to use Arc<Value>
├── runtime/
│   └── state.rs         # Modified: Add UI state management
└── validation/
    └── error.rs         # Modified: Add path field to ValidationError

tests/
├── immutability_tests.rs    # New: Verify schema immutability
├── ergonomics_tests.rs      # New: Test new convenience APIs
├── performance_tests.rs     # New: Benchmark optimizations
└── migration_tests.rs       # New: Backward compatibility with deprecations

benches/
├── event_cloning.rs         # New: Measure Arc<Value> improvement
└── transactional.rs         # New: Measure stack buffer optimization

docs/
└── MIGRATION-0.3-to-0.4.md  # New: Migration guide for breaking changes
```

**Structure Decision**: Single project structure (src/, tests/, benches/). No new crates needed - all changes within existing paramdef crate. Documentation in specs/ directory as per project standards.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| Breaking changes (remove set_collapsed, set_visibility_rule) | Fix Principle I (Immutability-First) architectural violation | Keeping violations - undermines entire architecture |
| Add 15+ convenience methods | Reduce 40-50% boilerplate, improve adoption | Users write boilerplate - poor DX, adoption suffers |
| Change Event to Arc<Value> | Fix 3x cloning on every set() with events | Keep cloning - 66% performance waste in hot path |

**Architectural Debt Being Paid**: Panel::collapsed violates immutability since project inception. This feature pays that debt by properly separating schema/runtime layers.

## Phase 0: Research & Planning

### Research Tasks

#### R1: Runtime State Management Patterns

**Question**: How should Panel UI state (collapsed/expanded) be stored in Context without breaking schema immutability?

**Options to explore**:
1. **Separate UiStateManager** - New component in Context for all UI state
2. **Extend RuntimeNode** - Add ui_state field to RuntimeNode<Panel>
3. **HashMap in Context** - Simple Key → UiState mapping

**Research needed**:
- Pros/cons of each approach
- Impact on existing Context API
- Thread safety considerations
- Serialization requirements

**Deliverable**: Decision matrix in research.md with recommended approach

---

#### R2: Migration Strategy for Breaking Changes

**Question**: How to deprecate set_collapsed() and set_visibility_rule() with minimal user pain?

**Options to explore**:
1. **Hard break immediately** - Remove methods, provide migration guide
2. **Deprecate for 2 versions** - #[deprecated] with helpful messages
3. **Runtime warning** - Methods stay but emit warnings

**Research needed**:
- Rust deprecation best practices
- How other crates handle pre-1.0 breaking changes
- Tooling for automated migration (cargo fix compatibility)

**Deliverable**: Deprecation plan with timeline and messaging

---

#### R3: ValueBuilder API Design

**Question**: What API provides ergonomic Value::Object construction without complexity?

**Options to explore**:
1. **Fluent builder** - ValueBuilder::new().field().field().build()
2. **Macro-based** - value_obj!{ "name" => "Alice", "age" => 30 }
3. **From trait magic** - Object::from([("name", "Alice"), ...])

**Research needed**:
- User expectations from similar builders (serde_json, HashMap)
- Type inference challenges
- Performance implications

**Deliverable**: API design with examples in research.md

---

#### R4: Event Arc<Value> Migration

**Question**: Can we change Event enum to Arc<Value> without breaking existing event subscribers?

**Research needed**:
- Current Event usage patterns in examples
- Backward compatibility strategy
- Performance measurement approach (before/after benchmarks)

**Deliverable**: Migration plan and benchmark baseline

---

#### R5: Error Message Hint System

**Question**: How to add actionable hints to Error types without breaking serialization?

**Options to explore**:
1. **Separate hint() method** - Error::hint() → Option<&str>
2. **Embedded in Display** - Include hint in error message
3. **Structured ErrorContext** - Additional context field

**Research needed**:
- serde compatibility (if hints should serialize)
- Error trait impl impact
- thiserror macro compatibility

**Deliverable**: Design document with implementation approach

---

### Research Deliverables

**Output file**: `specs/002-code-quality-improvements/research.md`

**Structure**:
```markdown
# Research: Code Quality Improvements

## R1: Runtime State Management
**Decision**: [Chosen approach]
**Rationale**: [Why chosen]
**Alternatives**: [What else considered]
**Implementation notes**: [Key points]

## R2: Migration Strategy
...

[Repeat for R3, R4, R5]

## Summary
- All NEEDS CLARIFICATION resolved: YES/NO
- Ready for Phase 1: YES/NO
```

---

## Phase 1: Design & Contracts

**Prerequisites:** research.md complete with all decisions made

### Design Artifacts

#### D1: Data Model (`data-model.md`)

**Entities to define**:

1. **PanelState**
   - Fields: collapsed (bool), last_interaction (Option<Instant>)
   - Location: Context or RuntimeNode
   - Serialization: Required for persistence
   - Default: collapsed = false

2. **ValidationError (enhanced)**
   - New field: path (String) - dot-separated path like "user.address.email"
   - Field: field (String) - leaf field name
   - Field: code (String) - error type identifier
   - Field: message (String) - human-readable message
   - Backward compatibility: Existing fields unchanged

3. **ValueBuilder**
   - Fields: fields (Vec<(Key, Value)>)
   - Methods: field(key, value) → Self, build() → Value
   - Type conversions: impl<T: Into<Value>> support

4. **RollbackStorage**
   - Enum: Small([Option<Value>; 8]) | Large(FxHashMap<Key, Option<Value>>)
   - Purpose: Efficient transactional rollback
   - Used in: Context::set_many_transactional

5. **ErrorHint**
   - Fields: code (String), suggestion (String), example (Option<String>)
   - Purpose: Actionable error guidance
   - Attached to: Error variants

**Relationships**:
- Context HAS-MANY PanelState (via FxHashMap<Key, PanelState>)
- ValidationError CONTAINS path + field (composition)
- ValueBuilder PRODUCES Value (factory pattern)

---

#### D2: API Contracts (`contracts/`)

**File**: `context-api.md`

```rust
// Context convenience methods
impl Context {
    /// Creates context from schema, auto-wrapping in Arc
    pub fn from_schema(schema: Schema) -> Self;
    
    /// Get text with fallback default
    pub fn get_text_or<'a>(&'a self, key: &str, default: &'a str) -> &'a str;
    
    /// Get int with fallback default
    pub fn get_int_or(&self, key: &str, default: i64) -> i64;
    
    /// Get bool with fallback default
    pub fn get_bool_or(&self, key: &str, default: bool) -> bool;
    
    /// Bulk getter returning iterator
    pub fn get_many<'a, I>(&'a self, keys: I) -> impl Iterator<Item = (&'a Key, Option<&'a Value>)>
    where I: IntoIterator<Item = &'a str>;
    
    /// Set Panel UI state (collapsed/expanded)
    pub fn set_panel_collapsed(&mut self, key: &Key, collapsed: bool) -> Result<()>;
    
    /// Get Panel UI state
    pub fn is_panel_collapsed(&self, key: &Key) -> bool;
}
```

**File**: `builder-api.md`

```rust
// Text convenience constructors
impl Text {
    /// Create required text field with label
    pub fn required(key: impl Into<Key>, label: impl Into<SmartStr>) -> Self;
}

// Text validation shortcuts
#[cfg(feature = "validation")]
impl<S: TextSubtype> TextBuilder<S> {
    pub fn validate_required(self) -> Self;
    pub fn validate_email(self) -> Self;
    pub fn validate_min_length(self, min: usize) -> Self;
    pub fn validate_max_length(self, max: usize) -> Self;
}

// Number shortcuts
impl Number {
    pub fn required(key: impl Into<Key>, label: impl Into<SmartStr>) -> Self;
}

#[cfg(feature = "validation")]
impl<S: NumberSubtype> NumberBuilder<S> {
    pub fn validate_required(self) -> Self;
    pub fn validate_min(self, min: f64) -> Self;
    pub fn validate_max(self, max: f64) -> Self;
}

// Boolean shortcuts
impl Boolean {
    pub fn required(key: impl Into<Key>, label: impl Into<SmartStr>) -> Self;
}

// Object bulk fields
impl ObjectBuilder {
    pub fn fields(self, pairs: impl IntoIterator<Item = (impl Into<Key>, impl Node + 'static)>) -> Self;
}
```

**File**: `error-api.md`

```rust
// Enhanced ValidationError
pub struct ValidationError {
    pub path: String,        // "user.address.email"
    pub field: String,       // "email"
    pub code: String,        // "invalid_format"
    pub message: String,     // "Invalid email format"
}

// Error with hints
impl Error {
    pub fn hint(&self) -> Option<&str>;
}

// Example hints
TypeMismatch { key, expected, actual } 
  → hint: "Use ctx.get_{expected_type}() for {expected} values"

NotFound { key }
  → hint: "Available keys: {keys}. Did you mean '{suggestion}'?"
```

**File**: `performance-api.md`

```rust
// Event with Arc<Value>
pub enum Event {
    ValueChanged {
        key: Key,
        old_value: Option<Arc<Value>>,  // Shared, not cloned
        new_value: Arc<Value>,           // Shared, not cloned
    },
    // ... other variants
}

// ValueBuilder
pub struct ValueBuilder {
    fields: Vec<(Key, Value)>,
}

impl ValueBuilder {
    pub fn new() -> Self;
    pub fn field(mut self, key: impl Into<Key>, value: impl Into<Value>) -> Self;
    pub fn build(self) -> Value;
}

impl Value {
    pub fn object_builder() -> ValueBuilder;
}
```

---

#### D3: Quickstart (`quickstart.md`)

**Purpose**: Verify the improved API with real examples

```markdown
# Quickstart: Code Quality Improvements

## Before: Creating a Required Field (4 lines)

```rust
let email = Text::builder("email")
    .label("Email Address")
    .required()
    .build();
```

## After: Creating a Required Field (1 line)

```rust
let email = Text::required("email", "Email Address");
```

## Before: Validation Setup (5+ lines)

```rust
use paramdef::validation::Rule;
use paramdef::expr::Expr;

let email = Text::builder("email")
    .rules(Rules::from_rules([
        Rule::local(Expr::required()),
        Rule::local(Expr::email()),
    ]))
    .build();
```

## After: Validation Setup (2 lines)

```rust
let email = Text::builder("email")
    .validate_required()
    .validate_email()
    .build();
```

## Before: Context Creation (2 lines)

```rust
let schema = Schema::builder()...build();
let ctx = Context::new(Arc::new(schema));
```

## After: Context Creation (1 line)

```rust
let schema = Schema::builder()...build();
let ctx = Context::from_schema(schema);
```

## Before: Error Recovery (Manual unwrap_or)

```rust
let name = ctx.get_text("name").unwrap_or("Anonymous");
```

## After: Error Recovery (Built-in)

```rust
let name = ctx.get_text_or("name", "Anonymous");
```

## Validation Errors with Paths

```rust
// Before: "email" - which email?
Error::Validation { fields: vec!["email"] }

// After: Full path
Error::Validation { 
    errors: vec![ValidationError {
        path: "user.address.email",
        field: "email",
        code: "invalid_format",
        message: "Invalid email format",
    }]
}
```

## Error Hints

```rust
// Before: Just the error
Err(Error::TypeMismatch { 
    key: "age", 
    expected: ValueKind::Int, 
    actual: ValueKind::Text 
})

// After: With actionable hint
Err(Error::TypeMismatch { ... })
// Error message: "Type mismatch for 'age': expected Int, got Text. 
//                 Hint: Use ctx.get_int() for integer values."
```
```

**Validation**: All examples must compile and run successfully.

---

## Phase 1 Complete: Re-evaluate Constitution Check

After completing Phase 1 design:

### ✅ Compliance Verified

1. **Immutability-First**: PanelState design keeps schema immutable ✅
2. **Composition**: No new node types, only convenience methods ✅
3. **Feature Flags**: All new APIs respect feature gates ✅
4. **TDD**: Test structure defined, ready for Red-Green-Refactor ✅

### 📋 Documentation Updated

- [ ] Update docs/01-ARCHITECTURE.md (Panel state management)
- [ ] Update docs/17-DESIGN-DECISIONS.md (Why Arc<Value> in events)
- [ ] Update CLAUDE.md (New convenience APIs)
- [ ] Create docs/MIGRATION-0.3-to-0.4.md

---

## Implementation Phases (Post-Planning)

### Week 1: Immutability Fixes (P1 Critical)

**Tasks** (will be in tasks.md):
- Remove Panel::collapsed field
- Add PanelState to Context
- Remove/deprecate set_collapsed() 
- Deprecate set_visibility_rule() on all 23 types
- Write immutability tests
- Update migration guide

**Deliverable**: Schema types are truly immutable

---

### Week 2: API Ergonomics (P1 High)

**Tasks** (will be in tasks.md):
- Add Context::from_schema()
- Add Text::required(), Number::required(), Boolean::required()
- Add validation shortcuts (.validate_email(), etc.)
- Add error recovery methods (get_*_or())
- Enhance ValidationError with paths
- Add Object::fields() bulk method
- Add ValueBuilder

**Deliverable**: 40-50% less boilerplate code

---

### Week 3: Performance (P2)

**Tasks** (will be in tasks.md):
- Change Event to Arc<Value>
- Add RollbackStorage with stack buffer
- Add Context::get_many() bulk getter
- Write performance benchmarks
- Verify 66% clone reduction, 20-30% throughput gain

**Deliverable**: Performance improvements verified

---

### Week 4: Documentation (P2)

**Tasks** (will be in tasks.md):
- Fix all doc examples (remove `ignore`)
- Create COOKBOOK.md with 10+ recipes
- Add error handling guide
- Correct type count (14→23)
- Add error hints to all Error variants
- Review and polish API docs

**Deliverable**: 95%+ examples compile, comprehensive cookbook

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 0 (Research)**: No dependencies - can start immediately
- **Phase 1 (Design)**: Depends on Phase 0 completion
- **Week 1 (Immutability)**: Depends on Phase 1, BLOCKS all other weeks
- **Week 2 (Ergonomics)**: Depends on Week 1 completion
- **Week 3 (Performance)**: Can run parallel with Week 2 (different modules)
- **Week 4 (Documentation)**: Depends on Weeks 1-3 (documents the changes)

### Critical Path

```
Phase 0 (Research) → Phase 1 (Design) → Week 1 (Immutability) → {
    Week 2 (Ergonomics) ↘
                         → Week 4 (Documentation)
    Week 3 (Performance) ↗
}
```

**Minimum viable delivery**: Week 1 + Week 2 = Immutability fixes + Ergonomics (2 weeks)

---

## Success Metrics

| Metric | Before | Target | How to Measure |
|--------|--------|--------|----------------|
| Mutable schema fields | 1 (Panel) | 0 | Grep for `pub.*mut` in schema types |
| `&mut self` on schema | 2 | 0 | Grep for trait methods with `&mut self` |
| Boilerplate reduction | 100% | 50-60% | Line count in examples before/after |
| Doc examples compile | ~80% | 95%+ | `cargo test --doc` success rate |
| Value clones per set() | 3 | 1 | Benchmark event emission |
| Throughput improvement | baseline | +20-30% | Criterion benchmarks |
| Validation error paths | 0% | 100% | Test nested object validation |

---

## Risk Mitigation

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Breaking changes anger users | MEDIUM | HIGH | Deprecation warnings, migration guide, pre-1.0 communication |
| Performance regressions | LOW | MEDIUM | Comprehensive benchmarks, revert if regression >5% |
| API bloat (too many methods) | MEDIUM | MEDIUM | Mark advanced methods, group in docs, follow conventions |
| Test coverage drops | LOW | HIGH | Enforce 90%+ coverage in CI, TDD for all changes |
| Migration complexity | MEDIUM | MEDIUM | Provide cargo-fix compatible deprecations where possible |

---

## Next Steps

1. ✅ **Complete Phase 0**: Generate research.md with all decisions
2. ✅ **Complete Phase 1**: Generate data-model.md, contracts/, quickstart.md
3. ⏭️ **Generate tasks.md**: Use `/speckit.tasks` command (NOT part of this command)
4. ⏭️ **Implementation**: Follow tasks.md with TDD discipline

**Note**: This planning phase ends here. Task generation is a separate command.
