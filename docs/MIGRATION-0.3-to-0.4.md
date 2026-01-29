# Migration Guide: 0.3.x → 0.4.0

This guide helps you migrate from paramdef 0.3.x to 0.4.0.

## Overview

Version 0.4.0 introduces critical architectural improvements to enforce immutability in the schema layer:

- **Panel collapsed state** moved from schema to `Context::ui_state()`
- **Visibility mutations** deprecated (use builder-time `.visible_when()`)
- **Layout trait** simplified (removed mutable methods)

These changes fix architectural violations and enable safe concurrent schema sharing with `Arc<Schema>` without requiring `Mutex`.

## Breaking Changes

### 1. Panel Collapsed State → Context UI State

**Problem**: `Panel::collapsed` field violated immutability principle.

**Before (0.3.x)**:
```rust
use paramdef::types::group::Panel;
use paramdef::types::traits::Layout;

let mut panel = Panel::builder("settings")
    .collapsed(true)
    .build();

// Mutating schema after construction
panel.set_collapsed(false);

// Reading collapsed state from schema
if panel.is_collapsed() {
    println!("Panel is collapsed");
}
```

**After (0.4.0)**:
```rust
use paramdef::types::group::Panel;
use paramdef::context::Context;
use paramdef::schema::Schema;
use std::sync::Arc;

// Panel construction - .collapsed() is now an initial state HINT
let panel = Panel::builder("settings")
    .collapsed(true)  // Initial state hint only
    .build();

let schema = Arc::new(Schema::builder().parameter(panel).build());
let mut ctx = Context::new(schema);

// UI state managed through Context
ctx.set_panel_collapsed("settings", false);

// Reading collapsed state from Context
if ctx.is_panel_collapsed(&"settings".into()) {
    println!("Panel is collapsed");
}
```

**Key Changes**:
- ❌ Removed: `Panel::is_collapsed()` method
- ❌ Removed: `Panel::set_collapsed()` method  
- ❌ Removed: `Layout::is_collapsed()` trait method
- ❌ Removed: `Layout::set_collapsed()` trait method
- ✅ Added: `Context::ui_state()` - access to `UiStateManager`
- ✅ Added: `Context::is_panel_collapsed(key)` - check collapsed state
- ✅ Added: `Context::set_panel_collapsed(key, collapsed)` - set collapsed state

### 2. Visibility Rules - Build-Time Only

**Problem**: `set_visibility_rule(&mut self)` allowed schema mutation after construction.

**Before (0.3.x)**:
```rust
use paramdef::types::leaf::Text;
use paramdef::types::traits::Visibility;
use paramdef::visibility::when;
use paramdef::core::Value;

let mut field = Text::builder("advanced_option").build();

// Mutating schema after construction
field.set_visibility_rule(Some(when("show_advanced").eq(Value::Bool(true))));
```

**After (0.4.0)**:
```rust
use paramdef::types::leaf::Text;
use paramdef::visibility::when;
use paramdef::core::Value;

// Set visibility during construction (immutable after .build())
let field = Text::builder("advanced_option")
    .visible_when(when("show_advanced").eq(Value::Bool(true)))
    .build();
```

**Deprecation Warning**:
```rust
// This still compiles but emits a deprecation warning:
warning: use of deprecated method `paramdef::types::traits::Visibility::set_visibility_rule`: 
         Set visibility via builder. Use .visible_when() during construction. 
         Schema should be immutable after build().
```

**Key Changes**:
- ⚠️ Deprecated: `Visibility::set_visibility_rule(&mut self)` (will be removed in 0.5.0)
- ✅ Use instead: `.visible_when(rule)` on builder before calling `.build()`

## Benefits of These Changes

### ✅ True Schema Immutability

```rust
use std::sync::Arc;
use paramdef::schema::Schema;
use paramdef::context::Context;

// Schema can be safely shared across contexts without Mutex
let schema = Arc::new(Schema::builder()
    .parameter(Panel::builder("panel1").build())
    .parameter(Text::builder("field1").build())
    .build());

// Each context has independent runtime state
let mut ctx1 = Context::new(Arc::clone(&schema));
let mut ctx2 = Context::new(Arc::clone(&schema));

// Different UI states in each context
ctx1.set_panel_collapsed("panel1", true);  // ctx1: collapsed
ctx2.set_panel_collapsed("panel1", false); // ctx2: expanded

// Schema remains immutable - only Arc needed, no Mutex!
```

### ✅ Thread-Safe Schema Sharing

```rust
use std::thread;

let schema = Arc::new(Schema::builder()
    .parameter(Panel::builder("settings").build())
    .build());

// Spawn 10 threads, each with their own Context
let handles: Vec<_> = (0..10)
    .map(|i| {
        let schema_clone = Arc::clone(&schema);
        thread::spawn(move || {
            let mut ctx = Context::new(schema_clone);
            ctx.set_panel_collapsed("settings", i % 2 == 0);
            // Each thread has independent UI state
        })
    })
    .collect();

for handle in handles {
    handle.join().unwrap();
}
```

### ✅ Cleaner Architecture

**Before**: Schema mixed immutable definition with mutable runtime state  
**After**: Clean separation - Schema (immutable) + Context (mutable runtime state)

This aligns with the three-layer architecture:
1. **Schema Layer** (immutable, `Arc`-shareable) - WHAT parameters exist
2. **Runtime Layer** (mutable, per-context) - CURRENT state of parameters
3. **Value Layer** (data representation) - Parameter values

## Migration Checklist

- [ ] **Search and replace `panel.set_collapsed()`**
  - Replace with `ctx.set_panel_collapsed(panel_key, value)`
  
- [ ] **Search and replace `panel.is_collapsed()`**
  - Replace with `ctx.is_panel_collapsed(&panel_key.into())`

- [ ] **Search for `set_visibility_rule` calls**
  - Move visibility rules to builder: `.visible_when(rule)`
  - Remove post-construction `set_visibility_rule()` calls

- [ ] **Update tests that mutate schema after construction**
  - Ensure all schema configuration happens in builder chain
  - Move runtime state operations to Context

- [ ] **Run tests with deprecation warnings enabled**
  ```bash
  cargo test 2>&1 | grep "deprecated"
  ```

- [ ] **Fix all deprecation warnings before 0.5.0**
  - `set_visibility_rule` will be removed in 0.5.0

## Timeline

- **0.4.0 (current)**: Deprecation warnings active, old APIs still work
- **0.5.0 (future)**: Deprecated methods removed, breaking change for code still using old APIs

## Need Help?

- Check the [examples/](../examples/) directory for updated patterns
- See [tests/immutability_tests.rs](../tests/immutability_tests.rs) for comprehensive examples
- Review [docs/01-ARCHITECTURE.md](./01-ARCHITECTURE.md) for three-layer architecture explanation
- Open an issue at https://github.com/your-repo/paramdef/issues if you encounter migration problems
