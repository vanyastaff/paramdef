# Capability: Leaf Parameter Types

## ADDED Requirements

### Requirement: Text Parameter Type
The system SHALL provide a Text parameter type for string values with TextSubtype, optional min/max length, pattern validation, allowed values, validators, transformers, and visibility conditions.

#### Scenario: Create basic text parameter
- **WHEN** creating Text::builder("name").build()
- **THEN** it creates a text parameter with Generic subtype and no constraints

#### Scenario: Create email parameter
- **WHEN** creating Text::email("email")
- **THEN** it creates a text parameter with Email subtype and email validation

#### Scenario: Validate text length
- **WHEN** setting a value on a Text with min_length(3) and max_length(50)
- **THEN** values shorter than 3 or longer than 50 are rejected

#### Scenario: Apply text transformers
- **WHEN** setting a value on a Text with trim() transformer
- **THEN** leading/trailing whitespace is removed before storage

### Requirement: Number Parameter Type
The system SHALL provide a Number parameter type for numeric values with NumberSubtype, optional unit, range constraints (hard and soft), step, validators, and transformers.

#### Scenario: Create integer parameter
- **WHEN** creating Number::integer("count")
- **THEN** it creates a number parameter accepting only i64 values

#### Scenario: Create float parameter
- **WHEN** creating Number::float("temperature")
- **THEN** it creates a number parameter accepting f64 values

#### Scenario: Apply range constraints
- **WHEN** setting a value on a Number with min(0.0) and max(100.0)
- **THEN** values outside [0.0, 100.0] are rejected (hard constraint)

#### Scenario: Use soft constraints for UI
- **WHEN** a Number has soft_min(0.0) and soft_max(10.0) but hard_max(100.0)
- **THEN** UI slider shows [0, 10] but user can type values up to 100

#### Scenario: Apply unit conversions
- **WHEN** a Number has unit Length(Meter) and user displays in Feet
- **THEN** the system converts the value using the unit conversion factor

### Requirement: Boolean Parameter Type
The system SHALL provide a Boolean parameter type for true/false toggles with default value and optional visibility conditions.

#### Scenario: Create boolean parameter
- **WHEN** creating Boolean::builder("enabled").default(true).build()
- **THEN** it creates a boolean parameter with default value true

#### Scenario: Toggle boolean value
- **WHEN** setting a Boolean value to true then false
- **THEN** both values are accepted without validation

### Requirement: Vector Parameter Type
The system SHALL provide a Vector parameter type for fixed-size numeric arrays with VectorSubtype, component-wise constraints, and validators.

#### Scenario: Create vector3 parameter
- **WHEN** creating Vector::vector3("position").default_vec3([0.0, 0.0, 0.0]).build()
- **THEN** it creates a vector with 3 components of type f64

#### Scenario: Create color RGBA parameter
- **WHEN** creating Vector::color_rgba("color")
- **THEN** it creates a vector with 4 components (r, g, b, a) with default range [0.0, 1.0]

#### Scenario: Validate vector components
- **WHEN** setting a Vector value with fewer or more components than specified
- **THEN** the value is rejected with type mismatch error

#### Scenario: Access component names
- **WHEN** a Vector has ColorRgba subtype
- **THEN** component_names() returns ["r", "g", "b", "a"]

### Requirement: Select Parameter Type (Unified)
The system SHALL provide a unified Select parameter type supporting single and multiple selection modes, static and dynamic option sources, with selectable options containing value, label, icon, and enabled state.

#### Scenario: Create single-select with static options
- **WHEN** creating Select::single("method").options(["GET", "POST", "PUT", "DELETE"]).build()
- **THEN** it creates a single-selection parameter with Value::Text

#### Scenario: Create multi-select with static options
- **WHEN** creating Select::multiple("tags").min(1).max(5).options(["rust", "python", "javascript"]).build()
- **THEN** it creates a multiple-selection parameter with Value::Array

#### Scenario: Create single-select with dynamic options
- **WHEN** creating Select::single("user").dynamic_loader(user_loader).build()
- **THEN** options are loaded asynchronously when needed

#### Scenario: Validate selection constraints
- **WHEN** a multi-select has min(2) and max(4)
- **THEN** selections with fewer than 2 or more than 4 items are rejected

### Requirement: Leaf Parameter Builders
The system SHALL provide fluent builder patterns for all leaf parameter types with chainable methods for metadata, flags, constraints, validators, transformers, and visibility.

#### Scenario: Build text with fluent API
- **WHEN** creating Text::builder("username").label("Username").required().min_length(3).max_length(32).pattern(r"^[a-z0-9_]+$").build()
- **THEN** all constraints are applied to the parameter

#### Scenario: Build number with fluent API
- **WHEN** creating Number::float("price").label("Price").unit(Currency).min(0.0).step(0.01).build()
- **THEN** all metadata and constraints are configured

### Requirement: Leaf Convenience Constructors
The system SHALL provide convenience constructors for common parameter patterns: Text::email(), Text::url(), Text::password(), Number::integer(), Number::float(), Number::percentage(), Vector::vector3(), Vector::color_rgba(), Select::single(), Select::multiple().

#### Scenario: Create email with convenience constructor
- **WHEN** creating Text::email("email")
- **THEN** it automatically sets Email subtype and email validation

#### Scenario: Create percentage with convenience constructor
- **WHEN** creating Number::percentage("completion")
- **THEN** it automatically sets range [0.0, 100.0] and Percentage subtype

### Requirement: Leaf Parameter Validation Integration
The system SHALL integrate validation for all leaf types using the Validatable trait, running transformers before validators, and supporting both sync and async validation.

#### Scenario: Run sync validators
- **WHEN** setting a value on a Text with required() and email() validators
- **THEN** both validators run synchronously before accepting the value

#### Scenario: Run async validators
- **WHEN** setting a value on a Text with async username_available() validator
- **THEN** the validator runs asynchronously after debounce delay

#### Scenario: Transform before validate
- **WHEN** setting a value on a Text with trim() transformer and min_length(3) validator
- **THEN** transformation happens first, then validation on transformed value

### Requirement: Leaf Parameter Type Safety
The system SHALL enforce type safety for all leaf parameters: Text uses String, Number uses i64 or f64, Boolean uses bool, Vector uses Vec<f64>, Select uses String or Vec<String>.

#### Scenario: Type-safe getters
- **WHEN** calling get_value() on a Text
- **THEN** it returns Option<&String> with compile-time type safety

#### Scenario: Type-safe setters
- **WHEN** calling set_value("hello") on a Text
- **THEN** it accepts &str or String with From conversion

#### Scenario: Type mismatch rejection
- **WHEN** calling from_value(Value::Int(42)) on a Text
- **THEN** it returns Err(TypeMismatch) error
