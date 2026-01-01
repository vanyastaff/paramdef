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
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn to_base(&self, value: f64) -> f64 {
        match self {
            // Length (base: meters)
            Self::Millimeters => value / 1000.0,
            Self::Centimeters => value / 100.0,
            Self::Meters => value,
            Self::Kilometers => value * 1000.0,
            Self::Inches => value * 0.0254,
            Self::Feet => value * 0.3048,
            Self::Miles => value * 1609.344,

            // Time (base: seconds)
            Self::Milliseconds => value / 1000.0,
            Self::Seconds => value,
            Self::Minutes => value * 60.0,
            Self::Hours => value * 3600.0,
            Self::Days => value * 86400.0,

            // Rotation (base: degrees)
            Self::Degrees => value,
            Self::Radians => value * 180.0 / std::f64::consts::PI,
            Self::Turns => value * 360.0,

            // Data (base: bytes)
            Self::Bytes => value,
            Self::Kilobytes => value * 1024.0,
            Self::Megabytes => value * 1024.0 * 1024.0,
            Self::Gigabytes => value * 1024.0 * 1024.0 * 1024.0,
            Self::Terabytes => value * 1024.0 * 1024.0 * 1024.0 * 1024.0,

            // Temperature (base: celsius)
            Self::Celsius => value,
            Self::Fahrenheit => (value - 32.0) * 5.0 / 9.0,
            Self::Kelvin => value - 273.15,

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
            Self::Millimeters => value * 1000.0,
            Self::Centimeters => value * 100.0,
            Self::Meters => value,
            Self::Kilometers => value / 1000.0,
            Self::Inches => value / 0.0254,
            Self::Feet => value / 0.3048,
            Self::Miles => value / 1609.344,

            // Time (base: seconds)
            Self::Milliseconds => value * 1000.0,
            Self::Seconds => value,
            Self::Minutes => value / 60.0,
            Self::Hours => value / 3600.0,
            Self::Days => value / 86400.0,

            // Rotation (base: degrees)
            Self::Degrees => value,
            Self::Radians => value * std::f64::consts::PI / 180.0,
            Self::Turns => value / 360.0,

            // Data (base: bytes)
            Self::Bytes => value,
            Self::Kilobytes => value / 1024.0,
            Self::Megabytes => value / (1024.0 * 1024.0),
            Self::Gigabytes => value / (1024.0 * 1024.0 * 1024.0),
            Self::Terabytes => value / (1024.0 * 1024.0 * 1024.0 * 1024.0),

            // Temperature (base: celsius)
            Self::Celsius => value,
            Self::Fahrenheit => value * 9.0 / 5.0 + 32.0,
            Self::Kelvin => value + 273.15,

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
}
