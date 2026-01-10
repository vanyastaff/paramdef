//! Number units with conversion support.
//!
//! Units provide measurement context for numeric values. Each unit category
//! has a base unit, and conversions happen through that base.
//!
//! # Example
//!
//! ```
//! use paramdef::subtype::NumberUnit;
//!
//! let meters = NumberUnit::Meters;
//! let cm_value = 150.0;
//!
//! // Convert to base (meters)
//! let base = meters.to_base(cm_value); // Still 150.0 because meters IS base
//!
//! // Convert from centimeters to meters
//! let cm = NumberUnit::Centimeters;
//! let m_value = cm.to_base(150.0); // 1.5 meters
//! ```

// === Conversion Constants ===

// Length conversions (to meters)
const MM_TO_M: f64 = 0.001;
const CM_TO_M: f64 = 0.01;
const KM_TO_M: f64 = 1000.0;
const INCH_TO_M: f64 = 0.0254;
const FOOT_TO_M: f64 = 0.3048;
const MILE_TO_M: f64 = 1609.344;

// Time conversions (to seconds)
const MS_TO_S: f64 = 0.001;
const MIN_TO_S: f64 = 60.0;
const HOUR_TO_S: f64 = 3600.0;
const DAY_TO_S: f64 = 86400.0;

// Rotation conversions (to degrees)
const RAD_TO_DEG: f64 = 180.0 / std::f64::consts::PI;
const TURN_TO_DEG: f64 = 360.0;

// Data conversions (to bytes)
const KB_TO_B: f64 = 1024.0;
const MB_TO_B: f64 = 1024.0 * 1024.0;
const GB_TO_B: f64 = 1024.0 * 1024.0 * 1024.0;
const TB_TO_B: f64 = 1024.0 * 1024.0 * 1024.0 * 1024.0;

// Temperature conversions
const FAHRENHEIT_OFFSET: f64 = 32.0;
const FAHRENHEIT_SCALE: f64 = 9.0 / 5.0;
const KELVIN_OFFSET: f64 = 273.15;

/// Measurement units for numeric values.
///
/// Units are organized into categories, each with a base unit:
/// - Length: Meters (base)
/// - Time: Seconds (base)
/// - Rotation: Degrees (base)
/// - Data: Bytes (base)
/// - Temperature: Celsius (base)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum NumberUnit {
    // === Length ===
    /// Millimeters (1/1000 meter)
    Millimeters,
    /// Centimeters (1/100 meter)
    Centimeters,
    /// Meters (base unit for length)
    #[default]
    Meters,
    /// Kilometers (1000 meters)
    Kilometers,
    /// Inches (0.0254 meters)
    Inches,
    /// Feet (0.3048 meters)
    Feet,
    /// Miles (1609.344 meters)
    Miles,

    // === Time ===
    /// Milliseconds (1/1000 second)
    Milliseconds,
    /// Seconds (base unit for time)
    Seconds,
    /// Minutes (60 seconds)
    Minutes,
    /// Hours (3600 seconds)
    Hours,
    /// Days (86400 seconds)
    Days,

    // === Rotation ===
    /// Degrees (base unit for rotation)
    Degrees,
    /// Radians (π/180 degrees)
    Radians,
    /// Turns (360 degrees)
    Turns,

    // === Data ===
    /// Bytes (base unit for data)
    Bytes,
    /// Kilobytes (1024 bytes)
    Kilobytes,
    /// Megabytes (1024² bytes)
    Megabytes,
    /// Gigabytes (1024³ bytes)
    Gigabytes,
    /// Terabytes (1024⁴ bytes)
    Terabytes,

    // === Temperature ===
    /// Celsius (base unit for temperature)
    Celsius,
    /// Fahrenheit
    Fahrenheit,
    /// Kelvin
    Kelvin,

    // === Percentage ===
    /// Percentage (0-100)
    Percent,
    /// Factor (0-1)
    Factor,

    // === No unit ===
    /// No unit / dimensionless
    None,
}

impl NumberUnit {
    /// Returns the display suffix for this unit.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::subtype::NumberUnit;
    ///
    /// assert_eq!(NumberUnit::Meters.display_suffix(), "m");
    /// assert_eq!(NumberUnit::Percent.display_suffix(), "%");
    /// ```
    #[must_use]
    pub const fn display_suffix(&self) -> &'static str {
        match self {
            // Length
            Self::Millimeters => "mm",
            Self::Centimeters => "cm",
            Self::Meters => "m",
            Self::Kilometers => "km",
            Self::Inches => "in",
            Self::Feet => "ft",
            Self::Miles => "mi",

            // Time
            Self::Milliseconds => "ms",
            Self::Seconds => "s",
            Self::Minutes => "min",
            Self::Hours => "h",
            Self::Days => "d",

            // Rotation
            Self::Degrees => "°",
            Self::Radians => "rad",
            Self::Turns => "rev",

            // Data
            Self::Bytes => "B",
            Self::Kilobytes => "KB",
            Self::Megabytes => "MB",
            Self::Gigabytes => "GB",
            Self::Terabytes => "TB",

            // Temperature
            Self::Celsius => "°C",
            Self::Fahrenheit => "°F",
            Self::Kelvin => "K",

            // Percentage
            Self::Percent => "%",

            // Factor and None have no suffix
            Self::Factor | Self::None => "",
        }
    }

    /// Converts a value from this unit to the base unit.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::subtype::NumberUnit;
    ///
    /// // 100 cm = 1 m
    /// let meters = NumberUnit::Centimeters.to_base(100.0);
    /// assert!((meters - 1.0).abs() < 0.001);
    /// ```
    ///
    /// # Note
    ///
    /// Temperature conversions do not validate physical constraints (e.g., temperatures
    /// below absolute zero). Use validation rules for physical correctness.
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn to_base(&self, value: f64) -> f64 {
        match self {
            // Length (base: meters)
            Self::Millimeters => value * MM_TO_M,
            Self::Centimeters => value * CM_TO_M,
            Self::Meters => value,
            Self::Kilometers => value * KM_TO_M,
            Self::Inches => value * INCH_TO_M,
            Self::Feet => value * FOOT_TO_M,
            Self::Miles => value * MILE_TO_M,

            // Time (base: seconds)
            Self::Milliseconds => value * MS_TO_S,
            Self::Seconds => value,
            Self::Minutes => value * MIN_TO_S,
            Self::Hours => value * HOUR_TO_S,
            Self::Days => value * DAY_TO_S,

            // Rotation (base: degrees)
            Self::Degrees => value,
            Self::Radians => value * RAD_TO_DEG,
            Self::Turns => value * TURN_TO_DEG,

            // Data (base: bytes)
            Self::Bytes => value,
            Self::Kilobytes => value * KB_TO_B,
            Self::Megabytes => value * MB_TO_B,
            Self::Gigabytes => value * GB_TO_B,
            Self::Terabytes => value * TB_TO_B,

            // Temperature (base: celsius)
            Self::Celsius => value,
            Self::Fahrenheit => (value - FAHRENHEIT_OFFSET) / FAHRENHEIT_SCALE,
            Self::Kelvin => value - KELVIN_OFFSET,

            // Percentage (base: factor 0-1)
            Self::Percent => value / 100.0,
            Self::Factor => value,

            // None
            Self::None => value,
        }
    }

    /// Converts a value from the base unit to this unit.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::subtype::NumberUnit;
    ///
    /// // 1 m = 100 cm
    /// let cm = NumberUnit::Centimeters.from_base(1.0);
    /// assert!((cm - 100.0).abs() < 0.001);
    /// ```
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn from_base(&self, value: f64) -> f64 {
        match self {
            // Length (base: meters)
            Self::Millimeters => value / MM_TO_M,
            Self::Centimeters => value / CM_TO_M,
            Self::Meters => value,
            Self::Kilometers => value / KM_TO_M,
            Self::Inches => value / INCH_TO_M,
            Self::Feet => value / FOOT_TO_M,
            Self::Miles => value / MILE_TO_M,

            // Time (base: seconds)
            Self::Milliseconds => value / MS_TO_S,
            Self::Seconds => value,
            Self::Minutes => value / MIN_TO_S,
            Self::Hours => value / HOUR_TO_S,
            Self::Days => value / DAY_TO_S,

            // Rotation (base: degrees)
            Self::Degrees => value,
            Self::Radians => value / RAD_TO_DEG,
            Self::Turns => value / TURN_TO_DEG,

            // Data (base: bytes)
            Self::Bytes => value,
            Self::Kilobytes => value / KB_TO_B,
            Self::Megabytes => value / MB_TO_B,
            Self::Gigabytes => value / GB_TO_B,
            Self::Terabytes => value / TB_TO_B,

            // Temperature (base: celsius)
            Self::Celsius => value,
            Self::Fahrenheit => value * FAHRENHEIT_SCALE + FAHRENHEIT_OFFSET,
            Self::Kelvin => value + KELVIN_OFFSET,

            // Percentage (base: factor 0-1)
            Self::Percent => value * 100.0,
            Self::Factor => value,

            // None
            Self::None => value,
        }
    }

    /// Converts a value from this unit to another unit.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::subtype::NumberUnit;
    ///
    /// // 1 km = 1000 m
    /// let meters = NumberUnit::Kilometers.convert_to(1.0, NumberUnit::Meters);
    /// assert!((meters - 1000.0).abs() < 0.001);
    /// ```
    #[must_use]
    pub fn convert_to(&self, value: f64, target: Self) -> f64 {
        let base = self.to_base(value);
        target.from_base(base)
    }

    /// Returns the category of this unit.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::subtype::NumberUnit;
    ///
    /// assert_eq!(NumberUnit::Meters.category(), "length");
    /// assert_eq!(NumberUnit::Seconds.category(), "time");
    /// ```
    #[must_use]
    pub const fn category(&self) -> &'static str {
        match self {
            Self::Millimeters
            | Self::Centimeters
            | Self::Meters
            | Self::Kilometers
            | Self::Inches
            | Self::Feet
            | Self::Miles => "length",

            Self::Milliseconds | Self::Seconds | Self::Minutes | Self::Hours | Self::Days => "time",

            Self::Degrees | Self::Radians | Self::Turns => "rotation",

            Self::Bytes | Self::Kilobytes | Self::Megabytes | Self::Gigabytes | Self::Terabytes => {
                "data"
            }

            Self::Celsius | Self::Fahrenheit | Self::Kelvin => "temperature",

            Self::Percent | Self::Factor => "percentage",

            Self::None => "none",
        }
    }

    /// Returns all units in the same category.
    ///
    /// Useful for UI dropdowns showing compatible unit options.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::subtype::NumberUnit;
    ///
    /// let compatible = NumberUnit::Meters.compatible_units();
    /// assert!(compatible.contains(&NumberUnit::Centimeters));
    /// assert!(compatible.contains(&NumberUnit::Kilometers));
    /// assert!(!compatible.contains(&NumberUnit::Seconds));
    /// ```
    #[must_use]
    pub fn compatible_units(&self) -> &'static [Self] {
        match self.category() {
            "length" => &[
                Self::Millimeters,
                Self::Centimeters,
                Self::Meters,
                Self::Kilometers,
                Self::Inches,
                Self::Feet,
                Self::Miles,
            ],
            "time" => &[
                Self::Milliseconds,
                Self::Seconds,
                Self::Minutes,
                Self::Hours,
                Self::Days,
            ],
            "rotation" => &[Self::Degrees, Self::Radians, Self::Turns],
            "data" => &[
                Self::Bytes,
                Self::Kilobytes,
                Self::Megabytes,
                Self::Gigabytes,
                Self::Terabytes,
            ],
            "temperature" => &[Self::Celsius, Self::Fahrenheit, Self::Kelvin],
            "percentage" => &[Self::Percent, Self::Factor],
            "none" => &[Self::None],
            _ => &[],
        }
    }

    /// Checks if conversion to target unit is valid (same category).
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::subtype::NumberUnit;
    ///
    /// assert!(NumberUnit::Meters.can_convert_to(NumberUnit::Kilometers));
    /// assert!(!NumberUnit::Meters.can_convert_to(NumberUnit::Seconds));
    /// ```
    #[must_use]
    pub fn can_convert_to(&self, target: Self) -> bool {
        self.category() == target.category()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_suffix() {
        assert_eq!(NumberUnit::Meters.display_suffix(), "m");
        assert_eq!(NumberUnit::Kilometers.display_suffix(), "km");
        assert_eq!(NumberUnit::Seconds.display_suffix(), "s");
        assert_eq!(NumberUnit::Degrees.display_suffix(), "°");
        assert_eq!(NumberUnit::Percent.display_suffix(), "%");
        assert_eq!(NumberUnit::None.display_suffix(), "");
    }

    // === Length Tests ===

    #[test]
    fn test_length_to_base() {
        assert!((NumberUnit::Millimeters.to_base(1000.0) - 1.0).abs() < 0.001);
        assert!((NumberUnit::Centimeters.to_base(100.0) - 1.0).abs() < 0.001);
        assert!((NumberUnit::Meters.to_base(1.0) - 1.0).abs() < 0.001);
        assert!((NumberUnit::Kilometers.to_base(1.0) - 1000.0).abs() < 0.001);
    }

    #[test]
    fn test_length_from_base() {
        assert!((NumberUnit::Centimeters.from_base(1.0) - 100.0).abs() < 0.001);
        assert!((NumberUnit::Kilometers.from_base(1000.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_length_convert() {
        let km = 1.0;
        let m = NumberUnit::Kilometers.convert_to(km, NumberUnit::Meters);
        assert!((m - 1000.0).abs() < 0.001);

        let cm = NumberUnit::Meters.convert_to(1.0, NumberUnit::Centimeters);
        assert!((cm - 100.0).abs() < 0.001);
    }

    // === Time Tests ===

    #[test]
    fn test_time_conversions() {
        assert!((NumberUnit::Minutes.to_base(1.0) - 60.0).abs() < 0.001);
        assert!((NumberUnit::Hours.to_base(1.0) - 3600.0).abs() < 0.001);
        assert!((NumberUnit::Days.to_base(1.0) - 86400.0).abs() < 0.001);
    }

    // === Rotation Tests ===

    #[test]
    fn test_rotation_conversions() {
        let rad = NumberUnit::Radians.to_base(std::f64::consts::PI);
        assert!((rad - 180.0).abs() < 0.001);

        let turns = NumberUnit::Turns.to_base(1.0);
        assert!((turns - 360.0).abs() < 0.001);
    }

    // === Data Tests ===

    #[test]
    fn test_data_conversions() {
        assert!((NumberUnit::Kilobytes.to_base(1.0) - 1024.0).abs() < 0.001);
        assert!((NumberUnit::Megabytes.to_base(1.0) - 1048576.0).abs() < 0.001);
    }

    // === Temperature Tests ===

    #[test]
    fn test_temperature_conversions() {
        // 32°F = 0°C
        let celsius = NumberUnit::Fahrenheit.to_base(32.0);
        assert!((celsius - 0.0).abs() < 0.001);

        // 212°F = 100°C
        let celsius = NumberUnit::Fahrenheit.to_base(212.0);
        assert!((celsius - 100.0).abs() < 0.001);

        // 0 K = -273.15°C
        let celsius = NumberUnit::Kelvin.to_base(0.0);
        assert!((celsius - (-273.15)).abs() < 0.001);
    }

    #[test]
    fn test_temperature_from_base() {
        // 0°C = 32°F
        let fahrenheit = NumberUnit::Fahrenheit.from_base(0.0);
        assert!((fahrenheit - 32.0).abs() < 0.001);

        // 0°C = 273.15 K
        let kelvin = NumberUnit::Kelvin.from_base(0.0);
        assert!((kelvin - 273.15).abs() < 0.001);
    }

    // === Percentage Tests ===

    #[test]
    fn test_percentage_conversions() {
        assert!((NumberUnit::Percent.to_base(50.0) - 0.5).abs() < 0.001);
        assert!((NumberUnit::Percent.from_base(0.5) - 50.0).abs() < 0.001);
    }

    // === Category Tests ===

    #[test]
    fn test_category() {
        assert_eq!(NumberUnit::Meters.category(), "length");
        assert_eq!(NumberUnit::Seconds.category(), "time");
        assert_eq!(NumberUnit::Degrees.category(), "rotation");
        assert_eq!(NumberUnit::Bytes.category(), "data");
        assert_eq!(NumberUnit::Celsius.category(), "temperature");
        assert_eq!(NumberUnit::Percent.category(), "percentage");
        assert_eq!(NumberUnit::None.category(), "none");
    }

    #[test]
    fn test_default() {
        assert_eq!(NumberUnit::default(), NumberUnit::Meters);
    }

    // === New Helper Method Tests ===

    #[test]
    fn test_compatible_units() {
        let length_units = NumberUnit::Meters.compatible_units();
        assert_eq!(length_units.len(), 7);
        assert!(length_units.contains(&NumberUnit::Centimeters));
        assert!(length_units.contains(&NumberUnit::Kilometers));
        assert!(!length_units.contains(&NumberUnit::Seconds));

        let time_units = NumberUnit::Seconds.compatible_units();
        assert_eq!(time_units.len(), 5);
        assert!(time_units.contains(&NumberUnit::Minutes));
        assert!(!time_units.contains(&NumberUnit::Meters));
    }

    #[test]
    fn test_can_convert_to() {
        // Same category - should succeed
        assert!(NumberUnit::Meters.can_convert_to(NumberUnit::Kilometers));
        assert!(NumberUnit::Centimeters.can_convert_to(NumberUnit::Miles));

        // Different categories - should fail
        assert!(!NumberUnit::Meters.can_convert_to(NumberUnit::Seconds));
        assert!(!NumberUnit::Celsius.can_convert_to(NumberUnit::Meters));

        // Same unit - should succeed
        assert!(NumberUnit::Meters.can_convert_to(NumberUnit::Meters));
    }

    #[test]
    fn test_can_convert_to_all_categories() {
        // Length
        assert!(NumberUnit::Millimeters.can_convert_to(NumberUnit::Inches));

        // Time
        assert!(NumberUnit::Milliseconds.can_convert_to(NumberUnit::Hours));

        // Rotation
        assert!(NumberUnit::Degrees.can_convert_to(NumberUnit::Radians));

        // Data
        assert!(NumberUnit::Bytes.can_convert_to(NumberUnit::Gigabytes));

        // Temperature
        assert!(NumberUnit::Celsius.can_convert_to(NumberUnit::Fahrenheit));

        // Percentage
        assert!(NumberUnit::Percent.can_convert_to(NumberUnit::Factor));

        // None
        assert!(NumberUnit::None.can_convert_to(NumberUnit::None));
    }
}
