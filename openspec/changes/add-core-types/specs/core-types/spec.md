## ADDED Requirements

### Requirement: Key Type
The system SHALL provide a `Key` type alias for parameter identification using stack-optimized strings.

#### Scenario: Key creation from string literal
- **WHEN** a Key is created from a string literal "my_param"
- **THEN** it stores the value without heap allocation (if <23 bytes)

#### Scenario: Key comparison
- **WHEN** two Keys with identical content are compared
- **THEN** they are equal

#### Scenario: Key display
- **WHEN** a Key is formatted for display
- **THEN** it shows the underlying string value

---

### Requirement: Metadata Structure
The system SHALL provide a `Metadata` struct containing parameter display information.

#### Scenario: Metadata with all fields
- **WHEN** Metadata is created with key, label, description, group, and tags
- **THEN** all fields are accessible via getters

#### Scenario: Metadata with minimal fields
- **WHEN** Metadata is created with only a key
- **THEN** optional fields (label, description, group, tags) default to None/empty

#### Scenario: Metadata builder pattern
- **WHEN** Metadata is built using the builder pattern
- **THEN** it allows fluent configuration of all fields

---

### Requirement: Schema Flags
The system SHALL provide a `Flags` bitflags type for schema-level parameter attributes.

#### Scenario: Required flag
- **WHEN** a parameter has the REQUIRED flag set
- **THEN** it indicates the parameter must have a value

#### Scenario: Readonly flag
- **WHEN** a parameter has the READONLY flag set
- **THEN** it indicates the parameter cannot be modified by users

#### Scenario: Hidden flag
- **WHEN** a parameter has the HIDDEN flag set
- **THEN** it indicates the parameter should not be displayed in UI

#### Scenario: Sensitive flag
- **WHEN** a parameter has the SENSITIVE flag set
- **THEN** it indicates the parameter contains sensitive data (passwords, tokens)

#### Scenario: Flag combinations
- **WHEN** multiple flags are combined using bitwise OR
- **THEN** all specified flags are set

#### Scenario: Flag checking
- **WHEN** a flag set is checked for a specific flag
- **THEN** it correctly reports whether the flag is present

---

### Requirement: Runtime State Flags
The system SHALL provide a `StateFlags` bitflags type for runtime parameter state.

#### Scenario: Dirty state
- **WHEN** a parameter value changes
- **THEN** the DIRTY flag can be set to indicate unsaved changes

#### Scenario: Touched state
- **WHEN** a user interacts with a parameter
- **THEN** the TOUCHED flag can be set to indicate user interaction

#### Scenario: Valid state
- **WHEN** a parameter passes validation
- **THEN** the VALID flag can be set to indicate valid state

#### Scenario: State flag independence
- **WHEN** StateFlags are modified
- **THEN** they do not affect schema-level Flags (they are separate types)

---

### Requirement: Value Enum
The system SHALL provide a `Value` enum as the unified runtime representation for all parameter values.

#### Scenario: Null value
- **WHEN** a Value::Null is created
- **THEN** it represents the absence of a value

#### Scenario: Boolean value
- **WHEN** a Value::Bool is created with true or false
- **THEN** it stores the boolean value

#### Scenario: Integer value
- **WHEN** a Value::Int is created with an i64
- **THEN** it stores the integer value

#### Scenario: Float value
- **WHEN** a Value::Float is created with an f64
- **THEN** it stores the floating-point value

#### Scenario: Text value
- **WHEN** a Value::Text is created with a string
- **THEN** it stores the string using SmartString

#### Scenario: Array value
- **WHEN** a Value::Array is created with a list of Values
- **THEN** it stores them in an Arc<[Value]> for cheap cloning

#### Scenario: Object value
- **WHEN** a Value::Object is created with key-value pairs
- **THEN** it stores them in an Arc<HashMap> for cheap cloning

#### Scenario: Binary value
- **WHEN** a Value::Binary is created with bytes
- **THEN** it stores them in an Arc<[u8]>

#### Scenario: Value type checking
- **WHEN** a Value is queried for its type
- **THEN** helper methods (is_null, is_bool, is_int, etc.) return correct results

#### Scenario: Value conversion
- **WHEN** a Value is converted to a concrete type
- **THEN** typed getters (as_bool, as_int, as_text, etc.) return Option<T>

---

### Requirement: Value JSON Conversion (serde feature)
The system SHALL provide JSON conversion for Value when the serde feature is enabled.

#### Scenario: Value to JSON
- **WHEN** a Value is converted to serde_json::Value
- **THEN** it produces the equivalent JSON representation

#### Scenario: JSON to Value
- **WHEN** a serde_json::Value is converted to Value
- **THEN** it produces the equivalent Value representation

#### Scenario: Value from string parsing
- **WHEN** a JSON string is parsed into a Value
- **THEN** FromStr produces the correct Value

#### Scenario: Value display as JSON
- **WHEN** a Value is formatted with Display
- **THEN** it produces compact JSON output

#### Scenario: Value pretty display
- **WHEN** a Value is formatted with {:#}
- **THEN** it produces pretty-printed JSON output

---

### Requirement: Error Types
The system SHALL provide error types for parameter operations using thiserror.

#### Scenario: Type mismatch error
- **WHEN** a Value is accessed with the wrong type getter
- **THEN** a TypeMismatch error can be created with expected and actual types

#### Scenario: Validation error
- **WHEN** a value fails validation
- **THEN** a ValidationError can be created with code and message

#### Scenario: Error display
- **WHEN** an error is formatted for display
- **THEN** it shows a human-readable message

#### Scenario: Error source chain
- **WHEN** an error wraps another error
- **THEN** the source is accessible via std::error::Error trait
