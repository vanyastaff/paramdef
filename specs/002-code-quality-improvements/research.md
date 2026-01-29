# Research: Code Quality Improvements

**Feature**: 002-code-quality-improvements  
**Created**: 2026-01-28  
**Research Status**: Complete  
**Ready for Implementation**: YES

---

## R1: Runtime State Management

**Decision**: Option (a) - Separate `UiStateManager` in Context  
**Priority**: P1 - CRITICAL

### Rationale

After analyzing the three-layer architecture and current Context implementation, a separate UI state manager is the cleanest solution that maintains architectural boundaries:

1. **Architectural Consistency**: Schema Layer (immutable) → Runtime Layer (mutable state) → Value Layer (data)
   - Panel's `collapsed` field breaks immutability by storing UI state in schema
   - RuntimeNode already handles Value and State, but State is for validation/dirty tracking
   - UI state (collapsed, scroll position, etc.) is orthogonal to parameter state

2. **Separation of Concerns**: 
   - RuntimeNode::State tracks: dirty, touched, valid, errors (parameter-specific)
   - UI state tracks: collapsed, selected tab, scroll position (presentation-specific)
   - Mixing these violates single responsibility principle

3. **Thread Safety**: 
   - Schema remains Send + Sync with zero synchronization
   - UiStateManager can be `!Send` if needed (UI is often single-threaded)
   - No impact on core schema sharing across threads

4. **Serialization**: 
   - UI state can be separately serialized/persisted
   - Schema serialization remains clean (no runtime state leakage)
   - Users can choose to save/restore UI state independently

### Alternatives Rejected

**Option (b) - Extend RuntimeNode**:
- ❌ Bloats RuntimeNode with UI concerns that don't apply to all nodes
- ❌ Forces all 23 node types to carry UI state baggage
- ❌ Mixes validation state with presentation state
- ✅ But: simpler API (single state location)

**Option (c) - HashMap in Context**:
- ❌ Type-unsafe (stores arbitrary state)
- ❌ No compile-time validation of state structure
- ❌ Harder to document and discover
- ✅ But: most flexible for future extensions

### Implementation

```rust
/// UI state manager for presentation-only state (not parameter data).
///
/// This stores state like collapsed panels, selected tabs, scroll positions
/// that are specific to UI presentation and should not be part of immutable schema.
pub struct UiStateManager {
    panel_states: FxHashMap<Key, PanelState>,
}

pub struct PanelState {
    collapsed: bool,
    last_interaction: Option<Instant>,
}

impl Context {
    pub fn ui_state(&self) -> &UiStateManager { /* ... */ }
    pub fn ui_state_mut(&mut self) -> &mut UiStateManager { /* ... */ }
    
    // Convenience methods
    pub fn is_panel_collapsed(&self, key: &str) -> bool { /* ... */ }
    pub fn set_panel_collapsed(&mut self, key: &str, collapsed: bool) { /* ... */ }
}
```

**Migration Path**:
1. Remove `collapsed: bool` from Panel struct
2. Add `collapsed(bool)` to PanelBuilder (sets default state only)
3. Add UiStateManager to Context with convenience methods
4. Update examples and documentation
5. Deprecate `Layout::set_collapsed()` for 2 versions before removal

**Files to Modify**:
- `src/types/group/panel.rs` - Remove field, update builder
- `src/types/traits/category.rs` - Deprecate Layout::set_collapsed
- `src/context/mod.rs` - Add UiStateManager
- Create `src/context/ui_state.rs` - New module

**Testing Strategy**:
- Unit tests: PanelState creation, mutation
- Integration tests: Multiple contexts with independent UI state
- Thread safety test: Schema shared, UI state per-thread
- Serialization test: UI state separate from schema JSON

---

## R2: Migration Strategy

**Decision**: Option (b) - Deprecate for 2 versions with clear migration path  
**Priority**: P1 - CRITICAL

### Rationale

1. **Pre-1.0 Flexibility**: paramdef is 0.3.1, so breaking changes are acceptable per SemVer
   - However, existing users deserve a smooth transition
   - Warnings give time to migrate without breaking production code

2. **Rust Best Practices** ([Rust Reference](https://doc.rust-lang.org/reference/attributes/diagnostics.html)):
   - `#[deprecated]` is the idiomatic approach
   - Supports `since` and `note` for migration guidance
   - Integrates with compiler warnings (respects `--cap-lints`)

3. **Community Standards** ([RFC 1270](https://rust-lang.github.io/rfcs/1270-deprecation.html)):
   - Deprecation is always done on version change
   - `note` should explain reasoning and provide alternatives
   - 1-2 version grace period is industry standard

4. **User Experience**:
   - Hard breaks frustrate early adopters
   - Runtime warnings are invisible in release mode
   - Compile-time warnings are visible, actionable, and preventable with `#[allow]`

### Alternatives Rejected

**Option (a) - Hard break**:
- ❌ Breaks existing code immediately
- ❌ No migration period for users
- ❌ Generates negative sentiment
- ✅ But: simplest implementation, cleaner codebase immediately

**Option (c) - Runtime warning**:
- ❌ Only visible when function is actually called
- ❌ Doesn't work in release mode (warnings stripped)
- ❌ No compiler integration (can't use `#[allow(deprecated)]`)
- ❌ Requires maintaining deprecated code paths forever
- ✅ But: softest migration path

### Implementation

```rust
// Phase 1 (v0.4.0): Deprecate with warnings
impl Layout for Panel {
    #[deprecated(
        since = "0.4.0",
        note = "Panel collapsed state is now managed per-context. \
                Use `context.set_panel_collapsed(key, collapsed)` instead. \
                This method will be removed in v0.6.0."
    )]
    fn set_collapsed(&mut self, collapsed: bool) {
        // Keep implementation for backward compat
        self.collapsed = collapsed;
    }
}

impl Visibility for Panel {
    #[deprecated(
        since = "0.4.0", 
        note = "Visibility rules must be set during construction. \
                Use `.visible_when(expr)` in the builder instead. \
                This method will be removed in v0.6.0."
    )]
    fn set_visibility_rule(&mut self, expr: Option<Rule>) {
        self.visibility = expr;
    }
}

// Phase 2 (v0.5.0): Keep deprecation warnings, no changes

// Phase 3 (v0.6.0): Remove deprecated methods entirely
// - Delete set_collapsed() implementation
// - Delete set_visibility_rule() implementation
// - Remove mutable fields from schema structs
```

**Migration Path**:

1. **v0.4.0** (Week 1): Add deprecation warnings
   - All old code still works
   - Compiler emits helpful warnings
   - Documentation shows new patterns

2. **v0.5.0** (Week 5): Maintain warnings
   - Give users 1 minor version to migrate
   - Monitor GitHub issues for migration pain points
   - Update examples in ecosystem

3. **v0.6.0** (Week 9): Hard removal
   - Delete deprecated methods
   - Clean up schema structs (truly immutable)
   - Update to Edition 2024 if beneficial

**CHANGELOG.md Entry**:

```markdown
## [0.4.0] - 2026-02-05

### Deprecated

- **Panel::set_collapsed()** - Panel collapsed state is now managed per-context via `Context::set_panel_collapsed()`. Will be removed in v0.6.0.
- **Visibility::set_visibility_rule()** - Visibility rules must be set during construction via builder pattern. Will be removed in v0.6.0.

### Added

- `Context::ui_state()` - Access UI state manager
- `Context::set_panel_collapsed()` - Set panel collapsed state per-context
- `UiStateManager` - Separate manager for presentation state

### Migration Guide

**Before:**
```rust
let mut panel = Panel::builder("settings").build();
panel.set_collapsed(true);
```

**After:**
```rust
let panel = Panel::builder("settings")
    .collapsed(true)  // Set initial state
    .build();
ctx.set_panel_collapsed("settings", true);  // Modify at runtime
```
```

**Testing Strategy**:
- Add `#[allow(deprecated)]` to old tests
- Create new tests using recommended patterns
- CI must compile with `-D warnings` (fails on deprecation)
- Add separate CI job with `--cap-lints warn` to test backward compat

---

## R3: ValueBuilder API

**Decision**: Option (a) - Fluent builder with type inference  
**Priority**: P2 - HIGH

### Rationale

1. **Consistency with Existing Patterns**: 
   - paramdef uses builders everywhere (TextBuilder, NumberBuilder, SchemaBuilder)
   - Users already understand `.builder()` → `.field()` → `.build()` pattern
   - No new cognitive overhead

2. **Type Safety**: 
   - Builder enforces valid Value::Object construction
   - Type system prevents invalid structures at compile time
   - IDE autocomplete works well with method chaining

3. **Ergonomics**: 
   ```rust
   // Before (verbose)
   let value = Value::Object(Arc::new(indexmap! {
       Key::from("name") => Value::Text("Alice".into()),
       Key::from("age") => Value::Int(30),
   }.into_iter().collect()));
   
   // After (fluent)
   let value = Value::object()
       .field("name", Value::text("Alice"))
       .field("age", Value::Int(30))
       .build();
   ```

4. **Performance**: 
   - Pre-allocates capacity if known
   - Single Arc allocation for final HashMap
   - No overhead vs manual construction

### Alternatives Rejected

**Option (b) - Macro-based**:
- ❌ Macros have worse error messages
- ❌ IDE support often incomplete (autocomplete, refactoring)
- ❌ Harder to document (macro rules vs methods)
- ✅ But: most concise syntax (`value!({ name: "Alice" })`)

**Option (c) - From trait**:
- ❌ Limited to standard Rust collections (HashMap, BTreeMap)
- ❌ Can't enforce Key type (From<&str> vs Key::from)
- ❌ No capacity pre-allocation
- ✅ But: minimal code, familiar pattern

### Implementation

```rust
pub struct ValueBuilder {
    capacity: Option<usize>,
    fields: IndexMap<Key, Value>,
}

impl Value {
    /// Creates a builder for Value::Object.
    pub fn object() -> ValueBuilder {
        ValueBuilder::new()
    }
    
    /// Creates a builder with known capacity.
    pub fn object_with_capacity(capacity: usize) -> ValueBuilder {
        ValueBuilder::with_capacity(capacity)
    }
}

impl ValueBuilder {
    pub fn new() -> Self {
        Self {
            capacity: None,
            fields: IndexMap::new(),
        }
    }
    
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            capacity: Some(capacity),
            fields: IndexMap::with_capacity(capacity),
        }
    }
    
    /// Adds a field to the object.
    pub fn field(mut self, key: impl Into<Key>, value: impl Into<Value>) -> Self {
        self.fields.insert(key.into(), value.into());
        self
    }
    
    /// Adds multiple fields at once.
    pub fn fields<I, K, V>(mut self, fields: I) -> Self 
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<Key>,
        V: Into<Value>,
    {
        self.fields.extend(
            fields.into_iter()
                .map(|(k, v)| (k.into(), v.into()))
        );
        self
    }
    
    /// Builds the Value::Object.
    pub fn build(self) -> Value {
        Value::Object(Arc::new(self.fields))
    }
}

// Convenience implementations
impl From<ValueBuilder> for Value {
    fn from(builder: ValueBuilder) -> Self {
        builder.build()
    }
}
```

**User Experience**:

```rust
// Simple case
let user = Value::object()
    .field("id", Value::Int(123))
    .field("name", Value::text("Alice"))
    .field("active", Value::Bool(true))
    .build();

// Nested objects
let profile = Value::object()
    .field("user", user)
    .field("settings", Value::object()
        .field("theme", Value::text("dark"))
        .field("notifications", Value::Bool(true))
        .build())
    .build();

// With capacity hint (performance)
let large = Value::object_with_capacity(100)
    .fields(generate_fields())  // Bulk insert
    .build();

// Implicit conversion
fn accept_value(v: impl Into<Value>) { /* ... */ }
accept_value(
    Value::object()
        .field("x", 1)
        .field("y", 2)
);  // No .build() needed!
```

**Files to Modify**:
- `src/core/value/mod.rs` - Add `object()` methods
- Create `src/core/value/builder.rs` - ValueBuilder implementation
- `src/core/value/mod.rs` - Export builder

**Testing Strategy**:
- Unit tests: empty, single field, multiple fields
- Capacity tests: verify pre-allocation works
- Nested tests: objects containing objects
- From trait test: implicit conversion
- Performance benchmark: builder vs manual vs macro

---

## R4: Event Arc<Value>

**Decision**: Change Event to `Arc<Value>` with backward-compatible accessor methods  
**Priority**: P2 - MEDIUM

### Rationale

1. **Performance Impact** ([Arc Performance Research](https://www.ardanlabs.com/blog/2024/11/fearless-concurrency-ep6-understanding-rust-arc-for-efficient-multithreading.html)):
   - Current: 3 Value clones per `set()` operation (old_value, new_value, event emission)
   - Proposed: 1 clone + Arc wrapping = 66% reduction
   - Arc is faster than cloning for data >64 bytes (common for objects/arrays)
   - Atomic refcount overhead is negligible compared to deep Value cloning

2. **Memory Efficiency**:
   - Large Values (1KB+ objects) shared via pointer instead of copied
   - Event subscribers don't trigger additional clones
   - Broadcast channel holds Arc, not full Value

3. **Backward Compatibility Strategy**:
   ```rust
   // Public API remains unchanged
   impl Event {
       pub fn old_value(&self) -> &Value {
           &self.old_value  // Deref happens automatically
       }
       
       pub fn new_value(&self) -> &Value {
           &self.new_value
       }
       
       // For users who need Arc (advanced)
       pub fn old_value_arc(&self) -> &Arc<Value> {
           &self.old_value
       }
   }
   ```

4. **Current Usage Patterns** (from grep search):
   - Zero external Arc<Value> usage found in codebase
   - All Event usage goes through reference accessors
   - No breaking changes for existing event subscribers

### Alternatives Rejected

**Keep cloning**:
- ❌ Performance bottleneck in high-frequency updates
- ❌ 3x memory churn for large values
- ✅ But: simplest (no change needed)

**Add Arc<Value> as new Event variant**:
- ❌ Doubles API surface (Event and ArcEvent)
- ❌ Users must choose which to subscribe to
- ❌ Type system doesn't prevent misuse
- ✅ But: perfect backward compatibility

**Use Cow<Value>**:
- ❌ Still clones on write (not helpful for events)
- ❌ More complex than Arc
- ❌ No ref-counting benefits
- ✅ But: can upgrade to owned if needed

### Implementation

```rust
// Phase 1: Change internal representation
pub enum Event {
    ValueChanging {
        key: Key,
        old_value: Arc<Value>,
        new_value: Arc<Value>,
    },
    ValueChanged {
        key: Key, 
        old_value: Arc<Value>,
        new_value: Arc<Value>,
    },
    // ... other variants
}

// Phase 2: Keep public API unchanged (Deref coercion)
impl Event {
    /// Returns reference to old value (zero-cost via Arc deref).
    pub fn old_value(&self) -> Option<&Value> {
        match self {
            Event::ValueChanging { old_value, .. } |
            Event::ValueChanged { old_value, .. } => Some(old_value),
            _ => None,
        }
    }
    
    /// Advanced: Get Arc for sharing across threads or long lifetime.
    pub fn old_value_arc(&self) -> Option<Arc<Value>> {
        match self {
            Event::ValueChanging { old_value, .. } |
            Event::ValueChanged { old_value, .. } => Some(Arc::clone(old_value)),
            _ => None,
        }
    }
}

// Phase 3: Update Context to emit Arc<Value>
impl Context {
    pub fn set(&mut self, key: &str, value: Value) -> Result<()> {
        let node = self.nodes.get_mut(key)?;
        
        #[cfg(feature = "events")]
        let old_value_arc = node.value()
            .map(|v| Arc::new(v.clone()));  // Clone ONCE here
        let new_value_arc = Arc::new(value.clone());  // Wrap new value
        
        #[cfg(feature = "events")]
        if let Some(ref bus) = self.event_bus {
            // old_value_arc and new_value_arc are Arc, no cloning!
            bus.emit(Event::value_changing(
                key, 
                old_value_arc.clone(),
                new_value_arc.clone()
            ));
        }
        
        node.set_value((*new_value_arc).clone());  // Unavoidable clone for storage
        
        #[cfg(feature = "events")]
        if let Some(ref bus) = self.event_bus {
            bus.emit(Event::value_changed(
                key,
                old_value_arc,
                new_value_arc
            ));
        }
        
        Ok(())
    }
}
```

**Migration Path**:
1. Change Event struct fields to Arc<Value>
2. Add convenience accessor methods (returning &Value)
3. Update Context::set() to wrap in Arc once
4. Run benchmarks to verify improvement
5. Document performance characteristics

**Benchmark Approach**:

```rust
// Before: 3 clones
#[bench]
fn bench_set_with_events_before(b: &mut Bencher) {
    let mut ctx = Context::with_event_bus(schema, bus);
    b.iter(|| {
        ctx.set("field", large_value.clone());
    });
}

// After: 1 clone + Arc
#[bench]
fn bench_set_with_events_after(b: &mut Bencher) {
    let mut ctx = Context::with_event_bus(schema, bus);
    b.iter(|| {
        ctx.set("field", large_value.clone());
    });
}

// Measure both throughput (ops/sec) and memory (allocations)
```

**Files to Modify**:
- `src/event/event.rs` - Change Value to Arc<Value>
- `src/context/mod.rs` - Update set() emission logic
- `benches/events.rs` - Add before/after benchmarks
- `docs/19-EVENT-SYSTEM.md` - Document performance characteristics

**Testing Strategy**:
- Unit tests: Event accessor methods return correct references
- Integration tests: Existing event tests pass unchanged
- Benchmark tests: Verify >50% reduction in cloning overhead
- Thread safety: Arc<Value> shares correctly across threads

---

## R5: Error Hints

**Decision**: Option (c) - Structured ErrorContext with thiserror integration  
**Priority**: P2 - HIGH

### Rationale

1. **thiserror Best Practices** ([thiserror documentation](https://docs.rs/thiserror)):
   - Custom Display impl can include context beyond field interpolation
   - `#[error]` attribute generates Display automatically
   - Hints can be part of Display output without breaking API

2. **Serialization Compatibility**:
   - `hint()` method doesn't affect serde (marked `#[serde(skip)]` or separate)
   - Display output includes hints for human logs
   - Structured error fields remain machine-readable

3. **Error Trait Impact**:
   - Hints don't affect `source()` or `std::error::Error` impl
   - Backward compatible (hint is additional info, not replacement)
   - No breaking changes to existing error handling code

4. **User Experience**:
   ```rust
   // Before
   Error: type mismatch for key 'age': expected Int, got Text
   
   // After  
   Error: type mismatch for key 'age': expected Int, got Text
   Hint: Use `ctx.get_int("age")` for integer values, or `ctx.get_text("age")` for text values
   ```

### Alternatives Rejected

**Option (a) - Separate hint() method**:
- ❌ Hints not visible in default error messages
- ❌ Users must explicitly call `.hint()` to see them
- ❌ Logs miss helpful context
- ✅ But: clean separation of concerns

**Option (b) - Embedded in Display**:
- ❌ Makes error strings very long
- ❌ Hard to parse programmatically (mixed with message)
- ❌ Can't disable hints in production
- ✅ But: always visible, no extra method calls

### Implementation

```rust
/// Errors that can occur during parameter operations.
#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("type mismatch for key '{key}': expected {expected}, got {actual}")]
    TypeMismatch {
        key: String,
        expected: ValueKind,
        actual: ValueKind,
        #[serde(skip)]
        hint: Option<String>,
    },
    
    #[error("validation failed: {message}")]
    Validation {
        code: String,
        message: String,
        fields: Vec<String>,
        #[serde(skip)]
        hint: Option<String>,
    },
    
    // ... other variants
}

impl Error {
    /// Returns an actionable hint for resolving this error.
    pub fn hint(&self) -> Option<&str> {
        match self {
            Self::TypeMismatch { hint, expected, actual, .. } => {
                hint.as_deref().or_else(|| {
                    Some(match (expected, actual) {
                        (ValueKind::Int, ValueKind::Text) => 
                            "Use `get_int()` instead of `get_text()`, or convert the value",
                        (ValueKind::Text, ValueKind::Int) => 
                            "Use `get_text()` instead of `get_int()`, or convert the value",
                        (ValueKind::Array, _) => 
                            "Use `get_array()` to access array values",
                        (ValueKind::Object, _) => 
                            "Use `get_object()` to access object fields",
                        _ => "Check the expected type and use the appropriate getter method",
                    })
                })
            }
            Self::Validation { hint, code, .. } => {
                hint.as_deref().or_else(|| {
                    Some(match code.as_str() {
                        "required" => "This field cannot be empty. Provide a value or remove the REQUIRED flag",
                        "email" => "Provide a valid email address (e.g., user@example.com)",
                        "min_length" => "The value is too short. Check the minimum length constraint",
                        "max_length" => "The value is too long. Check the maximum length constraint",
                        "out_of_range" => "The value is outside allowed bounds. Check min/max constraints",
                        _ => "Check the validation rules for this field",
                    })
                })
            }
            _ => None,
        }
    }
    
    /// Formats error with hint (for display/logging).
    pub fn with_hint(&self) -> String {
        match self.hint() {
            Some(hint) => format!("{}\nHint: {}", self, hint),
            None => self.to_string(),
        }
    }
}

// Display impl unchanged (thiserror generates it)
// Hint is accessed separately when needed

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // thiserror generates this, we augment in with_hint()
        write!(f, "{}", self)?;
        if let Some(hint) = self.hint() {
            write!(f, "\nHint: {}", hint)?;
        }
        Ok(())
    }
}
```

**User Experience**:

```rust
// Example 1: Type mismatch
match ctx.get("age") {
    Ok(value) => {
        if let Some(age) = value.as_int() {
            println!("Age: {}", age);
        }
    }
    Err(e) => {
        eprintln!("{}", e.with_hint());
        // Output:
        // type mismatch for key 'age': expected Int, got Text
        // Hint: Use `get_int()` instead of `get_text()`, or convert the value
    }
}

// Example 2: Validation error
match ctx.validate_all() {
    Ok(_) => println!("All valid"),
    Err(e) => {
        eprintln!("Validation failed:\n{}", e.with_hint());
        // Output:
        // validation failed: Email format is invalid
        // Hint: Provide a valid email address (e.g., user@example.com)
    }
}

// Example 3: Structured logging (serde compatible)
#[cfg(feature = "serde")]
{
    let json = serde_json::to_string(&error)?;
    // hint field is skipped, only core error data serialized
}

// Example 4: Programmatic handling
if let Some(hint) = error.hint() {
    show_tooltip(hint);  // UI feedback
}
```

**serde Compatibility**:

```rust
// Error serializes without hints
#[derive(Serialize, Deserialize)]
{
    "TypeMismatch": {
        "key": "age",
        "expected": "Int", 
        "actual": "Text"
        // hint is skipped via #[serde(skip)]
    }
}

// But Display includes hints for humans
println!("{}", error);
// type mismatch for key 'age': expected Int, got Text
// Hint: Use `get_int()` instead of `get_text()`, or convert the value
```

**Files to Modify**:
- `src/core/error.rs` - Add hint fields, hint() method, with_hint()
- Update all Error constructors to accept optional hint
- `docs/04-ERROR-HANDLING.md` - Document hint system
- Examples - Show hint usage in error handling

**Testing Strategy**:
- Unit tests: Each error variant returns correct hint
- Custom hints: Verify user-provided hints override defaults
- Serialization: Verify hints are skipped in JSON
- Display: Verify hints appear in Display output
- Backward compat: Old code without hints still works

---

## Summary

### All Decisions Made: YES

| Research Item | Decision | Priority | Effort |
|---------------|----------|----------|--------|
| R1: Runtime State | Separate UiStateManager in Context | P1 | 2 days |
| R2: Migration | Deprecate for 2 versions | P1 | 1 day |
| R3: ValueBuilder | Fluent builder with type inference | P2 | 4 hours |
| R4: Event Arc<Value> | Arc<Value> with backward-compat accessors | P2 | 4 hours |
| R5: Error Hints | Structured ErrorContext with thiserror | P2 | 3 hours |

### Ready for Phase 1: YES

**Phase 1 (Week 1) - Immutability Fixes** can begin immediately:

1. ✅ R1 decision (UiStateManager) provides clear implementation path
2. ✅ R2 decision (deprecation strategy) defines migration approach
3. ✅ All architectural concerns addressed
4. ✅ Testing strategies defined
5. ✅ Backward compatibility plan established

**Next Steps**:

1. Create `tasks.md` breaking down implementation into atomic tasks
2. Begin Phase 1 with Problem #1 (Panel collapsed field)
3. Implement R2 deprecation strategy alongside fixes
4. Run tests continuously during refactoring
5. Update documentation as changes are made

### Implementation Risks (LOW)

**Risk 1**: Performance regression from Arc<Value>  
- **Mitigation**: Benchmark-driven development, fallback to clone if slower
- **Probability**: Low (Arc is well-studied, benefits clear for >64 bytes)

**Risk 2**: Users ignore deprecation warnings  
- **Mitigation**: Clear migration guide, helpful error messages, 2-version grace period
- **Probability**: Medium (some users don't read warnings, but SemVer compliance expected)

**Risk 3**: UiStateManager adds API complexity  
- **Mitigation**: Convenience methods on Context hide complexity, clear docs
- **Probability**: Low (follows existing patterns, natural extension)

**Risk 4**: Error hints bloat error messages  
- **Mitigation**: Hints are opt-in via `.with_hint()`, reasonable max length (80 chars)
- **Probability**: Low (structured approach, skipped in serde)

**Risk 5**: ValueBuilder underused  
- **Mitigation**: Prominent examples, update cookbook, show in docs
- **Probability**: Medium (users may not discover it, but not harmful if unused)

---

## Sources

This research was informed by:

### Rust Language References
- [Diagnostics - The Rust Reference](https://doc.rust-lang.org/reference/attributes/diagnostics.html) - Deprecation attribute usage
- [RFC 1270: Deprecation](https://rust-lang.github.io/rfcs/1270-deprecation.html) - Rust deprecation RFC
- [Arc in std::sync](https://doc.rust-lang.org/std/sync/struct.Arc.html) - Arc documentation
- [thiserror documentation](https://docs.rs/thiserror) - Error handling patterns

### Performance Research
- [Understanding Rust ARC for Efficient Multithreading](https://www.ardanlabs.com/blog/2024/11/fearless-concurrency-ep6-understanding-rust-arc-for-efficient-multithreading.html) - Arc performance characteristics
- [Custom Errors: From Display to thiserror](https://www.woodruff.dev/custom-errors-from-display-to-thiserror/) - Error handling patterns

### Project Documentation
- `CLAUDE.md` - Project architecture and conventions
- `docs/01-ARCHITECTURE.md` - Three-layer architecture
- `.specify/memory/constitution.md` - Architectural invariants
- `CODEBASE_ANALYSIS_2026-01-28.md` - Problems identified

### Codebase Analysis
- `src/context/mod.rs` - Current Context implementation (41 methods, event system)
- `src/runtime/state.rs` - RuntimeNode State structure
- `src/types/group/panel.rs` - Panel with mutable collapsed field
- `src/core/error.rs` - Current error implementation (thiserror-based)
- `src/event/event.rs` - Event structure (currently clones Value)
- `Cargo.toml` - Dependencies (thiserror 2.0, tokio 1.43, no new deps needed)

---

**Research Complete**: 2026-01-28  
**Approved for Implementation**: YES  
**Next Document**: `tasks.md` (task breakdown for execution)
