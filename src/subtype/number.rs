//! Standard number subtypes.
//!
//! Number subtypes are categorized by their numeric type constraints:
//!
//! ## Integer-Only Subtypes
//! - [`Port`] - Network port number (1-65535)
//! - [`Count`] - Non-negative count
//! - [`Rating`] - Rating value (1-5)
//! - [`ByteCount`] - Byte count (file sizes)
//! - [`Index`] - Zero-based index
//!
//! ## Float-Only Subtypes
//! - [`Factor`] - Multiplicative factor (0-1)
//! - [`Percentage`] - Percentage (0-100)
//! - [`Angle`] - Angle in degrees (0-360)
//! - [`AngleRadians`] - Angle in radians (0-2π)
//!
//! ## Universal Subtypes
//! - [`Distance`] - Distance measurement
//! - [`Duration`] - Time duration
//! - [`Temperature`] - Temperature value
//! - [`Currency`] - Monetary value
//! - [`Speed`] - Speed/velocity
//! - [`Mass`] - Mass/weight
//! - [`GenericNumber`] - Unconstrained number

use crate::define_number_subtype;

// === Integer-Only Subtypes ===

define_number_subtype!(Port, int_only, u16, "port", range: (1, 65535));
define_number_subtype!(Count, int_only, u64, "count");
define_number_subtype!(Rating, int_only, u8, "rating", range: (1, 5));
define_number_subtype!(ByteCount, int_only, u64, "byte_count");
define_number_subtype!(Index, int_only, usize, "index");

// === Float-Only Subtypes ===

define_number_subtype!(Factor, float_only, f64, "factor", range: (0.0, 1.0));
define_number_subtype!(Percentage, float_only, f64, "percentage", range: (0.0, 100.0));
define_number_subtype!(Angle, float_only, f64, "angle", range: (0.0, 360.0));

/// Angle in radians.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct AngleRadians;

impl super::NumberSubtype for AngleRadians {
    type Value = f64;

    fn name() -> &'static str {
        "angle_radians"
    }

    fn default_range() -> Option<(Self::Value, Self::Value)> {
        Some((0.0, std::f64::consts::TAU))
    }
}

// === Universal Subtypes ===

define_number_subtype!(Distance, any, f64, "distance");
define_number_subtype!(Duration, any, f64, "duration");
define_number_subtype!(Temperature, any, f64, "temperature");
define_number_subtype!(Currency, any, f64, "currency");
define_number_subtype!(Speed, any, f64, "speed");
define_number_subtype!(Mass, any, f64, "mass");
define_number_subtype!(GenericNumber, any, f64, "generic");

#[cfg(test)]
mod tests {
    use super::*;
    use crate::subtype::NumberSubtype;

    // === Integer-Only Tests ===

    #[test]
    fn test_port_subtype() {
        assert_eq!(Port::name(), "port");
        assert_eq!(Port::default_range(), Some((1, 65535)));
    }

    #[test]
    fn test_count_subtype() {
        assert_eq!(Count::name(), "count");
        assert_eq!(Count::default_range(), None);
    }

    #[test]
    fn test_rating_subtype() {
        assert_eq!(Rating::name(), "rating");
        assert_eq!(Rating::default_range(), Some((1, 5)));
    }

    #[test]
    fn test_byte_count_subtype() {
        assert_eq!(ByteCount::name(), "byte_count");
    }

    #[test]
    fn test_index_subtype() {
        assert_eq!(Index::name(), "index");
    }

    // === Float-Only Tests ===

    #[test]
    fn test_factor_subtype() {
        assert_eq!(Factor::name(), "factor");
        assert_eq!(Factor::default_range(), Some((0.0, 1.0)));
    }

    #[test]
    fn test_percentage_subtype() {
        assert_eq!(Percentage::name(), "percentage");
        assert_eq!(Percentage::default_range(), Some((0.0, 100.0)));
    }

    #[test]
    fn test_angle_subtype() {
        assert_eq!(Angle::name(), "angle");
        assert_eq!(Angle::default_range(), Some((0.0, 360.0)));
    }

    #[test]
    fn test_angle_radians_subtype() {
        assert_eq!(AngleRadians::name(), "angle_radians");
        let range = AngleRadians::default_range().unwrap();
        assert!((range.0 - 0.0).abs() < f64::EPSILON);
        assert!((range.1 - std::f64::consts::TAU).abs() < f64::EPSILON);
    }

    // === Universal Tests ===

    #[test]
    fn test_distance_subtype() {
        assert_eq!(Distance::name(), "distance");
        assert_eq!(Distance::default_range(), None);
    }

    #[test]
    fn test_duration_subtype() {
        assert_eq!(Duration::name(), "duration");
    }

    #[test]
    fn test_temperature_subtype() {
        assert_eq!(Temperature::name(), "temperature");
    }

    #[test]
    fn test_currency_subtype() {
        assert_eq!(Currency::name(), "currency");
    }

    #[test]
    fn test_speed_subtype() {
        assert_eq!(Speed::name(), "speed");
    }

    #[test]
    fn test_mass_subtype() {
        assert_eq!(Mass::name(), "mass");
    }

    #[test]
    fn test_generic_number_subtype() {
        assert_eq!(GenericNumber::name(), "generic");
    }
}
