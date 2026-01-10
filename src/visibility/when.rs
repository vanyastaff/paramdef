//! Fluent builder for visibility conditions.
//!
//! Provides a convenient API for creating visibility expressions that check
//! other field values.

use crate::core::{Key, SmartStr, Value};
use crate::expr::{Expr, Rule};

/// Fluent builder for visibility conditions.
///
/// Creates visibility rules that check another field's value.
///
/// # Example
///
/// ```ignore
/// use paramdef::visibility::when;
/// use paramdef::core::Value;
///
/// // Check if field equals a value
/// let rule = when("mode").eq(Value::text("advanced"));
///
/// // Check if field is true
/// let rule = when("enabled").is_true();
///
/// // Check string starts with
/// let rule = when("name").starts_with("admin");
///
/// // Combine conditions
/// let rule = when("age").gte(18.0).and(when("verified").is_true());
/// ```
#[must_use]
pub struct When {
    key: Key,
}

impl When {
    /// Create a new visibility builder for the given field.
    pub fn new(key: impl Into<Key>) -> Self {
        Self { key: key.into() }
    }

    // === Value Comparisons ===

    /// Field equals the specified value.
    #[must_use]
    pub fn eq(self, value: Value) -> Rule {
        Rule::field(self.key, Expr::eq(value))
    }

    /// Field does not equal the specified value.
    #[must_use]
    pub fn ne(self, value: Value) -> Rule {
        Rule::field(self.key, Expr::ne(value))
    }

    /// Numeric field is less than threshold.
    #[must_use]
    pub fn lt(self, threshold: f64) -> Rule {
        Rule::field(self.key, Expr::lt(threshold))
    }

    /// Numeric field is greater than threshold.
    #[must_use]
    pub fn gt(self, threshold: f64) -> Rule {
        Rule::field(self.key, Expr::gt(threshold))
    }

    /// Numeric field is less than or equal to threshold.
    #[must_use]
    pub fn lte(self, threshold: f64) -> Rule {
        Rule::field(self.key, Expr::lte(threshold))
    }

    /// Numeric field is greater than or equal to threshold.
    #[must_use]
    pub fn gte(self, threshold: f64) -> Rule {
        Rule::field(self.key, Expr::gte(threshold))
    }

    /// Numeric field is between min and max (inclusive).
    #[must_use]
    pub fn between(self, min: f64, max: f64) -> Rule {
        Rule::field(self.key, Expr::between(min, max))
    }

    // === String Operations ===

    /// Field starts with the specified prefix.
    #[must_use]
    pub fn starts_with(self, prefix: impl Into<SmartStr>) -> Rule {
        Rule::field(self.key, Expr::starts_with(prefix))
    }

    /// Field ends with the specified suffix.
    #[must_use]
    pub fn ends_with(self, suffix: impl Into<SmartStr>) -> Rule {
        Rule::field(self.key, Expr::ends_with(suffix))
    }

    /// Field contains the specified value.
    #[must_use]
    pub fn contains(self, value: Value) -> Rule {
        Rule::field(self.key, Expr::contains(value))
    }

    /// Field matches the regex pattern.
    #[cfg(feature = "validation")]
    #[must_use]
    pub fn matches(self, pattern: impl Into<String>) -> Rule {
        Rule::field(self.key, Expr::matches(pattern))
    }

    // === Length Checks ===

    /// Field length is at least min.
    #[must_use]
    pub fn length_min(self, min: usize) -> Rule {
        Rule::field(self.key, Expr::min_length(min))
    }

    /// Field length is at most max.
    #[must_use]
    pub fn length_max(self, max: usize) -> Rule {
        Rule::field(self.key, Expr::max_length(max))
    }

    /// Field length is exactly len.
    #[must_use]
    pub fn length(self, len: usize) -> Rule {
        Rule::field(self.key, Expr::length(len))
    }

    /// Field length is between min and max (inclusive).
    #[must_use]
    pub fn length_between(self, min: usize, max: usize) -> Rule {
        Rule::field(self.key, Expr::length_between(min, max))
    }

    // === State Checks ===

    /// Field is not null/undefined.
    #[must_use]
    pub fn is_set(self) -> Rule {
        Rule::field(self.key, Expr::is_set())
    }

    /// Field is empty (null, empty string, empty array).
    #[must_use]
    pub fn is_empty(self) -> Rule {
        Rule::field(self.key, Expr::is_empty())
    }

    /// Field is null.
    #[must_use]
    pub fn is_null(self) -> Rule {
        Rule::field(self.key, Expr::is_null())
    }

    /// Field is not empty.
    #[must_use]
    pub fn is_not_empty(self) -> Rule {
        Rule::field(self.key, Expr::is_not_empty())
    }

    /// Boolean field is true.
    #[must_use]
    pub fn is_true(self) -> Rule {
        Rule::field(self.key, Expr::is_true())
    }

    /// Boolean field is false.
    #[must_use]
    pub fn is_false(self) -> Rule {
        Rule::field(self.key, Expr::is_false())
    }

    /// Field passes validation.
    #[must_use]
    pub fn is_valid(self) -> Rule {
        Rule::field(self.key, Expr::is_valid())
    }

    // === Set Operations ===

    /// Field value is one of the allowed values.
    #[must_use]
    pub fn one_of(self, values: Vec<Value>) -> Rule {
        Rule::field(self.key, Expr::one_of(values))
    }
}

/// Create a visibility condition for a field.
///
/// This is a convenience function that creates a [`When`] builder.
///
/// # Example
///
/// ```ignore
/// use paramdef::visibility::when;
/// use paramdef::core::Value;
///
/// let rule = when("mode").eq(Value::text("advanced"));
/// ```
pub fn when(key: impl Into<Key>) -> When {
    When::new(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_when_eq() {
        let rule = when("mode").eq(Value::text("advanced"));
        assert!(rule.is_field());
        assert_eq!(rule.field_key(), Some(&Key::from("mode")));
    }

    #[test]
    fn test_when_is_true() {
        let rule = when("enabled").is_true();
        assert!(rule.is_field());
        assert_eq!(rule.field_key(), Some(&Key::from("enabled")));
    }

    #[test]
    fn test_when_starts_with() {
        let rule = when("name").starts_with("admin");
        assert!(rule.is_field());
        assert_eq!(rule.field_key(), Some(&Key::from("name")));
    }

    #[test]
    fn test_when_between() {
        let rule = when("age").between(18.0, 65.0);
        assert!(rule.is_field());
        assert_eq!(rule.field_key(), Some(&Key::from("age")));
    }
}
