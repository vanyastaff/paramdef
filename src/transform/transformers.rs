//! Built-in transformers.
//!
//! This module provides struct-based transformers for complex transformation
//! scenarios that benefit from configuration or state.

use crate::Value;

use super::Transformer;

/// Clamps numeric values to a range.
///
/// # Example
///
/// ```
/// use paramdef::transform::{Clamp, Transformer};
/// use paramdef::Value;
///
/// let clamp = Clamp::new(0.0, 100.0);
///
/// assert_eq!(clamp.transform(&Value::Int(-10)), Value::Int(0));
/// assert_eq!(clamp.transform(&Value::Int(50)), Value::Int(50));
/// assert_eq!(clamp.transform(&Value::Int(150)), Value::Int(100));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Clamp {
    /// Minimum value.
    pub min: f64,
    /// Maximum value.
    pub max: f64,
}

impl Clamp {
    /// Creates a new clamp transformer.
    #[must_use]
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }
}

impl Transformer for Clamp {
    fn name(&self) -> &'static str {
        "clamp"
    }

    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    fn transform(&self, value: &Value) -> Value {
        match value {
            Value::Int(n) => {
                let clamped = (*n as f64).clamp(self.min, self.max);
                if clamped >= i64::MIN as f64 && clamped <= i64::MAX as f64 {
                    Value::Int(clamped as i64)
                } else {
                    Value::Float(clamped)
                }
            }
            Value::Float(n) => Value::Float(n.clamp(self.min, self.max)),
            _ => value.clone(),
        }
    }

    fn applies_to(&self, value: &Value) -> bool {
        matches!(value, Value::Int(_) | Value::Float(_))
    }
}

/// Rounds numeric values to a specified number of decimal places.
///
/// # Example
///
/// ```
/// use paramdef::transform::{Round, Transformer};
/// use paramdef::Value;
///
/// let round = Round::new(2);
///
/// assert_eq!(round.transform(&Value::Float(3.14159)), Value::Float(3.14));
/// assert_eq!(round.transform(&Value::Float(2.5)), Value::Float(2.5));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Round {
    /// Number of decimal places.
    pub decimals: u32,
}

impl Round {
    /// Creates a new round transformer.
    #[must_use]
    pub fn new(decimals: u32) -> Self {
        Self { decimals }
    }

    /// Creates a transformer that rounds to whole numbers.
    #[must_use]
    pub fn whole() -> Self {
        Self { decimals: 0 }
    }
}

impl Transformer for Round {
    fn name(&self) -> &'static str {
        "round"
    }

    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    fn transform(&self, value: &Value) -> Value {
        let factor = 10_f64.powi(i32::try_from(self.decimals).unwrap_or(i32::MAX));

        match value {
            Value::Int(n) => {
                // Integers don't need rounding for decimal places
                if self.decimals == 0 {
                    value.clone()
                } else {
                    let rounded = ((*n as f64) * factor).round() / factor;
                    if rounded.fract() == 0.0
                        && rounded >= i64::MIN as f64
                        && rounded <= i64::MAX as f64
                    {
                        Value::Int(rounded as i64)
                    } else {
                        Value::Float(rounded)
                    }
                }
            }
            Value::Float(n) => {
                let rounded = (n * factor).round() / factor;
                Value::Float(rounded)
            }
            _ => value.clone(),
        }
    }

    fn applies_to(&self, value: &Value) -> bool {
        matches!(value, Value::Int(_) | Value::Float(_))
    }
}

/// Truncates text to a maximum length.
///
/// # Example
///
/// ```
/// use paramdef::transform::{Truncate, Transformer};
/// use paramdef::Value;
///
/// let truncate = Truncate::new(10).with_suffix("...");
///
/// let value = Value::text("Hello, World!");
/// assert_eq!(truncate.transform(&value).as_text(), Some("Hello, ..."));
/// ```
#[derive(Debug, Clone)]
pub struct Truncate {
    /// Maximum length in characters.
    pub max_length: usize,
    /// Suffix to add if truncated.
    pub suffix: Option<String>,
}

impl Truncate {
    /// Creates a new truncate transformer.
    #[must_use]
    pub fn new(max_length: usize) -> Self {
        Self {
            max_length,
            suffix: None,
        }
    }

    /// Adds a suffix to show when text is truncated.
    #[must_use]
    pub fn with_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    /// Adds ellipsis suffix ("...").
    #[must_use]
    pub fn with_ellipsis(self) -> Self {
        self.with_suffix("...")
    }
}

impl Transformer for Truncate {
    fn name(&self) -> &'static str {
        "truncate"
    }

    fn transform(&self, value: &Value) -> Value {
        if let Some(s) = value.as_text() {
            if s.chars().count() <= self.max_length {
                return value.clone();
            }

            let suffix_len = self.suffix.as_ref().map_or(0, |s| s.chars().count());
            let target_len = self.max_length.saturating_sub(suffix_len);
            let truncated: String = s.chars().take(target_len).collect();

            match &self.suffix {
                Some(suf) => Value::text(format!("{truncated}{suf}")),
                None => Value::text(truncated),
            }
        } else {
            value.clone()
        }
    }

    fn applies_to(&self, value: &Value) -> bool {
        value.as_text().is_some()
    }
}

/// Replaces substrings in text.
///
/// # Example
///
/// ```
/// use paramdef::transform::{Replace, Transformer};
/// use paramdef::Value;
///
/// let replace = Replace::new("foo", "bar");
///
/// let value = Value::text("foo baz foo");
/// assert_eq!(replace.transform(&value).as_text(), Some("bar baz bar"));
/// ```
#[derive(Debug, Clone)]
pub struct Replace {
    /// Pattern to find.
    pub from: String,
    /// Replacement string.
    pub to: String,
}

impl Replace {
    /// Creates a new replace transformer.
    #[must_use]
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
        }
    }
}

impl Transformer for Replace {
    fn name(&self) -> &'static str {
        "replace"
    }

    fn transform(&self, value: &Value) -> Value {
        if let Some(s) = value.as_text() {
            Value::text(s.replace(&self.from, &self.to))
        } else {
            value.clone()
        }
    }

    fn applies_to(&self, value: &Value) -> bool {
        value.as_text().is_some()
    }
}

/// Provides a default value for Null.
///
/// # Example
///
/// ```
/// use paramdef::transform::{Default, Transformer};
/// use paramdef::Value;
///
/// let default = Default::new(Value::text("N/A"));
///
/// assert_eq!(default.transform(&Value::Null).as_text(), Some("N/A"));
/// assert_eq!(default.transform(&Value::text("hello")).as_text(), Some("hello"));
/// ```
#[derive(Debug, Clone)]
pub struct Default {
    /// Default value to use for Null.
    pub value: Value,
}

impl Default {
    /// Creates a new default transformer.
    #[must_use]
    pub fn new(value: Value) -> Self {
        Self { value }
    }

    /// Creates a default transformer with an integer value.
    #[must_use]
    pub fn int(value: i64) -> Self {
        Self::new(Value::Int(value))
    }

    /// Creates a default transformer with a float value.
    #[must_use]
    pub fn float(value: f64) -> Self {
        Self::new(Value::Float(value))
    }

    /// Creates a default transformer with a text value.
    #[must_use]
    pub fn text(value: &str) -> Self {
        Self::new(Value::text(value))
    }

    /// Creates a default transformer with a boolean value.
    #[must_use]
    pub fn bool(value: bool) -> Self {
        Self::new(Value::Bool(value))
    }
}

impl Transformer for Default {
    fn name(&self) -> &'static str {
        "default"
    }

    fn transform(&self, value: &Value) -> Value {
        if matches!(value, Value::Null) {
            self.value.clone()
        } else {
            value.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === Clamp Tests ===

    #[test]
    fn test_clamp_int_below() {
        let clamp = Clamp::new(0.0, 100.0);
        assert_eq!(clamp.transform(&Value::Int(-10)), Value::Int(0));
    }

    #[test]
    fn test_clamp_int_above() {
        let clamp = Clamp::new(0.0, 100.0);
        assert_eq!(clamp.transform(&Value::Int(150)), Value::Int(100));
    }

    #[test]
    fn test_clamp_int_in_range() {
        let clamp = Clamp::new(0.0, 100.0);
        assert_eq!(clamp.transform(&Value::Int(50)), Value::Int(50));
    }

    #[test]
    fn test_clamp_float() {
        let clamp = Clamp::new(0.0, 1.0);
        assert_eq!(clamp.transform(&Value::Float(1.5)), Value::Float(1.0));
    }

    #[test]
    fn test_clamp_non_numeric() {
        let clamp = Clamp::new(0.0, 100.0);
        let value = Value::text("hello");
        assert_eq!(clamp.transform(&value), value);
    }

    #[test]
    fn test_clamp_applies_to() {
        let clamp = Clamp::new(0.0, 100.0);
        assert!(clamp.applies_to(&Value::Int(42)));
        assert!(clamp.applies_to(&Value::Float(3.14)));
        assert!(!clamp.applies_to(&Value::text("hello")));
    }

    // === Round Tests ===

    #[test]
    fn test_round_float() {
        let round = Round::new(2);
        assert_eq!(
            round.transform(&Value::Float(3.14159)),
            Value::Float(3.14)
        );
    }

    #[test]
    fn test_round_whole() {
        let round = Round::whole();
        assert_eq!(round.transform(&Value::Float(3.7)), Value::Float(4.0));
    }

    #[test]
    fn test_round_int_no_change() {
        let round = Round::new(2);
        assert_eq!(round.transform(&Value::Int(42)), Value::Int(42));
    }

    // === Truncate Tests ===

    #[test]
    fn test_truncate_long_string() {
        let truncate = Truncate::new(5);
        let value = Value::text("Hello, World!");
        assert_eq!(truncate.transform(&value).as_text(), Some("Hello"));
    }

    #[test]
    fn test_truncate_with_suffix() {
        let truncate = Truncate::new(8).with_suffix("...");
        let value = Value::text("Hello, World!");
        assert_eq!(truncate.transform(&value).as_text(), Some("Hello..."));
    }

    #[test]
    fn test_truncate_short_string() {
        let truncate = Truncate::new(20);
        let value = Value::text("Hello");
        assert_eq!(truncate.transform(&value).as_text(), Some("Hello"));
    }

    #[test]
    fn test_truncate_with_ellipsis() {
        let truncate = Truncate::new(10).with_ellipsis();
        let value = Value::text("This is a long string");
        assert_eq!(truncate.transform(&value).as_text(), Some("This is..."));
    }

    // === Replace Tests ===

    #[test]
    fn test_replace_single() {
        let replace = Replace::new("foo", "bar");
        let value = Value::text("foo baz");
        assert_eq!(replace.transform(&value).as_text(), Some("bar baz"));
    }

    #[test]
    fn test_replace_multiple() {
        let replace = Replace::new("a", "X");
        let value = Value::text("banana");
        assert_eq!(replace.transform(&value).as_text(), Some("bXnXnX"));
    }

    #[test]
    fn test_replace_no_match() {
        let replace = Replace::new("xyz", "abc");
        let value = Value::text("hello");
        assert_eq!(replace.transform(&value).as_text(), Some("hello"));
    }

    // === Default Tests ===

    #[test]
    fn test_default_null() {
        let default = Default::text("N/A");
        assert_eq!(default.transform(&Value::Null).as_text(), Some("N/A"));
    }

    #[test]
    fn test_default_non_null() {
        let default = Default::text("N/A");
        let value = Value::text("hello");
        assert_eq!(default.transform(&value).as_text(), Some("hello"));
    }

    #[test]
    fn test_default_int() {
        let default = Default::int(0);
        assert_eq!(default.transform(&Value::Null), Value::Int(0));
    }

    #[test]
    fn test_default_float() {
        let default = Default::float(0.0);
        assert_eq!(default.transform(&Value::Null), Value::Float(0.0));
    }

    #[test]
    fn test_default_bool() {
        let default = Default::bool(false);
        assert_eq!(default.transform(&Value::Null), Value::Bool(false));
    }

    // === Name Tests ===

    #[test]
    fn test_transformer_names() {
        assert_eq!(Clamp::new(0.0, 100.0).name(), "clamp");
        assert_eq!(Round::new(2).name(), "round");
        assert_eq!(Truncate::new(10).name(), "truncate");
        assert_eq!(Replace::new("a", "b").name(), "replace");
        assert_eq!(Default::int(0).name(), "default");
    }
}
