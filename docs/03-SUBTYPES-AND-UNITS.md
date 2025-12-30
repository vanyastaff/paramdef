# Subtypes and Units

Complete reference for node subtypes and the unit system.

---

## Type-Safe Subtype System

paramdef uses **trait-based subtypes** with compile-time safety:

```rust
// ✓ Compiles - Port is valid for integers
Number::<i64>::builder("port").subtype(Port).build();

// ✓ Compiles - Factor is valid for floats
Number::<f64>::builder("opacity").subtype(Factor).build();

// ✓ Compiles - Position3D requires size 3
Vector::<f64, 3>::builder("pos").subtype(Position3D).build();

// ✗ ERROR: Port doesn't implement NumberSubtype<f64>
Number::<f64>::builder("port").subtype(Port).build();

// ✗ ERROR: Quaternion requires size 4, not 3
Vector::<f64, 3>::builder("rot").subtype(Quaternion).build();
```

---

## Trait Definitions

```rust
/// Trait for Number subtypes - parameterized by numeric type
pub trait NumberSubtype<T: Numeric> {
    /// Default range for this subtype (if any)
    fn default_range() -> Option<(T, T)> { None }
    
    /// Semantic name for display
    fn name() -> &'static str;
}

/// Trait for Vector subtypes - parameterized by size
pub trait VectorSubtype<const N: usize> {
    /// Semantic name for display
    fn name() -> &'static str;
    
    /// Default component range (if any)
    fn default_range() -> Option<(f64, f64)> { None }
}

/// Trait for Text subtypes - no type parameter (all work with String)
pub trait TextSubtype {
    /// Semantic name for display
    fn name() -> &'static str;
    
    /// Validation pattern (if any)
    fn pattern() -> Option<&'static str> { None }
    
    /// Placeholder text
    fn placeholder() -> Option<&'static str> { None }
}
```

---

## Subtype Macros

Macros eliminate boilerplate when defining subtypes:

```rust
// === Number Subtypes ===
define_number_subtype! {
    // Integer-only subtypes
    int_only {
        /// Network port (0-65535)
        Port,
        /// Item count (non-negative integer)
        Count,
        /// Star rating (typically 0-5)
        Rating,
        /// Byte count
        ByteCount,
        /// Index into a collection
        Index,
    }
    
    // Float-only subtypes
    float_only {
        /// Normalized factor (0.0-1.0)
        Factor,
        /// Percentage (0-100)
        Percentage,
        /// Angle in degrees
        Angle,
        /// Angle in radians
        AngleRadians,
    }
    
    // Universal subtypes (any numeric type)
    any {
        /// Linear distance
        Distance,
        /// Time duration
        Duration,
        /// Temperature value
        Temperature,
        /// Currency/money value
        Currency,
        /// Speed/velocity magnitude
        Speed,
        /// Mass/weight
        Mass,
        /// Generic numeric value
        Generic,
    }
}

// === Vector Subtypes ===
define_vector_subtype! {
    // Size 2
    size_2 {
        /// 2D position (XY)
        Position2D,
        /// 2D size (width, height)
        Size2D,
        /// Texture coordinates
        Uv,
        /// Latitude/Longitude
        LatLong,
        /// Min/Max range
        MinMax,
        /// 2D direction (normalized)
        Direction2D,
        /// 2D scale factors
        Scale2D,
        /// Generic 2D vector
        Vector2,
    }
    
    // Size 3
    size_3 {
        /// 3D position (XYZ)
        Position3D,
        /// 3D direction (normalized)
        Direction3D,
        /// Surface normal
        Normal,
        /// 3D scale factors
        Scale3D,
        /// Translation vector
        Translation,
        /// Euler angles (pitch, yaw, roll)
        Euler,
        /// Euler angles in radians
        EulerRadians,
        /// RGB color (0-1)
        ColorRgb,
        /// HSV color
        ColorHsv,
        /// HSL color
        ColorHsl,
        /// Linear RGB (for rendering)
        ColorLinear,
        /// Velocity vector
        Velocity,
        /// Acceleration vector
        Acceleration,
        /// Force vector
        Force,
        /// 3D texture coordinates
        Uvw,
        /// Latitude/Longitude/Altitude
        LatLongAlt,
        /// Generic 3D vector
        Vector3,
    }
    
    // Size 4
    size_4 {
        /// Quaternion rotation (XYZW)
        Quaternion,
        /// Axis-angle rotation
        AxisAngle,
        /// RGBA color (0-1)
        ColorRgba,
        /// sRGB with gamma
        ColorGamma,
        /// 2D bounds (minX, minY, maxX, maxY)
        Bounds2D,
        /// Generic 4D vector
        Vector4,
    }
    
    // Size 6
    size_6 {
        /// 3D bounds/AABB (minX, minY, minZ, maxX, maxY, maxZ)
        Bounds3D,
    }
    
    // Size 9
    size_9 {
        /// 3x3 matrix (row-major)
        Matrix3x3,
    }
    
    // Size 16
    size_16 {
        /// 4x4 transformation matrix (row-major)
        Matrix4x4,
    }
}

// === Text Subtypes ===
define_text_subtype! {
    // Basic
    Plain,
    MultiLine,
    
    // Network
    Email { pattern: r"^[^\s@]+@[^\s@]+\.[^\s@]+$", placeholder: "user@example.com" },
    Url { pattern: r"^https?://", placeholder: "https://example.com" },
    Domain { placeholder: "example.com" },
    IpAddressV4 { pattern: r"^\d{1,3}(\.\d{1,3}){3}$", placeholder: "192.168.1.1" },
    IpAddressV6 { placeholder: "2001:0db8::1" },
    Hostname { placeholder: "server.example.com" },
    
    // Paths
    FilePath { placeholder: "/path/to/file.txt" },
    DirPath { placeholder: "/path/to/dir" },
    FileName { placeholder: "document.pdf" },
    
    // Security
    Secret,
    Password,
    ApiKey,
    BearerToken,
    
    // Identifiers
    Uuid { pattern: r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$" },
    Slug { pattern: r"^[a-z0-9]+(?:-[a-z0-9]+)*$", placeholder: "my-blog-post" },
    
    // Date/Time (ISO 8601)
    DateTime { placeholder: "2024-01-15T14:30:00Z" },
    Date { placeholder: "2024-01-15" },
    Time { placeholder: "14:30:00" },
    
    // Structured Data
    Json,
    Yaml,
    Toml,
    Xml,
    
    // Code
    Code(CodeLanguage),
    Sql,
    Regex,
    Expression,
}
```

---

## Macro Implementation

```rust
/// Define number subtypes with compile-time type constraints
#[macro_export]
macro_rules! define_number_subtype {
    // Integer-only subtypes
    (int_only { $($(#[$meta:meta])* $name:ident),* $(,)? }) => {
        $(
            $(#[$meta])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name;
            
            impl NumberSubtype<i8> for $name {}
            impl NumberSubtype<i16> for $name {}
            impl NumberSubtype<i32> for $name {}
            impl NumberSubtype<i64> for $name {}
            impl NumberSubtype<i128> for $name {}
            impl NumberSubtype<u8> for $name {}
            impl NumberSubtype<u16> for $name {}
            impl NumberSubtype<u32> for $name {}
            impl NumberSubtype<u64> for $name {}
            impl NumberSubtype<u128> for $name {}
            impl NumberSubtype<isize> for $name {}
            impl NumberSubtype<usize> for $name {}
        )*
    };
    
    // Float-only subtypes
    (float_only { $($(#[$meta:meta])* $name:ident),* $(,)? }) => {
        $(
            $(#[$meta])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name;
            
            impl NumberSubtype<f32> for $name {}
            impl NumberSubtype<f64> for $name {}
        )*
    };
    
    // Universal subtypes (any numeric type)
    (any { $($(#[$meta:meta])* $name:ident),* $(,)? }) => {
        $(
            $(#[$meta])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name;
            
            impl<T: Numeric> NumberSubtype<T> for $name {}
        )*
    };
}

/// Define vector subtypes with compile-time size constraints
#[macro_export]
macro_rules! define_vector_subtype {
    (size_2 { $($(#[$meta:meta])* $name:ident),* $(,)? }) => {
        $(
            $(#[$meta])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name;
            
            impl VectorSubtype<2> for $name {}
        )*
    };
    
    (size_3 { $($(#[$meta:meta])* $name:ident),* $(,)? }) => {
        $(
            $(#[$meta])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name;
            
            impl VectorSubtype<3> for $name {}
        )*
    };
    
    (size_4 { $($(#[$meta:meta])* $name:ident),* $(,)? }) => {
        $(
            $(#[$meta])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name;
            
            impl VectorSubtype<4> for $name {}
        )*
    };
    
    (size_6 { $($(#[$meta:meta])* $name:ident),* $(,)? }) => {
        $(
            $(#[$meta])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name;
            
            impl VectorSubtype<6> for $name {}
        )*
    };
    
    (size_9 { $($(#[$meta:meta])* $name:ident),* $(,)? }) => {
        $(
            $(#[$meta])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name;
            
            impl VectorSubtype<9> for $name {}
        )*
    };
    
    (size_16 { $($(#[$meta:meta])* $name:ident),* $(,)? }) => {
        $(
            $(#[$meta])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name;
            
            impl VectorSubtype<16> for $name {}
        )*
    };
}

/// Define text subtypes (all work with String)
#[macro_export]
macro_rules! define_text_subtype {
    ($($name:ident $({ $($field:ident : $value:expr),* $(,)? })?),* $(,)?) => {
        $(
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub struct $name;
            
            impl TextSubtype for $name {
                fn name() -> &'static str { stringify!($name) }
                
                $(
                    $(
                        define_text_subtype!(@field $field $value);
                    )*
                )?
            }
        )*
    };
    
    (@field pattern $value:expr) => {
        fn pattern() -> Option<&'static str> { Some($value) }
    };
    
    (@field placeholder $value:expr) => {
        fn placeholder() -> Option<&'static str> { Some($value) }
    };
}
```

---

## Subtype + Unit Pattern

Inspired by Blender, paramdef separates **semantic meaning** (subtype) from **measurement system** (unit):

```rust
Number::<f64>::builder("height")
    .subtype(Distance)           // WHAT it represents
    .unit(NumberUnit::Length)    // HOW to measure/display
    .build()
```

**Benefits:**
- Automatic unit conversion (meters <-> feet)
- User preference support (metric vs imperial)
- Localization-friendly display
- Compile-time type safety

---

## Composition Table

All specialized behaviors are built from **Base Type + Subtype + Flags**:

### Text-Based Specializations

| Specialization | Base | Subtype | Flags |
|----------------|------|---------|-------|
| Password | Text | Password | SENSITIVE, WRITE_ONLY |
| API Key | Text | ApiKey | SENSITIVE, WRITE_ONLY |
| Email | Text | Email | - |
| URL | Text | Url | - |
| File Path | Text | FilePath | - |
| DateTime | Text | DateTime | - |
| Code (Python) | Text | Code(Python) | - |
| JSON Editor | Text | Json | - |

### Number-Based Specializations

| Specialization | Base | Subtype | Unit | Constraints |
|----------------|------|---------|------|-------------|
| Network Port | Number<u16> | Port | - | 0-65535 |
| Percentage | Number<f64> | Percentage | - | 0-100 |
| Rating | Number<i32> | Rating | - | 0-5 |
| Temperature | Number<f64> | Temperature | Temperature | - |
| Distance | Number<f64> | Distance | Length | - |
| Angle | Number<f64> | Angle | Rotation | - |

### Vector-Based Specializations

| Specialization | Base | Subtype |
|----------------|------|---------|
| Color RGB | Vector<f64, 3> | ColorRgb |
| Color RGBA | Vector<f64, 4> | ColorRgba |
| Position 2D | Vector<f64, 2> | Position2D |
| Position 3D | Vector<f64, 3> | Position3D |
| Euler Angles | Vector<f64, 3> | Euler |
| Quaternion | Vector<f64, 4> | Quaternion |

---

## NumberSubtype Reference

### Integer-Only (impl for i8..i128, u8..u128)

| Subtype | Description | Typical Range |
|---------|-------------|---------------|
| `Port` | Network port | 0-65535 |
| `Count` | Item count | >= 0 |
| `Rating` | Star rating | 0-5 |
| `ByteCount` | Byte count | >= 0 |
| `Index` | Array index | >= 0 |

### Float-Only (impl for f32, f64)

| Subtype | Description | Typical Range |
|---------|-------------|---------------|
| `Factor` | Normalized value | 0.0-1.0 |
| `Percentage` | Percentage | 0-100 |
| `Angle` | Angle in degrees | 0-360 |
| `AngleRadians` | Angle in radians | 0-2π |

### Universal (impl for all Numeric)

| Subtype | Description | Unit |
|---------|-------------|------|
| `Distance` | Linear distance | Length |
| `Duration` | Time span | Time |
| `Temperature` | Temperature | Temperature |
| `Currency` | Money value | - |
| `Speed` | Velocity magnitude | - |
| `Mass` | Mass/weight | Mass |
| `Generic` | No semantic meaning | - |

---

## VectorSubtype Reference

### Size 2

| Subtype | Description |
|---------|-------------|
| `Position2D` | 2D position (XY) |
| `Size2D` | Width/height |
| `Uv` | Texture coordinates |
| `LatLong` | Latitude/Longitude |
| `MinMax` | Min/Max range |
| `Direction2D` | 2D direction (normalized) |
| `Scale2D` | 2D scale factors |
| `Vector2` | Generic 2D vector |

### Size 3

| Subtype | Description |
|---------|-------------|
| `Position3D` | 3D position (XYZ) |
| `Direction3D` | 3D direction (normalized) |
| `Normal` | Surface normal |
| `Scale3D` | 3D scale factors |
| `Translation` | Translation vector |
| `Euler` | Euler angles (degrees) |
| `EulerRadians` | Euler angles (radians) |
| `ColorRgb` | RGB color (0-1) |
| `ColorHsv` | HSV color |
| `ColorHsl` | HSL color |
| `ColorLinear` | Linear RGB |
| `Velocity` | Velocity vector |
| `Acceleration` | Acceleration vector |
| `Force` | Force vector |
| `Uvw` | 3D texture coordinates |
| `LatLongAlt` | Lat/Long/Altitude |
| `Vector3` | Generic 3D vector |

### Size 4

| Subtype | Description |
|---------|-------------|
| `Quaternion` | Quaternion rotation (XYZW) |
| `AxisAngle` | Axis-angle rotation |
| `ColorRgba` | RGBA color (0-1) |
| `ColorGamma` | sRGB with gamma |
| `Bounds2D` | 2D bounds (minX, minY, maxX, maxY) |
| `Vector4` | Generic 4D vector |

### Size 6

| Subtype | Description |
|---------|-------------|
| `Bounds3D` | 3D AABB (min XYZ, max XYZ) |

### Size 9 / 16

| Subtype | Description |
|---------|-------------|
| `Matrix3x3` | 3x3 matrix (row-major) |
| `Matrix4x4` | 4x4 transform matrix |

---

## TextSubtype Reference

### Basic Text

| Subtype | Description |
|---------|-------------|
| `Plain` | Generic text |
| `MultiLine` | Multi-line text area |

### Network & Identifiers

| Subtype | Validation | Placeholder |
|---------|------------|-------------|
| `Email` | RFC 5322 | `user@example.com` |
| `Url` | URL format | `https://example.com` |
| `Domain` | Domain format | `example.com` |
| `IpAddressV4` | IPv4 pattern | `192.168.1.1` |
| `IpAddressV6` | IPv6 format | `2001:0db8::1` |
| `Hostname` | Hostname format | `server.example.com` |

### Paths & Files

| Subtype | Placeholder |
|---------|-------------|
| `FilePath` | `/path/to/file.txt` |
| `DirPath` | `/path/to/dir` |
| `FileName` | `document.pdf` |

### Security

| Subtype | Behavior |
|---------|----------|
| `Secret` | Masked + SENSITIVE flag |
| `Password` | Masked display |
| `ApiKey` | Masked + validation |
| `BearerToken` | Masked |

### Identifiers

| Subtype | Validation | Placeholder |
|---------|------------|-------------|
| `Uuid` | UUID format | `123e4567-e89b-...` |
| `Slug` | URL slug pattern | `my-blog-post` |

### Date/Time (ISO 8601)

| Subtype | Placeholder |
|---------|-------------|
| `DateTime` | `2024-01-15T14:30:00Z` |
| `Date` | `2024-01-15` |
| `Time` | `14:30:00` |

### Structured Data

| Subtype | Syntax Highlighting |
|---------|---------------------|
| `Json` | Yes |
| `Yaml` | Yes |
| `Toml` | Yes |
| `Xml` | Yes |

### Code & Expressions

| Subtype | Description |
|---------|-------------|
| `Code(CodeLanguage)` | Source code with language |
| `Sql` | SQL query |
| `Regex` | Regular expression |
| `Expression` | Template expression |

---

## CodeLanguage Enum

```rust
pub enum CodeLanguage {
    // Systems
    Rust, C, Cpp, Zig,
    
    // JVM
    Java, Kotlin, Scala,
    
    // .NET
    CSharp, FSharp,
    
    // Web
    JavaScript, TypeScript,
    
    // Scripting
    Python, Ruby, PHP, Lua,
    
    // Functional
    Haskell, Elixir, Clojure,
    
    // Modern
    Go, Swift, Dart,
    
    // Shell
    Shell, PowerShell,
    
    // Database
    Sql,
    
    // Custom
    Custom(String),
}
```

---

## NumberUnit Reference

Units provide **measurement system** selection and automatic conversion.

### Length Units

| Unit | Base | Suffixes |
|------|------|----------|
| `Length` | m | auto (user pref) |
| `LengthMetric` | m | mm/cm/m/km |
| `LengthImperial` | m | in/ft/yd/mi |

### Rotation Units

| Unit | Base | Suffix |
|------|------|--------|
| `Rotation` | rad | user pref |
| `RotationDegrees` | ° | ° |
| `RotationRadians` | rad | rad |

### Time Units

| Unit | Suffix |
|------|--------|
| `Time` | auto |
| `TimeSeconds` | s/ms/μs/ns |
| `TimeMinutes` | min/h/d |

### Temperature Units

| Unit | Suffix |
|------|--------|
| `Temperature` | user pref |
| `TemperatureCelsius` | °C |
| `TemperatureFahrenheit` | °F |
| `TemperatureKelvin` | K |

### Data Units

| Unit | Base | Suffix |
|------|------|--------|
| `BytesDecimal` | 1000 | KB/MB/GB |
| `BytesBinary` | 1024 | KiB/MiB/GiB |

---

## User Preferences

Units adapt to user preferences:

```rust
pub struct UserPreferences {
    pub unit_system: UnitSystem,           // Metric | Imperial
    pub angle_unit: AngleUnit,             // Degrees | Radians
    pub temperature_unit: TemperatureUnit, // Celsius | Fahrenheit | Kelvin
}
```

**Example:**
```
American user:  "5.9 ft"
European user:  "1.8 m"
Stored value:   1.8 (meters)
```

---

## IntoBuilder Trait

Subtypes implement `IntoBuilder` for ergonomic node creation with sensible defaults:

```rust
pub trait IntoBuilder {
    type Builder;
    fn builder(self, key: impl Into<Key>) -> Self::Builder;
}
```

Each `IntoBuilder` implementation provides a complete "package" for the subtype:
- **Subtype** — semantic meaning
- **Validators** — validation rules
- **Transformers** — value normalization
- **UiHints** — UI defaults

### Number Subtype Implementations

```rust
impl IntoBuilder for Port {
    type Builder = NumberBuilder<u16, Port>;
    
    fn builder(self, key: impl Into<Key>) -> Self::Builder {
        Number::<u16>::builder(key)
            .subtype(self)
            // Validation
            .range(1, 65535)
            // UI
            .ui_hints(NumberUiHints::new()
                .suffix("port"))
    }
}

impl IntoBuilder for Factor {
    type Builder = NumberBuilder<f64, Factor>;
    
    fn builder(self, key: impl Into<Key>) -> Self::Builder {
        Number::<f64>::builder(key)
            .subtype(self)
            // Validation
            .range(0.0, 1.0)
            // Transform
            .transform(clamp(0.0, 1.0))
            // UI
            .ui_hints(NumberUiHints::slider(0.0, 1.0)
                .step(0.01)
                .precision(2))
    }
}

impl IntoBuilder for Percentage {
    type Builder = NumberBuilder<f64, Percentage>;
    
    fn builder(self, key: impl Into<Key>) -> Self::Builder {
        Number::<f64>::builder(key)
            .subtype(self)
            // Validation
            .range(0.0, 100.0)
            // Transform
            .transform(clamp(0.0, 100.0))
            // UI
            .ui_hints(NumberUiHints::slider(0.0, 100.0)
                .step(1.0)
                .suffix("%"))
    }
}

impl IntoBuilder for Rating {
    type Builder = NumberBuilder<u8, Rating>;
    
    fn builder(self, key: impl Into<Key>) -> Self::Builder {
        Number::<u8>::builder(key)
            .subtype(self)
            // Validation
            .range(0, 5)
            // UI
            .ui_hints(NumberUiHints::new()
                .slider(0, 5)
                .step(1))
    }
}
```

### Vector Subtype Implementations

```rust
impl IntoBuilder for Position3D {
    type Builder = VectorBuilder<f64, 3, Position3D>;
    
    fn builder(self, key: impl Into<Key>) -> Self::Builder {
        Vector::<f64, 3>::builder(key)
            .subtype(self)
            // UI
            .ui_hints(VectorUiHints::new()
                .labels(["X", "Y", "Z"]))
    }
}

impl IntoBuilder for ColorRgba {
    type Builder = VectorBuilder<f64, 4, ColorRgba>;
    
    fn builder(self, key: impl Into<Key>) -> Self::Builder {
        Vector::<f64, 4>::builder(key)
            .subtype(self)
            // Validation
            .component_range(0.0, 1.0)
            // Transform
            .transform(clamp_components(0.0, 1.0))
            // UI
            .ui_hints(VectorUiHints::new()
                .labels(["R", "G", "B", "A"])
                .compact())
    }
}

impl IntoBuilder for Quaternion {
    type Builder = VectorBuilder<f64, 4, Quaternion>;
    
    fn builder(self, key: impl Into<Key>) -> Self::Builder {
        Vector::<f64, 4>::builder(key)
            .subtype(self)
            // Validation
            .normalized()
            // Transform
            .transform(normalize())
            // UI
            .ui_hints(VectorUiHints::new()
                .labels(["X", "Y", "Z", "W"]))
    }
}
```

### Text Subtype Implementations

```rust
impl IntoBuilder for Email {
    type Builder = TextBuilder<Email>;
    
    fn builder(self, key: impl Into<Key>) -> Self::Builder {
        Text::builder(key)
            .subtype(self)
            // Validation
            .validate(email())
            // Transform
            .transform(trim().and_then(lowercase()))
            // UI
            .ui_hints(TextUiHints::new()
                .placeholder("user@example.com"))
    }
}

impl IntoBuilder for Url {
    type Builder = TextBuilder<Url>;
    
    fn builder(self, key: impl Into<Key>) -> Self::Builder {
        Text::builder(key)
            .subtype(self)
            // Validation
            .validate(url())
            // Transform
            .transform(trim())
            // UI
            .ui_hints(TextUiHints::new()
                .placeholder("https://example.com"))
    }
}

impl IntoBuilder for Secret {
    type Builder = TextBuilder<Secret>;
    
    fn builder(self, key: impl Into<Key>) -> Self::Builder {
        Text::builder(key)
            .subtype(self)
            // Flags
            .flags(Flags::SENSITIVE | Flags::WRITE_ONLY)
            // UI
            .ui_hints(TextUiHints::new()
                .masked())
    }
}
```

### Overriding Defaults

All defaults can be overridden:

```rust
// Use defaults
Factor.builder("opacity").build()

// Override UI hints
Factor.builder("opacity")
    .ui_hints(NumberUiHints::slider(0.0, 1.0).step(0.1))  // coarser step
    .build()

// Override transform
Factor.builder("opacity")
    .transform(round_to(0.1))  // round instead of clamp
    .build()

// Override validation
Factor.builder("opacity")
    .clear_validators()
    .range(0.0, 0.5)  // custom range
    .build()
```

---

## Usage Examples

With `IntoBuilder`, subtype becomes the entry point:

```rust
// Network port — includes range(1, 65535) validator
Port.builder("http_port").default(8080).build()

// Opacity — includes range(0.0, 1.0) validator
Factor.builder("opacity").default(1.0).build()

// Rating — includes range(0, 5) validator
Rating.builder("stars").default(5).build()

// 3D position
Position3D.builder("location").default([0.0, 0.0, 0.0]).build()

// RGBA color — includes component_range(0.0, 1.0) validator
ColorRgba.builder("tint").default([1.0, 1.0, 1.0, 1.0]).build()

// Quaternion — includes normalized() validator
Quaternion.builder("rotation").default([0.0, 0.0, 0.0, 1.0]).build()

// Email — includes email() validator
Email.builder("contact").build()

// URL — includes url() validator
Url.builder("website").build()
```

### Verbose Syntax (still available)

```rust
// Equivalent to Port.builder("port")
Number::<u16>::builder("port")
    .subtype(Port)
    .range(1, 65535)
    .default(8080)
    .build()

// Equivalent to Position3D.builder("location")
Vector::<f64, 3>::builder("location")
    .subtype(Position3D)
    .default([0.0, 0.0, 0.0])
    .build()
```

---

## Unit Conversion

```rust
impl NumberUnit {
    /// Convert from base unit to display unit
    pub fn from_base(&self, value: f64, target: &str) -> f64;
    
    /// Convert from display unit to base unit
    pub fn to_base(&self, value: f64, source: &str) -> f64;
    
    /// Get display suffix based on value magnitude
    pub fn display_suffix(&self, value: f64, prefs: &UserPreferences) -> &str;
}
```

**Examples:**
```rust
// Length: meters <-> feet
LengthMetric.from_base(1.8, "m")    // 1.8
LengthImperial.from_base(1.8, "ft") // 5.9

// Temperature: Celsius <-> Fahrenheit
TemperatureCelsius.to_base(20.0, "°C")     // 20.0
TemperatureFahrenheit.to_base(68.0, "°F")  // 20.0

// Rotation: radians <-> degrees
RotationRadians.from_base(PI, "rad")  // 3.14159
RotationDegrees.from_base(PI, "°")    // 180.0
```
