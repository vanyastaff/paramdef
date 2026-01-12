# paramdef API Improvements - Technical Specification

**Version:** 0.4.0 (Breaking Changes Allowed)  
**Status:** In Development (Pre-Public Release)  
**Author:** paramdef team  
**Date:** 2026-01-11

---

## Executive Summary

This specification defines five major API improvements for `paramdef` inspired by `gpui-form` analysis and deep codebase study. The improvements focus on **Developer Experience (DX)** while maintaining the core philosophy of "the serde of parameter schemas" - a headless, type-safe, zero-cost abstraction system.

**Key Improvements:**
1. **Typed Context Getters** - Reduce boilerplate from 3 lines to 1
2. **Builder Presets** - Extend existing convenience constructors
3. **Batch Operations** - Transactional multi-value updates
4. **Value Builders** - Fluent API for complex objects
5. **Context Iterators** - Zero-allocation state-based iteration

**Timeline:** ~2 weeks (10-14 working days) sequential implementation

---

## 1. Typed Context Getters

### Problem Statement

Current API requires verbose chaining for type-safe value extraction:

```rust
// Current (3 lines, error-prone)
let email = ctx.get("email")
    .and_then(|v| v.as_text())
    .ok_or_else(|| Error::not_found("email"))?;
```

### Solution Design

Add typed getters with **three-variant error handling** for maximum information:

```rust
// New API (1 line, explicit errors)
let email = ctx.get_text("email")?;  // Result<&str, Error>
```

### Error Type Variants

**Decision:** Three distinct error types for different failure modes.

```rust
// src/core/error.rs
pub enum Error {
    /// Key doesn't exist in schema
    NotFound { key: Key },
    
    /// Value is Value::Null (user didn't provide)
    NullValue { key: Key },
    
    /// Type mismatch (e.g., Value::Int when expecting Text)
    TypeMismatch { 
        key: Key, 
        expected: ValueKind, 
        actual: ValueKind 
    },
    
    // ... existing variants
}
```

**Rationale:**
- Maximum information for debugging
- Allows different handling strategies (NotFound = schema bug, NullValue = validation)
- Clear distinction between compile-time (schema) and runtime (null) errors

### Null Handling Philosophy

**Decision:** Two methods for explicit control.

```rust
// src/context/mod.rs or src/context/typed.rs
impl Context {
    /// Strict - returns NullValue error if value is null
    pub fn get_text(&self, key: &str) -> Result<&str, Error> {
        match self.get(key) {
            None => Err(Error::NotFound { key: key.into() }),
            Some(Value::Null) => Err(Error::NullValue { key: key.into() }),
            Some(v) => v.as_text()
                .ok_or_else(|| Error::TypeMismatch { 
                    key: key.into(),
                    expected: ValueKind::Text,
                    actual: v.kind(),
                })
        }
    }
    
    /// With fallback to schema default
    pub fn get_text_or_default(&self, key: &str) -> Result<&str, Error> {
        match self.get_text(key) {
            Ok(text) => Ok(text),
            Err(Error::NullValue { .. }) => {
                // Lookup default from schema
                self.schema()
                    .get(key)
                    .and_then(|node| {
                        node.as_any()
                            .downcast_ref::<Text<_>>()
                            .and_then(|t| t.default_str())
                    })
                    .ok_or_else(|| Error::NullValue { key: key.into() })
            }
            Err(e) => Err(e),
        }
    }
}
```

**Rationale:**
- Explicit > implicit (Rust philosophy)
- User chooses fallback behavior
- Separates runtime state from schema defaults

### Complete API Surface

```rust
// src/context/typed.rs (new module)
impl Context {
    // Primitive types
    pub fn get_text(&self, key: &str) -> Result<&str, Error>;
    pub fn get_text_or_default(&self, key: &str) -> Result<&str, Error>;
    
    pub fn get_int(&self, key: &str) -> Result<i64, Error>;
    pub fn get_int_or_default(&self, key: &str) -> Result<i64, Error>;
    
    pub fn get_float(&self, key: &str) -> Result<f64, Error>;
    pub fn get_float_or_default(&self, key: &str) -> Result<f64, Error>;
    
    pub fn get_bool(&self, key: &str) -> Result<bool, Error>;
    pub fn get_bool_or_default(&self, key: &str) -> Result<bool, Error>;
    
    // Collections
    pub fn get_array(&self, key: &str) -> Result<&[Value], Error>;
    pub fn get_object(&self, key: &str) -> Result<&IndexMap<Key, Value>, Error>;
    
    pub fn get_binary(&self, key: &str) -> Result<&[u8], Error>;
}
```

### Implementation Notes

- **File:** `src/context/typed.rs` (new module)
- **Re-export:** Add methods to `Context` impl in `src/context/mod.rs`
- **Tests:** Inline tests in `src/context/typed.rs`
- **Estimated time:** 1-2 days

---

## 2. Builder Presets

### Problem Statement

Existing presets (`Text::email()`, `Number::port()`) are useful but limited. Need more coverage for common use cases.

### Design Philosophy

**Decision:** Presets return **Builder** with sensible defaults for:
- UI hints (placeholders, patterns)
- Validation rules (email regex, port range)
- Flags (sensitive for passwords)

**User can override** everything via builder pattern:

```rust
Text::required_email("contact")
    .readonly()  // override
    .build()
```

### New Presets

#### Text Presets

```rust
// src/types/leaf/text.rs
impl Text<Plain> {
    /// Required email with validation
    pub fn required_email(key: impl Into<Key>) -> TextBuilder<Email> {
        TextBuilder::new(key)
            .subtype(Email)
            .required()
    }
    
    /// Multiline text area
    pub fn textarea(key: impl Into<Key>) -> TextBuilder<MultiLine> {
        TextBuilder::new(key)
            .subtype(MultiLine)
    }
    
    /// URL slug (lowercase, hyphens)
    pub fn slug(key: impl Into<Key>) -> TextBuilder<Slug> {
        TextBuilder::new(key)
            .subtype(Slug)
    }
    
    /// Phone number (E.164 format)
    pub fn phone(key: impl Into<Key>) -> TextBuilder<PhoneNumber> {
        TextBuilder::new(key)
            .subtype(PhoneNumber)
    }
    
    /// UUID field
    pub fn uuid(key: impl Into<Key>) -> TextBuilder<Uuid> {
        TextBuilder::new(key)
            .subtype(Uuid)
    }
}
```

#### Number Presets

```rust
// src/types/leaf/number.rs
impl Number<GenericNumber> {
    /// Opacity factor (0.0-1.0) with default 1.0
    pub fn opacity(key: impl Into<Key>) -> NumberBuilder<Factor> {
        NumberBuilder::new(key, Factor)
            .default(1.0)
    }
    
    /// Positive count (>= 0) with default 0
    pub fn count(key: impl Into<Key>) -> NumberBuilder<Count> {
        NumberBuilder::new(key, Count)
            .default(0.0)
    }
    
    /// Year field
    pub fn year(key: impl Into<Key>) -> NumberBuilder<Year> {
        NumberBuilder::new(key, Year)
    }
    
    /// Percentage (0-100) with default 100
    pub fn percentage_full(key: impl Into<Key>) -> NumberBuilder<Percentage> {
        NumberBuilder::new(key, Percentage)
            .default(100.0)
    }
    
    /// Rating (1-5) with default 5
    pub fn rating_max(key: impl Into<Key>) -> NumberBuilder<Rating> {
        NumberBuilder::new(key, Rating)
            .default(5.0)
    }
}
```

### Naming Convention

**Pattern:** `{modifier}_{subtype}` or `{subtype}_{variant}`

Examples:
- `required_email()` - modifier + subtype
- `percentage_full()` - subtype + variant (default 100)
- `rating_max()` - subtype + variant (default 5)

**Rationale:** Prefix for modifiers reads naturally in English.

### Implementation Notes

- **Files:** Extend existing `src/types/leaf/text.rs` and `src/types/leaf/number.rs`
- **Tests:** Add to existing test modules
- **Documentation:** Docstring for each preset with example
- **Estimated time:** 2-3 days

---

## 3. Batch Operations

### Problem Statement

Setting multiple values requires individual calls with separate error handling and event emission.

### Solution Design

Add batch methods with **two transaction modes**:

1. **Transactional** (all-or-nothing)
2. **Partial** (best-effort)

### API Design

```rust
// src/context/batch.rs (new module) or extend src/context/mod.rs
impl Context {
    /// Transactional: all succeed or all rollback
    pub fn set_many_transactional<I>(&mut self, values: I) -> Result<(), Error>
    where
        I: IntoIterator<Item = (impl AsRef<str>, Value)>,
    {
        // Implementation:
        // 1. Validate all keys exist
        // 2. Store old values for rollback
        // 3. Apply all changes
        // 4. Emit individual events (BatchBegin/BatchEnd wrapper)
        // 5. On error: restore old values, emit Reverted events
    }
    
    /// Partial: apply as many as possible, return errors
    pub fn set_many_partial<I>(&mut self, values: I) -> Vec<Result<(), Error>>
    where
        I: IntoIterator<Item = (impl AsRef<str>, Value)>,
    {
        // Implementation:
        // 1. BatchBegin event
        // 2. Try each set(), collect results
        // 3. Emit individual ValueChanging/ValueChanged for successful
        // 4. Emit SetFailed for errors (new event type)
        // 5. BatchEnd event
    }
    
    /// Convenience: load from HashMap
    pub fn load_from_map(&mut self, map: HashMap<Key, Value>) -> Result<(), Error> {
        self.set_many_transactional(map)
    }
    
    /// Convenience: save dirty values to HashMap
    pub fn save_dirty_to_map(&self) -> HashMap<Key, Value> {
        self.collect_dirty_values()
    }
}
```

### Event Emission Strategy

**Decision:** Register **all individual events** for each field change.

**Transactional mode:**
```
BatchBegin { description: "Transactional update" }
ValueChanging { key: "name", old: null, new: "Alice" }
ValueChanged { key: "name", old: null, new: "Alice" }
Dirtied { key: "name" }
ValueChanging { key: "email", ... }
ValueChanged { key: "email", ... }
Dirtied { key: "email" }
... (for each field)
[On error: Reverted events]
BatchEnd { success: true/false }
```

**Partial mode:**
```
BatchBegin { description: "Partial update" }
ValueChanging { key: "name", ... }
ValueChanged { key: "name", ... }
SetFailed { key: "invalid_key", error: NotFound }
ValueChanging { key: "email", ... }
ValueChanged { key: "email", ... }
BatchEnd { success: false, partial: true }
```

**Rationale:**
- UI listeners can update each field individually
- Event history provides audit trail
- Subscribers don't need special bulk event handling

### New Event Types

```rust
// src/event/types.rs
pub enum Event {
    // ... existing variants
    
    /// Batch operation started
    BatchBegin { 
        id: u64, 
        description: Option<SmartStr> 
    },
    
    /// Batch operation ended
    BatchEnd { 
        id: u64, 
        success: bool, 
        partial: bool  // true if some succeeded, some failed
    },
    
    /// Individual set failed in batch
    SetFailed { 
        key: Key, 
        error: Error 
    },
    
    /// Value reverted due to transaction rollback
    Reverted { 
        key: Key, 
        old_value: Value, 
        failed_value: Value 
    },
}
```

### Implementation Notes

- **File:** Extend `src/context/mod.rs` or new `src/context/batch.rs`
- **Tests:** Transactional rollback, partial success, event ordering
- **Estimated time:** 3-4 days

---

## 4. Value Builders

### Problem Statement

Building complex nested objects requires verbose array of tuples syntax:

```rust
Value::object([
    ("host", Value::text("localhost")),
    ("port", Value::Int(5432)),
])
```

### Solution Design

**Decision:** Support **multiple styles** for flexibility.

### Builder API

```rust
// src/core/value/builder.rs (new file)
pub struct ObjectBuilder {
    map: IndexMap<Key, Value>,
}

impl ObjectBuilder {
    pub fn new() -> Self {
        Self { map: IndexMap::new() }
    }
    
    pub fn with_capacity(capacity: usize) -> Self {
        Self { map: IndexMap::with_capacity(capacity) }
    }
    
    // Generic field
    pub fn field(mut self, key: impl Into<Key>, value: Value) -> Self {
        self.map.insert(key.into(), value);
        self
    }
    
    // Typed convenience methods
    pub fn text(self, key: impl Into<Key>, value: impl Into<SmartStr>) -> Self {
        self.field(key, Value::text(value))
    }
    
    pub fn int(self, key: impl Into<Key>, value: i64) -> Self {
        self.field(key, Value::Int(value))
    }
    
    pub fn float(self, key: impl Into<Key>, value: f64) -> Self {
        self.field(key, Value::Float(value))
    }
    
    pub fn bool(self, key: impl Into<Key>, value: bool) -> Self {
        self.field(key, Value::Bool(value))
    }
    
    // Nested objects with closure
    pub fn nested<F>(mut self, key: impl Into<Key>, f: F) -> Self 
    where
        F: FnOnce(ObjectBuilder) -> ObjectBuilder,
    {
        let nested = f(ObjectBuilder::new());
        self.map.insert(key.into(), nested.build());
        self
    }
    
    pub fn build(self) -> Value {
        Value::Object(Arc::new(self.map))
    }
}

impl Value {
    pub fn build_object() -> ObjectBuilder {
        ObjectBuilder::new()
    }
}
```

### Usage Examples

**Style 1: Flat builder**
```rust
let config = Value::build_object()
    .text("host", "localhost")
    .int("port", 5432)
    .text("database", "mydb")
    .bool("ssl", true)
    .build();
```

**Style 2: Nested with closure**
```rust
let config = Value::build_object()
    .nested("database", |db| db
        .text("host", "localhost")
        .int("port", 5432)
        .nested("credentials", |cred| cred
            .text("username", "admin")
            .text("password", "secret")
        )
    )
    .build();
```

**Style 3: Imperative (still supported)**
```rust
let credentials = Value::build_object()
    .text("username", "admin")
    .text("password", "secret")
    .build();

let config = Value::build_object()
    .text("host", "localhost")
    .field("credentials", credentials)
    .build();
```

### Array Support

**Decision:** Keep existing `Value::array()` + add `From` impls for convenience.

```rust
// src/core/value/convert.rs
impl<T, const N: usize> From<[T; N]> for Value 
where
    T: Into<Value>,
{
    fn from(arr: [T; N]) -> Self {
        Value::array(arr.into_iter().map(Into::into))
    }
}

impl From<Vec<Value>> for Value {
    fn from(vec: Vec<Value>) -> Self {
        Value::array(vec)
    }
}

// Usage
let tags: Value = ["rust", "paramdef", "forms"]
    .map(Value::text)
    .into();  // via From
```

### Implementation Notes

- **File:** `src/core/value/builder.rs` (new)
- **Tests:** Flat, nested, mixed styles
- **Estimated time:** 2-3 days

---

## 5. Context Iterators

### Problem Statement

No convenient way to iterate over subsets of values (all non-null, touched, invalid).

### Solution Design

Add **zero-allocation iterators** using filter chains.

### API Design

```rust
// Add to src/context/mod.rs
impl Context {
    /// Iterates over all non-null values
    pub fn values(&self) -> impl Iterator<Item = (&Key, &Value)> + '_ {
        self.nodes
            .iter()
            .filter_map(|(k, n)| n.value().map(|v| (k, v)))
    }
    
    /// Iterates over touched values
    pub fn touched_values(&self) -> impl Iterator<Item = (&Key, &Value)> + '_ {
        self.nodes
            .iter()
            .filter(|(_, n)| n.state().is_touched())
            .filter_map(|(k, n)| n.value().map(|v| (k, v)))
    }
    
    /// Iterates over invalid values (failed validation)
    pub fn invalid_values(&self) -> impl Iterator<Item = (&Key, &Value)> + '_ {
        self.nodes
            .iter()
            .filter(|(_, n)| !n.state().is_valid())
            .filter_map(|(k, n)| n.value().map(|v| (k, v)))
    }
    
    /// Iterates over valid values
    pub fn valid_values(&self) -> impl Iterator<Item = (&Key, &Value)> + '_ {
        self.nodes
            .iter()
            .filter(|(_, n)| n.state().is_valid())
            .filter_map(|(k, n)| n.value().map(|v| (k, v)))
    }
    
    /// Already exists, keeping for reference
    pub fn dirty_values(&self) -> impl Iterator<Item = (&Key, &Value)> + '_ {
        // existing implementation
    }
}
```

### Use Cases

**UI partial updates:**
```rust
// Update only touched fields
for (key, value) in ctx.touched_values() {
    api.update_field(key, value).await?;
}
```

**Error display:**
```rust
// Show validation errors
for (key, _) in ctx.invalid_values() {
    let errors = ctx.node(key).unwrap().state().errors();
    eprintln!("{}: {:?}", key, errors);
}
```

### Performance Considerations

- Filter order: `state check` (cheap) → `value check` (also cheap)
- Zero heap allocations
- Lazy evaluation (stops early if consumer breaks)

### Implementation Notes

- **File:** Add methods to `src/context/mod.rs`
- **Tests:** Empty contexts, all states, mixed states
- **Estimated time:** 1-2 days

---

## Implementation Phases

### Phase 1: Typed Context Getters (Days 1-2)

**Files:**
- `src/core/error.rs` - Add NullValue, TypeMismatch variants
- `src/context/typed.rs` - New module with typed getters (or extend mod.rs)
- `src/context/mod.rs` - Add methods or re-export

**Tasks:**
1. Add error variants to Error enum
2. Implement `get_text()`, `get_int()`, `get_float()`, `get_bool()`
3. Implement `get_array()`, `get_object()`, `get_binary()`
4. Implement `*_or_default()` variants
5. Write unit tests for all error cases
6. Add doctests with examples

**Deliverable:** Typed getters fully functional with comprehensive tests.

---

### Phase 2: Builder Presets (Days 3-5)

**Files:**
- `src/types/leaf/text.rs` - Add text presets
- `src/types/leaf/number.rs` - Add number presets

**Tasks:**
1. Add `Text::required_email()`, `textarea()`, `slug()`, `phone()`, `uuid()`
2. Add `Number::opacity()`, `count()`, `year()`, `percentage_full()`, `rating_max()`
3. Write unit tests for each preset
4. Add doctests showing usage

**Deliverable:** 10+ new presets with tests and documentation.

---

### Phase 3: Batch Operations (Days 6-9)

**Files:**
- `src/context/mod.rs` or `src/context/batch.rs` - Batch methods
- `src/event/types.rs` - Add batch event types

**Tasks:**
1. Implement `set_many_transactional()` with rollback
2. Implement `set_many_partial()` with error collection
3. Add `load_from_map()`, `save_dirty_to_map()` convenience
4. Add BatchBegin, BatchEnd, SetFailed, Reverted events
5. Write transactional rollback tests
6. Write partial success tests
7. Test event emission order

**Deliverable:** Batch operations with full test coverage.

---

### Phase 4: Value Builders (Days 10-12)

**Files:**
- `src/core/value/builder.rs` - New module
- `src/core/value/convert.rs` - Add From impls for arrays
- `src/core/value/mod.rs` - Re-export builder

**Tasks:**
1. Implement ObjectBuilder with flat API
2. Add `nested()` method with closure support
3. Add typed convenience methods (text, int, float, bool)
4. Implement `Value::build_object()` entry point
5. Add `From<[T; N]>` and `From<Vec<Value>>` for arrays
6. Write tests for all builder styles

**Deliverable:** Value builders supporting multiple styles.

---

### Phase 5: Context Iterators (Days 13-14)

**Files:**
- `src/context/mod.rs` - Add iterator methods

**Tasks:**
1. Implement `values()` iterator
2. Implement `touched_values()` iterator
3. Implement `invalid_values()` iterator
4. Implement `valid_values()` iterator
5. Write tests for each iterator
6. Test edge cases (empty, all states, mixed)

**Deliverable:** Context iterators with zero-allocation guarantees.

---

## Success Criteria

### Functional Requirements

- ✅ Typed getters reduce boilerplate from 3 lines to 1
- ✅ Three error variants provide clear failure information
- ✅ `or_default()` methods fall back to schema defaults
- ✅ Builder presets cover 10+ common subtypes
- ✅ Transactional batch operations rollback on failure
- ✅ Partial batch operations continue on error
- ✅ Individual events emitted for each value change
- ✅ Value builders support flat, nested, and imperative styles
- ✅ Iterators provide zero-allocation filtering

### Non-Functional Requirements

- ✅ Breaking changes allowed (pre-public release)
- ✅ Zero-cost abstractions (no runtime overhead)
- ✅ Comprehensive test coverage (90%+)
- ✅ Documentation for all public APIs

---

## Final Notes

This specification is **complete and ready for implementation** in a **NEW SESSION**.

All design decisions made through interview:
- ✅ Error handling: three variants (NotFound, NullValue, TypeMismatch)
- ✅ Null handling: two methods (strict + or_default)
- ✅ Presets: return Builder with sensible defaults
- ✅ Batch: two transaction modes (transactional + partial)
- ✅ Events: all individual events registered
- ✅ Value builders: multiple styles supported
- ✅ Iterators: zero-allocation filter chains
- ✅ Implementation: sequential by priority (Phase 1-5)

**Timeline:** 10-14 working days

---

**END OF SPECIFICATION**
