# Capability: Subtypes and Units System

## ADDED Requirements

### Requirement: TextSubtype Enumeration
The system SHALL provide a TextSubtype enum with 56 semantic variants for text-based parameters, including Generic, SingleLine, MultiLine, RichText, Code, Json, Xml, Yaml, Toml, Email, URL, FilePath, DirectoryPath, Uuid, Slug, Username, Secret, Password, ApiKey, Token, IpAddress, Port, Hostname, Domain, Regex, Color, DateTimeString, DateString, TimeString, DurationString, CronExpression, Locale, Currency, CountryCode, LanguageCode, MimeType, ContentType, UserAgent, Markdown, Html, Css, Sql, JavaScript, TypeScript, Python, Rust, Go, Java, Cpp, Shell, Diff, Log, Csv, and Custom.

#### Scenario: Identify code subtypes
- **WHEN** checking if a TextSubtype is code-related using is_code()
- **THEN** it returns true for Code, Json, Xml, Yaml, JavaScript, TypeScript, Python, Rust, etc.

#### Scenario: Identify sensitive subtypes
- **WHEN** checking if a TextSubtype is sensitive using is_sensitive()
- **THEN** it returns true for Secret, Password, ApiKey, Token

#### Scenario: Get MIME type hints
- **WHEN** calling mime_type_hint() on structured text subtypes
- **THEN** it returns Some("application/json") for Json, Some("text/xml") for Xml, etc.

### Requirement: NumberSubtype Enumeration
The system SHALL provide a NumberSubtype enum with 60 semantic variants for numeric parameters, including Generic, Integer, Float, Decimal, Percentage, Ratio, Count, Index, Id, Port, Distance, Speed, Acceleration, Temperature, Mass, Weight, Volume, Area, Angle, Duration, TimeOffset, Timestamp, Currency, Price, Tax, Discount, Rating, Score, Level, Progress, Capacity, Bandwidth, Frequency, Resolution, Dpi, Brightness, Contrast, Saturation, Hue, Opacity, Latitude, Longitude, Altitude, Depth, Height, Width, Length, Radius, Diameter, Circumference, Filesize, MemorySize, CpuUsage, GpuUsage, NetworkLatency, Timeout, RetryCount, MaxRetries, Threshold, Limit, Offset, PageSize, and Custom.

#### Scenario: Identify integer-only subtypes
- **WHEN** checking if a NumberSubtype requires integers using is_integer_only()
- **THEN** it returns true for Count, Index, Port, RetryCount, PageSize

#### Scenario: Get default units
- **WHEN** calling default_unit() on Distance subtype
- **THEN** it returns Some(Unit::Length(LengthUnit::Meter))

### Requirement: VectorSubtype Enumeration
The system SHALL provide a VectorSubtype enum with 35 semantic variants for fixed-size arrays, including Generic, Vector2, Vector3, Vector4, Position2D, Position3D, Scale2D, Scale3D, Direction2D, Direction3D, Velocity2D, Velocity3D, Acceleration2D, Acceleration3D, ColorRgb, ColorRgba, ColorHsv, ColorHsva, ColorHsl, ColorHsla, Rotation2D, Rotation3D, EulerAngles, Quaternion, Matrix2x2, Matrix3x3, Matrix4x4, Translation2D, Translation3D, UvCoordinate, BoundingBox2D, BoundingBox3D, Padding, Margin, and Custom.

#### Scenario: Get component count
- **WHEN** calling component_count() on Vector3
- **THEN** it returns Some(3)

#### Scenario: Get component names
- **WHEN** calling component_names() on ColorRgba
- **THEN** it returns Some(&["r", "g", "b", "a"])

#### Scenario: Identify color vectors
- **WHEN** checking if a VectorSubtype is a color using is_color()
- **THEN** it returns true for ColorRgb, ColorRgba, ColorHsv, etc.

### Requirement: Unit System with 17 Categories
The system SHALL provide a Unit enum supporting 17 measurement categories: None, Length, Temperature, Time, Mass, Speed, Angle, Volume, Area, Data, Frequency, Energy, Power, Pressure, Percentage, Currency, and Custom.

#### Scenario: Length unit conversions
- **WHEN** converting length units (Meter, Kilometer, Foot, Inch, etc.)
- **THEN** conversion_to_base() returns the factor to base unit (Meter)

#### Scenario: Temperature unit conversions
- **WHEN** converting temperature units (Celsius, Fahrenheit, Kelvin)
- **THEN** conversion_to_base() returns the factor or offset to Kelvin

#### Scenario: No unit required
- **WHEN** using Unit::None
- **THEN** no unit conversion is applied

### Requirement: Subtype Helper Methods
The system SHALL provide helper methods for each subtype enum: TextSubtype::is_code(), is_sensitive(), is_structured(), mime_type_hint(); NumberSubtype::is_integer_only(), is_float_only(), default_unit(); VectorSubtype::component_count(), component_names(), is_color().

#### Scenario: Check if text is structured data
- **WHEN** calling is_structured() on Json, Xml, Yaml, Toml, Csv subtypes
- **THEN** it returns true

#### Scenario: Check vector component count
- **WHEN** calling component_count() on any VectorSubtype
- **THEN** it returns Some(n) for fixed-size vectors, None for custom

### Requirement: Type-Safe Subtype Macros
The system SHALL provide define_number_subtype! and define_vector_subtype! macros for DRY trait-based subtype definitions with compile-time safety.

#### Scenario: Define a number subtype with constraints
- **WHEN** using define_number_subtype! macro to create Distance subtype
- **THEN** it constrains acceptable types to numeric and includes default unit

#### Scenario: Define a vector subtype with size
- **WHEN** using define_vector_subtype! macro to create Position3D subtype
- **THEN** it encodes component count = 3 at the type level

### Requirement: Subtype-Unit Separation
The system SHALL separate semantic meaning (subtype) from measurement system (unit), following the Blender RNA pattern.

#### Scenario: Distance with different units
- **WHEN** creating a Distance parameter with unit Meter
- **THEN** the system allows conversion to Foot, Kilometer, etc. while preserving Distance semantics

#### Scenario: Temperature with different units
- **WHEN** creating a Temperature parameter
- **THEN** it can be displayed in Celsius, Fahrenheit, or Kelvin based on user preference
