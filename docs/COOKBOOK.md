# paramdef Cookbook

**Practical recipes for common use cases**

This cookbook provides copy-paste examples for common scenarios using paramdef. Each recipe is production-ready and follows best practices.

---

## Table of Contents

1. [Creating a Simple Form with Required Fields](#recipe-1-simple-form-with-required-fields)
2. [Adding Validation with Shortcuts](#recipe-2-validation-shortcuts)
3. [Handling Validation Errors with Paths](#recipe-3-validation-error-handling)
4. [Building Complex Objects with ValueBuilder](#recipe-4-complex-objects)
5. [Transactional Bulk Updates](#recipe-5-transactional-updates)
6. [Event-Driven Reactive Updates](#recipe-6-reactive-updates)
7. [Conditional Field Visibility](#recipe-7-conditional-visibility)
8. [Nested Object Structures](#recipe-8-nested-structures)
9. [Custom Validation Logic](#recipe-9-custom-validation)
10. [Error Recovery Patterns](#recipe-10-error-recovery)
11. [Performance Optimization Tips](#recipe-11-performance-tips)
12. [Working with Subtypes and Units](#recipe-12-subtypes-and-units)

---

## Recipe 1: Simple Form with Required Fields

**Use Case**: Create a user registration form with email, password, and optional name.

```rust
use paramdef::prelude::*;
use std::sync::Arc;

// Create schema
let schema = Arc::new(Schema::builder()
    .parameter(Text::email("email")
        .label("Email Address")
        .placeholder("user@example.com")
        .required()
        .build())
    .parameter(Text::password("password")
        .label("Password")
        .placeholder("Enter a strong password")
        .required()
        .build())
    .parameter(Text::builder("name")
        .label("Full Name (Optional)")
        .placeholder("John Doe")
        .build())
    .build());

// Create runtime context
let mut ctx = Context::new(schema);

// Set values
ctx.set("email", Value::text("alice@example.com"))?;
ctx.set("password", Value::text("SecurePass123!"))?;
// name is optional, left empty

// Check if required fields are filled
if ctx.is_valid() {
    println!("Form ready to submit!");
}
```

**Key Points**:
- Use `.required()` shortcut for mandatory fields
- Email and password subtypes provide built-in validation
- Context tracks field state automatically

---

## Recipe 2: Validation Shortcuts

**Use Case**: Add common validation rules without verbose boilerplate.

```rust
use paramdef::prelude::*;

// Email with validation
let email = Text::email("user_email")
    .required()
    .build();

// Password with minimum length
let password = Text::password("user_password")
    .required()
    .build();

// Username with length constraints
let username = Text::builder("username")
    .required()
    .build();

// Age with range validation
let age = Number::integer("age")
    .label("Age")
    .soft_min(0.0)
    .soft_max(120.0)
    .hard_min(0.0)  // Validation enforced
    .build();

// Phone number
let phone = Text::phone("phone")
    .label("Phone Number")
    .placeholder("+1 (555) 123-4567")
    .build();

// URL validation
let website = Text::url("website")
    .label("Website")
    .placeholder("https://example.com")
    .build();
```

**Key Points**:
- Subtypes provide semantic meaning and validation
- `soft_min/soft_max` are UI hints
- `hard_min/hard_max` enforce validation
- Required fields fail validation if empty

---

## Recipe 3: Validation Error Handling

**Use Case**: Validate form data and provide helpful error messages.

```rust
use paramdef::prelude::*;
use std::sync::Arc;

let schema = Arc::new(Schema::builder()
    .parameter(Text::email("email").required().build())
    .parameter(Number::integer("age")
        .hard_min(18.0)
        .hard_max(100.0)
        .build())
    .build());

let mut ctx = Context::new(schema);

// Set invalid data
ctx.set("email", Value::text("not-an-email"))?;
ctx.set("age", Value::Float(15.0))?;  // Below minimum

// Check for validation errors
if !ctx.is_valid() {
    // Get all invalid values
    for (key, _value) in ctx.invalid_values() {
        if let Some(node) = ctx.node(key) {
            let errors = node.state().errors();
            println!("{}: {} error(s)", key, errors.len());
            
            for error in errors {
                eprintln!("  - {}", error);
            }
        }
    }
}

// Check specific field
if let Some(node) = ctx.node("email") {
    if !node.state().is_valid() {
        println!("Email validation failed");
    }
}
```

**Key Points**:
- Use `ctx.invalid_values()` to iterate over failed fields
- Access validation errors via `node.state().errors()`
- Check individual fields with `ctx.node(key)`
- Zero-copy iteration with references

---

## Recipe 4: Complex Objects with ValueBuilder

**Use Case**: Build nested JSON-like structures programmatically.

```rust
use paramdef::core::Value;

// Using ValueBuilder for complex objects
let user = Value::build_object()
    .text("name", "Alice Smith")
    .int("age", 30)
    .bool("active", true)
    .nested("address", |addr| {
        addr.text("street", "123 Main St")
            .text("city", "Springfield")
            .text("country", "USA")
            .int("zip", 12345)
    })
    .fields("roles", vec![
        Value::text("admin"),
        Value::text("user"),
    ])
    .build();

// Result:
// {
//   "name": "Alice Smith",
//   "age": 30,
//   "active": true,
//   "address": {
//     "street": "123 Main St",
//     "city": "Springfield",
//     "country": "USA",
//     "zip": 12345
//   },
//   "roles": ["admin", "user"]
// }
```

**Key Points**:
- `Value::build_object()` provides fluent API
- `.nested()` creates sub-objects
- `.fields()` adds arrays
- Zero overhead - compiles to direct construction

---

## Recipe 5: Transactional Bulk Updates

**Use Case**: Update multiple fields atomically - all succeed or all rollback.

```rust
use paramdef::prelude::*;
use std::sync::Arc;

let schema = Arc::new(Schema::builder()
    .parameter(Text::builder("first_name").required().build())
    .parameter(Text::builder("last_name").required().build())
    .parameter(Text::email("email").required().build())
    .parameter(Number::port("port").build())
    .build());

let mut ctx = Context::new(schema);

// Transactional update - all or nothing
let result = ctx.set_many_transactional([
    ("first_name", Value::text("Alice")),
    ("last_name", Value::text("Smith")),
    ("email", Value::text("alice@example.com")),
    ("port", Value::Float(8080.0)),
]);

match result {
    Ok(()) => println!("All fields updated successfully"),
    Err(e) => {
        println!("Transaction failed: {}", e);
        // All changes rolled back automatically
    }
}

// Verify atomicity
assert_eq!(ctx.get("first_name"), if result.is_ok() { 
    Some(&Value::text("Alice"))
} else {
    None  // Rolled back
});
```

**Key Points**:
- **Small transactions (≤8 fields)**: Zero heap allocations (stack-optimized)
- **Large transactions (>8 fields)**: Single heap allocation
- Automatic rollback on any error
- Events emitted as batch (if events feature enabled)

**Performance**:
- 8 fields: ~1.77µs (stack buffer)
- 100 fields: ~22.5µs (heap HashMap)
- Linear scaling: ~221-225ns per field

---

## Recipe 6: Event-Driven Reactive Updates

**Use Case**: Build reactive UIs that respond to parameter changes.

```rust
use paramdef::prelude::*;
use paramdef::event::{Event, EventBus};
use std::sync::Arc;

#[cfg(feature = "events")]
async fn reactive_example() -> Result<(), Box<dyn std::error::Error>> {
    let schema = Arc::new(Schema::builder()
        .parameter(Text::builder("name").build())
        .parameter(Number::integer("age").build())
        .build());

    // Create event bus
    let bus = EventBus::new(64);
    let mut subscriber = bus.subscribe();
    
    // Create context with events
    let mut ctx = Context::with_event_bus(schema, bus);

    // Spawn background task to handle events
    tokio::spawn(async move {
        loop {
            match subscriber.recv().await {
                Ok(Event::ValueChanged { key, new_value, .. }) => {
                    println!("Field '{}' changed to: {:?}", key, new_value);
                    // Update UI, trigger computations, etc.
                }
                Ok(Event::Validated { key, is_valid, .. }) => {
                    println!("Field '{}' validation: {}", key, 
                        if is_valid { "✓" } else { "✗" });
                }
                Ok(Event::Dirtied { key }) => {
                    println!("Field '{}' modified (unsaved)", key);
                }
                _ => {}
            }
        }
    });

    // Make changes - events are broadcast automatically
    ctx.set("name", Value::text("Alice"))?;
    ctx.set("age", Value::Int(30))?;

    Ok(())
}
```

**Key Points**:
- Use `EventBus` with `tokio::broadcast` channel
- Subscribe before creating context
- Events include `ValueChanging`, `ValueChanged`, `Validated`, `Dirtied`
- **Performance**: ~86ns overhead per update, 1.71M fields/sec throughput

**Event Types**:
- `ValueChanging` - Before change (can be used for validation)
- `ValueChanged` - After change (Arc<Value> for efficiency)
- `Validated` - After validation run
- `Touched` - User interacted with field
- `Dirtied` - First unsaved change
- `AllCleaned` - All changes marked clean
- `ContextReset` - Context reset to initial state

---

## Recipe 7: Conditional Field Visibility

**Use Case**: Show/hide fields based on other field values.

```rust
use paramdef::prelude::*;
use paramdef::expr::{Expr, when};

#[cfg(feature = "validation")]
fn conditional_visibility_example() {
    use std::sync::Arc;
    
    let schema = Arc::new(Schema::builder()
        // Main toggle
        .parameter(Boolean::builder("show_advanced")
            .label("Show Advanced Settings")
            .default(false)
            .build())
        
        // Conditionally visible field
        .parameter(Text::builder("api_key")
            .label("API Key")
            .visible_when(when("show_advanced").is_true())
            .build())
        
        // Show if mode is "custom"
        .parameter(Text::builder("mode")
            .label("Mode")
            .default(Value::text("simple"))
            .build())
        
        .parameter(Text::builder("custom_config")
            .label("Custom Configuration")
            .visible_when(when("mode").eq(Value::text("custom")))
            .build())
        
        .build());

    let ctx = Context::new(schema);
    
    // Check visibility
    if let Some(node) = ctx.schema().get("api_key") {
        // Visibility is determined by expression evaluation
        // UI frameworks can query this to show/hide fields
    }
}
```

**Key Points**:
- Use `visible_when()` with expression rules
- Expressions reference other fields with `when(key)`
- Common patterns: `is_true()`, `eq()`, `ne()`, `in_array()`
- 40+ built-in expression types available

---

## Recipe 8: Nested Object Structures

**Use Case**: Define complex hierarchical data structures.

```rust
use paramdef::prelude::*;
use paramdef::types::container::Object;
use std::sync::Arc;

let schema = Arc::new(Schema::builder()
    .parameter(Object::builder("database")
        .label("Database Configuration")
        .field(Text::builder("host")
            .label("Host")
            .default(Value::text("localhost"))
            .build())
        .field(Number::port("port")
            .default(5432.0)
            .build())
        .field(Text::builder("username")
            .required()
            .build())
        .field(Text::password("password")
            .required()
            .build())
        .build())
    .parameter(Object::builder("cache")
        .label("Cache Settings")
        .field(Boolean::builder("enabled")
            .default(true)
            .build())
        .field(Number::integer("ttl")
            .label("TTL (seconds)")
            .default(300.0)
            .build())
        .build())
    .build());

let mut ctx = Context::new(schema);

// Set nested values
ctx.set("database", Value::build_object()
    .text("host", "db.example.com")
    .float("port", 5432.0)
    .text("username", "admin")
    .text("password", "secret")
    .build())?;

// Access nested values
if let Some(db_value) = ctx.get("database") {
    if let Some(obj) = db_value.as_object() {
        if let Some(host) = obj.get("host") {
            println!("Database host: {}", host.as_text().unwrap());
        }
    }
}
```

**Key Points**:
- Use `Object` parameter for structured data
- Nest objects arbitrarily deep
- Access with `as_object()` and HashMap-like API
- Schema defines structure, Context holds values

---

## Recipe 9: Custom Validation Logic

**Use Case**: Implement domain-specific validation rules.

```rust
use paramdef::prelude::*;

#[cfg(feature = "validation")]
mod custom_validation {
    use super::*;
    use paramdef::validation::{Validator, ValidationContext, ValidationResult, ValidationError};

    // Custom validator: check if email domain is allowed
    pub struct AllowedDomainValidator {
        allowed_domains: Vec<String>,
    }

    impl AllowedDomainValidator {
        pub fn new(domains: Vec<&str>) -> Self {
            Self {
                allowed_domains: domains.iter().map(|s| s.to_string()).collect(),
            }
        }
    }

    impl Validator for AllowedDomainValidator {
        fn validate(&self, value: &Value, ctx: &ValidationContext) -> ValidationResult {
            if let Some(email) = value.as_text() {
                if let Some(domain) = email.split('@').nth(1) {
                    if self.allowed_domains.iter().any(|d| d == domain) {
                        return Ok(());
                    }
                    return Err(ValidationError::custom(format!(
                        "Email domain '{}' not allowed. Allowed domains: {}",
                        domain,
                        self.allowed_domains.join(", ")
                    )));
                }
            }
            Err(ValidationError::invalid_format("Invalid email format"))
        }

        fn name(&self) -> &'static str {
            "allowed_domain"
        }
    }

    // Usage
    pub fn example() {
        let validator = AllowedDomainValidator::new(vec!["example.com", "company.com"]);
        
        // Attach to parameter (pseudo-code, actual API may vary)
        let email = Text::email("corporate_email")
            .label("Corporate Email")
            .build();
        
        // Validator would be attached via Rules system
    }
}
```

**Key Points**:
- Implement `Validator` trait for custom logic
- Access context for cross-field validation
- Return descriptive `ValidationError` messages
- Validators are composable and reusable

---

## Recipe 10: Error Recovery Patterns

**Use Case**: Gracefully handle errors and provide recovery paths.

```rust
use paramdef::prelude::*;
use std::sync::Arc;

fn error_recovery_example() -> Result<(), Box<dyn std::error::Error>> {
    let schema = Arc::new(Schema::builder()
        .parameter(Text::builder("name").required().build())
        .parameter(Text::email("email").required().build())
        .build());

    let mut ctx = Context::new(schema);

    // Pattern 1: Try-recover with defaults
    let result = ctx.set("email", Value::text("invalid-email"));
    if let Err(e) = result {
        eprintln!("Failed to set email: {}", e);
        // Recover: use empty value or default
        ctx.set("email", Value::text(""))?;
    }

    // Pattern 2: Partial updates with error collection
    let results = ctx.set_many_partial([
        ("name", Value::text("Alice")),
        ("unknown_field", Value::text("ignored")),  // Will fail
        ("email", Value::text("alice@example.com")),
    ]);

    // Process results
    let mut failed_count = 0;
    for (i, result) in results.iter().enumerate() {
        if let Err(e) = result {
            eprintln!("Field {} failed: {}", i, e);
            failed_count += 1;
        }
    }
    println!("Completed with {} errors (out of {})", failed_count, results.len());

    // Pattern 3: Transaction with retry
    let mut retry_count = 0;
    const MAX_RETRIES: usize = 3;

    loop {
        match ctx.set_many_transactional([
            ("name", Value::text("Alice")),
            ("email", Value::text("alice@example.com")),
        ]) {
            Ok(()) => {
                println!("Transaction succeeded after {} retries", retry_count);
                break;
            }
            Err(e) if retry_count < MAX_RETRIES => {
                eprintln!("Transaction failed: {}. Retrying...", e);
                retry_count += 1;
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            Err(e) => {
                eprintln!("Transaction failed after {} retries: {}", MAX_RETRIES, e);
                return Err(e.into());
            }
        }
    }

    Ok(())
}
```

**Key Points**:
- Use `set_many_partial()` for best-effort updates
- Use `set_many_transactional()` for atomicity
- Collect and log errors for debugging
- Implement retry logic for transient failures

**Error Types**:
- `NotFound` - Key doesn't exist in schema
- `TypeMismatch` - Value type doesn't match expected
- `ValidationFailed` - Value failed validation rules
- `OutOfRange` - Numeric value outside hard constraints

---

## Recipe 11: Performance Optimization Tips

**Use Case**: Maximize performance for high-throughput scenarios.

```rust
use paramdef::prelude::*;
use std::sync::Arc;

fn performance_optimized_example() {
    // Tip 1: Pre-allocate schema with capacity
    let schema = Schema::builder_with_capacity(100)
        // ... add 100 parameters
        .build();

    // Tip 2: Share schema across contexts (Arc is cheap to clone)
    let schema = Arc::new(schema);
    let ctx1 = Context::new(Arc::clone(&schema));
    let ctx2 = Context::new(Arc::clone(&schema));

    // Tip 3: Use zero-copy iterators instead of collect_values()
    let mut ctx = Context::new(schema);
    
    // ❌ Allocates HashMap and clones all values
    let all_values = ctx.collect_values();
    
    // ✅ Zero allocations, returns references
    for (key, value) in ctx.values() {
        // Use references directly
        println!("{}: {:?}", key, value);
    }

    // Tip 4: Use dirty_values() to process only changed fields
    for (key, value) in ctx.dirty_values() {
        // Only modified fields
        println!("Modified: {} = {:?}", key, value);
    }

    // Tip 5: Batch updates for better performance
    // ❌ Slower: Individual calls
    for i in 0..100 {
        ctx.set(&format!("field_{}", i), Value::Int(i as i64)).ok();
    }
    
    // ✅ Faster: Transactional batch (18% faster for 50+ fields)
    let updates: Vec<_> = (0..100)
        .map(|i| (format!("field_{}", i), Value::Int(i as i64)))
        .collect();
    ctx.set_many_transactional(updates).ok();

    // Tip 6: Use get_many() for bulk reads
    let keys = vec!["field_0", "field_1", "field_2"];
    let values = ctx.get_many(&keys);  // Single call, better cache locality

    // Tip 7: Disable events for bulk operations if not needed
    // Events add ~86ns overhead per update
    // Create context without event bus for headless operations
    let fast_ctx = Context::new(Arc::clone(&ctx.schema()));
}
```

**Performance Numbers** (from Phase 5 benchmarks):
- **Transactional updates**: 1.77µs for 8 fields (stack), 22.5µs for 100 fields (heap)
- **Event overhead**: ~86ns per update
- **With events**: 1.71M fields/second
- **Without events**: 4.02M fields/second
- **Batch speedup**: 18% faster for 50+ fields

**Memory Optimization**:
- Small transactions (≤8 fields): Zero heap allocations
- Large transactions (>8 fields): Single heap allocation
- Arc<Value> in events: 66% fewer clones

---

## Recipe 12: Working with Subtypes and Units

**Use Case**: Use semantic subtypes and unit conversions for domain modeling.

```rust
use paramdef::prelude::*;
use paramdef::subtype::{NumberUnit, Distance, Angle, Temperature};

fn subtypes_and_units_example() {
    // Distance parameter with unit conversion
    let height = Number::builder("height")
        .label("Height")
        .subtype(Distance)
        .unit(NumberUnit::Length(LengthUnit::Meter))
        .default(1.75)  // meters
        .build();

    // Port number (specific subtype)
    let port = Number::port("server_port")
        .label("Server Port")
        .default(8080.0)
        .hard_min(1.0)
        .hard_max(65535.0)
        .build();

    // Percentage (0-100 with % display)
    let completion = Number::percentage("progress")
        .label("Completion")
        .default(0.0)
        .soft_max(100.0)
        .build();

    // Factor/multiplier (decimal percentage)
    let scale = Number::builder("scale")
        .label("Scale Factor")
        .subtype(paramdef::subtype::Factor)
        .default(1.0)
        .soft_min(0.1)
        .soft_max(2.0)
        .build();

    // Year (integer year value)
    let birth_year = Number::year("birth_year")
        .label("Birth Year")
        .default(2000.0)
        .soft_min(1900.0)
        .soft_max(2024.0)
        .build();

    // Text subtypes
    let email = Text::email("user_email").build();
    let password = Text::password("user_password").build();
    let phone = Text::phone("phone_number").build();
    let url = Text::url("website").build();
    let slug = Text::slug("post_slug").build();
    let uuid = Text::uuid("record_id").build();

    // Unit conversion (available for compatible units)
    let meters = 100.0;
    if let Some(kilometers) = NumberUnit::Length(LengthUnit::Meter)
        .convert_to(meters, NumberUnit::Length(LengthUnit::Kilometer)) {
        println!("{} meters = {} kilometers", meters, kilometers);
        // 100 meters = 0.1 kilometers
    }
}
```

**Available Subtypes**:

**Number**:
- Generic, Integer, Float, Port, Percentage, Factor, Distance, Angle, Temperature, Year, Rating, Count

**Text**:
- Plain, Email, Password, Phone, URL, Code, Markdown, HTML, JSON, XML, Slug, UUID

**Benefits**:
- Semantic meaning improves code readability
- Built-in validation for common patterns
- UI hints for appropriate input widgets
- Unit conversion for compatible types
- 60 subtypes × 17 unit categories = rich type system

---

## Best Practices Summary

### Schema Design
1. ✅ Use specific subtypes over generic types
2. ✅ Mark required fields explicitly with `.required()`
3. ✅ Provide sensible defaults where applicable
4. ✅ Use soft constraints for UI hints, hard constraints for validation
5. ✅ Share schemas via `Arc` across multiple contexts

### Runtime Operations
6. ✅ Use transactional updates for related fields
7. ✅ Use partial updates for independent fields
8. ✅ Enable events for reactive UIs
9. ✅ Disable events for batch/headless operations
10. ✅ Use zero-copy iterators (`values()`, `dirty_values()`) over `collect_values()`

### Validation
11. ✅ Leverage subtype-based validation (email, URL, etc.)
12. ✅ Implement custom validators for domain logic
13. ✅ Use `ValidationContext` for cross-field validation
14. ✅ Provide helpful error messages

### Performance
15. ✅ Batch updates when possible (18% faster for 50+ fields)
16. ✅ Use stack-optimized transactions for ≤8 fields
17. ✅ Share schemas across contexts
18. ✅ Use `get_many()` for bulk reads

---

## Additional Resources

- **API Documentation**: Run `cargo doc --open --all-features`
- **Architecture Guide**: `docs/01-ARCHITECTURE.md`
- **Type System Reference**: `docs/02-TYPE-SYSTEM.md`
- **Performance Tuning**: `specs/002-code-quality-improvements/BASELINE.md`
- **Examples**: `examples/` directory

---

## Need Help?

If you have questions or need additional recipes:
- 📖 Check the full documentation: `cargo doc --open`
- 💬 File an issue on GitHub
- 🔍 Search examples: `examples/` directory

**Version**: 0.3.1  
**Last Updated**: 2026-01-29
