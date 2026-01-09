# Visibility System

**Conditional display with declarative expressions**

Version: 1.0  
Feature: `visibility`

---

## Overview

The visibility system enables dynamic show/hide behavior for parameters based on other parameter values. This is essential for building adaptive UIs that only display relevant options.

### Design Influences

| Source | Pattern Adopted |
|--------|-----------------|
| JSON Schema | Conditional schemas (`if`/`then`/`else`) |
| React Hook Form | Field dependencies with `watch()` |
| Formik | Conditional rendering based on values |
| Angular Forms | Dynamic form controls |

---

## Quick Start

```rust
use paramdef::visibility::Expr;
use paramdef::context::Context;
use paramdef::core::Value;
use paramdef::schema::Schema;
use paramdef::types::leaf::{Boolean, Text};
use std::sync::Arc;

// Create schema
let schema = Arc::new(Schema::builder()
    .parameter(Boolean::builder("show_advanced").default(false).build())
    .parameter(Text::builder("advanced_option").build())
    .build());

let mut ctx = Context::new(schema);

// Create visibility condition
let expr = Expr::is_true("show_advanced");

// Evaluate visibility
assert_eq!(expr.eval(&ctx), false); // Not visible initially

ctx.set("show_advanced", Value::Bool(true));
assert_eq!(expr.eval(&ctx), true); // Now visible

// Get dependencies for reactive updates
let deps = expr.dependencies();
assert_eq!(deps.len(), 1);
```

---

## Expression Types

### Value Comparisons
- `Expr::eq(key, value)` - Equals specific value
- `Expr::ne(key, value)` - Not equals
- `Expr::lt(key, threshold)` - Less than (numeric)
- `Expr::gt(key, threshold)` - Greater than (numeric)
- `Expr::lte(key, threshold)` - Less than or equal
- `Expr::gte(key, threshold)` - Greater than or equal

### State Checks
- `Expr::is_set(key)` - Has a value (not null)
- `Expr::is_empty(key)` - Empty (null, "", [])
- `Expr::is_true(key)` - Boolean is true
- `Expr::is_false(key)` - Boolean is false
- `Expr::is_valid(key)` - Passes validation

### Collection Operations
- `Expr::one_of(key, values)` - Value in list
- `Expr::contains(key, value)` - String/array contains value

### Logical Operators
- `Expr::and(exprs)` - All must be true
- `Expr::or(exprs)` - At least one true
- `Expr::negate(expr)` - Inverts result

---

## Examples

### Simple Condition

```rust
// Show only when mode is "advanced"
let expr = Expr::eq("mode", Value::text("advanced"));
```

### Numeric Range

```rust
// Show only for adults (age >= 18)
let expr = Expr::gte("age", 18.0);
```

### Compound Logic

```rust
// Show if (premium AND age >= 18) OR admin
let expr = Expr::or(vec![
    Expr::and(vec![
        Expr::is_true("premium"),
        Expr::gte("age", 18.0),
    ]),
    Expr::is_true("admin"),
]);
```

### Collection Membership

```rust
// Show only for specific modes
let expr = Expr::one_of(
    "mode",
    vec![
        Value::text("advanced"),
        Value::text("expert"),
        Value::text("custom"),
    ],
);
```

---

## Dependency Tracking

The `dependencies()` method returns all parameter keys that an expression depends on:

```rust
let expr = Expr::and(vec![
    Expr::is_true("enabled"),
    Expr::eq("mode", Value::text("advanced")),
]);

let deps = expr.dependencies(); // ["enabled", "mode"]
```

This is used for reactive updates - when a dependency changes, visibility can be re-evaluated.

---

## Evaluation Behavior

### Type Mismatches

If a parameter has the wrong type, the expression returns `false`:

```rust
// Expecting boolean, but value is string
let expr = Expr::is_true("name");
ctx.set("name", Value::text("Alice"));
assert_eq!(expr.eval(&ctx), false); // Type mismatch
```

### Missing Parameters

If a parameter doesn't exist, most expressions return `false`:

```rust
let expr = Expr::eq("missing", Value::text("value"));
assert_eq!(expr.eval(&ctx), false); // Parameter doesn't exist
```

Exception: `IsEmpty` returns `true` for missing parameters.

---

## Best Practices

1. **Keep expressions simple** - Complex logic is hard to debug
2. **Use compound expressions sparingly** - Prefer multiple simple conditions
3. **Document complex visibility rules** - Explain business logic
4. **Test edge cases** - Missing values, type mismatches, etc.

---

## Integration with Events

When used with the event system, visibility can be reactively updated:

```rust
#[cfg(feature = "events")]
{
    let bus = EventBus::new(64);
    let mut ctx = Context::with_event_bus(schema, bus.clone());
    
    // Subscribe to changes
    let mut sub = bus.subscribe();
    
    // When a dependency changes, re-evaluate visibility
    tokio::spawn(async move {
        while let Ok(event) = sub.recv().await {
            if let Event::ValueChanged { key, .. } = event {
                // Re-evaluate expressions that depend on this key
            }
        }
    });
}
```

---

## Future Enhancements

Potential additions (not yet implemented):

1. **Custom Functions**: User-defined evaluation functions
2. **Context Variables**: Access to system state (current user, time, etc.)
3. **Expression Serialization**: Save/load expressions from JSON
4. **Expression Optimization**: Compile complex expressions for performance

---

## Performance Characteristics

| Operation | Time Complexity | Notes |
|-----------|----------------|-------|
| `eval()` | O(n) | n = number of sub-expressions |
| `dependencies()` | O(n) | Cached after first call recommended |
| Simple comparison | O(1) | Direct value lookup |
| `And`/`Or` | O(n * m) | n = exprs, m = avg eval cost |

---

## Comparison with Industry

| Feature | JSON Schema | Formik | paramdef | Match |
|---------|-------------|--------|----------|-------|
| Conditional display | `if`/`then` | `watch()` | `Expr::*` | ✓ |
| Dependency tracking | ❌ | ✓ | ✓ | ✓ |
| Type-safe evaluation | ✓ | ❌ | ✓ | Partial |
| Compound logic | ✓ | ✓ | ✓ | ✓ |
| Serializable | ✓ | ❌ | ✓ (with serde) | ✓ |

---

## Conclusion

The visibility system provides a declarative way to control parameter display with:
- Simple, composable expressions
- Automatic dependency tracking
- Type-safe evaluation
- Integration with validation and events

**Phase 5 (Visibility System) is complete and ready for use!**
