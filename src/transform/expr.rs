//! Declarative transform expressions.
//!
//! This module provides the [`Transform`] enum for declarative value transformations.
//! These cover ~80% of common use cases and can be serialized/deserialized with serde.

use crate::Value;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Declarative value transformation expression.
///
/// `Transform` covers common transformation patterns in a serializable format.
/// For complex transformations, use the [`Transformer`](super::Transformer) trait.
///
/// # String Transformations
///
/// - [`Trim`](Transform::Trim) - Remove leading/trailing whitespace
/// - [`TrimStart`](Transform::TrimStart) - Remove leading whitespace
/// - [`TrimEnd`](Transform::TrimEnd) - Remove trailing whitespace
/// - [`Lowercase`](Transform::Lowercase) - Convert to lowercase
/// - [`Uppercase`](Transform::Uppercase) - Convert to uppercase
/// - [`Capitalize`](Transform::Capitalize) - Capitalize first letter of each word
///
/// # Numeric Transformations
///
/// - [`Abs`](Transform::Abs) - Absolute value
/// - [`Ceil`](Transform::Ceil) - Round up to nearest integer
/// - [`Floor`](Transform::Floor) - Round down to nearest integer
/// - [`Round`](Transform::Round) - Round to nearest integer
/// - [`RoundTo`](Transform::RoundTo) - Round to N decimal places
/// - [`Clamp`](Transform::Clamp) - Constrain to min/max range
///
/// # Null Handling
///
/// - [`DefaultTo`](Transform::DefaultTo) - Replace Null with default value
/// - [`NullIf`](Transform::NullIf) - Replace specific value with Null
///
/// # Composition
///
/// - [`Sequence`](Transform::Sequence) - Apply multiple transformations in order
///
/// # Example
///
/// ```
/// use paramdef::transform::Transform;
/// use paramdef::Value;
///
/// // Trim and lowercase
/// let value = Value::text("  HELLO WORLD  ");
/// let result = Transform::Sequence(vec![
///     Transform::Trim,
///     Transform::Lowercase,
/// ]).apply(&value);
/// assert_eq!(result.as_text(), Some("hello world"));
///
/// // Clamp numeric value
/// let value = Value::Int(150);
/// let result = Transform::Clamp { min: 0.0, max: 100.0 }.apply(&value);
/// assert_eq!(result, Value::Int(100));
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", rename_all = "snake_case"))]
pub enum Transform {
    // === String Transformations ===
    /// Remove leading and trailing whitespace.
    Trim,

    /// Remove leading whitespace only.
    TrimStart,

    /// Remove trailing whitespace only.
    TrimEnd,

    /// Convert to lowercase.
    Lowercase,

    /// Convert to uppercase.
    Uppercase,

    /// Capitalize first letter of each word.
    Capitalize,

    /// Collapse consecutive whitespace into single space.
    CollapseWhitespace,

    /// Remove all whitespace.
    RemoveWhitespace,

    /// Replace substring with another.
    Replace {
        /// Pattern to find.
        from: String,
        /// Replacement string.
        to: String,
    },

    /// Truncate to maximum length.
    Truncate {
        /// Maximum length in characters.
        max_length: usize,
        /// Suffix to add if truncated (e.g., "...").
        #[cfg_attr(feature = "serde", serde(default))]
        suffix: Option<String>,
    },

    /// Pad string to minimum length.
    Pad {
        /// Minimum length.
        min_length: usize,
        /// Character to pad with.
        #[cfg_attr(feature = "serde", serde(default = "default_pad_char"))]
        char: char,
        /// Pad at start (true) or end (false).
        #[cfg_attr(feature = "serde", serde(default))]
        start: bool,
    },

    // === Numeric Transformations ===
    /// Take absolute value.
    Abs,

    /// Round up to nearest integer.
    Ceil,

    /// Round down to nearest integer.
    Floor,

    /// Round to nearest integer.
    Round,

    /// Round to N decimal places.
    RoundTo {
        /// Number of decimal places.
        decimals: u32,
    },

    /// Constrain value to range.
    Clamp {
        /// Minimum value.
        min: f64,
        /// Maximum value.
        max: f64,
    },

    // === Null Handling ===
    /// Replace Null with default value.
    DefaultTo {
        /// Default value to use.
        value: Value,
    },

    /// Replace matching value with Null.
    NullIf {
        /// Value to replace with Null.
        value: Value,
    },

    /// Replace empty string with Null.
    NullIfEmpty,

    // === Composition ===
    /// Apply multiple transformations in sequence.
    Sequence(Vec<Transform>),
}

#[cfg(feature = "serde")]
fn default_pad_char() -> char {
    ' '
}

impl Transform {
    /// Apply this transformation to a value.
    ///
    /// Returns a new value with the transformation applied.
    /// If the transformation is not applicable to the value type,
    /// returns the value unchanged.
    #[must_use]
    pub fn apply(&self, value: &Value) -> Value {
        match self {
            // String transformations
            Transform::Trim => apply_to_text(value, |s| s.trim().to_string()),
            Transform::TrimStart => apply_to_text(value, |s| s.trim_start().to_string()),
            Transform::TrimEnd => apply_to_text(value, |s| s.trim_end().to_string()),
            Transform::Lowercase => apply_to_text(value, str::to_lowercase),
            Transform::Uppercase => apply_to_text(value, str::to_uppercase),
            Transform::Capitalize => apply_to_text(value, capitalize_words),
            Transform::CollapseWhitespace => apply_to_text(value, collapse_whitespace),
            Transform::RemoveWhitespace => apply_to_text(value, |s| {
                s.chars().filter(|c| !c.is_whitespace()).collect()
            }),
            Transform::Replace { from, to } => {
                apply_to_text(value, |s| s.replace(from.as_str(), to.as_str()))
            }
            Transform::Truncate { max_length, suffix } => apply_to_text(value, |s| {
                truncate_string(s, *max_length, suffix.as_deref())
            }),
            Transform::Pad {
                min_length,
                char,
                start,
            } => apply_to_text(value, |s| pad_string(s, *min_length, *char, *start)),

            // Numeric transformations
            Transform::Abs => apply_to_number(value, f64::abs),
            Transform::Ceil => apply_to_number(value, f64::ceil),
            Transform::Floor => apply_to_number(value, f64::floor),
            Transform::Round => apply_to_number(value, f64::round),
            Transform::RoundTo { decimals } => {
                let factor = 10_f64.powi(i32::try_from(*decimals).unwrap_or(i32::MAX));
                apply_to_number(value, |n| (n * factor).round() / factor)
            }
            Transform::Clamp { min, max } => apply_to_number(value, |n| n.clamp(*min, *max)),

            // Null handling
            Transform::DefaultTo { value: default } => {
                if matches!(value, Value::Null) {
                    default.clone()
                } else {
                    value.clone()
                }
            }
            Transform::NullIf { value: match_value } => {
                if value == match_value {
                    Value::Null
                } else {
                    value.clone()
                }
            }
            Transform::NullIfEmpty => {
                if let Some(s) = value.as_text() {
                    if s.is_empty() {
                        Value::Null
                    } else {
                        value.clone()
                    }
                } else {
                    value.clone()
                }
            }

            // Composition
            Transform::Sequence(transforms) => {
                transforms.iter().fold(value.clone(), |v, t| t.apply(&v))
            }
        }
    }

    /// Create a trim transformation.
    #[must_use]
    pub fn trim() -> Self {
        Self::Trim
    }

    /// Create a lowercase transformation.
    #[must_use]
    pub fn lowercase() -> Self {
        Self::Lowercase
    }

    /// Create an uppercase transformation.
    #[must_use]
    pub fn uppercase() -> Self {
        Self::Uppercase
    }

    /// Create a capitalize transformation.
    #[must_use]
    pub fn capitalize() -> Self {
        Self::Capitalize
    }

    /// Create a clamp transformation.
    #[must_use]
    pub fn clamp(min: f64, max: f64) -> Self {
        Self::Clamp { min, max }
    }

    /// Create a round-to-decimals transformation.
    #[must_use]
    pub fn round_to(decimals: u32) -> Self {
        Self::RoundTo { decimals }
    }

    /// Create a default value transformation.
    #[must_use]
    pub fn default_to(value: Value) -> Self {
        Self::DefaultTo { value }
    }

    /// Create a truncate transformation.
    #[must_use]
    pub fn truncate(max_length: usize) -> Self {
        Self::Truncate {
            max_length,
            suffix: None,
        }
    }

    /// Create a truncate transformation with ellipsis suffix.
    #[must_use]
    pub fn truncate_with_ellipsis(max_length: usize) -> Self {
        Self::Truncate {
            max_length,
            suffix: Some("...".to_string()),
        }
    }

    /// Create a replace transformation.
    #[must_use]
    pub fn replace(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self::Replace {
            from: from.into(),
            to: to.into(),
        }
    }

    /// Chain this transformation with another.
    #[must_use]
    pub fn then(self, other: Transform) -> Self {
        match self {
            Transform::Sequence(mut transforms) => {
                transforms.push(other);
                Transform::Sequence(transforms)
            }
            _ => Transform::Sequence(vec![self, other]),
        }
    }
}

/// Apply a transformation function to text values.
fn apply_to_text<F>(value: &Value, f: F) -> Value
where
    F: FnOnce(&str) -> String,
{
    if let Some(s) = value.as_text() {
        Value::text(f(s))
    } else {
        value.clone()
    }
}

/// Apply a transformation function to numeric values.
#[allow(clippy::cast_precision_loss)]
fn apply_to_number<F>(value: &Value, f: F) -> Value
where
    F: FnOnce(f64) -> f64,
{
    match value {
        Value::Int(n) => {
            let result = f(*n as f64);
            // If result is a whole number and fits in i64, return Int
            if result.fract() == 0.0 {
                #[allow(clippy::cast_possible_truncation)]
                if result >= i64::MIN as f64 && result <= i64::MAX as f64 {
                    return Value::Int(result as i64);
                }
            }
            Value::Float(result)
        }
        Value::Float(n) => {
            let result = f(*n);
            // If result is a whole number, could return Int, but Float is expected
            Value::Float(result)
        }
        _ => value.clone(),
    }
}

/// Capitalize the first letter of each word.
fn capitalize_words(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let upper: String = first.to_uppercase().collect();
                    let rest: String = chars.collect();
                    format!("{upper}{rest}")
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Collapse consecutive whitespace into single space.
fn collapse_whitespace(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut prev_was_space = false;

    for c in s.chars() {
        if c.is_whitespace() {
            if !prev_was_space {
                result.push(' ');
                prev_was_space = true;
            }
        } else {
            result.push(c);
            prev_was_space = false;
        }
    }

    result
}

/// Truncate string to maximum length.
fn truncate_string(s: &str, max_length: usize, suffix: Option<&str>) -> String {
    if s.chars().count() <= max_length {
        return s.to_string();
    }

    match suffix {
        Some(suf) => {
            let suffix_len = suf.chars().count();

            // If suffix alone is longer than max_length, truncate the suffix itself
            if suffix_len >= max_length {
                return suf.chars().take(max_length).collect();
            }

            // Otherwise, take (max_length - suffix_len) chars from string + suffix
            let target_len = max_length - suffix_len;
            let truncated: String = s.chars().take(target_len).collect();
            format!("{truncated}{suf}")
        }
        None => s.chars().take(max_length).collect(),
    }
}

/// Pad string to minimum length.
fn pad_string(s: &str, min_length: usize, pad_char: char, at_start: bool) -> String {
    let current_len = s.chars().count();
    if current_len >= min_length {
        return s.to_string();
    }

    let padding: String = std::iter::repeat_n(pad_char, min_length - current_len).collect();

    if at_start {
        format!("{padding}{s}")
    } else {
        format!("{s}{padding}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === String Transformation Tests ===

    #[test]
    fn test_trim() {
        let value = Value::text("  hello  ");
        assert_eq!(Transform::Trim.apply(&value).as_text(), Some("hello"));
    }

    #[test]
    fn test_trim_start() {
        let value = Value::text("  hello  ");
        assert_eq!(
            Transform::TrimStart.apply(&value).as_text(),
            Some("hello  ")
        );
    }

    #[test]
    fn test_trim_end() {
        let value = Value::text("  hello  ");
        assert_eq!(Transform::TrimEnd.apply(&value).as_text(), Some("  hello"));
    }

    #[test]
    fn test_lowercase() {
        let value = Value::text("HELLO World");
        assert_eq!(
            Transform::Lowercase.apply(&value).as_text(),
            Some("hello world")
        );
    }

    #[test]
    fn test_uppercase() {
        let value = Value::text("hello World");
        assert_eq!(
            Transform::Uppercase.apply(&value).as_text(),
            Some("HELLO WORLD")
        );
    }

    #[test]
    fn test_capitalize() {
        let value = Value::text("hello world");
        assert_eq!(
            Transform::Capitalize.apply(&value).as_text(),
            Some("Hello World")
        );
    }

    #[test]
    fn test_collapse_whitespace() {
        let value = Value::text("hello   world\t\nfoo");
        assert_eq!(
            Transform::CollapseWhitespace.apply(&value).as_text(),
            Some("hello world foo")
        );
    }

    #[test]
    fn test_remove_whitespace() {
        let value = Value::text("hello world");
        assert_eq!(
            Transform::RemoveWhitespace.apply(&value).as_text(),
            Some("helloworld")
        );
    }

    #[test]
    fn test_replace() {
        let value = Value::text("hello world");
        let transform = Transform::Replace {
            from: "world".to_string(),
            to: "rust".to_string(),
        };
        assert_eq!(transform.apply(&value).as_text(), Some("hello rust"));
    }

    #[test]
    fn test_truncate() {
        let value = Value::text("hello world");
        let transform = Transform::Truncate {
            max_length: 5,
            suffix: None,
        };
        assert_eq!(transform.apply(&value).as_text(), Some("hello"));
    }

    #[test]
    fn test_truncate_with_suffix() {
        let value = Value::text("hello world");
        let transform = Transform::Truncate {
            max_length: 8,
            suffix: Some("...".to_string()),
        };
        assert_eq!(transform.apply(&value).as_text(), Some("hello..."));
    }

    #[test]
    fn test_truncate_no_op() {
        let value = Value::text("hello");
        let transform = Transform::Truncate {
            max_length: 10,
            suffix: None,
        };
        assert_eq!(transform.apply(&value).as_text(), Some("hello"));
    }

    #[test]
    fn test_truncate_suffix_longer_than_max() {
        // Edge case: suffix itself is longer than max_length
        let value = Value::text("hello world");
        let transform = Transform::Truncate {
            max_length: 2,
            suffix: Some("...".to_string()),
        };
        // Should truncate suffix itself to max_length
        assert_eq!(transform.apply(&value).as_text(), Some(".."));
    }

    #[test]
    fn test_truncate_suffix_equals_max() {
        // Edge case: suffix length equals max_length
        let value = Value::text("hello world");
        let transform = Transform::Truncate {
            max_length: 3,
            suffix: Some("...".to_string()),
        };
        // Should return just the suffix
        assert_eq!(transform.apply(&value).as_text(), Some("..."));
    }

    #[test]
    fn test_pad_start() {
        let value = Value::text("42");
        let transform = Transform::Pad {
            min_length: 5,
            char: '0',
            start: true,
        };
        assert_eq!(transform.apply(&value).as_text(), Some("00042"));
    }

    #[test]
    fn test_pad_end() {
        let value = Value::text("hi");
        let transform = Transform::Pad {
            min_length: 5,
            char: '-',
            start: false,
        };
        assert_eq!(transform.apply(&value).as_text(), Some("hi---"));
    }

    // === Numeric Transformation Tests ===

    #[test]
    fn test_abs_positive() {
        let value = Value::Int(42);
        assert_eq!(Transform::Abs.apply(&value), Value::Int(42));
    }

    #[test]
    fn test_abs_negative() {
        let value = Value::Int(-42);
        assert_eq!(Transform::Abs.apply(&value), Value::Int(42));
    }

    #[test]
    fn test_abs_float() {
        let value = Value::Float(-3.14);
        assert_eq!(Transform::Abs.apply(&value), Value::Float(3.14));
    }

    #[test]
    fn test_ceil() {
        let value = Value::Float(3.2);
        assert_eq!(Transform::Ceil.apply(&value), Value::Float(4.0));
    }

    #[test]
    fn test_floor() {
        let value = Value::Float(3.8);
        assert_eq!(Transform::Floor.apply(&value), Value::Float(3.0));
    }

    #[test]
    fn test_round() {
        let value = Value::Float(3.5);
        assert_eq!(Transform::Round.apply(&value), Value::Float(4.0));
    }

    #[test]
    fn test_round_to() {
        let value = Value::Float(3.14159);
        let transform = Transform::RoundTo { decimals: 2 };
        assert_eq!(transform.apply(&value), Value::Float(3.14));
    }

    #[test]
    fn test_clamp_below() {
        let value = Value::Int(-10);
        let transform = Transform::Clamp {
            min: 0.0,
            max: 100.0,
        };
        assert_eq!(transform.apply(&value), Value::Int(0));
    }

    #[test]
    fn test_clamp_above() {
        let value = Value::Int(150);
        let transform = Transform::Clamp {
            min: 0.0,
            max: 100.0,
        };
        assert_eq!(transform.apply(&value), Value::Int(100));
    }

    #[test]
    fn test_clamp_in_range() {
        let value = Value::Int(50);
        let transform = Transform::Clamp {
            min: 0.0,
            max: 100.0,
        };
        assert_eq!(transform.apply(&value), Value::Int(50));
    }

    // === Null Handling Tests ===

    #[test]
    fn test_default_to_null() {
        let value = Value::Null;
        let transform = Transform::DefaultTo {
            value: Value::text("default"),
        };
        assert_eq!(transform.apply(&value).as_text(), Some("default"));
    }

    #[test]
    fn test_default_to_non_null() {
        let value = Value::text("hello");
        let transform = Transform::DefaultTo {
            value: Value::text("default"),
        };
        assert_eq!(transform.apply(&value).as_text(), Some("hello"));
    }

    #[test]
    fn test_null_if_match() {
        let value = Value::text("");
        let transform = Transform::NullIf {
            value: Value::text(""),
        };
        assert_eq!(transform.apply(&value), Value::Null);
    }

    #[test]
    fn test_null_if_no_match() {
        let value = Value::text("hello");
        let transform = Transform::NullIf {
            value: Value::text(""),
        };
        assert_eq!(transform.apply(&value).as_text(), Some("hello"));
    }

    #[test]
    fn test_null_if_empty() {
        let value = Value::text("");
        assert_eq!(Transform::NullIfEmpty.apply(&value), Value::Null);
    }

    #[test]
    fn test_null_if_empty_non_empty() {
        let value = Value::text("hello");
        assert_eq!(
            Transform::NullIfEmpty.apply(&value).as_text(),
            Some("hello")
        );
    }

    // === Composition Tests ===

    #[test]
    fn test_sequence() {
        let value = Value::text("  HELLO WORLD  ");
        let transform = Transform::Sequence(vec![
            Transform::Trim,
            Transform::Lowercase,
            Transform::Capitalize,
        ]);
        assert_eq!(transform.apply(&value).as_text(), Some("Hello World"));
    }

    #[test]
    fn test_then_chaining() {
        let transform = Transform::Trim
            .then(Transform::Lowercase)
            .then(Transform::Capitalize);

        let value = Value::text("  HELLO WORLD  ");
        assert_eq!(transform.apply(&value).as_text(), Some("Hello World"));
    }

    // === Non-Applicable Type Tests ===

    #[test]
    fn test_string_transform_on_number() {
        let value = Value::Int(42);
        assert_eq!(Transform::Trim.apply(&value), Value::Int(42));
    }

    #[test]
    fn test_number_transform_on_string() {
        let value = Value::text("hello");
        assert_eq!(Transform::Abs.apply(&value).as_text(), Some("hello"));
    }

    // === Convenience Method Tests ===

    #[test]
    fn test_convenience_methods() {
        assert_eq!(Transform::trim(), Transform::Trim);
        assert_eq!(Transform::lowercase(), Transform::Lowercase);
        assert_eq!(Transform::uppercase(), Transform::Uppercase);
        assert_eq!(Transform::capitalize(), Transform::Capitalize);
        assert_eq!(
            Transform::clamp(0.0, 100.0),
            Transform::Clamp {
                min: 0.0,
                max: 100.0
            }
        );
        assert_eq!(Transform::round_to(2), Transform::RoundTo { decimals: 2 });
        assert_eq!(
            Transform::default_to(Value::Int(0)),
            Transform::DefaultTo {
                value: Value::Int(0)
            }
        );
        assert_eq!(
            Transform::truncate(10),
            Transform::Truncate {
                max_length: 10,
                suffix: None
            }
        );
        assert_eq!(
            Transform::truncate_with_ellipsis(10),
            Transform::Truncate {
                max_length: 10,
                suffix: Some("...".to_string())
            }
        );
        assert_eq!(
            Transform::replace("a", "b"),
            Transform::Replace {
                from: "a".to_string(),
                to: "b".to_string()
            }
        );
    }
}
