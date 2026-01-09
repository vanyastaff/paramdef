//! Built-in validators for common use cases.
//!
//! These validators implement the [`Validator`] trait and provide
//! reusable validation logic beyond what [`Expr`] can express.
//!
//! # Available Validators
//!
//! - [`Required`] - Value must be present and non-empty
//! - [`Length`] - String length constraints
//! - [`Range`] - Numeric range constraints
//! - [`Pattern`] - Regex pattern matching
//! - [`Unique`] - Cross-field uniqueness check
//! - [`PasswordStrength`] - Password complexity validation
//! - [`Conditional`] - Conditional validation based on other fields

#![allow(clippy::cast_precision_loss)]

use super::context::ValidationContext;
use super::result::{Error, ValidationResult};
use super::traits::Validator;
use crate::core::Value;

/// Validates that a value is present and non-empty.
#[derive(Debug, Clone, Copy, Default)]
pub struct Required;

impl Validator for Required {
    fn validate(&self, value: &Value, _ctx: &ValidationContext<'_>) -> ValidationResult {
        match value {
            Value::Null => Err(Error::required().into()),
            Value::Text(s) if s.is_empty() => Err(Error::required().into()),
            Value::Array(a) if a.is_empty() => Err(Error::required().into()),
            Value::Object(o) if o.is_empty() => Err(Error::required().into()),
            _ => Ok(()),
        }
    }

    fn name(&self) -> &'static str {
        "Required"
    }
}

/// Validates string length is within bounds.
#[derive(Debug, Clone, Copy)]
pub struct Length {
    /// Minimum length (inclusive).
    pub min: Option<usize>,
    /// Maximum length (inclusive).
    pub max: Option<usize>,
}

impl Length {
    /// Creates a length validator with minimum constraint.
    #[must_use]
    pub const fn min(min: usize) -> Self {
        Self {
            min: Some(min),
            max: None,
        }
    }

    /// Creates a length validator with maximum constraint.
    #[must_use]
    pub const fn max(max: usize) -> Self {
        Self {
            min: None,
            max: Some(max),
        }
    }

    /// Creates a length validator with both constraints.
    #[must_use]
    pub const fn between(min: usize, max: usize) -> Self {
        Self {
            min: Some(min),
            max: Some(max),
        }
    }

    /// Creates a length validator for exact length.
    #[must_use]
    pub const fn exact(len: usize) -> Self {
        Self {
            min: Some(len),
            max: Some(len),
        }
    }
}

impl Validator for Length {
    fn validate(&self, value: &Value, _ctx: &ValidationContext<'_>) -> ValidationResult {
        let len = match value {
            Value::Text(s) => s.chars().count(),
            Value::Array(a) => a.len(),
            _ => return Ok(()),
        };

        if let Some(min) = self.min {
            if len < min {
                return Err(Error::min_length(min, len).into());
            }
        }

        if let Some(max) = self.max {
            if len > max {
                return Err(Error::max_length(max, len).into());
            }
        }

        Ok(())
    }

    fn name(&self) -> &'static str {
        "Length"
    }
}

/// Validates numeric values are within a range.
#[derive(Debug, Clone, Copy)]
pub struct Range {
    /// Minimum value (inclusive unless `exclusive_min` is true).
    pub min: Option<f64>,
    /// Maximum value (inclusive unless `exclusive_max` is true).
    pub max: Option<f64>,
    /// Whether minimum is exclusive.
    pub exclusive_min: bool,
    /// Whether maximum is exclusive.
    pub exclusive_max: bool,
}

impl Range {
    /// Creates a range validator with minimum constraint.
    #[must_use]
    pub const fn min(min: f64) -> Self {
        Self {
            min: Some(min),
            max: None,
            exclusive_min: false,
            exclusive_max: false,
        }
    }

    /// Creates a range validator with maximum constraint.
    #[must_use]
    pub const fn max(max: f64) -> Self {
        Self {
            min: None,
            max: Some(max),
            exclusive_min: false,
            exclusive_max: false,
        }
    }

    /// Creates a range validator with both constraints (inclusive).
    #[must_use]
    pub const fn between(min: f64, max: f64) -> Self {
        Self {
            min: Some(min),
            max: Some(max),
            exclusive_min: false,
            exclusive_max: false,
        }
    }

    /// Creates an exclusive range (min < value < max).
    #[must_use]
    pub const fn exclusive(min: f64, max: f64) -> Self {
        Self {
            min: Some(min),
            max: Some(max),
            exclusive_min: true,
            exclusive_max: true,
        }
    }
}

impl Validator for Range {
    fn validate(&self, value: &Value, _ctx: &ValidationContext<'_>) -> ValidationResult {
        let num = match value {
            Value::Int(n) => *n as f64,
            Value::Float(n) => *n,
            _ => return Ok(()),
        };

        if let Some(min) = self.min {
            if self.exclusive_min {
                if num <= min {
                    return Err(Error::exclusive_min(min, num).into());
                }
            } else if num < min {
                return Err(Error::min_value(min, num).into());
            }
        }

        if let Some(max) = self.max {
            if self.exclusive_max {
                if num >= max {
                    return Err(Error::exclusive_max(max, num).into());
                }
            } else if num > max {
                return Err(Error::max_value(max, num).into());
            }
        }

        Ok(())
    }

    fn name(&self) -> &'static str {
        "Range"
    }
}

/// Validates that a value matches another field.
#[derive(Debug, Clone)]
pub struct Match {
    /// Key of the field to match against.
    pub other_key: crate::core::SmartStr,
    /// Error message if values don't match.
    pub message: Option<crate::core::SmartStr>,
}

impl Match {
    /// Creates a match validator.
    #[must_use]
    pub fn new(other_key: impl Into<crate::core::SmartStr>) -> Self {
        Self {
            other_key: other_key.into(),
            message: None,
        }
    }

    /// Sets a custom error message.
    #[must_use]
    pub fn with_message(mut self, message: impl Into<crate::core::SmartStr>) -> Self {
        self.message = Some(message.into());
        self
    }
}

impl Validator for Match {
    fn validate(&self, value: &Value, ctx: &ValidationContext<'_>) -> ValidationResult {
        if let Some(other) = ctx.get(&self.other_key) {
            if value != other {
                let message = self
                    .message
                    .clone()
                    .unwrap_or_else(|| format!("Value must match {}", self.other_key).into());
                return Err(Error::custom("match", message).into());
            }
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Match"
    }
}

/// Validates password strength.
#[derive(Debug, Clone, Copy)]
#[allow(clippy::struct_excessive_bools)]
pub struct PasswordStrength {
    /// Minimum length required.
    pub min_length: usize,
    /// Require at least one uppercase letter.
    pub require_uppercase: bool,
    /// Require at least one lowercase letter.
    pub require_lowercase: bool,
    /// Require at least one digit.
    pub require_digit: bool,
    /// Require at least one special character.
    pub require_special: bool,
}

impl Default for PasswordStrength {
    fn default() -> Self {
        Self {
            min_length: 8,
            require_uppercase: true,
            require_lowercase: true,
            require_digit: true,
            require_special: false,
        }
    }
}

impl PasswordStrength {
    /// Creates a password strength validator with default settings.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets minimum length.
    #[must_use]
    pub const fn min_length(mut self, len: usize) -> Self {
        self.min_length = len;
        self
    }

    /// Requires uppercase letters.
    #[must_use]
    pub const fn require_uppercase(mut self, required: bool) -> Self {
        self.require_uppercase = required;
        self
    }

    /// Requires lowercase letters.
    #[must_use]
    pub const fn require_lowercase(mut self, required: bool) -> Self {
        self.require_lowercase = required;
        self
    }

    /// Requires digits.
    #[must_use]
    pub const fn require_digit(mut self, required: bool) -> Self {
        self.require_digit = required;
        self
    }

    /// Requires special characters.
    #[must_use]
    pub const fn require_special(mut self, required: bool) -> Self {
        self.require_special = required;
        self
    }
}

impl Validator for PasswordStrength {
    fn validate(&self, value: &Value, _ctx: &ValidationContext<'_>) -> ValidationResult {
        let password = match value {
            Value::Text(s) => s.as_str(),
            _ => return Ok(()),
        };

        if password.len() < self.min_length {
            return Err(Error::custom(
                "password_length",
                format!("Password must be at least {} characters", self.min_length),
            )
            .into());
        }

        if self.require_uppercase && !password.chars().any(char::is_uppercase) {
            return Err(Error::custom(
                "password_uppercase",
                "Password must contain at least one uppercase letter",
            )
            .into());
        }

        if self.require_lowercase && !password.chars().any(char::is_lowercase) {
            return Err(Error::custom(
                "password_lowercase",
                "Password must contain at least one lowercase letter",
            )
            .into());
        }

        if self.require_digit && !password.chars().any(|c| c.is_ascii_digit()) {
            return Err(Error::custom(
                "password_digit",
                "Password must contain at least one digit",
            )
            .into());
        }

        if self.require_special
            && !password
                .chars()
                .any(|c| !c.is_alphanumeric() && !c.is_whitespace())
        {
            return Err(Error::custom(
                "password_special",
                "Password must contain at least one special character",
            )
            .into());
        }

        Ok(())
    }

    fn name(&self) -> &'static str {
        "PasswordStrength"
    }
}

/// Validates conditionally based on another field's value.
#[derive(Debug)]
pub struct When<V: Validator> {
    /// Key of the field to check.
    condition_key: crate::core::SmartStr,
    /// Expected value of the condition field.
    expected_value: Value,
    /// Validator to apply when condition is met.
    then_validator: V,
}

impl<V: Validator> When<V> {
    /// Creates a conditional validator.
    #[must_use]
    pub fn new(
        condition_key: impl Into<crate::core::SmartStr>,
        expected_value: Value,
        then_validator: V,
    ) -> Self {
        Self {
            condition_key: condition_key.into(),
            expected_value,
            then_validator,
        }
    }
}

impl<V: Validator> Validator for When<V> {
    fn validate(&self, value: &Value, ctx: &ValidationContext<'_>) -> ValidationResult {
        // Check if condition is met
        if let Some(condition_value) = ctx.get(&self.condition_key) {
            if condition_value == &self.expected_value {
                return self.then_validator.validate(value, ctx);
            }
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "When"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::Schema;
    use crate::types::leaf::Text;
    use crate::validation::context::NoValues;
    use std::collections::HashMap;
    use std::sync::Arc;

    fn create_test_context<'a>(
        key: &'a crate::core::Key,
        schema: &'a Arc<Schema>,
        values: &'a dyn super::super::context::ValueAccess,
    ) -> ValidationContext<'a> {
        ValidationContext::new(key, schema, values)
    }

    #[test]
    fn test_required() {
        let schema = Arc::new(
            Schema::builder()
                .parameter(Text::builder("test").build())
                .build(),
        );
        let key = "test".into();
        let values = NoValues;
        let ctx = create_test_context(&key, &schema, &values);

        let v = Required;
        assert!(v.validate(&Value::text("hello"), &ctx).is_ok());
        assert!(v.validate(&Value::Int(0), &ctx).is_ok());
        assert!(v.validate(&Value::Null, &ctx).is_err());
        assert!(v.validate(&Value::text(""), &ctx).is_err());
    }

    #[test]
    fn test_length() {
        let schema = Arc::new(
            Schema::builder()
                .parameter(Text::builder("test").build())
                .build(),
        );
        let key = "test".into();
        let values = NoValues;
        let ctx = create_test_context(&key, &schema, &values);

        let v = Length::between(3, 10);
        assert!(v.validate(&Value::text("abc"), &ctx).is_ok());
        assert!(v.validate(&Value::text("abcdefghij"), &ctx).is_ok());
        assert!(v.validate(&Value::text("ab"), &ctx).is_err());
        assert!(v.validate(&Value::text("abcdefghijk"), &ctx).is_err());
    }

    #[test]
    fn test_range() {
        let schema = Arc::new(
            Schema::builder()
                .parameter(Text::builder("test").build())
                .build(),
        );
        let key = "test".into();
        let values = NoValues;
        let ctx = create_test_context(&key, &schema, &values);

        let v = Range::between(0.0, 100.0);
        assert!(v.validate(&Value::Int(50), &ctx).is_ok());
        assert!(v.validate(&Value::Int(0), &ctx).is_ok());
        assert!(v.validate(&Value::Int(100), &ctx).is_ok());
        assert!(v.validate(&Value::Int(-1), &ctx).is_err());
        assert!(v.validate(&Value::Int(101), &ctx).is_err());

        let v = Range::exclusive(0.0, 100.0);
        assert!(v.validate(&Value::Int(50), &ctx).is_ok());
        assert!(v.validate(&Value::Int(0), &ctx).is_err());
        assert!(v.validate(&Value::Int(100), &ctx).is_err());
    }

    #[test]
    fn test_match_validator() {
        let schema = Arc::new(
            Schema::builder()
                .parameter(Text::builder("password").build())
                .parameter(Text::builder("password_confirm").build())
                .build(),
        );

        let mut values: HashMap<crate::core::Key, Value> = HashMap::new();
        values.insert("password".into(), Value::text("secret123"));
        values.insert("password_confirm".into(), Value::text("secret123"));

        let key = "password".into();
        let ctx = create_test_context(&key, &schema, &values);

        let v = Match::new("password_confirm");
        assert!(v.validate(&Value::text("secret123"), &ctx).is_ok());

        // Different values
        let mut values2: HashMap<crate::core::Key, Value> = HashMap::new();
        values2.insert("password_confirm".into(), Value::text("different"));

        let ctx2 = create_test_context(&key, &schema, &values2);
        assert!(v.validate(&Value::text("secret123"), &ctx2).is_err());
    }

    #[test]
    fn test_password_strength() {
        let schema = Arc::new(
            Schema::builder()
                .parameter(Text::builder("test").build())
                .build(),
        );
        let key = "test".into();
        let values = NoValues;
        let ctx = create_test_context(&key, &schema, &values);

        let v = PasswordStrength::new();
        assert!(v.validate(&Value::text("Abc12345"), &ctx).is_ok());
        assert!(v.validate(&Value::text("abc"), &ctx).is_err()); // too short
        assert!(v.validate(&Value::text("abcdefgh"), &ctx).is_err()); // no uppercase
        assert!(v.validate(&Value::text("ABCDEFGH"), &ctx).is_err()); // no lowercase
        assert!(v.validate(&Value::text("Abcdefgh"), &ctx).is_err()); // no digit

        let v = PasswordStrength::new().require_special(true);
        assert!(v.validate(&Value::text("Abc12345"), &ctx).is_err()); // no special
        assert!(v.validate(&Value::text("Abc1234!"), &ctx).is_ok());
    }

    #[test]
    fn test_when() {
        let schema = Arc::new(
            Schema::builder()
                .parameter(Text::builder("has_email").build())
                .parameter(Text::builder("email").build())
                .build(),
        );

        let mut values: HashMap<crate::core::Key, Value> = HashMap::new();
        values.insert("has_email".into(), Value::Bool(true));

        let key = "email".into();
        let ctx = create_test_context(&key, &schema, &values);

        // When has_email is true, email is required
        let v = When::new("has_email", Value::Bool(true), Required);

        // With condition met, validation applies
        assert!(v.validate(&Value::Null, &ctx).is_err());
        assert!(v.validate(&Value::text("test@example.com"), &ctx).is_ok());

        // With condition not met, validation skipped
        let mut values2: HashMap<crate::core::Key, Value> = HashMap::new();
        values2.insert("has_email".into(), Value::Bool(false));
        let ctx2 = create_test_context(&key, &schema, &values2);

        assert!(v.validate(&Value::Null, &ctx2).is_ok());
    }
}
