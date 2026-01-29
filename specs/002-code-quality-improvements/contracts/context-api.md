# Contract: Context API Enhancements

**Feature**: 002-code-quality-improvements  
**Module**: `src/context/mod.rs`  
**Status**: Phase 1 Design

---

## Overview

This contract defines new convenience methods for the `Context` type to reduce boilerplate and improve ergonomics. All methods maintain backward compatibility and follow existing Context patterns.

---

## 1. Context::from_schema()

**Purpose**: Eliminate Arc wrapping boilerplate when creating contexts.

**Signature**:
```rust
impl Context {
    /// Creates a new context from a schema, wrapping it in Arc automatically.
    ///
    /// This is a convenience method that eliminates the need to manually
    /// wrap the schema in Arc.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::context::Context;
    /// use paramdef::schema::Schema;
    ///
    /// let schema = Schema::builder()
    ///     .parameter(Text::builder("name").build())
    ///     .build();
    ///
    /// // Before: Manual Arc wrapping
    /// let ctx = Context::new(Arc::new(schema));
    ///
    /// // After: Automatic wrapping
    /// let ctx = Context::from_schema(schema);
    /// ```
    #[must_use]
    pub fn from_schema(schema: Schema) -> Self {
        Self::new(Arc::new(schema))
    }
}
```

**Contract**:
- **Input**: `Schema` (owned)
- **Output**: `Context`
- **Behavior**: Wraps schema in Arc and calls `Context::new()`
- **Performance**: Zero overhead (single Arc allocation)
- **Thread Safety**: Yes (Arc is Send + Sync)

**Before/After**:
```rust
// Before (3 lines, manual Arc)
let schema = Schema::builder()...build();
let arc_schema = Arc::new(schema);
let ctx = Context::new(arc_schema);

// After (2 lines, automatic Arc)
let schema = Schema::builder()...build();
let ctx = Context::from_schema(schema);
```

---

## 2. Context::get_*_or() - Fallback Getters

**Purpose**: Provide default values for missing/invalid keys without manual unwrap_or.

### 2.1 get_text_or()

**Signature**:
```rust
impl Context {
    /// Gets a text value with a fallback default.
    ///
    /// Returns the default if:
    /// - Key doesn't exist
    /// - Value is not Text type
    /// - Value is Null
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::context::Context;
    ///
    /// let name = ctx.get_text_or("username", "Anonymous");
    /// assert_eq!(name, "Anonymous"); // If key missing or wrong type
    /// ```
    #[must_use]
    pub fn get_text_or<'a>(&'a self, key: &str, default: &'a str) -> &'a str {
        self.get_text(key).unwrap_or(default)
    }
}
```

### 2.2 get_int_or()

**Signature**:
```rust
impl Context {
    /// Gets an integer value with a fallback default.
    ///
    /// # Example
    ///
    /// ```
    /// let port = ctx.get_int_or("port", 8080);
    /// ```
    #[must_use]
    pub fn get_int_or(&self, key: &str, default: i64) -> i64 {
        self.get_int(key).unwrap_or(default)
    }
}
```

### 2.3 get_float_or()

**Signature**:
```rust
impl Context {
    /// Gets a float value with a fallback default.
    ///
    /// # Example
    ///
    /// ```
    /// let rate = ctx.get_float_or("rate", 1.0);
    /// ```
    #[must_use]
    pub fn get_float_or(&self, key: &str, default: f64) -> f64 {
        self.get_float(key).unwrap_or(default)
    }
}
```

### 2.4 get_bool_or()

**Signature**:
```rust
impl Context {
    /// Gets a boolean value with a fallback default.
    ///
    /// # Example
    ///
    /// ```
    /// let enabled = ctx.get_bool_or("feature_flag", false);
    /// ```
    #[must_use]
    pub fn get_bool_or(&self, key: &str, default: bool) -> bool {
        self.get_bool(key).unwrap_or(default)
    }
}
```

**Contract (all variants)**:
- **Input**: `key: &str`, `default: T` (where T is the return type)
- **Output**: `T` (never fails, always returns default or value)
- **Behavior**: 
  - Try to get and convert value
  - Return default if key missing, wrong type, or null
  - No error logging (silent fallback)
- **Performance**: O(1) hash lookup + O(1) type check
- **Thread Safety**: Yes (immutable borrows)

**Before/After**:
```rust
// Before (error-prone)
let name = ctx.get_text("username").unwrap_or("Anonymous");
let port = ctx.get_int("port").unwrap_or(8080);
let enabled = ctx.get_bool("feature").unwrap_or(false);

// After (cleaner)
let name = ctx.get_text_or("username", "Anonymous");
let port = ctx.get_int_or("port", 8080);
let enabled = ctx.get_bool_or("feature", false);
```

---

## 3. Context::get_many() - Bulk Getter

**Purpose**: Efficiently retrieve multiple values in one call.

**Signature**:
```rust
impl Context {
    /// Gets multiple values at once, returning an iterator.
    ///
    /// Each item is `(&Key, Option<&Value>)` where `None` means key not found.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::context::Context;
    ///
    /// let keys = ["username", "email", "age"];
    /// for (key, value) in ctx.get_many(keys) {
    ///     match value {
    ///         Some(v) => println!("{}: {:?}", key, v),
    ///         None => eprintln!("Key {} not found", key),
    ///     }
    /// }
    /// ```
    pub fn get_many<'a, I>(
        &'a self,
        keys: I,
    ) -> impl Iterator<Item = (&'a Key, Option<&'a Value>)>
    where
        I: IntoIterator<Item = &'a str>,
    {
        keys.into_iter().map(move |key_str| {
            let key = Key::from(key_str);
            let value = self.get(&key).ok();
            (
                // Try to get key from context, otherwise create temporary
                self.nodes
                    .keys()
                    .find(|k| k.as_str() == key_str)
                    .unwrap_or(&key),
                value,
            )
        })
    }
}
```

**Contract**:
- **Input**: `keys: I where I: IntoIterator<Item = &str>`
- **Output**: Iterator over `(&Key, Option<&Value>)`
- **Behavior**: 
  - Lazy evaluation (iterator-based)
  - Keys not found return `(key, None)`
  - No errors thrown
- **Performance**: O(n) where n = number of keys (n hash lookups)
- **Thread Safety**: Yes (immutable borrows, iterator is Send if Context is)

**Use Cases**:
```rust
// Example 1: Batch validation
let required = ["username", "email", "password"];
let missing: Vec<_> = ctx.get_many(required)
    .filter_map(|(key, value)| {
        if value.is_none() {
            Some(key)
        } else {
            None
        }
    })
    .collect();

if !missing.is_empty() {
    return Err(Error::MissingRequired {
        fields: missing.iter().map(|k| k.to_string()).collect(),
    });
}

// Example 2: Serialize subset of fields
let export_fields = ["id", "name", "created_at"];
let export_data: HashMap<_, _> = ctx.get_many(export_fields)
    .filter_map(|(key, value)| {
        value.map(|v| (key.clone(), v.clone()))
    })
    .collect();
```

---

## 4. Context UI State Methods

**Purpose**: Convenient access to UI state without direct UiStateManager interaction.

### 4.1 set_panel_collapsed()

**Signature**:
```rust
impl Context {
    /// Sets whether a panel is collapsed.
    ///
    /// This manages UI presentation state, not parameter data.
    ///
    /// # Example
    ///
    /// ```
    /// ctx.set_panel_collapsed("advanced_settings", true);
    /// ```
    pub fn set_panel_collapsed(&mut self, key: &str, collapsed: bool) {
        self.ui_state.set_panel_collapsed(Key::from(key), collapsed);
        
        #[cfg(feature = "events")]
        if let Some(ref bus) = self.event_bus {
            bus.emit(Event::UiStateChanged {
                key: Key::from(key),
                state_type: "panel_collapsed",
                new_state: Value::Bool(collapsed),
            });
        }
    }
}
```

### 4.2 is_panel_collapsed()

**Signature**:
```rust
impl Context {
    /// Returns whether a panel is collapsed.
    ///
    /// Returns `false` if the panel has no tracked state (defaults to expanded).
    ///
    /// # Example
    ///
    /// ```
    /// if ctx.is_panel_collapsed("advanced_settings") {
    ///     // Render collapsed UI
    /// }
    /// ```
    #[must_use]
    pub fn is_panel_collapsed(&self, key: &str) -> bool {
        self.ui_state.is_panel_collapsed(&Key::from(key))
    }
}
```

### 4.3 toggle_panel_collapsed()

**Signature**:
```rust
impl Context {
    /// Toggles a panel's collapsed state.
    ///
    /// If the panel has no state, it is set to collapsed (true).
    ///
    /// # Example
    ///
    /// ```
    /// // User clicks collapse/expand button
    /// ctx.toggle_panel_collapsed("advanced_settings");
    /// ```
    pub fn toggle_panel_collapsed(&mut self, key: &str) {
        let key = Key::from(key);
        let current = self.ui_state.is_panel_collapsed(&key);
        self.set_panel_collapsed(key.as_str(), !current);
    }
}
```

### 4.4 ui_state() / ui_state_mut()

**Signature**:
```rust
impl Context {
    /// Returns a reference to the UI state manager.
    ///
    /// Use this for advanced UI state operations beyond panels.
    #[must_use]
    pub fn ui_state(&self) -> &UiStateManager {
        &self.ui_state
    }

    /// Returns a mutable reference to the UI state manager.
    ///
    /// # Example
    ///
    /// ```
    /// // Direct access for advanced operations
    /// ctx.ui_state_mut().clear(); // Reset all UI state
    /// ```
    pub fn ui_state_mut(&mut self) -> &mut UiStateManager {
        &mut self.ui_state
    }
}
```

**Contract (all UI state methods)**:
- **Thread Safety**: UI state is NOT shared (per-Context)
- **Events**: If `events` feature enabled, `UiStateChanged` event emitted
- **Serialization**: UI state can be serialized separately from parameter data
- **Performance**: O(1) hash lookup
- **Independence**: UI state changes don't affect parameter validation/state

**Event Structure** (with events feature):
```rust
// New event variant added to Event enum
Event::UiStateChanged {
    key: Key,
    state_type: &'static str, // "panel_collapsed", "scroll_position", etc.
    new_state: Value,
}
```

---

## 5. Context::set_many_transactional()

**Purpose**: Atomic multi-field updates with automatic rollback on validation failure.

**Signature**:
```rust
impl Context {
    /// Sets multiple values transactionally.
    ///
    /// If any validation fails, ALL changes are rolled back atomically.
    ///
    /// # Performance
    ///
    /// - Transactions with ≤8 fields use stack allocation (zero heap)
    /// - Transactions with >8 fields use heap allocation
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::core::Value;
    ///
    /// let updates = vec![
    ///     ("username", Value::text("alice")),
    ///     ("email", Value::text("alice@example.com")),
    ///     ("age", Value::Int(30)),
    /// ];
    ///
    /// // Either all succeed or none are applied
    /// ctx.set_many_transactional(updates)?;
    /// ```
    #[cfg(feature = "validation")]
    pub fn set_many_transactional<I, K, V>(&mut self, updates: I) -> Result<()>
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<Key>,
        V: Into<Value>,
    {
        use crate::context::rollback::RollbackStorage;

        let updates: Vec<_> = updates
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();

        // Optimize storage based on update count
        let mut rollback = RollbackStorage::with_capacity(updates.len());

        // Phase 1: Store old values
        for (key, _) in &updates {
            let old_value = self.get(key).ok().cloned();
            rollback.store(key.clone(), old_value);
        }

        // Phase 2: Apply changes with validation
        for (key, new_value) in &updates {
            if let Err(e) = self.set(key, new_value.clone()) {
                // Validation failed - rollback ALL changes
                self.rollback(rollback);
                return Err(e);
            }
        }

        Ok(())
    }

    /// Internal rollback helper
    fn rollback(&mut self, storage: RollbackStorage) {
        for (key, old_value) in storage.iter() {
            match old_value {
                Some(value) => {
                    let _ = self.set(key, value.clone());
                }
                None => {
                    let _ = self.set(key, Value::Null);
                }
            }
        }
    }
}
```

**Contract**:
- **Atomicity**: All-or-nothing semantics
- **Input**: Iterator over `(Key, Value)` pairs
- **Output**: `Result<()>` - Ok if all succeed, Err with first failure
- **Behavior**:
  - Store old values before any changes
  - Apply changes sequentially
  - On first error, restore all old values
  - Return the error that caused rollback
- **Performance**:
  - ≤8 fields: Zero heap allocations
  - >8 fields: Single heap allocation for HashMap
- **Events**: If enabled, emits ValueChanging/ValueChanged for successful updates only (rollback is silent)

**Use Cases**:
```rust
// Example 1: Form submission (all-or-nothing)
let form_data = vec![
    ("first_name", Value::text("Alice")),
    ("last_name", Value::text("Smith")),
    ("email", Value::text("alice@example.com")),
];

match ctx.set_many_transactional(form_data) {
    Ok(_) => show_success("Profile updated"),
    Err(e) => show_error(&e.with_hint()),
}

// Example 2: Configuration import
let config = load_config_file()?;
ctx.set_many_transactional(config.into_iter())?;
```

---

## Integration with Existing Context Methods

**No Breaking Changes**: All new methods are additive.

**Consistency**: All new methods follow existing patterns:
- `#[must_use]` for getters
- `Result<()>` for setters that can fail
- `&str` for key parameters (converted to `Key` internally)
- Event emission when `events` feature enabled

**Documentation**: All methods have examples and clear behavior documentation.

---

## Testing Requirements

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_schema() {
        let schema = Schema::builder().build();
        let ctx = Context::from_schema(schema);
        assert_eq!(ctx.len(), 0);
    }

    #[test]
    fn test_get_text_or_default() {
        let mut ctx = setup_context();
        assert_eq!(ctx.get_text_or("missing", "default"), "default");
        ctx.set("present", Value::text("value"));
        assert_eq!(ctx.get_text_or("present", "default"), "value");
    }

    #[test]
    fn test_get_many() {
        let mut ctx = setup_context();
        ctx.set("a", Value::Int(1));
        ctx.set("c", Value::Int(3));
        
        let keys = ["a", "b", "c"];
        let results: Vec<_> = ctx.get_many(keys).collect();
        
        assert_eq!(results.len(), 3);
        assert!(results[0].1.is_some()); // "a" exists
        assert!(results[1].1.is_none());  // "b" missing
        assert!(results[2].1.is_some()); // "c" exists
    }

    #[test]
    fn test_panel_collapsed() {
        let mut ctx = setup_context();
        
        assert!(!ctx.is_panel_collapsed("panel1"));
        ctx.set_panel_collapsed("panel1", true);
        assert!(ctx.is_panel_collapsed("panel1"));
        
        ctx.toggle_panel_collapsed("panel1");
        assert!(!ctx.is_panel_collapsed("panel1"));
    }

    #[test]
    #[cfg(feature = "validation")]
    fn test_transactional_rollback() {
        let mut ctx = setup_context_with_validation();
        
        let updates = vec![
            ("valid_field", Value::text("ok")),
            ("invalid_field", Value::text("")), // Fails required validation
        ];
        
        let result = ctx.set_many_transactional(updates);
        assert!(result.is_err());
        
        // Verify rollback - "valid_field" should NOT be set
        assert!(ctx.get("valid_field").is_err());
    }
}
```

### Integration Tests
```rust
// tests/context_api_integration.rs

#[test]
fn test_workflow_with_new_apis() {
    let schema = create_test_schema();
    let mut ctx = Context::from_schema(schema);
    
    // Use fallback getters for optional config
    let theme = ctx.get_text_or("theme", "light");
    let max_items = ctx.get_int_or("max_items", 100);
    
    // Bulk updates
    let config = vec![
        ("username", Value::text("alice")),
        ("email", Value::text("alice@example.com")),
    ];
    ctx.set_many_transactional(config).unwrap();
    
    // UI state management
    ctx.set_panel_collapsed("advanced", true);
    assert!(ctx.is_panel_collapsed("advanced"));
}
```

---

## Performance Benchmarks

```rust
// benches/context_api.rs

#[bench]
fn bench_from_schema(b: &mut Bencher) {
    let schema = create_large_schema();
    b.iter(|| {
        Context::from_schema(schema.clone())
    });
}

#[bench]
fn bench_get_many_10_fields(b: &mut Bencher) {
    let ctx = setup_context_with_10_fields();
    let keys = ["f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "f10"];
    
    b.iter(|| {
        ctx.get_many(keys).collect::<Vec<_>>()
    });
}

#[bench]
fn bench_transactional_small(b: &mut Bencher) {
    let mut ctx = setup_context();
    let updates = create_5_field_updates();
    
    b.iter(|| {
        ctx.set_many_transactional(updates.clone()).unwrap()
    });
}

#[bench]
fn bench_transactional_large(b: &mut Bencher) {
    let mut ctx = setup_context();
    let updates = create_20_field_updates();
    
    b.iter(|| {
        ctx.set_many_transactional(updates.clone()).unwrap()
    });
}
```

**Expected Results**:
- `from_schema`: < 5% slower than manual Arc (negligible)
- `get_many(10)`: ~10x faster than 10 individual `get()` calls (reduced hash overhead)
- `transactional_small`: Zero heap allocations (stack buffer)
- `transactional_large`: Single heap allocation, ~95% performance of individual sets

---

## API Stability

**Versioning**: All new methods added in v0.4.0

**Deprecation**: None (additive changes only)

**Future Compatibility**: All methods use `impl Trait` for iterators to allow future optimization without breaking changes

---

**Document Status**: Complete  
**Next Steps**: Implement in `src/context/mod.rs` and `src/context/ui_state.rs`
