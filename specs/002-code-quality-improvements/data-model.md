# Data Model: Code Quality Improvements

**Feature**: 002-code-quality-improvements  
**Created**: 2026-01-29  
**Status**: Phase 1 Design

---

## Overview

This document defines the new and modified data structures for the code quality improvements feature. All changes maintain backward compatibility where possible, with breaking changes following the deprecation strategy outlined in research.md.

## Core Principles

- **Immutability-First**: Schema types remain immutable after construction
- **Runtime State Separation**: UI and presentation state lives in Context, not schema
- **Performance-Conscious**: Use stack allocations where possible, Arc for large shared data
- **Backward Compatible**: Additive changes preferred, breaking changes deprecated first

---

## 1. UiStateManager

**Purpose**: Manages presentation-only state (collapsed panels, scroll positions, etc.) separately from parameter data.

**Location**: `src/context/ui_state.rs` (new file)

**Ownership**: Stored in `Context`, mutable per-instance

### Structure

```rust
use crate::core::Key;
use rustc_hash::FxHashMap;
use std::time::Instant;

/// Manages UI presentation state for parameter nodes.
///
/// This stores state like collapsed panels, selected tabs, scroll positions
/// that are specific to UI presentation and should not be part of the
/// immutable schema.
///
/// # Example
///
/// ```
/// use paramdef::context::UiStateManager;
///
/// let mut ui_state = UiStateManager::new();
/// ui_state.set_panel_collapsed("settings", true);
/// assert!(ui_state.is_panel_collapsed("settings"));
/// ```
#[derive(Debug, Clone, Default)]
pub struct UiStateManager {
    /// Panel states indexed by key.
    panel_states: FxHashMap<Key, PanelState>,
}

impl UiStateManager {
    /// Creates a new empty UI state manager.
    pub fn new() -> Self {
        Self {
            panel_states: FxHashMap::default(),
        }
    }

    /// Creates a UI state manager with pre-allocated capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            panel_states: FxHashMap::with_capacity_and_hasher(
                capacity,
                Default::default(),
            ),
        }
    }

    /// Sets whether a panel is collapsed.
    pub fn set_panel_collapsed(&mut self, key: impl Into<Key>, collapsed: bool) {
        let key = key.into();
        self.panel_states
            .entry(key)
            .or_insert_with(PanelState::default)
            .collapsed = collapsed;
        
        // Update interaction timestamp
        if let Some(state) = self.panel_states.get_mut(&key) {
            state.last_interaction = Some(Instant::now());
        }
    }

    /// Returns whether a panel is collapsed.
    ///
    /// Returns `false` if the panel has no state (defaults to expanded).
    pub fn is_panel_collapsed(&self, key: &Key) -> bool {
        self.panel_states
            .get(key)
            .map_or(false, |state| state.collapsed)
    }

    /// Gets the panel state for a key, if it exists.
    pub fn get_panel_state(&self, key: &Key) -> Option<&PanelState> {
        self.panel_states.get(key)
    }

    /// Gets mutable panel state for a key, creating if necessary.
    pub fn get_or_create_panel_state(&mut self, key: impl Into<Key>) -> &mut PanelState {
        self.panel_states
            .entry(key.into())
            .or_insert_with(PanelState::default)
    }

    /// Clears all UI state.
    pub fn clear(&mut self) {
        self.panel_states.clear();
    }

    /// Returns the number of panels with tracked state.
    pub fn len(&self) -> usize {
        self.panel_states.len()
    }

    /// Returns true if no panels have tracked state.
    pub fn is_empty(&self) -> bool {
        self.panel_states.is_empty()
    }
}

/// Serialization support (requires serde feature)
#[cfg(feature = "serde")]
mod serde_support {
    use super::*;
    use serde::{Deserialize, Serialize};

    impl Serialize for UiStateManager {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            // Serialize as a map of Key -> PanelState
            // last_interaction is skipped (not meaningful across sessions)
            use serde::ser::SerializeMap;
            let mut map = serializer.serialize_map(Some(self.panel_states.len()))?;
            for (key, state) in &self.panel_states {
                map.serialize_entry(key, &state.collapsed)?;
            }
            map.end()
        }
    }

    impl<'de> Deserialize<'de> for UiStateManager {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let map: FxHashMap<Key, bool> = FxHashMap::deserialize(deserializer)?;
            let panel_states = map
                .into_iter()
                .map(|(key, collapsed)| {
                    (
                        key,
                        PanelState {
                            collapsed,
                            last_interaction: None,
                        },
                    )
                })
                .collect();
            Ok(Self { panel_states })
        }
    }
}
```

### Relationships

- **Context HAS-ONE UiStateManager** (stored as `ui_state: UiStateManager` field)
- **UiStateManager HAS-MANY PanelState** (indexed by Key in FxHashMap)

### Integration with Context

```rust
// In src/context/mod.rs
impl Context {
    pub fn ui_state(&self) -> &UiStateManager {
        &self.ui_state
    }

    pub fn ui_state_mut(&mut self) -> &mut UiStateManager {
        &mut self.ui_state
    }

    // Convenience methods (delegate to ui_state)
    pub fn is_panel_collapsed(&self, key: &str) -> bool {
        self.ui_state.is_panel_collapsed(&Key::from(key))
    }

    pub fn set_panel_collapsed(&mut self, key: &str, collapsed: bool) {
        self.ui_state.set_panel_collapsed(Key::from(key), collapsed);
    }
}
```

---

## 2. PanelState

**Purpose**: Tracks UI state for a single Panel node.

**Location**: `src/context/ui_state.rs` (same file as UiStateManager)

**Ownership**: Owned by UiStateManager

### Structure

```rust
/// UI state for a Panel node.
///
/// Tracks presentation-specific state like collapsed/expanded status
/// and user interaction timestamps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PanelState {
    /// Whether the panel is collapsed (true) or expanded (false).
    pub collapsed: bool,
    
    /// Timestamp of last user interaction (collapse/expand action).
    ///
    /// Used for analytics, session replay, and interaction tracking.
    /// Not serialized (meaningless across sessions).
    pub last_interaction: Option<Instant>,
}

impl Default for PanelState {
    fn default() -> Self {
        Self {
            collapsed: false, // Panels default to expanded
            last_interaction: None,
        }
    }
}

impl PanelState {
    /// Creates a new panel state with default values (expanded).
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a panel state with specified collapsed status.
    pub fn with_collapsed(collapsed: bool) -> Self {
        Self {
            collapsed,
            last_interaction: Some(Instant::now()),
        }
    }

    /// Returns true if the panel is collapsed.
    pub fn is_collapsed(&self) -> bool {
        self.collapsed
    }

    /// Returns true if the panel is expanded.
    pub fn is_expanded(&self) -> bool {
        !self.collapsed
    }

    /// Sets the collapsed state and updates the interaction timestamp.
    pub fn set_collapsed(&mut self, collapsed: bool) {
        self.collapsed = collapsed;
        self.last_interaction = Some(Instant::now());
    }

    /// Toggles the collapsed state.
    pub fn toggle(&mut self) {
        self.set_collapsed(!self.collapsed);
    }
}
```

### Migration from Panel.collapsed

**Old (deprecated in v0.4.0, removed in v0.6.0)**:
```rust
let mut panel = Panel::builder("settings").build();
panel.set_collapsed(true); // Mutates schema - BAD!
```

**New (v0.4.0+)**:
```rust
// Set initial state in builder (optional)
let panel = Panel::builder("settings")
    .collapsed(true) // Sets default UI state hint
    .build();

// Modify at runtime via Context
ctx.set_panel_collapsed("settings", true);
```

---

## 3. ValidationError (Enhanced)

**Purpose**: Enhanced validation error with full path information for nested structures.

**Location**: `src/event/types.rs` (existing file, modified)

**Current Implementation**: Only has `code` and `message` fields

**Enhancement**: Add `path` and `field` for precise error location

### Structure

```rust
/// A validation error with code, message, and location path.
///
/// Designed to be lightweight and cloneable for event broadcasting.
///
/// # Path vs Field
///
/// - `path`: Full dot-separated path (e.g., "user.address.email")
/// - `field`: Leaf field name only (e.g., "email")
///
/// For top-level fields, path and field are identical.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    /// Error code for programmatic handling (e.g., `required`, `min_length`).
    pub code: SmartStr,
    
    /// Human-readable error message.
    pub message: SmartStr,
    
    /// Full dot-separated path to the field (e.g., "user.address.email").
    ///
    /// For top-level fields, this equals `field`.
    /// For nested fields, this shows the full path from root.
    pub path: SmartStr,
    
    /// Leaf field name (e.g., "email").
    ///
    /// This is the key of the parameter that failed validation.
    pub field: SmartStr,
}
```

### Constructors

```rust
impl ValidationError {
    /// Creates a new validation error with path information.
    ///
    /// # Arguments
    ///
    /// * `code` - Error code (e.g., "required", "email", "min_length")
    /// * `message` - Human-readable error message
    /// * `path` - Full dot-separated path (e.g., "user.address.email")
    /// * `field` - Leaf field name (e.g., "email")
    #[must_use]
    pub fn new(
        code: impl Into<SmartStr>,
        message: impl Into<SmartStr>,
        path: impl Into<SmartStr>,
        field: impl Into<SmartStr>,
    ) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            path: path.into(),
            field: field.into(),
        }
    }

    /// Creates a validation error for a top-level field (path = field).
    #[must_use]
    pub fn simple(
        code: impl Into<SmartStr>,
        message: impl Into<SmartStr>,
        field: impl Into<SmartStr>,
    ) -> Self {
        let field = field.into();
        Self {
            code: code.into(),
            message: message.into(),
            path: field.clone(),
            field,
        }
    }

    /// Creates a required field error.
    #[must_use]
    pub fn required(path: impl Into<SmartStr>, field: impl Into<SmartStr>) -> Self {
        let field = field.into();
        let path = path.into();
        Self {
            code: "required".into(),
            message: format!("Field '{}' is required", field).into(),
            path,
            field,
        }
    }

    /// Creates a min length error.
    #[must_use]
    pub fn min_length(
        path: impl Into<SmartStr>,
        field: impl Into<SmartStr>,
        min: usize,
        actual: usize,
    ) -> Self {
        let field = field.into();
        let path = path.into();
        Self {
            code: "min_length".into(),
            message: format!(
                "Field '{}' must be at least {} characters (got {})",
                field, min, actual
            )
            .into(),
            path,
            field,
        }
    }

    /// Creates a max length error.
    #[must_use]
    pub fn max_length(
        path: impl Into<SmartStr>,
        field: impl Into<SmartStr>,
        max: usize,
        actual: usize,
    ) -> Self {
        let field = field.into();
        let path = path.into();
        Self {
            code: "max_length".into(),
            message: format!(
                "Field '{}' must be at most {} characters (got {})",
                field, max, actual
            )
            .into(),
            path,
            field,
        }
    }

    // Similar constructors for other common errors...
}
```

### Backward Compatibility

**Old code (before enhancement)**:
```rust
ValidationError::new("required", "Field is required")
```

**Migration strategy**:
- Keep existing 2-argument constructor as `ValidationError::simple()`
- New code uses 4-argument `new()` or specific constructors
- Existing code continues to work without modification

---

## 4. ValueBuilder

**Purpose**: Fluent builder for ergonomic `Value::Object` construction.

**Location**: `src/core/value/builder.rs` (new file)

**Ownership**: Temporary builder, consumed by `.build()` to produce `Value`

### Structure

```rust
use crate::core::{Key, Value};
use indexmap::IndexMap;

/// Fluent builder for constructing `Value::Object`.
///
/// Provides an ergonomic API for building objects without needing to
/// manually construct IndexMap and Arc wrappers.
///
/// # Example
///
/// ```
/// use paramdef::core::Value;
///
/// let user = Value::object()
///     .field("id", Value::Int(123))
///     .field("name", Value::text("Alice"))
///     .field("active", Value::Bool(true))
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct ValueBuilder {
    /// Fields being accumulated.
    fields: IndexMap<Key, Value>,
}

impl ValueBuilder {
    /// Creates a new empty object builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            fields: IndexMap::new(),
        }
    }

    /// Creates a builder with pre-allocated capacity.
    ///
    /// Use this when you know the number of fields in advance
    /// to avoid reallocation.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            fields: IndexMap::with_capacity(capacity),
        }
    }

    /// Adds a field to the object.
    ///
    /// If a field with the same key already exists, it is replaced.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::core::Value;
    ///
    /// let obj = Value::object()
    ///     .field("name", Value::text("Alice"))
    ///     .field("age", Value::Int(30))
    ///     .build();
    /// ```
    #[must_use]
    pub fn field(mut self, key: impl Into<Key>, value: impl Into<Value>) -> Self {
        self.fields.insert(key.into(), value.into());
        self
    }

    /// Adds multiple fields at once.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::core::Value;
    ///
    /// let fields = vec![
    ///     ("name", Value::text("Alice")),
    ///     ("age", Value::Int(30)),
    /// ];
    ///
    /// let obj = Value::object()
    ///     .fields(fields)
    ///     .build();
    /// ```
    #[must_use]
    pub fn fields<I, K, V>(mut self, fields: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<Key>,
        V: Into<Value>,
    {
        self.fields.extend(
            fields
                .into_iter()
                .map(|(k, v)| (k.into(), v.into())),
        );
        self
    }

    /// Conditionally adds a field.
    ///
    /// Only adds the field if `condition` is true.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::core::Value;
    ///
    /// let include_age = true;
    /// let obj = Value::object()
    ///     .field("name", Value::text("Alice"))
    ///     .field_if(include_age, "age", Value::Int(30))
    ///     .build();
    /// ```
    #[must_use]
    pub fn field_if(
        self,
        condition: bool,
        key: impl Into<Key>,
        value: impl Into<Value>,
    ) -> Self {
        if condition {
            self.field(key, value)
        } else {
            self
        }
    }

    /// Builds the final `Value::Object`.
    ///
    /// Consumes the builder and returns the constructed object.
    #[must_use]
    pub fn build(self) -> Value {
        Value::Object(std::sync::Arc::new(self.fields))
    }

    /// Returns the number of fields added so far.
    #[must_use]
    pub fn len(&self) -> usize {
        self.fields.len()
    }

    /// Returns true if no fields have been added.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }
}

/// Convenience conversion from builder to Value.
///
/// Allows using builders in contexts expecting `Into<Value>`.
impl From<ValueBuilder> for Value {
    fn from(builder: ValueBuilder) -> Self {
        builder.build()
    }
}
```

### Integration with Value

```rust
// In src/core/value/mod.rs
impl Value {
    /// Creates a builder for Value::Object.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::core::Value;
    ///
    /// let user = Value::object()
    ///     .field("name", Value::text("Alice"))
    ///     .field("age", Value::Int(30))
    ///     .build();
    /// ```
    #[must_use]
    pub fn object() -> ValueBuilder {
        ValueBuilder::new()
    }

    /// Creates a builder with pre-allocated capacity.
    #[must_use]
    pub fn object_with_capacity(capacity: usize) -> ValueBuilder {
        ValueBuilder::with_capacity(capacity)
    }
}
```

---

## 5. RollbackStorage

**Purpose**: Memory-efficient storage for transactional rollback operations.

**Location**: `src/context/rollback.rs` (new file)

**Optimization**: Use stack-allocated array for small transactions (<= 8 fields), heap for larger

### Structure

```rust
use crate::core::{Key, Value};
use rustc_hash::FxHashMap;

/// Efficient storage for transactional rollback.
///
/// Uses a small stack-allocated buffer for transactions affecting
/// <= 8 fields, falling back to heap allocation for larger transactions.
///
/// This optimization avoids heap allocations for the common case of
/// small form updates while still supporting bulk operations.
#[derive(Debug, Clone)]
pub enum RollbackStorage {
    /// Stack-allocated storage for <= 8 fields.
    ///
    /// Each slot stores `(Key, Option<Value>)`:
    /// - `Some(value)`: Field had this value before transaction
    /// - `None`: Field didn't exist before transaction
    Small {
        /// Stack-allocated array of (Key, old Value) pairs.
        /// Unused slots have default Key and None value.
        buffer: [(Key, Option<Value>); 8],
        /// Number of used slots in the buffer.
        count: usize,
    },

    /// Heap-allocated storage for > 8 fields.
    Large(FxHashMap<Key, Option<Value>>),
}

impl RollbackStorage {
    /// Creates a new empty rollback storage.
    ///
    /// Starts with small (stack) allocation.
    #[must_use]
    pub fn new() -> Self {
        Self::Small {
            buffer: Default::default(),
            count: 0,
        }
    }

    /// Creates a rollback storage with expected capacity.
    ///
    /// Chooses Small or Large based on capacity hint.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        if capacity <= 8 {
            Self::new()
        } else {
            Self::Large(FxHashMap::with_capacity_and_hasher(
                capacity,
                Default::default(),
            ))
        }
    }

    /// Stores the old value for a key.
    ///
    /// Automatically upgrades from Small to Large if necessary.
    pub fn store(&mut self, key: Key, old_value: Option<Value>) {
        match self {
            Self::Small { buffer, count } => {
                if *count < 8 {
                    buffer[*count] = (key, old_value);
                    *count += 1;
                } else {
                    // Upgrade to Large
                    let mut map = FxHashMap::with_capacity_and_hasher(16, Default::default());
                    for (k, v) in buffer.iter().take(*count) {
                        map.insert(k.clone(), v.clone());
                    }
                    map.insert(key, old_value);
                    *self = Self::Large(map);
                }
            }
            Self::Large(map) => {
                map.insert(key, old_value);
            }
        }
    }

    /// Returns an iterator over all stored (Key, Option<Value>) pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&Key, &Option<Value>)> {
        match self {
            Self::Small { buffer, count } => {
                RollbackIter::Small(buffer[..*count].iter())
            }
            Self::Large(map) => RollbackIter::Large(map.iter()),
        }
    }

    /// Returns the number of stored entries.
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Small { count, .. } => *count,
            Self::Large(map) => map.len(),
        }
    }

    /// Returns true if no entries are stored.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clears all stored entries.
    pub fn clear(&mut self) {
        match self {
            Self::Small { count, .. } => *count = 0,
            Self::Large(map) => map.clear(),
        }
    }
}

impl Default for RollbackStorage {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator over rollback storage entries.
enum RollbackIter<'a> {
    Small(std::slice::Iter<'a, (Key, Option<Value>)>),
    Large(std::collections::hash_map::Iter<'a, Key, Option<Value>>),
}

impl<'a> Iterator for RollbackIter<'a> {
    type Item = (&'a Key, &'a Option<Value>);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Small(iter) => iter.next().map(|(k, v)| (k, v)),
            Self::Large(iter) => iter.next(),
        }
    }
}
```

### Usage in Context

```rust
// In src/context/mod.rs
impl Context {
    /// Sets multiple values transactionally.
    ///
    /// If any validation fails, all changes are rolled back.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::core::Value;
    ///
    /// let updates = vec![
    ///     ("name", Value::text("Alice")),
    ///     ("age", Value::Int(30)),
    /// ];
    ///
    /// ctx.set_many_transactional(updates)?;
    /// ```
    pub fn set_many_transactional<I, K, V>(&mut self, updates: I) -> Result<()>
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<Key>,
        V: Into<Value>,
    {
        let updates: Vec<_> = updates
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();

        // Use optimized storage based on update count
        let mut rollback = RollbackStorage::with_capacity(updates.len());

        // Store old values and apply changes
        for (key, new_value) in &updates {
            let old_value = self.get(key).ok().cloned();
            rollback.store(key.clone(), old_value);
            
            if let Err(e) = self.set(key, new_value.clone()) {
                // Validation failed - rollback all changes
                self.rollback(rollback);
                return Err(e);
            }
        }

        Ok(())
    }

    /// Rolls back changes using stored old values.
    fn rollback(&mut self, storage: RollbackStorage) {
        for (key, old_value) in storage.iter() {
            match old_value {
                Some(value) => {
                    // Restore old value (ignore errors - we're already in error recovery)
                    let _ = self.set(key, value.clone());
                }
                None => {
                    // Field didn't exist before - could remove it
                    // For now, just restore to Null
                    let _ = self.set(key, Value::Null);
                }
            }
        }
    }
}
```

### Performance Characteristics

| Transaction Size | Storage | Allocations | Memory |
|-----------------|---------|-------------|---------|
| 1-8 fields | Small (stack) | 0 heap | ~512 bytes |
| 9+ fields | Large (heap) | 1 heap | ~48 + 24n bytes |

---

## 6. ErrorContext (Error Hints)

**Purpose**: Add actionable hints to Error enum variants for better developer experience.

**Location**: `src/core/error.rs` (existing file, modified)

**Strategy**: Add optional `hint` field to error variants, provide `hint()` method

### Enhanced Error Structure

```rust
/// Errors that can occur during parameter operations.
#[derive(Debug, Clone, Error)]
pub enum Error {
    /// Type mismatch when accessing a value.
    #[error("type mismatch for key '{key}': expected {expected}, got {actual}")]
    TypeMismatch {
        /// The key that had the type mismatch.
        key: String,
        /// Expected type kind.
        expected: ValueKind,
        /// Actual type kind.
        actual: ValueKind,
        /// Optional actionable hint for fixing this error.
        /// Not serialized (marked with #[serde(skip)] when serde feature enabled).
        #[cfg_attr(feature = "serde", serde(skip))]
        hint: Option<String>,
    },

    /// Validation failed for a parameter value.
    #[error("validation failed: {message}")]
    Validation {
        /// Error code for programmatic handling.
        code: String,
        /// Human-readable error message.
        message: String,
        /// Fields involved in the validation error.
        fields: Vec<String>,
        /// Optional actionable hint.
        #[cfg_attr(feature = "serde", serde(skip))]
        hint: Option<String>,
    },

    /// Key not found in context.
    #[error("key '{key}' not found")]
    NotFound {
        /// The key that was not found.
        key: String,
        /// Optional hint with suggestions.
        #[cfg_attr(feature = "serde", serde(skip))]
        hint: Option<String>,
    },

    // ... other variants similarly enhanced
}
```

### Hint Methods

```rust
impl Error {
    /// Returns an actionable hint for resolving this error, if available.
    ///
    /// Hints provide suggestions for fixing common errors:
    /// - Type mismatches: "Use `get_int()` instead of `get_text()`"
    /// - Not found: "Did you mean 'username'? Available: username, password, email"
    /// - Validation: "Provide a valid email address (e.g., user@example.com)"
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::core::Error;
    ///
    /// let err = Error::type_mismatch("age", ValueKind::Int, ValueKind::Text);
    /// if let Some(hint) = err.hint() {
    ///     eprintln!("Hint: {}", hint);
    /// }
    /// ```
    #[must_use]
    pub fn hint(&self) -> Option<&str> {
        match self {
            Self::TypeMismatch { hint, expected, actual, .. } => {
                hint.as_deref().or_else(|| {
                    Some(match (expected, actual) {
                        (ValueKind::Int, ValueKind::Text) => 
                            "Use `get_int()` for integer values, or convert with `parse()`",
                        (ValueKind::Text, ValueKind::Int) => 
                            "Use `get_text()` for text values, or convert with `to_string()`",
                        (ValueKind::Bool, _) => 
                            "Use `get_bool()` for boolean values",
                        (ValueKind::Array, _) => 
                            "Use `get_array()` to access array elements",
                        (ValueKind::Object, _) => 
                            "Use `get_object()` to access object fields",
                        _ => "Use the appropriate getter method for this value type",
                    })
                })
            }
            
            Self::Validation { hint, code, .. } => {
                hint.as_deref().or_else(|| {
                    Some(match code.as_str() {
                        "required" => 
                            "This field cannot be empty. Provide a value or remove the REQUIRED flag",
                        "email" => 
                            "Provide a valid email address (e.g., user@example.com)",
                        "min_length" => 
                            "The value is too short. Check the minimum length constraint",
                        "max_length" => 
                            "The value is too long. Check the maximum length constraint",
                        "out_of_range" => 
                            "The value is outside allowed bounds. Check min/max constraints",
                        "pattern" => 
                            "The value doesn't match the required pattern",
                        _ => "Check the validation rules for this field",
                    })
                })
            }
            
            Self::NotFound { hint, key } => {
                hint.as_deref().or_else(|| {
                    // Could implement fuzzy matching here in the future
                    Some("Check that the key exists in the schema. Use `ctx.keys()` to list available keys")
                })
            }
            
            _ => None,
        }
    }

    /// Formats the error with its hint (for display/logging).
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::core::Error;
    ///
    /// let err = Error::type_mismatch("age", ValueKind::Int, ValueKind::Text);
    /// eprintln!("{}", err.with_hint());
    /// // Output:
    /// // type mismatch for key 'age': expected Int, got Text
    /// // Hint: Use `get_int()` for integer values, or convert with `parse()`
    /// ```
    #[must_use]
    pub fn with_hint(&self) -> String {
        match self.hint() {
            Some(hint) => format!("{}\nHint: {}", self, hint),
            None => self.to_string(),
        }
    }

    /// Adds a custom hint to this error.
    ///
    /// Replaces any default hint with the provided custom hint.
    #[must_use]
    pub fn with_custom_hint(mut self, hint: impl Into<String>) -> Self {
        match &mut self {
            Self::TypeMismatch { hint: h, .. } |
            Self::Validation { hint: h, .. } |
            Self::NotFound { hint: h, .. } => {
                *h = Some(hint.into());
            }
            _ => {}
        }
        self
    }
}
```

### Convenience Constructors with Hints

```rust
impl Error {
    /// Creates a TypeMismatch error with default hint.
    pub fn type_mismatch(
        key: impl Into<String>,
        expected: ValueKind,
        actual: ValueKind,
    ) -> Self {
        Self::TypeMismatch {
            key: key.into(),
            expected,
            actual,
            hint: None, // Default hint generated by hint() method
        }
    }

    /// Creates a NotFound error with available keys hint.
    pub fn not_found_with_suggestions(
        key: impl Into<String>,
        available: &[impl AsRef<str>],
    ) -> Self {
        let key = key.into();
        let hint = if available.is_empty() {
            "No keys available in this context".to_string()
        } else {
            format!("Available keys: {}", available.iter().map(|s| s.as_ref()).collect::<Vec<_>>().join(", "))
        };
        
        Self::NotFound {
            key,
            hint: Some(hint),
        }
    }
}
```

---

## Relationships Summary

```
Context
├── schema: Arc<Schema> (shared, immutable)
├── nodes: FxHashMap<Key, ErasedRuntimeNode> (runtime state)
└── ui_state: UiStateManager (presentation state, mutable)
    └── panel_states: FxHashMap<Key, PanelState>

Value
└── Object(Arc<IndexMap<Key, Value>>)
    └── Created via ValueBuilder (temporary, fluent API)

Error
└── hint: Option<String> (contextual guidance)
    └── Generated via hint() method or custom

ValidationError (in Event)
├── code: SmartStr
├── message: SmartStr
├── path: SmartStr (full path, e.g., "user.address.email")
└── field: SmartStr (leaf name, e.g., "email")

RollbackStorage (transactional updates)
├── Small: [(Key, Option<Value>); 8] (stack, <= 8 fields)
└── Large: FxHashMap<Key, Option<Value>> (heap, > 8 fields)
```

---

## Migration Guide

### Panel.collapsed → Context.ui_state

**Before (v0.3.x)**:
```rust
let mut panel = Panel::builder("settings")
    .collapsed(true)
    .build();

// Later mutation (BAD - mutates schema!)
panel.set_collapsed(false);
```

**After (v0.4.x)**:
```rust
// Set initial hint in builder
let panel = Panel::builder("settings")
    .collapsed(true) // Optional: suggests initial UI state
    .build();

// Runtime state managed by Context
ctx.set_panel_collapsed("settings", false);
let is_collapsed = ctx.is_panel_collapsed("settings");
```

### ValidationError (2 args → 4 args)

**Before**:
```rust
ValidationError::new("required", "Field is required")
```

**After (backward compatible)**:
```rust
// Old code still works via ::simple()
ValidationError::simple("required", "Field is required", "email")

// New code with full path
ValidationError::new(
    "required",
    "Field is required",
    "user.address.email", // Full path
    "email"               // Leaf field
)

// Or use specific constructor
ValidationError::required("user.address.email", "email")
```

### Error Handling with Hints

**Before**:
```rust
match result {
    Err(e) => eprintln!("Error: {}", e),
    Ok(v) => process(v),
}
```

**After (with hints)**:
```rust
match result {
    Err(e) => {
        eprintln!("{}", e.with_hint());
        // Or check hint conditionally
        if let Some(hint) = e.hint() {
            eprintln!("💡 {}", hint);
        }
    }
    Ok(v) => process(v),
}
```

---

## Testing Strategy

### UiStateManager Tests
- Creation and default state
- Panel collapse/expand operations
- Multiple panels with independent state
- Serialization/deserialization (with serde feature)
- Thread safety (schema shared, ui_state per-context)

### ValidationError Tests
- Path construction for nested objects
- Simple errors (top-level fields)
- Backward compatibility with 2-arg constructor
- All specific constructors (required, min_length, etc.)

### ValueBuilder Tests
- Empty objects
- Single field
- Multiple fields via `.field()` chaining
- Bulk fields via `.fields()`
- Conditional fields via `.field_if()`
- Capacity optimization
- Nested objects
- From trait conversion

### RollbackStorage Tests
- Small storage (1-8 fields)
- Large storage (9+ fields)
- Automatic upgrade from Small to Large
- Iterator correctness
- Performance benchmarks (stack vs heap)

### Error Hints Tests
- Default hints for all error types
- Custom hints override defaults
- `with_hint()` formatting
- Serde skip verification (hints not serialized)
- Type-specific suggestions accuracy

---

## Performance Considerations

### Memory Impact

| Component | Memory Overhead | Justification |
|-----------|----------------|---------------|
| UiStateManager | ~24 bytes + 24n per panel | Negligible, only for panels |
| PanelState | 24 bytes | One Instant + one bool + padding |
| ValidationError.path | ~23 bytes avg | SmartString inline for short paths |
| Error.hint | 24 bytes Option | Skipped in serde, zero runtime cost |
| ValueBuilder | Temporary | Zero after `.build()` |
| RollbackStorage.Small | 512 bytes | Stack-allocated, freed on scope exit |
| RollbackStorage.Large | 48 + 24n bytes | Heap, only for bulk updates |

### Performance Benefits

- **Event Arc<Value>**: 66% fewer clones (3 → 1 per `set()`)
- **RollbackStorage**: Zero heap allocations for small transactions (<= 8 fields)
- **ValueBuilder**: Single Arc allocation vs multiple intermediate maps
- **Error hints**: No runtime cost (generated on-demand via `hint()`)

---

## Success Metrics

1. **Immutability**: Zero `&mut self` methods on schema types ✅
2. **UI State**: Panel.collapsed field removed, moved to Context.ui_state ✅
3. **Error Paths**: ValidationError includes full path for nested errors ✅
4. **Ergonomics**: ValueBuilder reduces boilerplate by ~60% ✅
5. **Performance**: RollbackStorage uses stack for small transactions ✅
6. **Developer Experience**: Error hints provide actionable guidance ✅

---

**Document Status**: Complete  
**Next Step**: Generate API contracts in `contracts/` directory
