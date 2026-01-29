# Contract: Builder API Conveniences

**Feature**: 002-code-quality-improvements  
**Modules**: `src/types/leaf/*.rs`, `src/types/container/object.rs`  
**Status**: Phase 1 Design

---

## Overview

This contract defines convenience constructors and builder shortcuts for common parameter patterns, reducing boilerplate by 40-50% for typical use cases.

**Design Principle**: Provide ergonomic shortcuts WITHOUT creating new node types (composition over proliferation).

---

## 1. Required Field Constructors

**Purpose**: Reduce 4-line required field creation to 1 line.

### 1.1 Text::required()

**Signature**:
```rust
impl Text {
    /// Creates a required text field with label.
    ///
    /// Equivalent to:
    /// ```ignore
    /// Text::builder(key)
    ///     .label(label)
    ///     .flags(Flags::REQUIRED)
    ///     .build()
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::types::leaf::Text;
    ///
    /// // Before (4 lines)
    /// let email = Text::builder("email")
    ///     .label("Email Address")
    ///     .flags(Flags::REQUIRED)
    ///     .build();
    ///
    /// // After (1 line)
    /// let email = Text::required("email", "Email Address");
    /// ```
    #[must_use]
    pub fn required(key: impl Into<Key>, label: impl Into<SmartStr>) -> Self {
        Self::builder(key)
            .label(label)
            .flags(Flags::REQUIRED)
            .build()
    }
}
```

**Contract**:
- **Input**: `key` (any type implementing `Into<Key>`), `label` (any type implementing `Into<SmartStr>`)
- **Output**: Fully constructed `Text` node
- **Behavior**: Creates builder, sets label and REQUIRED flag, builds
- **Default Values**: All other fields use builder defaults
- **Performance**: Zero overhead vs manual builder (same code path)

---

### 1.2 Number::required()

**Signature**:
```rust
impl Number {
    /// Creates a required number field with label.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::types::leaf::Number;
    ///
    /// let age = Number::required("age", "Age");
    /// ```
    #[must_use]
    pub fn required(key: impl Into<Key>, label: impl Into<SmartStr>) -> Self {
        Self::builder(key)
            .label(label)
            .flags(Flags::REQUIRED)
            .build()
    }
}
```

---

### 1.3 Boolean::required()

**Signature**:
```rust
impl Boolean {
    /// Creates a required boolean field with label.
    ///
    /// Note: For checkboxes, required typically means "must be checked".
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::types::leaf::Boolean;
    ///
    /// let terms = Boolean::required("accept_terms", "Accept Terms");
    /// ```
    #[must_use]
    pub fn required(key: impl Into<Key>, label: impl Into<SmartStr>) -> Self {
        Self::builder(key)
            .label(label)
            .flags(Flags::REQUIRED)
            .build()
    }
}
```

**Contract (all required() methods)**:
- **Consistency**: Same signature pattern across all leaf types
- **Immutability**: Returned node is immutable (as all schema nodes)
- **Builder Access**: Users can still use `.builder()` for custom configuration
- **No Magic**: Just a shortcut, no hidden behavior

**Before/After Comparison**:
```rust
// Before: 12 lines for 3 required fields
let username = Text::builder("username")
    .label("Username")
    .flags(Flags::REQUIRED)
    .build();

let age = Number::builder("age")
    .label("Age")
    .flags(Flags::REQUIRED)
    .build();

let active = Boolean::builder("active")
    .label("Active")
    .flags(Flags::REQUIRED)
    .build();

// After: 3 lines for 3 required fields
let username = Text::required("username", "Username");
let age = Number::required("age", "Age");
let active = Boolean::required("active", "Active");
```

**Savings**: 75% reduction in lines of code.

---

## 2. Validation Builder Shortcuts

**Purpose**: Fluent API for common validation rules without manual Rules/Expr construction.

**Feature Guard**: Only available with `#[cfg(feature = "validation")]`

### 2.1 TextBuilder Validation Shortcuts

**Signatures**:
```rust
#[cfg(feature = "validation")]
impl<S: TextSubtype> TextBuilder<S> {
    /// Adds required validation rule.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::types::leaf::Text;
    ///
    /// // Before (5+ lines)
    /// let email = Text::builder("email")
    ///     .rules(Rules::from_rules([
    ///         Rule::local(Expr::required()),
    ///     ]))
    ///     .build();
    ///
    /// // After (2 lines)
    /// let email = Text::builder("email")
    ///     .validate_required()
    ///     .build();
    /// ```
    #[must_use]
    pub fn validate_required(self) -> Self {
        self.rules(
            self.rules
                .unwrap_or_default()
                .with_rule(Rule::local(Expr::required()))
        )
    }

    /// Adds email validation rule.
    ///
    /// # Example
    ///
    /// ```
    /// let email = Text::builder("email")
    ///     .validate_required()
    ///     .validate_email()
    ///     .build();
    /// ```
    #[must_use]
    pub fn validate_email(self) -> Self {
        self.rules(
            self.rules
                .unwrap_or_default()
                .with_rule(Rule::local(Expr::email()))
        )
    }

    /// Adds minimum length validation rule.
    ///
    /// # Example
    ///
    /// ```
    /// let password = Text::builder("password")
    ///     .validate_min_length(8)
    ///     .build();
    /// ```
    #[must_use]
    pub fn validate_min_length(self, min: usize) -> Self {
        self.rules(
            self.rules
                .unwrap_or_default()
                .with_rule(Rule::local(Expr::min_length(min)))
        )
    }

    /// Adds maximum length validation rule.
    ///
    /// # Example
    ///
    /// ```
    /// let bio = Text::builder("bio")
    ///     .validate_max_length(500)
    ///     .build();
    /// ```
    #[must_use]
    pub fn validate_max_length(self, max: usize) -> Self {
        self.rules(
            self.rules
                .unwrap_or_default()
                .with_rule(Rule::local(Expr::max_length(max)))
        )
    }

    /// Adds pattern (regex) validation rule.
    ///
    /// # Example
    ///
    /// ```
    /// let phone = Text::builder("phone")
    ///     .validate_pattern(r"^\+?[1-9]\d{1,14}$")
    ///     .build();
    /// ```
    #[must_use]
    pub fn validate_pattern(self, pattern: impl Into<SmartStr>) -> Self {
        self.rules(
            self.rules
                .unwrap_or_default()
                .with_rule(Rule::local(Expr::pattern(pattern)))
        )
    }

    /// Adds URL validation rule.
    #[must_use]
    pub fn validate_url(self) -> Self {
        self.rules(
            self.rules
                .unwrap_or_default()
                .with_rule(Rule::local(Expr::url()))
        )
    }
}
```

**Contract**:
- **Chaining**: All methods return `Self` for fluent chaining
- **Accumulation**: Multiple calls accumulate rules (don't replace)
- **Order Independent**: Rules can be added in any order
- **Builder State**: Rules are stored in builder, compiled on `.build()`

**Before/After Comparison**:
```rust
// Before: Verbose rule construction (10+ lines)
use paramdef::validation::{Rule, Rules};
use paramdef::expr::Expr;

let email = Text::builder("email")
    .label("Email Address")
    .rules(Rules::from_rules([
        Rule::local(Expr::required()),
        Rule::local(Expr::email()),
        Rule::local(Expr::max_length(100)),
    ]))
    .build();

// After: Fluent validation (5 lines)
let email = Text::builder("email")
    .label("Email Address")
    .validate_required()
    .validate_email()
    .validate_max_length(100)
    .build();
```

**Savings**: 50% reduction + improved readability.

---

### 2.2 NumberBuilder Validation Shortcuts

**Signatures**:
```rust
#[cfg(feature = "validation")]
impl<S: NumberSubtype> NumberBuilder<S> {
    /// Adds required validation rule.
    #[must_use]
    pub fn validate_required(self) -> Self {
        self.rules(
            self.rules
                .unwrap_or_default()
                .with_rule(Rule::local(Expr::required()))
        )
    }

    /// Adds minimum value validation rule.
    ///
    /// # Example
    ///
    /// ```
    /// let age = Number::builder("age")
    ///     .validate_min(0.0)
    ///     .validate_max(120.0)
    ///     .build();
    /// ```
    #[must_use]
    pub fn validate_min(self, min: f64) -> Self {
        self.rules(
            self.rules
                .unwrap_or_default()
                .with_rule(Rule::local(Expr::min_value(min)))
        )
    }

    /// Adds maximum value validation rule.
    #[must_use]
    pub fn validate_max(self, max: f64) -> Self {
        self.rules(
            self.rules
                .unwrap_or_default()
                .with_rule(Rule::local(Expr::max_value(max)))
        )
    }

    /// Adds range validation rule (min and max).
    ///
    /// # Example
    ///
    /// ```
    /// let percentage = Number::builder("percentage")
    ///     .validate_range(0.0, 100.0)
    ///     .build();
    /// ```
    #[must_use]
    pub fn validate_range(self, min: f64, max: f64) -> Self {
        self.rules(
            self.rules
                .unwrap_or_default()
                .with_rule(Rule::local(Expr::min_value(min)))
                .with_rule(Rule::local(Expr::max_value(max)))
        )
    }

    /// Adds positive number validation (> 0).
    #[must_use]
    pub fn validate_positive(self) -> Self {
        self.validate_min(0.0)
    }

    /// Adds integer validation (no fractional part).
    ///
    /// Note: This is different from NumberSubtype::Integer.
    /// This validates at runtime, subtype is compile-time.
    #[must_use]
    pub fn validate_integer(self) -> Self {
        self.rules(
            self.rules
                .unwrap_or_default()
                .with_rule(Rule::local(Expr::integer()))
        )
    }
}
```

**Before/After Comparison**:
```rust
// Before: Manual range validation (7+ lines)
let percentage = Number::builder("percentage")
    .label("Percentage")
    .rules(Rules::from_rules([
        Rule::local(Expr::min_value(0.0)),
        Rule::local(Expr::max_value(100.0)),
    ]))
    .build();

// After: Fluent range validation (3 lines)
let percentage = Number::builder("percentage")
    .label("Percentage")
    .validate_range(0.0, 100.0)
    .build();
```

---

### 2.3 BooleanBuilder Validation Shortcuts

**Signatures**:
```rust
#[cfg(feature = "validation")]
impl BooleanBuilder {
    /// Adds required validation rule.
    ///
    /// For booleans, "required" typically means "must be true".
    ///
    /// # Example
    ///
    /// ```
    /// let terms = Boolean::builder("accept_terms")
    ///     .label("Accept Terms")
    ///     .validate_required()
    ///     .build();
    /// ```
    #[must_use]
    pub fn validate_required(self) -> Self {
        self.rules(
            self.rules
                .unwrap_or_default()
                .with_rule(Rule::local(Expr::required()))
        )
    }

    /// Adds "must be true" validation.
    ///
    /// Alias for validate_required() with clearer semantics.
    #[must_use]
    pub fn validate_must_be_true(self) -> Self {
        self.validate_required()
    }
}
```

---

## 3. Object::fields() - Bulk Field Addition

**Purpose**: Add multiple child nodes at once without repeated `.child()` calls.

**Signature**:
```rust
impl ObjectBuilder {
    /// Adds multiple child fields at once.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::types::container::Object;
    /// use paramdef::types::leaf::{Text, Number};
    ///
    /// // Before: Repeated .child() calls (7+ lines)
    /// let user = Object::builder("user")
    ///     .child(Text::required("username", "Username"))
    ///     .child(Text::required("email", "Email"))
    ///     .child(Number::required("age", "Age"))
    ///     .build();
    ///
    /// // After: Single .fields() call (4 lines)
    /// let user = Object::builder("user")
    ///     .fields([
    ///         ("username", Text::required("username", "Username")),
    ///         ("email", Text::required("email", "Email")),
    ///         ("age", Number::required("age", "Age")),
    ///     ])
    ///     .build();
    /// ```
    #[must_use]
    pub fn fields<I, K, N>(mut self, pairs: I) -> Self
    where
        I: IntoIterator<Item = (K, N)>,
        K: Into<Key>,
        N: Node + 'static,
    {
        for (key, node) in pairs {
            let key = key.into();
            // Verify the node's key matches
            debug_assert_eq!(
                node.key(),
                &key,
                "Field key mismatch: expected {:?}, got {:?}",
                key,
                node.key()
            );
            self.children.push(Arc::new(node));
        }
        self
    }
}
```

**Contract**:
- **Input**: Iterator over `(Key, Node)` pairs
- **Output**: Builder with children added
- **Behavior**: 
  - Validates that each node's key matches the provided key (debug mode)
  - Wraps each node in Arc automatically
  - Extends existing children (doesn't replace)
- **Performance**: Same as repeated `.child()` calls
- **Ergonomics**: Cleaner syntax for many fields

**Alternative API** (if key redundancy is unwanted):
```rust
// Option 1: Fields() infers keys from nodes
let user = Object::builder("user")
    .fields_inferred([
        Text::required("username", "Username"),
        Text::required("email", "Email"),
        Number::required("age", "Age"),
    ])
    .build();

// Option 2: Mix with individual .child() calls
let user = Object::builder("user")
    .fields([
        ("username", Text::required("username", "Username")),
        ("email", Text::required("email", "Email")),
    ])
    .child(conditional_field()) // Mix and match
    .build();
```

**Decision**: Provide both `.fields()` (explicit keys) and `.fields_inferred()` (infer from nodes).

```rust
impl ObjectBuilder {
    /// Adds multiple child fields, inferring keys from nodes.
    ///
    /// # Example
    ///
    /// ```
    /// let user = Object::builder("user")
    ///     .fields_inferred([
    ///         Text::required("username", "Username"),
    ///         Text::required("email", "Email"),
    ///     ])
    ///     .build();
    /// ```
    #[must_use]
    pub fn fields_inferred<I, N>(mut self, nodes: I) -> Self
    where
        I: IntoIterator<Item = N>,
        N: Node + 'static,
    {
        for node in nodes {
            self.children.push(Arc::new(node));
        }
        self
    }
}
```

---

## 4. Combined Examples

### Example 1: Registration Form

**Before (verbose - 30+ lines)**:
```rust
use paramdef::schema::Schema;
use paramdef::types::leaf::{Text, Number, Boolean};
use paramdef::validation::{Rule, Rules};
use paramdef::expr::Expr;

let schema = Schema::builder()
    .parameter(
        Text::builder("username")
            .label("Username")
            .flags(Flags::REQUIRED)
            .rules(Rules::from_rules([
                Rule::local(Expr::required()),
                Rule::local(Expr::min_length(3)),
                Rule::local(Expr::max_length(20)),
            ]))
            .build()
    )
    .parameter(
        Text::builder("email")
            .label("Email")
            .flags(Flags::REQUIRED)
            .rules(Rules::from_rules([
                Rule::local(Expr::required()),
                Rule::local(Expr::email()),
            ]))
            .build()
    )
    .parameter(
        Number::builder("age")
            .label("Age")
            .flags(Flags::REQUIRED)
            .rules(Rules::from_rules([
                Rule::local(Expr::required()),
                Rule::local(Expr::min_value(18.0)),
            ]))
            .build()
    )
    .parameter(
        Boolean::builder("accept_terms")
            .label("Accept Terms")
            .flags(Flags::REQUIRED)
            .rules(Rules::from_rules([
                Rule::local(Expr::required()),
            ]))
            .build()
    )
    .build();
```

**After (concise - 12 lines)**:
```rust
use paramdef::schema::Schema;
use paramdef::types::leaf::{Text, Number, Boolean};

let schema = Schema::builder()
    .parameter(
        Text::required("username", "Username")
            .validate_min_length(3)
            .validate_max_length(20)
    )
    .parameter(
        Text::required("email", "Email")
            .validate_email()
    )
    .parameter(
        Number::required("age", "Age")
            .validate_min(18.0)
    )
    .parameter(
        Boolean::required("accept_terms", "Accept Terms")
    )
    .build();
```

**Savings**: 60% reduction (30 → 12 lines).

---

### Example 2: API Configuration Object

**Before (25+ lines)**:
```rust
let api_config = Object::builder("api_config")
    .label("API Configuration")
    .child(
        Text::builder("endpoint")
            .label("Endpoint URL")
            .flags(Flags::REQUIRED)
            .build()
    )
    .child(
        Number::builder("timeout")
            .label("Timeout (seconds)")
            .default(30.0)
            .build()
    )
    .child(
        Number::builder("retries")
            .label("Max Retries")
            .default(3.0)
            .build()
    )
    .child(
        Boolean::builder("verify_ssl")
            .label("Verify SSL")
            .default(true)
            .build()
    )
    .build();
```

**After (9 lines)**:
```rust
let api_config = Object::builder("api_config")
    .label("API Configuration")
    .fields_inferred([
        Text::required("endpoint", "Endpoint URL"),
        Number::builder("timeout").label("Timeout (seconds)").default(30.0).build(),
        Number::builder("retries").label("Max Retries").default(3.0).build(),
        Boolean::builder("verify_ssl").label("Verify SSL").default(true).build(),
    ])
    .build();
```

**Savings**: 64% reduction (25 → 9 lines).

---

## API Consistency Table

| Method | Module | Feature Gate | Returns | Purpose |
|--------|--------|--------------|---------|---------|
| `Text::required()` | leaf::text | none | `Text` | Required text field |
| `Number::required()` | leaf::number | none | `Number` | Required number field |
| `Boolean::required()` | leaf::boolean | none | `Boolean` | Required boolean field |
| `.validate_required()` | All builders | validation | `Self` | Add required rule |
| `.validate_email()` | TextBuilder | validation | `Self` | Add email rule |
| `.validate_min_length()` | TextBuilder | validation | `Self` | Add min length rule |
| `.validate_max_length()` | TextBuilder | validation | `Self` | Add max length rule |
| `.validate_pattern()` | TextBuilder | validation | `Self` | Add regex rule |
| `.validate_url()` | TextBuilder | validation | `Self` | Add URL rule |
| `.validate_min()` | NumberBuilder | validation | `Self` | Add min value rule |
| `.validate_max()` | NumberBuilder | validation | `Self` | Add max value rule |
| `.validate_range()` | NumberBuilder | validation | `Self` | Add min+max rules |
| `.validate_positive()` | NumberBuilder | validation | `Self` | Add >0 rule |
| `.validate_integer()` | NumberBuilder | validation | `Self` | Add integer rule |
| `.validate_must_be_true()` | BooleanBuilder | validation | `Self` | Add true rule |
| `.fields()` | ObjectBuilder | none | `Self` | Add multiple children |
| `.fields_inferred()` | ObjectBuilder | none | `Self` | Add children (infer keys) |

---

## Testing Requirements

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_required() {
        let field = Text::required("username", "Username");
        assert_eq!(field.key().as_str(), "username");
        assert_eq!(field.metadata().label(), Some("Username"));
        assert!(field.flags().contains(Flags::REQUIRED));
    }

    #[test]
    #[cfg(feature = "validation")]
    fn test_validate_email() {
        let field = Text::builder("email")
            .validate_required()
            .validate_email()
            .build();
        
        // Verify rules are added
        let rules = field.rules().unwrap();
        assert_eq!(rules.len(), 2);
    }

    #[test]
    #[cfg(feature = "validation")]
    fn test_validate_range() {
        let field = Number::builder("age")
            .validate_range(0.0, 120.0)
            .build();
        
        let rules = field.rules().unwrap();
        assert_eq!(rules.len(), 2); // min and max
    }

    #[test]
    fn test_object_fields() {
        let obj = Object::builder("user")
            .fields_inferred([
                Text::required("name", "Name"),
                Number::required("age", "Age"),
            ])
            .build();
        
        assert_eq!(obj.children().len(), 2);
    }
}
```

### Integration Tests
```rust
// tests/builder_shortcuts.rs

#[test]
fn test_full_form_with_shortcuts() {
    let schema = Schema::builder()
        .parameter(
            Object::builder("registration")
                .fields_inferred([
                    Text::required("username", "Username")
                        .validate_min_length(3),
                    Text::required("email", "Email")
                        .validate_email(),
                    Number::required("age", "Age")
                        .validate_range(18.0, 120.0),
                ])
                .build()
        )
        .build();
    
    let mut ctx = Context::from_schema(schema);
    
    // Validate that required fields work
    assert!(ctx.validate_all().is_err()); // Missing values
    
    ctx.set("registration.username", Value::text("alice"));
    ctx.set("registration.email", Value::text("alice@example.com"));
    ctx.set("registration.age", Value::Float(25.0));
    
    assert!(ctx.validate_all().is_ok());
}
```

---

## Documentation Requirements

### Module-Level Documentation
Each builder module should document the shortcuts:

```rust
//! # Validation Shortcuts
//!
//! When the `validation` feature is enabled, builders provide convenient
//! methods for adding common validation rules:
//!
//! ```
//! let email = Text::builder("email")
//!     .validate_required()
//!     .validate_email()
//!     .validate_max_length(100)
//!     .build();
//! ```
//!
//! These are equivalent to manually constructing Rules:
//!
//! ```ignore
//! let email = Text::builder("email")
//!     .rules(Rules::from_rules([
//!         Rule::local(Expr::required()),
//!         Rule::local(Expr::email()),
//!         Rule::local(Expr::max_length(100)),
//!     ]))
//!     .build();
//! ```
```

### Method Documentation
All methods must have:
- Summary line
- Example showing before/after
- Link to equivalent manual construction (for transparency)

---

## Migration Path

**No Breaking Changes**: All methods are additive.

**Discovery**:
- IDE autocomplete will suggest shortcuts
- Documentation shows both approaches
- Examples updated to use shortcuts (with comments showing old way)

**Adoption**:
- Users can migrate incrementally
- Old verbose style still works (no deprecation)
- New projects naturally use shortcuts

---

**Document Status**: Complete  
**Next Steps**: Implement in respective builder modules
