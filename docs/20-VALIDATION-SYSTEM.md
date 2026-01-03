# Validation System

This document describes the hybrid validation system in paramdef, which combines declarative expressions with programmatic validators.

## Overview

The validation system provides a flexible, extensible approach to parameter validation:

- **Declarative rules** via `Expr` - Serializable expressions covering ~80% of common cases
- **Programmatic validators** via `Validator` trait - Rust-native logic for complex cases

This hybrid design is inspired by:
- **JSON Schema** - Declarative constraints (required, minLength, pattern)
- **Zod/Yup** - Chainable validation with cross-field support
- **React Hook Form** - Resolver pattern for library integration
- **CEL (Kubernetes)** - Expression language for policy validation
- **garde/nutype** - Rust validation with derive macros

## Architecture

```text
Rule (validation definition)
  ├── Expr - Declarative, JSON-serializable expressions
  └── Fn   - Programmatic Validator trait implementations

ValidationContext (cross-field access)
  ├── Current value being validated
  ├── Schema reference for metadata
  └── ValueAccess for sibling values

ValidationResult = Result<(), ValidationOutcome>
  └── ValidationOutcome contains Vec<Error>
```

## Quick Start

### Declarative Validation (Most Common)

```rust
use paramdef::validation::{Rule, Rules};

// Create validation rules
let rules = Rules::from_rules([
    Rule::required(),
    Rule::min_length(3),
    Rule::max_length(50),
    Rule::pattern(r"^[a-zA-Z]+$"),
]);

// Validate a value
let value = Value::text("hello");
rules.validate(&value, &ctx)?;
```

### Programmatic Validation

```rust
use paramdef::validation::{Rule, Validator, ValidationContext, ValidationResult};

// Custom validator for complex logic
let rule = Rule::custom("is_even", |value, _ctx| {
    if let Value::Int(n) = value {
        if n % 2 != 0 {
            return Err(Error::custom("even", "Value must be even").into());
        }
    }
    Ok(())
});
```

## Declarative Expressions (Expr)

The `Expr` enum provides a comprehensive set of validation expressions:

### Presence Validation

```rust
Expr::Required  // Value must not be null or empty
```

### String Constraints

```rust
Expr::MinLength(3)           // Minimum 3 characters
Expr::MaxLength(100)         // Maximum 100 characters
Expr::Length(10)             // Exactly 10 characters
Expr::Pattern(r"^\d+$")      // Regex pattern
Expr::Email                  // Valid email format
Expr::Url                    // Valid URL format
Expr::Uuid                   // Valid UUID format
Expr::StartsWith("prefix")   // Starts with string
Expr::EndsWith("suffix")     // Ends with string
Expr::Contains("substr")     // Contains substring
```

### Numeric Constraints

```rust
Expr::Min(0.0)              // Minimum value (inclusive)
Expr::Max(100.0)            // Maximum value (inclusive)
Expr::ExclusiveMin(0.0)     // Minimum value (exclusive)
Expr::ExclusiveMax(100.0)   // Maximum value (exclusive)
Expr::MultipleOf(5.0)       // Must be multiple of 5
Expr::Positive              // Value > 0
Expr::Negative              // Value < 0
Expr::NonNegative           // Value >= 0
Expr::Integer               // No fractional part
```

### Array Constraints

```rust
Expr::MinItems(1)           // Minimum 1 item
Expr::MaxItems(10)          // Maximum 10 items
Expr::ItemCount(5)          // Exactly 5 items
Expr::UniqueItems           // All items unique
```

### Enum/Const Constraints

```rust
Expr::OneOf(vec![Value::text("a"), Value::text("b")])  // Allowed values
Expr::Const(Value::Int(42))                            // Exact value
```

### Logical Operators

```rust
// All must pass
Expr::And(vec![Expr::Required, Expr::MinLength(3)])

// At least one must pass
Expr::Or(vec![Expr::Email, Expr::Url])

// Must fail (negation)
Expr::Not(Box::new(Expr::Required))

// Conditional validation
Expr::If {
    condition: Box::new(Expr::Required),
    then: Box::new(Expr::MinLength(3)),
    otherwise: None,
}
```

### Cross-Field Validation

```rust
Expr::EqualTo("confirm_password")      // Must equal another field
Expr::NotEqualTo("old_password")       // Must differ from another field
Expr::LessThan("max_value")            // Must be less than another field
Expr::GreaterThan("min_value")         // Must be greater than another field
```

## Rule Convenience Methods

The `Rule` type provides convenient constructors:

```rust
// String rules
Rule::required()
Rule::min_length(3)
Rule::max_length(100)
Rule::length(10)
Rule::pattern(r"^\d+$")
Rule::email()
Rule::url()
Rule::uuid()

// Numeric rules
Rule::min(0.0)
Rule::max(100.0)
Rule::range(0.0, 100.0)    // Combined min and max
Rule::positive()
Rule::negative()
Rule::non_negative()
Rule::integer()
Rule::multiple_of(5.0)

// Array rules
Rule::min_items(1)
Rule::max_items(10)
Rule::unique_items()

// Enum rules
Rule::one_of([Value::text("a"), Value::text("b")])
Rule::string_enum(["active", "inactive", "pending"])
Rule::constant(Value::Int(42))

// Cross-field rules
Rule::equal_to("other_field")
Rule::not_equal_to("other_field")

// Logical rules
Rule::all([Rule::required(), Rule::min_length(3)])
Rule::any([Rule::email(), Rule::url()])
Rule::not(Rule::required())
```

## Validator Trait

For complex validation logic, implement the `Validator` trait:

```rust
use paramdef::validation::{Validator, ValidationContext, ValidationResult, Error};
use paramdef::core::Value;

#[derive(Debug)]
struct PasswordMatch;

impl Validator for PasswordMatch {
    fn validate(&self, value: &Value, ctx: &ValidationContext<'_>) -> ValidationResult {
        let password = value.as_text().unwrap_or("");
        let confirm = ctx.get("password_confirm")
            .and_then(|v| v.as_text())
            .unwrap_or("");
        
        if password != confirm {
            return Err(Error::custom("password_mismatch", "Passwords do not match").into());
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "PasswordMatch"
    }
}
```

## Built-in Validators

The library provides several pre-built validators:

### Required

```rust
use paramdef::validation::Required;

let validator = Required;
// Fails for null, empty string, empty array, empty object
```

### Length

```rust
use paramdef::validation::Length;

Length::min(3)           // Minimum 3
Length::max(100)         // Maximum 100
Length::between(3, 100)  // Between 3 and 100
Length::exact(10)        // Exactly 10
```

### Range

```rust
use paramdef::validation::Range;

Range::min(0.0)              // Minimum (inclusive)
Range::max(100.0)            // Maximum (inclusive)
Range::between(0.0, 100.0)   // Between (inclusive)
Range::exclusive(0.0, 100.0) // Between (exclusive)
```

### Match

```rust
use paramdef::validation::Match;

// Validate against another field
Match::new("password_confirm")
    .with_message("Passwords must match")
```

### PasswordStrength

```rust
use paramdef::validation::PasswordStrength;

PasswordStrength::new()
    .min_length(8)
    .require_uppercase(true)
    .require_lowercase(true)
    .require_digit(true)
    .require_special(true)
```

### When (Conditional)

```rust
use paramdef::validation::{When, Required};

// When has_email is true, email is required
When::new("has_email", Value::Bool(true), Required)
```

## ValidationContext

The `ValidationContext` provides access to:

- **Current key** - The parameter being validated
- **Schema** - For metadata lookup
- **Sibling values** - For cross-field validation

```rust
use paramdef::validation::ValidationContext;

fn validate_with_context(value: &Value, ctx: &ValidationContext<'_>) -> ValidationResult {
    // Get current key
    let key = ctx.key();
    
    // Get sibling value
    if let Some(other) = ctx.get("other_field") {
        // Cross-field validation
    }
    
    // Check if sibling exists
    if ctx.has("optional_field") {
        // ...
    }
    
    // Get metadata
    if let Some(meta) = ctx.get_metadata("some_field") {
        // Access labels, descriptions, etc.
    }
    
    Ok(())
}
```

## Error Handling

### Error Type

```rust
use paramdef::validation::Error;

// Built-in error constructors
Error::required()
Error::min_length(5, 3)
Error::max_length(10, 15)
Error::min_value(0.0, -5.0)
Error::max_value(100.0, 150.0)
Error::pattern("^[a-z]+$")
Error::email()
Error::url()
Error::custom("my_code", "Custom error message")
```

### ValidationOutcome

Multiple errors can be collected:

```rust
use paramdef::validation::ValidationOutcome;

// Single error
let outcome = ValidationOutcome::single(Error::required());

// Multiple errors
let outcome = ValidationOutcome::multiple([
    Error::min_length(5, 3),
    Error::pattern("^[a-z]+$"),
]);

// Access errors
for error in outcome.errors() {
    println!("[{}] {}", error.code, error.message);
}
```

### Collecting All Errors

By default, validation stops at the first error. To collect all errors:

```rust
let rules = Rules::from_rules([
    Rule::required(),
    Rule::min_length(5),
    Rule::max_length(10),
]);

// Fail-fast (returns first error)
let result = rules.validate(&value, &ctx);

// Collect all errors
let all_errors = rules.validate_all(&value, &ctx);
for error in all_errors {
    println!("{}", error);
}
```

## Serde Support

With the `serde` feature enabled, `Expr` can be serialized:

```rust
use paramdef::validation::Expr;
use serde_json;

let expr = Expr::And(vec![
    Expr::Required,
    Expr::MinLength(3),
    Expr::Email,
]);

let json = serde_json::to_string(&expr)?;
// {"type":"and","0":[{"type":"required"},{"type":"minLength","0":3},{"type":"email"}]}

let parsed: Expr = serde_json::from_str(&json)?;
```

## Performance Considerations

1. **Regex Caching** - Pattern expressions use a thread-local regex cache to avoid recompilation

2. **Arc Sharing** - Rules use `Arc<dyn Validator>` for cheap cloning and schema sharing

3. **Early Exit** - Default validation stops at first error; use `validate_all` for comprehensive checking

4. **Empty Rules** - Empty rule collections return `Ok(())` immediately

## Integration with Context

While the validation system is independent, it integrates with the event system:

```rust
#[cfg(all(feature = "validation", feature = "events"))]
fn validate_and_emit(ctx: &mut Context, key: &str, value: &Value) {
    let rules = get_rules_for(key);
    let validation_ctx = create_validation_context(key, ctx);
    
    match rules.validate(value, &validation_ctx) {
        Ok(()) => {
            if let Some(bus) = ctx.event_bus() {
                bus.emit(Event::valid(key));
            }
        }
        Err(outcome) => {
            if let Some(bus) = ctx.event_bus() {
                bus.emit(Event::validated(key, false, outcome.to_event_errors()));
            }
        }
    }
}
```

## Migration from Other Libraries

### From garde

```rust
// garde
#[derive(Validate)]
struct User {
    #[garde(length(min = 3, max = 50))]
    username: String,
}

// paramdef
let rules = Rules::from_rules([
    Rule::min_length(3),
    Rule::max_length(50),
]);
```

### From validator

```rust
// validator
#[derive(Validate)]
struct User {
    #[validate(email)]
    email: String,
}

// paramdef
let rules = Rules::from_rules([Rule::email()]);
```

## Best Practices

1. **Prefer Expr for Common Cases** - Declarative expressions are serializable and easier to debug

2. **Use Validators for Complex Logic** - Cross-field validation, async checks, database lookups

3. **Combine with Feature Flags** - Use `#[cfg(feature = "validation")]` for optional validation

4. **Cache Validation Rules** - Store `Rules` in the schema rather than recreating them

5. **Use Meaningful Error Codes** - Error codes like `"password_mismatch"` are easier to handle in UI than generic errors
