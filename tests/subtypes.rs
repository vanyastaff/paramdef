//! Integration tests for the subtype system.

use paramdef::subtype::{
    // Number subtypes
    Angle,
    // Vector subtypes
    ColorRgb,
    ColorRgba,
    Count,
    // Text subtypes
    Date,
    DateTime,
    Direction3D,
    Distance,
    Duration,
    Email,
    Euler,
    Factor,
    FilePath,
    GenericNumber,
    Index,
    Json,
    MultiLine,
    Normal,
    // Traits
    NumberSubtype,
    Numeric,
    NumericKind,
    Password,
    Percentage,
    Plain,
    Port,
    Position2D,
    Position3D,
    Quaternion,
    Rating,
    Secret,
    Size2D,
    Temperature,
    TextSubtype,
    Time,
    Url,
    Uuid,
    Uv,
    VectorSubtype,
};

// ============================================================================
// Number Subtypes
// ============================================================================

#[test]
fn test_number_subtype_names() {
    assert_eq!(GenericNumber::name(), "generic");
    assert_eq!(Port::name(), "port");
    assert_eq!(Percentage::name(), "percentage");
    assert_eq!(Factor::name(), "factor");
    assert_eq!(Count::name(), "count");
    assert_eq!(Index::name(), "index");
    assert_eq!(Rating::name(), "rating");
    assert_eq!(Distance::name(), "distance");
    assert_eq!(Duration::name(), "duration");
    assert_eq!(Temperature::name(), "temperature");
    assert_eq!(Angle::name(), "angle");
}

#[test]
fn test_number_subtype_ranges() {
    // Port has a default range
    let port_range = Port::default_range();
    assert!(port_range.is_some());
    let (min, max) = port_range.unwrap();
    assert_eq!(min, 1);
    assert_eq!(max, 65535);

    // Rating has a default range
    let rating_range = Rating::default_range();
    assert!(rating_range.is_some());
    let (min, max) = rating_range.unwrap();
    assert_eq!(min, 1);
    assert_eq!(max, 5);

    // Factor has a default range
    let factor_range = Factor::default_range();
    assert!(factor_range.is_some());
    let (min, max) = factor_range.unwrap();
    assert!((min - 0.0).abs() < f64::EPSILON);
    assert!((max - 1.0).abs() < f64::EPSILON);

    // Percentage has a default range
    let pct_range = Percentage::default_range();
    assert!(pct_range.is_some());
    let (min, max) = pct_range.unwrap();
    assert!((min - 0.0).abs() < f64::EPSILON);
    assert!((max - 100.0).abs() < f64::EPSILON);
}

// ============================================================================
// Text Subtypes
// ============================================================================

#[test]
fn test_text_subtype_names() {
    assert_eq!(Plain::name(), "plain");
    assert_eq!(Email::name(), "email");
    assert_eq!(Url::name(), "url");
    assert_eq!(Password::name(), "password");
    assert_eq!(Secret::name(), "secret");
    assert_eq!(MultiLine::name(), "multiline");
    assert_eq!(Json::name(), "json");
    assert_eq!(FilePath::name(), "file_path");
    assert_eq!(Date::name(), "date");
    assert_eq!(Time::name(), "time");
    assert_eq!(DateTime::name(), "datetime");
    assert_eq!(Uuid::name(), "uuid");
}

#[test]
fn test_text_subtype_sensitivity() {
    // Sensitive subtypes
    assert!(Password::is_sensitive());
    assert!(Secret::is_sensitive());

    // Non-sensitive subtypes
    assert!(!Plain::is_sensitive());
    assert!(!Email::is_sensitive());
    assert!(!Url::is_sensitive());
    assert!(!Json::is_sensitive());
}

#[test]
fn test_text_subtype_multiline() {
    // Multiline subtypes
    assert!(MultiLine::is_multiline());
    assert!(Json::is_multiline());

    // Single-line subtypes
    assert!(!Plain::is_multiline());
    assert!(!Email::is_multiline());
    assert!(!Password::is_multiline());
}

#[test]
fn test_text_subtype_patterns() {
    // Email has a pattern
    assert!(Email::pattern().is_some());

    // UUID has a pattern
    assert!(Uuid::pattern().is_some());

    // Plain has no pattern
    assert!(Plain::pattern().is_none());
}

#[test]
fn test_text_subtype_placeholders() {
    // Email has a placeholder
    assert!(Email::placeholder().is_some());

    // URL has a placeholder
    assert!(Url::placeholder().is_some());

    // Date/Time have placeholders
    assert!(Date::placeholder().is_some());
    assert!(Time::placeholder().is_some());
    assert!(DateTime::placeholder().is_some());
}

// ============================================================================
// Vector Subtypes
// ============================================================================

#[test]
fn test_vector_subtype_sizes() {
    // Size 2 subtypes
    assert_eq!(Position2D::SIZE, 2);
    assert_eq!(Size2D::SIZE, 2);
    assert_eq!(Uv::SIZE, 2);

    // Size 3 subtypes
    assert_eq!(Position3D::SIZE, 3);
    assert_eq!(Direction3D::SIZE, 3);
    assert_eq!(Euler::SIZE, 3);
    assert_eq!(Normal::SIZE, 3);
    assert_eq!(ColorRgb::SIZE, 3);

    // Size 4 subtypes
    assert_eq!(Quaternion::SIZE, 4);
    assert_eq!(ColorRgba::SIZE, 4);
}

#[test]
fn test_vector_subtype_names() {
    assert_eq!(Position2D::name(), "position_2d");
    assert_eq!(Position3D::name(), "position_3d");
    assert_eq!(Direction3D::name(), "direction_3d");
    assert_eq!(Normal::name(), "normal");
    assert_eq!(Euler::name(), "euler");
    assert_eq!(Quaternion::name(), "quaternion");
    assert_eq!(ColorRgb::name(), "color_rgb");
    assert_eq!(ColorRgba::name(), "color_rgba");
}

#[test]
fn test_vector_subtype_labels() {
    // Position3D should have XYZ labels
    let labels = Position3D::component_labels();
    assert_eq!(labels, ["X", "Y", "Z"]);

    // ColorRGB should have RGB labels
    let rgb_labels = ColorRgb::component_labels();
    assert_eq!(rgb_labels, ["R", "G", "B"]);

    // ColorRGBA should have RGBA labels
    let rgba_labels = ColorRgba::component_labels();
    assert_eq!(rgba_labels, ["R", "G", "B", "A"]);

    // UV should have UV labels
    let uv_labels = Uv::component_labels();
    assert_eq!(uv_labels, ["U", "V"]);
}

#[test]
fn test_vector_subtype_normalized() {
    // These should be normalized
    assert!(Normal::is_normalized());
    assert!(Direction3D::is_normalized());
    assert!(Quaternion::is_normalized());

    // These should not be normalized
    assert!(!Position3D::is_normalized());
    assert!(!ColorRgb::is_normalized());
    assert!(!Euler::is_normalized());
}

#[test]
fn test_vector_subtype_ranges() {
    // Colors have 0-1 range
    let rgb_range = ColorRgb::default_range();
    assert!(rgb_range.is_some());
    let (min, max) = rgb_range.unwrap();
    assert!((min - 0.0).abs() < f64::EPSILON);
    assert!((max - 1.0).abs() < f64::EPSILON);

    // Positions typically have no range
    assert!(Position3D::default_range().is_none());
}

// ============================================================================
// Numeric Trait
// ============================================================================

#[test]
fn test_numeric_kind() {
    assert_eq!(i32::kind(), NumericKind::I32);
    assert_eq!(i64::kind(), NumericKind::I64);
    assert_eq!(f32::kind(), NumericKind::F32);
    assert_eq!(f64::kind(), NumericKind::F64);
}

#[test]
fn test_numeric_kind_properties() {
    assert!(NumericKind::I32.is_integer());
    assert!(NumericKind::I64.is_integer());
    assert!(!NumericKind::F32.is_integer());
    assert!(!NumericKind::F64.is_integer());

    assert!(!NumericKind::I32.is_float());
    assert!(!NumericKind::I64.is_float());
    assert!(NumericKind::F32.is_float());
    assert!(NumericKind::F64.is_float());
}

#[test]
fn test_numeric_conversions() {
    // to_f64
    assert!((42i32.to_f64() - 42.0).abs() < f64::EPSILON);
    assert!((3.14f64.to_f64() - 3.14).abs() < f64::EPSILON);

    // from_f64
    let i: i32 = Numeric::from_f64(42.7);
    assert_eq!(i, 42);

    let f: f64 = Numeric::from_f64(3.14);
    assert!((f - 3.14).abs() < f64::EPSILON);
}

#[test]
fn test_numeric_zero_one() {
    assert_eq!(i32::zero(), 0);
    assert_eq!(i32::one(), 1);
    assert!((f64::zero() - 0.0).abs() < f64::EPSILON);
    assert!((f64::one() - 1.0).abs() < f64::EPSILON);
}
