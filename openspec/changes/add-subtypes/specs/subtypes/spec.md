## ADDED Requirements

### Requirement: NumberSubtype Trait
The system SHALL provide a `NumberSubtype<T>` trait parameterized by numeric type for compile-time type safety.

#### Scenario: Integer-only subtype with integer type
- **WHEN** `Port` subtype is used with `Number::<u16>`
- **THEN** compilation succeeds

#### Scenario: Integer-only subtype with float type
- **WHEN** `Port` subtype is used with `Number::<f64>`
- **THEN** compilation fails with trait bound error

#### Scenario: Float-only subtype with float type
- **WHEN** `Factor` subtype is used with `Number::<f64>`
- **THEN** compilation succeeds

#### Scenario: Universal subtype with any numeric
- **WHEN** `Distance` subtype is used with any numeric type
- **THEN** compilation succeeds

---

### Requirement: VectorSubtype Trait
The system SHALL provide a `VectorSubtype<const N: usize>` trait parameterized by vector size for compile-time size safety.

#### Scenario: Size-3 subtype with size-3 vector
- **WHEN** `Position3D` subtype is used with `Vector::<f64, 3>`
- **THEN** compilation succeeds

#### Scenario: Size-4 subtype with size-3 vector
- **WHEN** `Quaternion` subtype is used with `Vector::<f64, 3>`
- **THEN** compilation fails with trait bound error

#### Scenario: Size-4 subtype with size-4 vector
- **WHEN** `Quaternion` subtype is used with `Vector::<f64, 4>`
- **THEN** compilation succeeds

---

### Requirement: TextSubtype Trait
The system SHALL provide a `TextSubtype` trait with optional validation pattern and placeholder.

#### Scenario: TextSubtype with pattern
- **WHEN** `Email` subtype pattern() is called
- **THEN** it returns a regex pattern for email validation

#### Scenario: TextSubtype with placeholder
- **WHEN** `Email` subtype placeholder() is called
- **THEN** it returns "user@example.com"

#### Scenario: TextSubtype name
- **WHEN** any TextSubtype name() is called
- **THEN** it returns the subtype identifier string

---

### Requirement: NumberUnit Enum
The system SHALL provide a `NumberUnit` enum for measurement system selection and unit conversion.

#### Scenario: Length unit conversion
- **WHEN** a value in meters is converted to feet
- **THEN** the correct conversion factor is applied

#### Scenario: Temperature unit conversion
- **WHEN** 20°C is converted to Fahrenheit
- **THEN** it returns 68°F

#### Scenario: Display suffix
- **WHEN** a NumberUnit display_suffix is requested
- **THEN** it returns the appropriate unit suffix (m, ft, °C, etc.)

---

### Requirement: IntoBuilder Trait
The system SHALL provide an `IntoBuilder` trait allowing subtypes to be entry points for node creation.

#### Scenario: Subtype as builder entry point
- **WHEN** `Port.builder("http_port")` is called
- **THEN** it returns a NumberBuilder with Port subtype pre-configured

#### Scenario: IntoBuilder with default validation
- **WHEN** `Port.builder("port")` is used
- **THEN** the builder includes range(1, 65535) validator by default

#### Scenario: IntoBuilder defaults can be overridden
- **WHEN** `Port.builder("port").clear_validators().range(80, 443)` is called
- **THEN** only the custom range validator applies

---

### Requirement: Subtype Definition Macros
The system SHALL provide macros for defining subtypes without boilerplate.

#### Scenario: define_number_subtype for integer-only
- **WHEN** a subtype is defined in the int_only block
- **THEN** it implements NumberSubtype for all integer types only

#### Scenario: define_number_subtype for float-only
- **WHEN** a subtype is defined in the float_only block
- **THEN** it implements NumberSubtype for f32 and f64 only

#### Scenario: define_vector_subtype for size constraint
- **WHEN** a subtype is defined in the size_3 block
- **THEN** it implements VectorSubtype<3> only

#### Scenario: define_text_subtype with attributes
- **WHEN** a subtype is defined with pattern and placeholder
- **THEN** the trait methods return those values

---

### Requirement: Standard Number Subtypes
The system SHALL provide standard number subtypes covering common use cases.

#### Scenario: Port subtype exists
- **WHEN** Port subtype is referenced
- **THEN** it is available and integer-only

#### Scenario: Factor subtype exists
- **WHEN** Factor subtype is referenced
- **THEN** it is available and float-only with default range 0.0-1.0

#### Scenario: Percentage subtype exists
- **WHEN** Percentage subtype is referenced
- **THEN** it is available with default range 0-100

---

### Requirement: Standard Vector Subtypes
The system SHALL provide standard vector subtypes for 3D graphics and scientific computing.

#### Scenario: Position3D subtype exists
- **WHEN** Position3D subtype is referenced
- **THEN** it is available for size-3 vectors

#### Scenario: ColorRgba subtype exists
- **WHEN** ColorRgba subtype is referenced
- **THEN** it is available for size-4 vectors with component range 0-1

#### Scenario: Quaternion subtype exists
- **WHEN** Quaternion subtype is referenced
- **THEN** it is available for size-4 vectors with normalization

---

### Requirement: Standard Text Subtypes
The system SHALL provide standard text subtypes for common data formats.

#### Scenario: Email subtype exists
- **WHEN** Email subtype is referenced
- **THEN** it includes email validation pattern

#### Scenario: Url subtype exists
- **WHEN** Url subtype is referenced
- **THEN** it includes URL validation pattern

#### Scenario: Secret subtype exists
- **WHEN** Secret subtype is referenced
- **THEN** it implies SENSITIVE and WRITE_ONLY flags
