## ADDED Requirements

### Requirement: Visibility Trait
The system SHALL provide a `Visibility` trait for all 14 node types (feature = "visibility").

#### Scenario: Visibility trait methods
- **WHEN** Visibility trait is implemented
- **THEN** it provides visibility(), set_visibility(), is_visible(), dependencies()

#### Scenario: All nodes implement Visibility
- **WHEN** the visibility feature is enabled
- **THEN** all 14 node types implement Visibility

---

### Requirement: Expr Visibility Expression
The system SHALL provide an `Expr` enum for visibility conditions that evaluate to bool.

#### Scenario: Expr::Eq comparison
- **WHEN** Expr::Eq("mode", Value::text("advanced")) is evaluated
- **THEN** it returns true if mode equals "advanced"

#### Scenario: Expr::Ne comparison
- **WHEN** Expr::Ne("status", Value::text("disabled")) is evaluated
- **THEN** it returns true if status is not "disabled"

#### Scenario: Expr::IsTrue
- **WHEN** Expr::IsTrue("enabled") is evaluated
- **THEN** it returns true if enabled equals true

#### Scenario: Expr::IsSet
- **WHEN** Expr::IsSet("optional_field") is evaluated
- **THEN** it returns true if the field is not null

#### Scenario: Expr::IsEmpty
- **WHEN** Expr::IsEmpty("list") is evaluated
- **THEN** it returns true if list is "", [], or {}

#### Scenario: Expr::Lt numeric comparison
- **WHEN** Expr::Lt("count", 10.0) is evaluated
- **THEN** it returns true if count < 10

#### Scenario: Expr::And combinator
- **WHEN** Expr::And([expr1, expr2]) is evaluated
- **THEN** it returns true only if all expressions are true

#### Scenario: Expr::Or combinator
- **WHEN** Expr::Or([expr1, expr2]) is evaluated
- **THEN** it returns true if any expression is true

#### Scenario: Expr::Not inversion
- **WHEN** Expr::Not(expr) is evaluated
- **THEN** it returns the inverse of expr

#### Scenario: Expr::OneOf set membership
- **WHEN** Expr::OneOf("method", ["GET", "POST"]) is evaluated
- **THEN** it returns true if method is in the set

#### Scenario: Expr::IsValid validation state
- **WHEN** Expr::IsValid("email") is evaluated
- **THEN** it returns true if email passed validation

---

### Requirement: Expr Dependencies
The system SHALL track which keys an Expr depends on.

#### Scenario: Simple dependency
- **WHEN** Expr::Eq("mode", value).dependencies() is called
- **THEN** it returns ["mode"]

#### Scenario: Compound dependency
- **WHEN** Expr::And([Eq("a", v1), Eq("b", v2)]).dependencies() is called
- **THEN** it returns ["a", "b"]

---

### Requirement: VisibilityObserver
The system SHALL provide a `VisibilityObserver` for reactive visibility updates.

#### Scenario: Register expression
- **WHEN** observer.register(key, expr) is called
- **THEN** the expression is tracked for the key

#### Scenario: Evaluate on change
- **WHEN** a dependency value changes
- **THEN** dependent visibility expressions are re-evaluated

#### Scenario: Emit visibility events
- **WHEN** visibility changes
- **THEN** VisibilityChanged event is emitted

---

### Requirement: Validatable Trait
The system SHALL provide a `Validatable` trait for nodes with values (Container + Leaf).

#### Scenario: Validatable for Leaf
- **WHEN** Text, Number, Boolean, Vector, Select nodes exist
- **THEN** they implement Validatable

#### Scenario: Validatable for Container
- **WHEN** Object, List, Mode, Routing, Expirable nodes exist
- **THEN** they implement Validatable

#### Scenario: Group/Layout/Decoration not Validatable
- **WHEN** Group, Panel, Notice nodes exist
- **THEN** they do NOT implement Validatable (no own value)

---

### Requirement: Validator Trait
The system SHALL provide a `Validator` trait for synchronous validation.

#### Scenario: Validator validate method
- **WHEN** validator.validate(value) is called
- **THEN** it returns Result<(), ValidationError>

#### Scenario: Closure as Validator
- **WHEN** a closure matching the signature is used
- **THEN** it automatically implements Validator

---

### Requirement: AsyncValidator Trait
The system SHALL provide an `AsyncValidator` trait for async validation.

#### Scenario: AsyncValidator validate method
- **WHEN** validator.validate(value).await is called
- **THEN** it returns Result<(), ValidationError>

---

### Requirement: ValidationConfig
The system SHALL provide a `ValidationConfig` for per-parameter validation setup.

#### Scenario: ValidationConfig with sync validators
- **WHEN** ValidationConfig contains sync validators
- **THEN** they run in order

#### Scenario: ValidationConfig with async validators
- **WHEN** ValidationConfig contains async validators
- **THEN** they run after sync validators

#### Scenario: ValidationConfig debounce
- **WHEN** debounce_ms is set
- **THEN** async validation is debounced

---

### Requirement: Built-in Validators
The system SHALL provide built-in validators for common patterns.

#### Scenario: Required validator
- **WHEN** required() validator is used
- **THEN** it fails for null/empty values

#### Scenario: MinLength validator
- **WHEN** min_length(3) validator is used
- **THEN** it fails for strings shorter than 3

#### Scenario: MaxLength validator
- **WHEN** max_length(100) validator is used
- **THEN** it fails for strings longer than 100

#### Scenario: Range validator
- **WHEN** range(0, 100) validator is used
- **THEN** it fails for numbers outside 0-100

#### Scenario: Pattern validator
- **WHEN** pattern(regex) validator is used
- **THEN** it fails for strings not matching

#### Scenario: Email validator
- **WHEN** email() validator is used
- **THEN** it validates email format

#### Scenario: Url validator
- **WHEN** url() validator is used
- **THEN** it validates URL format

---

### Requirement: CrossValidator Trait
The system SHALL provide a `CrossValidator` trait for multi-parameter validation.

#### Scenario: CrossValidator depends_on
- **WHEN** cross_validator.depends_on() is called
- **THEN** it returns the list of dependent keys

#### Scenario: CrossValidator validate
- **WHEN** cross_validator.validate(context) is called
- **THEN** it validates relationships between parameters

---

### Requirement: Validation Integration
The system SHALL integrate validation with Context and events.

#### Scenario: Validate on set_value
- **WHEN** context.set_value() is called
- **THEN** sync validation runs automatically

#### Scenario: Validation events
- **WHEN** validation runs
- **THEN** ValidationStarted and ValidationPassed/Failed events emit

#### Scenario: Validation state flag
- **WHEN** validation passes
- **THEN** StateFlags::VALID is set
