//! Core traits for the subtype system.

use std::fmt::Debug;

/// Trait for numeric types that can be used with [`NumberSubtype`].
///
/// This trait provides bounds for numeric operations used in parameter
/// validation and range constraints.
///
/// # Implementors
///
/// All standard integer and float types implement this trait:
/// - Integers: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
/// - Unsigned: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
/// - Floats: `f32`, `f64`
pub trait Numeric: Copy + PartialOrd + Debug + Send + Sync + 'static {
    /// Returns zero for this numeric type.
    fn zero() -> Self;

    /// Returns one for this numeric type.
    fn one() -> Self;

    /// Converts from f64 (for unit conversions).
    fn from_f64(v: f64) -> Self;

    /// Converts to f64 (for unit conversions).
    fn to_f64(self) -> f64;
}

macro_rules! impl_numeric_int {
    ($($t:ty),*) => {
        $(
            impl Numeric for $t {
                #[inline]
                fn zero() -> Self { 0 }

                #[inline]
                fn one() -> Self { 1 }

                #[inline]
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                fn from_f64(v: f64) -> Self { v as Self }

                #[inline]
                #[allow(clippy::cast_precision_loss, clippy::cast_lossless)]
                fn to_f64(self) -> f64 { self as f64 }
            }
        )*
    };
}

macro_rules! impl_numeric_float {
    ($($t:ty),*) => {
        $(
            impl Numeric for $t {
                #[inline]
                fn zero() -> Self { 0.0 }

                #[inline]
                fn one() -> Self { 1.0 }

                #[inline]
                #[allow(clippy::cast_possible_truncation)]
                fn from_f64(v: f64) -> Self { v as Self }

                #[inline]
                #[allow(clippy::cast_lossless)]
                fn to_f64(self) -> f64 { self as f64 }
            }
        )*
    };
}

impl_numeric_int!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);
impl_numeric_float!(f32, f64);

/// Marker trait for integer types.
///
/// Used to constrain integer-only subtypes like [`Port`] or [`Count`].
#[allow(dead_code)]
pub trait Integer: Numeric {}

impl Integer for i8 {}
impl Integer for i16 {}
impl Integer for i32 {}
impl Integer for i64 {}
impl Integer for i128 {}
impl Integer for isize {}
impl Integer for u8 {}
impl Integer for u16 {}
impl Integer for u32 {}
impl Integer for u64 {}
impl Integer for u128 {}
impl Integer for usize {}

/// Marker trait for floating-point types.
///
/// Used to constrain float-only subtypes like [`Factor`] or [`Percentage`].
#[allow(dead_code)]
pub trait Float: Numeric {}

impl Float for f32 {}
impl Float for f64 {}

/// Trait for number subtypes with type constraints.
///
/// Number subtypes can be constrained to specific numeric types:
/// - Integer-only (e.g., `Port`, `Count`)
/// - Float-only (e.g., `Percentage`, `Angle`)
/// - Universal (e.g., `Distance`, `Duration`)
///
/// # Example
///
/// ```ignore
/// use paramdef::subtypes::{NumberSubtype, Port};
///
/// // Port is integer-only
/// let range = Port::default_range();
/// assert_eq!(range, Some((1, 65535)));
/// ```
pub trait NumberSubtype: Debug + Clone + Copy + Default + Send + Sync + 'static {
    /// The numeric type this subtype works with.
    type Value: Numeric;

    /// Returns the name of this subtype.
    fn name() -> &'static str;

    /// Returns the default range for this subtype, if any.
    #[must_use]
    fn default_range() -> Option<(Self::Value, Self::Value)> {
        None
    }

    /// Returns the default step for UI sliders.
    #[must_use]
    fn default_step() -> Option<Self::Value> {
        None
    }

    /// Returns the recommended unit for this subtype.
    #[must_use]
    fn recommended_unit() -> Option<super::NumberUnit> {
        None
    }
}

/// Trait for vector subtypes with size constraints.
///
/// Vector subtypes are constrained by size at compile time:
/// - Size 2: `Position2D`, `Size2D`, `UV`
/// - Size 3: `Position3D`, `ColorRgb`, `Euler`
/// - Size 4: `Quaternion`, `ColorRgba`
///
/// # Example
///
/// ```ignore
/// use paramdef::subtypes::{VectorSubtype, Position3D};
///
/// // Position3D is always size 3
/// assert_eq!(Position3D::SIZE, 3);
/// ```
pub trait VectorSubtype<const N: usize>:
    Debug + Clone + Copy + Default + Send + Sync + 'static
{
    /// Returns the name of this subtype.
    fn name() -> &'static str;

    /// The size of the vector (compile-time constant).
    const SIZE: usize = N;

    /// Returns labels for each component.
    fn component_labels() -> [&'static str; N];

    /// Returns the default range for components, if any.
    #[must_use]
    fn default_range() -> Option<(f64, f64)> {
        None
    }

    /// Returns whether this vector should be normalized.
    #[must_use]
    fn is_normalized() -> bool {
        false
    }
}

/// Trait for text subtypes with semantic meaning.
///
/// Text subtypes provide:
/// - Pattern hints for validation
/// - Placeholder text for UI
/// - Semantic meaning for proper rendering
///
/// # Example
///
/// ```ignore
/// use paramdef::subtypes::{TextSubtype, Email};
///
/// let pattern = Email::pattern();
/// let placeholder = Email::placeholder();
/// ```
pub trait TextSubtype: Debug + Clone + Copy + Default + Send + Sync + 'static {
    /// Returns the name of this subtype.
    fn name() -> &'static str;

    /// Returns a regex pattern for validation, if any.
    #[must_use]
    fn pattern() -> Option<&'static str> {
        None
    }

    /// Returns placeholder text for UI.
    #[must_use]
    fn placeholder() -> Option<&'static str> {
        None
    }

    /// Returns whether the input should be multiline.
    #[must_use]
    fn is_multiline() -> bool {
        false
    }

    /// Returns whether the value is sensitive (passwords, tokens).
    #[must_use]
    fn is_sensitive() -> bool {
        false
    }

    /// Returns the associated code language for code subtypes.
    #[must_use]
    fn code_language() -> Option<&'static str> {
        None
    }
}

/// Trait for converting a subtype into a parameter builder.
///
/// This enables the ergonomic subtype-first API pattern:
///
/// ```ignore
/// // Instead of:
/// Number::builder("port").subtype(Port).build()
///
/// // You can write:
/// Port::into_builder("port").build()
/// ```
pub trait IntoBuilder {
    /// The builder type returned.
    type Builder;

    /// Creates a builder for this subtype with the given key.
    fn into_builder(key: impl Into<crate::core::Key>) -> Self::Builder;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_trait_bounds() {
        fn assert_numeric<T: Numeric>() {}

        assert_numeric::<i8>();
        assert_numeric::<i16>();
        assert_numeric::<i32>();
        assert_numeric::<i64>();
        assert_numeric::<u8>();
        assert_numeric::<u16>();
        assert_numeric::<u32>();
        assert_numeric::<u64>();
        assert_numeric::<f32>();
        assert_numeric::<f64>();
    }

    #[test]
    fn test_numeric_zero_one() {
        assert_eq!(i32::zero(), 0);
        assert_eq!(i32::one(), 1);
        assert_eq!(f64::zero(), 0.0);
        assert_eq!(f64::one(), 1.0);
    }

    #[test]
    fn test_numeric_conversions() {
        let v: i32 = Numeric::from_f64(42.5);
        assert_eq!(v, 42);

        let f: f64 = 42i32.to_f64();
        assert!((f - 42.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_integer_marker() {
        fn assert_integer<T: Integer>() {}

        assert_integer::<i32>();
        assert_integer::<u64>();
        // assert_integer::<f64>(); // This would not compile
    }

    #[test]
    fn test_float_marker() {
        fn assert_float<T: Float>() {}

        assert_float::<f32>();
        assert_float::<f64>();
        // assert_float::<i32>(); // This would not compile
    }
}
