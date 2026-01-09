# Transform System

This document describes the paramdef transformation system for normalizing and modifying values before validation and storage.

## Overview

The transform system follows a **hybrid approach** similar to the validation system:

- **`Transform`** enum: Declarative transformations (~80% of use cases)
- **`Transformer`** trait: Programmatic transformations (~20% complex cases)

## Design Principles

### 1. Transform Before Validate

Following [OWASP best practices](https://cheatsheetseries.owasp.org/cheatsheets/Input_Validation_Cheat_Sheet.html), transformations are applied **before** validation:

```
Input → Transform → Validate → Store
```

This ensures validation rules see normalized data (e.g., trimmed strings, clamped numbers).

### 2. Idempotent Transformations

Transformations should be idempotent: applying them twice produces the same result as applying once:

```rust
transform(transform(x)) == transform(x)
```

### 3. Type-Preserving

Transformations preserve value types when possible:
- String transforms return strings
- Numeric transforms return numbers (Int or Float as appropriate)

## Declarative Transforms

The `Transform` enum covers common transformation patterns:

### String Transformations

```rust
use paramdef::transform::Transform;
use paramdef::Value;

// Whitespace handling
Transform::Trim                    // "  hello  " → "hello"
Transform::TrimStart               // "  hello  " → "hello  "
Transform::TrimEnd                 // "  hello  " → "  hello"
Transform::CollapseWhitespace      // "a   b\tc" → "a b c"
Transform::RemoveWhitespace        // "a b c" → "abc"

// Case transformations
Transform::Lowercase               // "HELLO" → "hello"
Transform::Uppercase               // "hello" → "HELLO"
Transform::Capitalize              // "hello world" → "Hello World"

// String manipulation
Transform::Replace { from, to }    // Replace substring
Transform::Truncate { max_length, suffix }  // Limit length
Transform::Pad { min_length, char, start }  // Pad to length
```

### Numeric Transformations

```rust
// Rounding
Transform::Abs                     // -5 → 5
Transform::Ceil                    // 3.2 → 4.0
Transform::Floor                   // 3.8 → 3.0
Transform::Round                   // 3.5 → 4.0
Transform::RoundTo { decimals: 2 } // 3.14159 → 3.14

// Range clamping
Transform::Clamp { min: 0.0, max: 100.0 }  // Constrain to range
```

### Null Handling

```rust
// Default values
Transform::DefaultTo { value }     // Null → default value
Transform::NullIf { value }        // specific value → Null
Transform::NullIfEmpty             // "" → Null
```

### Composition

```rust
// Chain transformations
Transform::Sequence(vec![
    Transform::Trim,
    Transform::Lowercase,
    Transform::Capitalize,
])

// Or use .then() method
Transform::Trim
    .then(Transform::Lowercase)
    .then(Transform::Capitalize)
```

## Programmatic Transformers

For complex transformations, implement the `Transformer` trait:

```rust
use paramdef::transform::Transformer;
use paramdef::Value;

struct PhoneFormatter;

impl Transformer for PhoneFormatter {
    fn name(&self) -> &'static str { "phone_formatter" }
    
    fn transform(&self, value: &Value) -> Value {
        if let Some(s) = value.as_text() {
            let digits: String = s.chars()
                .filter(|c| c.is_ascii_digit())
                .collect();
            
            if digits.len() == 10 {
                Value::text(format!("({}) {}-{}",
                    &digits[0..3], &digits[3..6], &digits[6..10]))
            } else {
                value.clone()
            }
        } else {
            value.clone()
        }
    }
}
```

### Function Transformers

Use `FnTransformer` for quick one-off transformations:

```rust
use paramdef::transform::FnTransformer;

let reverse = FnTransformer::new("reverse", |value| {
    if let Some(s) = value.as_text() {
        Value::text(s.chars().rev().collect::<String>())
    } else {
        value.clone()
    }
});
```

## Transforms Collection

Combine multiple transformations using `Transforms`:

```rust
use paramdef::transform::{Transform, Transforms};

// Declarative only
let text_normalize = Transforms::new()
    .trim()
    .lowercase()
    .collapse_whitespace();

// Mixed declarative and programmatic
let pipeline = Transforms::new()
    .push(Transform::Trim)
    .custom(PhoneFormatter)
    .func("uppercase_first", |v| {
        // Custom inline transformation
        v.clone()
    });

// Apply to value
let result = pipeline.apply(&value);
```

### Convenience Methods

`Transforms` provides fluent methods for common operations:

```rust
Transforms::new()
    .trim()                              // Remove whitespace
    .lowercase()                         // Convert to lowercase
    .uppercase()                         // Convert to uppercase
    .capitalize()                        // Capitalize words
    .clamp(0.0, 100.0)                  // Clamp numbers
    .round_to(2)                        // Round to decimals
    .default_to(Value::text("N/A"))     // Default for Null
    .truncate(100)                      // Limit length
    .truncate_with_ellipsis(100)        // Truncate with "..."
    .replace("foo", "bar")              // Replace substring
    .collapse_whitespace()              // Normalize spaces
    .null_if_empty()                    // Empty string to Null
```

## Built-in Transformers

The module provides struct-based transformers for configurable scenarios:

### Clamp

```rust
use paramdef::transform::Clamp;

let clamp = Clamp::new(0.0, 100.0);
clamp.transform(&Value::Int(150));  // → Value::Int(100)
```

### Round

```rust
use paramdef::transform::Round;

let round = Round::new(2);           // 2 decimal places
let whole = Round::whole();          // Round to integers
```

### Truncate

```rust
use paramdef::transform::Truncate;

let truncate = Truncate::new(50)
    .with_ellipsis();  // or .with_suffix("...")
```

### Replace

```rust
use paramdef::transform::Replace;

let replace = Replace::new("old", "new");
```

### Default

```rust
use paramdef::transform::Default;

let default = Default::text("N/A");
let default_num = Default::int(0);
```

## Industry Patterns

The transform system is inspired by:

### React Final Form

- `parse`: Input → Storage transformation
- `format`: Storage → Display transformation
- `normalize`: Transformation during parse

### Express Validator

Method chaining for sanitization:
```javascript
body('email').trim().normalizeEmail()
```

### OWASP Guidelines

1. Normalize before validate
2. Use canonical encoding
3. Sanitize at trust boundaries

## Integration with Validation

Transformations and validation work together:

```rust
// 1. User input arrives
let input = Value::text("  HELLO@EXAMPLE.COM  ");

// 2. Apply transformations
let transforms = Transforms::new()
    .trim()
    .lowercase();
let normalized = transforms.apply(&input);
// Result: "hello@example.com"

// 3. Validate normalized value
let rules = Rules::new().push(Rule::email());
let result = rules.validate(&normalized, &context);
```

## Best Practices

1. **Apply transformations before validation** - Always normalize input first
2. **Keep transformations pure** - No side effects, no external state
3. **Make transformations idempotent** - Safe to apply multiple times
4. **Preserve types when possible** - Int transforms return Int
5. **Use declarative `Transform` for common cases** - Better serialization
6. **Use `Transformer` trait for complex logic** - Full flexibility

## Serde Support

When the `serde` feature is enabled, `Transform` can be serialized:

```json
{
  "type": "sequence",
  "transforms": [
    { "type": "trim" },
    { "type": "lowercase" },
    { "type": "truncate", "max_length": 100, "suffix": "..." }
  ]
}
```

## Performance Considerations

- Transformations are applied lazily (on demand)
- Empty transform lists skip processing entirely
- String transformations use efficient iterators
- Numeric operations use native f64/i64 operations
