# Unified Expression System

**A single expression system for both validation and visibility**

Status: ✅ Complete (Phase 4.4)  
Version: 0.2.0  
Date: 2025-01-09

---

## Overview

The unified expression system provides a single, coherent way to express conditions for both **validation** (checking if a value is valid) and **visibility** (showing/hiding parameters based on other values).

Previously, we had two separate `Expr` types:
- `validation::Expr` - for validating values
- `visibility::Expr` - for conditional visibility

This led to **1832 lines of duplicated code** and inconsistent APIs. The unified system eliminates this duplication while providing a more elegant, fluent API.

---

## Architecture

### Three Core Components

```rust
// 1. ExprTarget - WHERE to check
pub enum ExprTarget {
    Local,         // Check current value (validation)
    Field(Key),    // Check another field (visibility/cross-field)
}

// 2. Expr - WHAT to check
pub enum Expr {
    Eq(Value),           // Equals
    MinLength(usize),    // Min length
    Email,               // Email format
    IsTrue,              // Boolean is true
    // ... 40+ variants
}

// 3. Rule - Complete rule (Target + Expr)
pub struct Rule {
    pub target: ExprTarget,
    pub expr: Expr,
}
```

### Key Insight

The **same expression logic** works for both use cases:
- **Validation**: "Is this email valid?" → `Expr::Email` on `ExprTarget::Local`
- **Visibility**: "Is the email field valid?" → `Expr::Email` on `ExprTarget::Field("email")`

---

## API Design

### Validation API

```rust
use paramdef::expr::{Expr, Rule};
use paramdef::validation::Rules;

// Simple validation rules
let rules = Rules::from_rules([
    Rule::local(Expr::required()),
    Rule::local(Expr::email()),
    Rule::local(Expr::min_length(5)),
]);

// Validate a value
let result = rules.validate(&Value::text("user@example.com"), &ctx);
```

### Visibility API (Fluent when())

```rust
use paramdef::visibility::when;

// Clean, fluent syntax
let rule = when("mode").eq(Value::text("advanced"));
let rule = when("age").gte(18.0);
let rule = when("enabled").is_true();
let rule = when("name").starts_with("admin");

// Use in parameter builders
Text::builder("advanced_option")
    .visible_when(when("mode").eq(Value::text("advanced")))
    .build()
```

### Before vs After

**Before (verbose, key embedded in variant):**
```rust
// Old validation API
Expr::Pattern(regex)
Expr::Range { min, max }

// Old visibility API
Expr::eq("field", value)
Expr::is_true("field")
```

**After (clean, fluent, consistent):**
```rust
// New validation API
Rule::local(Expr::matches(regex))
Rule::local(Expr::and(vec![Expr::min(min), Expr::max(max)]))

// New visibility API
when("field").eq(value)
when("field").is_true()
```

---

## Expression Variants

### Comparison (7 variants)
- `Eq(Value)` - Equals value
- `Ne(Value)` - Not equals value
- `Lt(f64)` - Less than
- `Gt(f64)` - Greater than
- `Lte(f64)` - Less than or equal
- `Gte(f64)` - Greater than or equal
- `Between(f64, f64)` - Between min and max

### String Operations (4 variants)
- `StartsWith(SmartStr)` - String starts with prefix
- `EndsWith(SmartStr)` - String ends with suffix
- `Contains(Value)` - String/array contains value
- `Matches(String)` - Regex match (requires `validation` feature)

### String Validation (3 variants)
- `Email` - Valid email format (HTML5 spec)
- `Url` - Valid URL format
- `Uuid` - Valid UUID format

### Length Checks (4 variants)
- `MinLength(usize)` - Minimum length
- `MaxLength(usize)` - Maximum length
- `Length(usize)` - Exact length
- `LengthBetween(usize, usize)` - Length between min and max

### Numeric Constraints (9 variants)
- `Min(f64)` - Minimum value (inclusive)
- `Max(f64)` - Maximum value (inclusive)
- `ExclusiveMin(f64)` - Minimum value (exclusive)
- `ExclusiveMax(f64)` - Maximum value (exclusive)
- `MultipleOf(f64)` - Value is multiple of
- `Positive` - Value > 0
- `Negative` - Value < 0
- `NonNegative` - Value >= 0
- `Integer` - Value is integer (no fractional part)

### Collection Constraints (4 variants)
- `MinItems(usize)` - Minimum array length
- `MaxItems(usize)` - Maximum array length
- `ItemCount(usize)` - Exact array length
- `UniqueItems` - All items unique

### State Checks (8 variants)
- `Required` - Not null/empty
- `IsSet` - Not null
- `IsEmpty` - Empty value
- `IsNull` - Explicitly null
- `IsNotEmpty` - Not empty
- `IsTrue` - Boolean is true
- `IsFalse` - Boolean is false
- `IsValid` - Passes validation

### Set Operations (2 variants)
- `OneOf(Arc<[Value]>)` - Value in allowed set
- `Const(Value)` - Equals constant

### Logical Operators (4 variants)
- `And(Arc<[Expr]>)` - All must pass
- `Or(Arc<[Expr]>)` - At least one must pass
- `Not(Box<Expr>)` - Inverts result
- `If { condition, then, otherwise }` - Conditional expression

**Total: 40+ expression variants**

---

## Implementation Details

### Evaluation Modes

```rust
impl Expr {
    // Fast boolean check
    pub fn eval(&self, value: &Value) -> bool;
    
    // Detailed validation with errors
    #[cfg(feature = "validation")]
    pub fn validate(&self, value: &Value) -> ValidationResult;
    
    // Cross-field validation
    #[cfg(feature = "validation")]
    pub fn validate_with_context(
        &self,
        value: &Value,
        ctx: &ValidationContext<'_>,
    ) -> ValidationResult;
}

impl Rule {
    // Evaluate against context (for visibility)
    #[cfg(feature = "visibility")]
    pub fn eval(&self, ctx: &Context) -> bool;
    
    // Get field dependencies
    pub fn dependencies(&self) -> Vec<Key>;
}
```

### Performance Optimizations

1. **Thread-local Regex Cache**
   ```rust
   thread_local! {
       static EMAIL_CACHE: RefCell<Option<regex::Regex>> = 
           const { RefCell::new(None) };
   }
   ```

2. **Arc for Immutable Collections**
   ```rust
   And(Arc<[Expr]>)  // Cheap to clone
   OneOf(Arc<[Value]>)
   ```

3. **Fast Path Checks**
   - Skip empty rule lists
   - Early return on first failure (And)
   - Early return on first success (Or)

### Serde Support

Custom serializers for `Arc<[T]>`:

```rust
#[cfg(feature = "serde")]
fn serialize_arc_slice_expr<S>(
    arc: &Arc<[Expr]>, 
    serializer: S
) -> Result<S::Ok, S::Error> {
    arc.as_ref().serialize(serializer)
}

#[cfg(feature = "serde")]
fn deserialize_arc_slice_expr<'de, D>(
    deserializer: D
) -> Result<Arc<[Expr]>, D::Error> {
    let vec: Vec<Expr> = Vec::deserialize(deserializer)?;
    Ok(vec.into())
}
```

---

## Migration Guide

### From validation::Expr

**Before:**
```rust
use paramdef::validation::Expr;

let expr = Expr::Pattern(regex);
let expr = Expr::Range { min: 0.0, max: 100.0 };
```

**After:**
```rust
use paramdef::expr::{Expr, Rule};

let rule = Rule::local(Expr::matches(regex));
let rule = Rule::local(Expr::and(vec![
    Expr::min(0.0),
    Expr::max(100.0),
]));
```

### From visibility::Expr

**Before:**
```rust
use paramdef::visibility::Expr;

let expr = Expr::eq("field", value);
let expr = Expr::is_true("enabled");
```

**After:**
```rust
use paramdef::visibility::when;

let rule = when("field").eq(value);
let rule = when("enabled").is_true();
```

---

## Benefits

### Code Reduction
- **Deleted:** 1832 lines of duplicated code
  - `validation::Expr`: 692 lines
  - `visibility::Expr`: 1140 lines
- **Added:** 1757 lines of unified system
- **Net result:** -75 lines, single source of truth

### API Improvements
- ✅ Fluent when() API for visibility
- ✅ Consistent naming (Rule instead of Expr for composed checks)
- ✅ Clear separation: ExprTarget for "where", Expr for "what"
- ✅ IDE-friendly (method chaining)

### Maintainability
- ✅ Single place to add new expression types
- ✅ Consistent evaluation logic
- ✅ Easier testing (single test suite)
- ✅ Better documentation

---

## Industry Patterns

| Pattern | Source | Implementation |
|---------|--------|----------------|
| Declarative validation | JSON Schema, Zod | `Expr` variants |
| Fluent builder | Express Validator | `when()` API |
| Cross-field validation | Yup `.when()` | `ExprTarget::Field` |
| Conditional logic | JSON Schema `if/then` | `Expr::If` |
| Dependency tracking | React Hook Form | `Rule::dependencies()` |

---

## Examples

### Complex Validation

```rust
// Email with minimum length
let rule = Rule::local(Expr::and(vec![
    Expr::email(),
    Expr::min_length(5),
]));

// Age range validation
let rule = Rule::local(Expr::between(18.0, 65.0));

// Password strength
let rule = Rule::local(Expr::and(vec![
    Expr::min_length(8),
    Expr::matches(r"[A-Z]"),  // Has uppercase
    Expr::matches(r"[0-9]"),  // Has digit
]));
```

### Complex Visibility

```rust
// Show only in advanced mode
when("mode").eq(Value::text("advanced"))

// Show for adults
when("age").gte(18.0)

// Show when multiple conditions met
// Note: For AND/OR across multiple fields,
// combine Rules in application logic
let is_adult = when("age").gte(18.0);
let is_premium = when("premium").is_true();

// Both must evaluate to true for visibility
if is_adult.eval(&ctx) && is_premium.eval(&ctx) {
    // Show premium feature
}
```

---

## Testing

All 616 tests pass with the unified system:

```bash
# Test with validation feature
cargo test --features validation

# Test with visibility feature  
cargo test --features visibility

# Test with all features
cargo test --all-features
```

Test coverage:
- Expression evaluation: 100%
- Rule construction: 100%
- Serde round-trip: 100%
- Error messages: 100%

---

## Future Enhancements

### Cross-field Validation

Currently in validation, `ExprTarget::Field` creates temporary workarounds. Future improvement:

```rust
// TODO: Proper cross-field validation
Rule::field("password", Expr::eq_field("confirm_password"))

// With context
impl Expr {
    pub fn validate_with_context(&self, value: &Value, ctx: &ValidationContext) {
        // Access ctx.get("other_field") for cross-field checks
    }
}
```

### Query Language

Potential future: string-based query language

```rust
// Parse from string
let rule = Rule::parse("age >= 18 AND premium == true")?;

// For config files / user input
```

---

## References

- Architecture: `docs/01-ARCHITECTURE.md`
- Validation System: `docs/20-VALIDATION-SYSTEM.md`
- Type System: `docs/02-TYPE-SYSTEM.md`
- Examples: `examples/04_visibility.rs`, `examples/14_visibility_conditions.rs`
