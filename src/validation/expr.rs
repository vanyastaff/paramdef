//! Declarative validation expressions.
//!
//! [`Expr`] provides a serializable, declarative way to define validation rules.
//! Covers ~80% of common validation cases without custom code.
//!
//! # Design Inspiration
//!
//! - **JSON Schema**: `required`, `minLength`, `maxLength`, `pattern`, `minimum`, `maximum`
//! - **Zod**: Chainable validation, `.email()`, `.url()`, `.regex()`
//! - **CEL (Kubernetes)**: Expression-based policy validation
//!
//! # Example
//!
//! ```ignore
//! use paramdef::validation::Expr;
//!
//! // String validations
//! let rules = vec![
//!     Expr::Required,
//!     Expr::MinLength(3),
//!     Expr::MaxLength(50),
//!     Expr::Pattern(r"^[a-zA-Z]+$".into()),
//! ];
//!
//! // Numeric validations
//! let rules = vec![
//!     Expr::Min(0.0),
//!     Expr::Max(100.0),
//!     Expr::MultipleOf(5.0),
//! ];
//!
//! // Composite validations
//! let rules = vec![
//!     Expr::And(vec![Expr::Required, Expr::MinLength(1)]),
//!     Expr::Or(vec![Expr::Email, Expr::Url]),
//! ];
//! ```

// Allow i64 to f64 casts - precision loss is acceptable for validation comparisons
#![allow(clippy::cast_precision_loss)]

use super::result::{Error, ValidationResult};
use crate::core::{SmartStr, Value};

/// Declarative validation expression.
///
/// Expressions are serializable and can be composed using logical operators.
/// Use [`Rule::Expr`](super::Rule::Expr) to wrap expressions in rules.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", rename_all = "camelCase"))]
#[non_exhaustive]
pub enum Expr {
    // === Presence ===
    /// Value must not be null or empty.
    Required,

    // === String Constraints ===
    /// Minimum string length (inclusive).
    MinLength(usize),

    /// Maximum string length (inclusive).
    MaxLength(usize),

    /// Exact string length.
    Length(usize),

    /// Regex pattern match.
    Pattern(SmartStr),

    /// Valid email format.
    Email,

    /// Valid URL format.
    Url,

    /// Valid UUID format.
    Uuid,

    /// Starts with prefix.
    StartsWith(SmartStr),

    /// Ends with suffix.
    EndsWith(SmartStr),

    /// Contains substring.
    Contains(SmartStr),

    // === Numeric Constraints ===
    /// Minimum value (inclusive).
    Min(f64),

    /// Maximum value (inclusive).
    Max(f64),

    /// Minimum value (exclusive).
    ExclusiveMin(f64),

    /// Maximum value (exclusive).
    ExclusiveMax(f64),

    /// Value must be a multiple of this number.
    MultipleOf(f64),

    /// Value must be positive (> 0).
    Positive,

    /// Value must be negative (< 0).
    Negative,

    /// Value must be non-negative (>= 0).
    NonNegative,

    /// Value must be an integer (no fractional part).
    Integer,

    // === Array Constraints ===
    /// Minimum number of items.
    MinItems(usize),

    /// Maximum number of items.
    MaxItems(usize),

    /// Exact number of items.
    ItemCount(usize),

    /// All items must be unique.
    UniqueItems,

    // === Enum/Const Constraints ===
    /// Value must be one of the specified values.
    OneOf(Vec<Value>),

    /// Value must equal this constant.
    Const(Value),

    // === Logical Operators ===
    /// All expressions must pass.
    And(Vec<Expr>),

    /// At least one expression must pass.
    Or(Vec<Expr>),

    /// Expression must fail (negation).
    Not(Box<Expr>),

    /// If condition passes, then consequent must pass.
    If {
        /// Condition expression.
        condition: Box<Expr>,
        /// Expression to evaluate if condition passes.
        then: Box<Expr>,
        /// Optional expression to evaluate if condition fails.
        otherwise: Option<Box<Expr>>,
    },

    // === Cross-Field (requires ValidationContext) ===
    /// Value must equal the value of another field.
    EqualTo(SmartStr),

    /// Value must not equal the value of another field.
    NotEqualTo(SmartStr),

    /// Value must be less than another field's value.
    LessThan(SmartStr),

    /// Value must be greater than another field's value.
    GreaterThan(SmartStr),
}

impl Expr {
    /// Evaluates this expression against a value.
    ///
    /// For cross-field expressions (`EqualTo`, `LessThan`, etc.), this returns `Ok`
    /// since they require a full `ValidationContext`. Use [`Expr::validate_with_context`]
    /// for those cases.
    ///
    /// # Errors
    ///
    /// Returns `Err(ValidationOutcome)` if the value fails validation.
    pub fn validate(&self, value: &Value) -> ValidationResult {
        match self {
            // === Presence ===
            Self::Required => validate_required(value),

            // === String Constraints ===
            Self::MinLength(min) => validate_min_length(value, *min),
            Self::MaxLength(max) => validate_max_length(value, *max),
            Self::Length(len) => validate_length(value, *len),
            Self::Pattern(pattern) => validate_pattern(value, pattern),
            Self::Email => validate_email(value),
            Self::Url => validate_url(value),
            Self::Uuid => validate_uuid(value),
            Self::StartsWith(prefix) => validate_starts_with(value, prefix),
            Self::EndsWith(suffix) => validate_ends_with(value, suffix),
            Self::Contains(substr) => validate_contains(value, substr),

            // === Numeric Constraints ===
            Self::Min(min) => validate_min(value, *min),
            Self::Max(max) => validate_max(value, *max),
            Self::ExclusiveMin(min) => validate_exclusive_min(value, *min),
            Self::ExclusiveMax(max) => validate_exclusive_max(value, *max),
            Self::MultipleOf(divisor) => validate_multiple_of(value, *divisor),
            Self::Positive => validate_positive(value),
            Self::Negative => validate_negative(value),
            Self::NonNegative => validate_non_negative(value),
            Self::Integer => validate_integer(value),

            // === Array Constraints ===
            Self::MinItems(min) => validate_min_items(value, *min),
            Self::MaxItems(max) => validate_max_items(value, *max),
            Self::ItemCount(count) => validate_item_count(value, *count),
            Self::UniqueItems => validate_unique_items(value),

            // === Enum/Const ===
            Self::OneOf(allowed) => validate_one_of(value, allowed),
            Self::Const(expected) => validate_const(value, expected),

            // === Logical Operators ===
            Self::And(exprs) => {
                for expr in exprs {
                    expr.validate(value)?;
                }
                Ok(())
            }
            Self::Or(exprs) => {
                if exprs.is_empty() {
                    return Ok(());
                }
                for expr in exprs {
                    if expr.validate(value).is_ok() {
                        return Ok(());
                    }
                }
                Err(Error::custom("or", "None of the conditions were satisfied").into())
            }
            Self::Not(expr) => {
                if expr.validate(value).is_ok() {
                    Err(Error::custom("not", "Condition should not be satisfied").into())
                } else {
                    Ok(())
                }
            }
            Self::If {
                condition,
                then,
                otherwise,
            } => {
                if condition.validate(value).is_ok() {
                    then.validate(value)
                } else if let Some(else_expr) = otherwise {
                    else_expr.validate(value)
                } else {
                    Ok(())
                }
            }

            // === Cross-Field (no-op without context) ===
            Self::EqualTo(_) | Self::NotEqualTo(_) | Self::LessThan(_) | Self::GreaterThan(_) => {
                Ok(())
            }
        }
    }

    /// Evaluates this expression with access to sibling values.
    ///
    /// Required for cross-field validation expressions.
    ///
    /// # Errors
    ///
    /// Returns `Err(ValidationOutcome)` if the value fails validation.
    pub fn validate_with_context(
        &self,
        value: &Value,
        ctx: &super::ValidationContext<'_>,
    ) -> ValidationResult {
        match self {
            Self::EqualTo(other_key) => {
                if let Some(other) = ctx.get(other_key) {
                    if value != other {
                        return Err(Error::custom(
                            "equal_to",
                            format!("Value must equal {other_key}"),
                        )
                        .into());
                    }
                }
                Ok(())
            }
            Self::NotEqualTo(other_key) => {
                if let Some(other) = ctx.get(other_key) {
                    if value == other {
                        return Err(Error::custom(
                            "not_equal_to",
                            format!("Value must not equal {other_key}"),
                        )
                        .into());
                    }
                }
                Ok(())
            }
            Self::LessThan(other_key) => validate_less_than(value, other_key, ctx),
            Self::GreaterThan(other_key) => validate_greater_than(value, other_key, ctx),
            // All other expressions delegate to simple validate
            _ => self.validate(value),
        }
    }
}

// === Validation Functions ===

fn validate_required(value: &Value) -> ValidationResult {
    match value {
        Value::Null => Err(Error::required().into()),
        Value::Text(s) if s.is_empty() => Err(Error::required().into()),
        Value::Array(a) if a.is_empty() => Err(Error::required().into()),
        Value::Object(o) if o.is_empty() => Err(Error::required().into()),
        _ => Ok(()),
    }
}

fn validate_min_length(value: &Value, min: usize) -> ValidationResult {
    if let Value::Text(s) = value {
        let len = s.chars().count();
        if len < min {
            return Err(Error::min_length(min, len).into());
        }
    }
    Ok(())
}

fn validate_max_length(value: &Value, max: usize) -> ValidationResult {
    if let Value::Text(s) = value {
        let len = s.chars().count();
        if len > max {
            return Err(Error::max_length(max, len).into());
        }
    }
    Ok(())
}

fn validate_length(value: &Value, expected: usize) -> ValidationResult {
    if let Value::Text(s) = value {
        let len = s.chars().count();
        if len != expected {
            return Err(Error::custom(
                "length",
                format!("Length must be exactly {expected}, got {len}"),
            )
            .into());
        }
    }
    Ok(())
}

fn validate_pattern(value: &Value, pattern: &str) -> ValidationResult {
    if let Value::Text(s) = value {
        // Use thread-local regex cache for performance
        use std::cell::RefCell;
        use std::collections::HashMap;

        // Limit cache size to prevent unbounded growth with user-supplied patterns
        const MAX_CACHE_SIZE: usize = 100;

        thread_local! {
            static REGEX_CACHE: RefCell<HashMap<String, Result<regex::Regex, regex::Error>>> =
                RefCell::new(HashMap::new());
        }

        let result = REGEX_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();

            // If cache is at capacity and pattern is not cached, clear oldest entries
            if cache.len() >= MAX_CACHE_SIZE && !cache.contains_key(pattern) {
                // Simple strategy: clear entire cache when full
                // More sophisticated LRU would track access times
                cache.clear();
            }

            let entry = cache
                .entry(pattern.to_string())
                .or_insert_with(|| regex::Regex::new(pattern));

            match entry {
                Ok(re) => {
                    if re.is_match(s) {
                        Ok(())
                    } else {
                        Err(Error::pattern(pattern).into())
                    }
                }
                Err(_) => Err(Error::custom("pattern", "Invalid regex pattern").into()),
            }
        });

        return result;
    }
    Ok(())
}

fn validate_email(value: &Value) -> ValidationResult {
    if let Value::Text(s) = value {
        // Simple email validation (RFC 5322 simplified)
        let has_at = s.contains('@');
        let parts: Vec<&str> = s.split('@').collect();
        let valid = has_at
            && parts.len() == 2
            && !parts[0].is_empty()
            && parts[1].contains('.')
            && !parts[1].starts_with('.')
            && !parts[1].ends_with('.');

        if !valid {
            return Err(Error::email().into());
        }
    }
    Ok(())
}

fn validate_url(value: &Value) -> ValidationResult {
    if let Value::Text(s) = value {
        // Simple URL validation
        let valid =
            s.starts_with("http://") || s.starts_with("https://") || s.starts_with("ftp://");

        if !valid {
            return Err(Error::url().into());
        }
    }
    Ok(())
}

fn validate_uuid(value: &Value) -> ValidationResult {
    if let Value::Text(s) = value {
        // UUID format: 8-4-4-4-12 hex digits
        let valid = s.len() == 36
            && s.chars().enumerate().all(|(i, c)| {
                if i == 8 || i == 13 || i == 18 || i == 23 {
                    c == '-'
                } else {
                    c.is_ascii_hexdigit()
                }
            });

        if !valid {
            return Err(Error::custom("uuid", "Invalid UUID format").into());
        }
    }
    Ok(())
}

fn validate_starts_with(value: &Value, prefix: &str) -> ValidationResult {
    if let Value::Text(s) = value {
        if !s.starts_with(prefix) {
            return Err(
                Error::custom("starts_with", format!("Value must start with '{prefix}'")).into(),
            );
        }
    }
    Ok(())
}

fn validate_ends_with(value: &Value, suffix: &str) -> ValidationResult {
    if let Value::Text(s) = value {
        if !s.ends_with(suffix) {
            return Err(
                Error::custom("ends_with", format!("Value must end with '{suffix}'")).into(),
            );
        }
    }
    Ok(())
}

fn validate_contains(value: &Value, substr: &str) -> ValidationResult {
    if let Value::Text(s) = value {
        if !s.contains(substr) {
            return Err(Error::custom("contains", format!("Value must contain '{substr}'")).into());
        }
    }
    Ok(())
}

fn validate_min(value: &Value, min: f64) -> ValidationResult {
    let num = match value {
        Value::Int(n) => *n as f64,
        Value::Float(n) => *n,
        _ => return Ok(()),
    };

    if num < min {
        return Err(Error::min_value(min, num).into());
    }
    Ok(())
}

fn validate_max(value: &Value, max: f64) -> ValidationResult {
    let num = match value {
        Value::Int(n) => *n as f64,
        Value::Float(n) => *n,
        _ => return Ok(()),
    };

    if num > max {
        return Err(Error::max_value(max, num).into());
    }
    Ok(())
}

fn validate_exclusive_min(value: &Value, min: f64) -> ValidationResult {
    let num = match value {
        Value::Int(n) => *n as f64,
        Value::Float(n) => *n,
        _ => return Ok(()),
    };

    if num <= min {
        return Err(Error::exclusive_min(min, num).into());
    }
    Ok(())
}

fn validate_exclusive_max(value: &Value, max: f64) -> ValidationResult {
    let num = match value {
        Value::Int(n) => *n as f64,
        Value::Float(n) => *n,
        _ => return Ok(()),
    };

    if num >= max {
        return Err(Error::exclusive_max(max, num).into());
    }
    Ok(())
}

fn validate_multiple_of(value: &Value, divisor: f64) -> ValidationResult {
    let num = match value {
        Value::Int(n) => *n as f64,
        Value::Float(n) => *n,
        _ => return Ok(()),
    };

    // Handle floating point precision
    let remainder = num % divisor;
    let epsilon = 1e-10;

    if remainder.abs() > epsilon && (divisor - remainder.abs()).abs() > epsilon {
        return Err(Error::multiple_of(divisor, num).into());
    }
    Ok(())
}

fn validate_positive(value: &Value) -> ValidationResult {
    let num = match value {
        Value::Int(n) => *n as f64,
        Value::Float(n) => *n,
        _ => return Ok(()),
    };

    if num <= 0.0 {
        return Err(Error::custom("positive", format!("Value must be positive, got {num}")).into());
    }
    Ok(())
}

fn validate_negative(value: &Value) -> ValidationResult {
    let num = match value {
        Value::Int(n) => *n as f64,
        Value::Float(n) => *n,
        _ => return Ok(()),
    };

    if num >= 0.0 {
        return Err(Error::custom("negative", format!("Value must be negative, got {num}")).into());
    }
    Ok(())
}

fn validate_non_negative(value: &Value) -> ValidationResult {
    let num = match value {
        Value::Int(n) => *n as f64,
        Value::Float(n) => *n,
        _ => return Ok(()),
    };

    if num < 0.0 {
        return Err(Error::custom(
            "non_negative",
            format!("Value must be non-negative, got {num}"),
        )
        .into());
    }
    Ok(())
}

fn validate_integer(value: &Value) -> ValidationResult {
    if let Value::Float(n) = value {
        if n.fract() != 0.0 {
            return Err(
                Error::custom("integer", format!("Value must be an integer, got {n}")).into(),
            );
        }
    }
    Ok(())
}

fn validate_min_items(value: &Value, min: usize) -> ValidationResult {
    if let Value::Array(arr) = value {
        if arr.len() < min {
            return Err(Error::min_items(min, arr.len()).into());
        }
    }
    Ok(())
}

fn validate_max_items(value: &Value, max: usize) -> ValidationResult {
    if let Value::Array(arr) = value {
        if arr.len() > max {
            return Err(Error::max_items(max, arr.len()).into());
        }
    }
    Ok(())
}

fn validate_item_count(value: &Value, expected: usize) -> ValidationResult {
    if let Value::Array(arr) = value {
        if arr.len() != expected {
            return Err(Error::custom(
                "item_count",
                format!(
                    "Array must have exactly {expected} items, got {}",
                    arr.len()
                ),
            )
            .into());
        }
    }
    Ok(())
}

fn validate_unique_items(value: &Value) -> ValidationResult {
    if let Value::Array(arr) = value {
        // O(n²) comparison but with early exit on first duplicate
        // Can't use HashSet because Value doesn't implement Hash+Eq
        // (Float values make Hash implementation non-trivial due to NaN)
        for i in 0..arr.len() {
            for j in (i + 1)..arr.len() {
                if arr[i] == arr[j] {
                    return Err(Error::unique_items().into());
                }
            }
        }
    }
    Ok(())
}

fn validate_one_of(value: &Value, allowed: &[Value]) -> ValidationResult {
    if !allowed.contains(value) {
        // Format all Value types for better error messages
        let allowed_strs: Vec<String> = allowed
            .iter()
            .map(|v| match v {
                Value::Text(s) => format!("\"{s}\""),
                Value::Int(n) => n.to_string(),
                Value::Float(f) => f.to_string(),
                Value::Bool(b) => b.to_string(),
                Value::Null => "null".to_string(),
                Value::Array(_) => "[array]".to_string(),
                Value::Object(_) => "{object}".to_string(),
                Value::Binary(_) => "[binary]".to_string(),
            })
            .collect();

        let allowed_refs: Vec<&str> = allowed_strs
            .iter()
            .map(std::string::String::as_str)
            .collect();
        return Err(Error::not_in_enum(&allowed_refs).into());
    }
    Ok(())
}

fn validate_const(value: &Value, expected: &Value) -> ValidationResult {
    if value != expected {
        let expected_str = match expected {
            Value::Text(s) => s.as_str(),
            _ => "<value>",
        };
        return Err(Error::not_const(expected_str).into());
    }
    Ok(())
}

fn validate_less_than(
    value: &Value,
    other_key: &str,
    ctx: &super::ValidationContext<'_>,
) -> ValidationResult {
    let num = match value {
        Value::Int(n) => *n as f64,
        Value::Float(n) => *n,
        _ => return Ok(()),
    };

    if let Some(other) = ctx.get(other_key) {
        let other_num = match other {
            Value::Int(n) => *n as f64,
            Value::Float(n) => *n,
            _ => return Ok(()),
        };

        if num >= other_num {
            return Err(
                Error::custom("less_than", format!("Value must be less than {other_key}")).into(),
            );
        }
    }
    Ok(())
}

fn validate_greater_than(
    value: &Value,
    other_key: &str,
    ctx: &super::ValidationContext<'_>,
) -> ValidationResult {
    let num = match value {
        Value::Int(n) => *n as f64,
        Value::Float(n) => *n,
        _ => return Ok(()),
    };

    if let Some(other) = ctx.get(other_key) {
        let other_num = match other {
            Value::Int(n) => *n as f64,
            Value::Float(n) => *n,
            _ => return Ok(()),
        };

        if num <= other_num {
            return Err(Error::custom(
                "greater_than",
                format!("Value must be greater than {other_key}"),
            )
            .into());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_required() {
        assert!(Expr::Required.validate(&Value::Null).is_err());
        assert!(Expr::Required.validate(&Value::text("")).is_err());
        assert!(Expr::Required.validate(&Value::text("hello")).is_ok());
        assert!(Expr::Required.validate(&Value::Int(0)).is_ok());
    }

    #[test]
    fn test_min_length() {
        assert!(Expr::MinLength(3).validate(&Value::text("ab")).is_err());
        assert!(Expr::MinLength(3).validate(&Value::text("abc")).is_ok());
        assert!(Expr::MinLength(3).validate(&Value::text("abcd")).is_ok());
    }

    #[test]
    fn test_max_length() {
        assert!(Expr::MaxLength(3).validate(&Value::text("abcd")).is_err());
        assert!(Expr::MaxLength(3).validate(&Value::text("abc")).is_ok());
        assert!(Expr::MaxLength(3).validate(&Value::text("ab")).is_ok());
    }

    #[test]
    fn test_pattern() {
        assert!(
            Expr::Pattern(r"^\d+$".into())
                .validate(&Value::text("123"))
                .is_ok()
        );
        assert!(
            Expr::Pattern(r"^\d+$".into())
                .validate(&Value::text("abc"))
                .is_err()
        );
    }

    #[test]
    fn test_email() {
        assert!(
            Expr::Email
                .validate(&Value::text("test@example.com"))
                .is_ok()
        );
        assert!(Expr::Email.validate(&Value::text("invalid")).is_err());
        assert!(Expr::Email.validate(&Value::text("@example.com")).is_err());
        assert!(Expr::Email.validate(&Value::text("test@")).is_err());
    }

    #[test]
    fn test_url() {
        assert!(
            Expr::Url
                .validate(&Value::text("https://example.com"))
                .is_ok()
        );
        assert!(
            Expr::Url
                .validate(&Value::text("http://example.com"))
                .is_ok()
        );
        assert!(Expr::Url.validate(&Value::text("example.com")).is_err());
    }

    #[test]
    fn test_uuid() {
        assert!(
            Expr::Uuid
                .validate(&Value::text("550e8400-e29b-41d4-a716-446655440000"))
                .is_ok()
        );
        assert!(Expr::Uuid.validate(&Value::text("invalid-uuid")).is_err());
    }

    #[test]
    fn test_min_max() {
        assert!(Expr::Min(0.0).validate(&Value::Int(-1)).is_err());
        assert!(Expr::Min(0.0).validate(&Value::Int(0)).is_ok());
        assert!(Expr::Max(100.0).validate(&Value::Int(101)).is_err());
        assert!(Expr::Max(100.0).validate(&Value::Int(100)).is_ok());
    }

    #[test]
    fn test_exclusive_min_max() {
        assert!(Expr::ExclusiveMin(0.0).validate(&Value::Int(0)).is_err());
        assert!(Expr::ExclusiveMin(0.0).validate(&Value::Int(1)).is_ok());
        assert!(
            Expr::ExclusiveMax(100.0)
                .validate(&Value::Int(100))
                .is_err()
        );
        assert!(Expr::ExclusiveMax(100.0).validate(&Value::Int(99)).is_ok());
    }

    #[test]
    fn test_multiple_of() {
        assert!(Expr::MultipleOf(5.0).validate(&Value::Int(10)).is_ok());
        assert!(Expr::MultipleOf(5.0).validate(&Value::Int(7)).is_err());
        assert!(Expr::MultipleOf(0.5).validate(&Value::Float(1.5)).is_ok());
    }

    #[test]
    fn test_positive_negative() {
        assert!(Expr::Positive.validate(&Value::Int(1)).is_ok());
        assert!(Expr::Positive.validate(&Value::Int(0)).is_err());
        assert!(Expr::Positive.validate(&Value::Int(-1)).is_err());

        assert!(Expr::Negative.validate(&Value::Int(-1)).is_ok());
        assert!(Expr::Negative.validate(&Value::Int(0)).is_err());
        assert!(Expr::Negative.validate(&Value::Int(1)).is_err());

        assert!(Expr::NonNegative.validate(&Value::Int(0)).is_ok());
        assert!(Expr::NonNegative.validate(&Value::Int(1)).is_ok());
        assert!(Expr::NonNegative.validate(&Value::Int(-1)).is_err());
    }

    #[test]
    fn test_min_max_items() {
        let arr = Value::array([Value::Int(1), Value::Int(2)]);
        assert!(Expr::MinItems(2).validate(&arr).is_ok());
        assert!(Expr::MinItems(3).validate(&arr).is_err());
        assert!(Expr::MaxItems(3).validate(&arr).is_ok());
        assert!(Expr::MaxItems(1).validate(&arr).is_err());
    }

    #[test]
    fn test_unique_items() {
        let unique = Value::array([Value::Int(1), Value::Int(2), Value::Int(3)]);
        let dups = Value::array([Value::Int(1), Value::Int(2), Value::Int(1)]);

        assert!(Expr::UniqueItems.validate(&unique).is_ok());
        assert!(Expr::UniqueItems.validate(&dups).is_err());
    }

    #[test]
    fn test_one_of() {
        let allowed = vec![Value::text("a"), Value::text("b"), Value::text("c")];
        assert!(
            Expr::OneOf(allowed.clone())
                .validate(&Value::text("a"))
                .is_ok()
        );
        assert!(Expr::OneOf(allowed).validate(&Value::text("d")).is_err());
    }

    #[test]
    fn test_and() {
        let expr = Expr::And(vec![Expr::Required, Expr::MinLength(3)]);
        assert!(expr.validate(&Value::text("abc")).is_ok());
        assert!(expr.validate(&Value::text("ab")).is_err());
        assert!(expr.validate(&Value::Null).is_err());
    }

    #[test]
    fn test_or() {
        let expr = Expr::Or(vec![Expr::Email, Expr::Url]);
        assert!(expr.validate(&Value::text("test@example.com")).is_ok());
        assert!(expr.validate(&Value::text("https://example.com")).is_ok());
        assert!(expr.validate(&Value::text("invalid")).is_err());
    }

    #[test]
    fn test_not() {
        let expr = Expr::Not(Box::new(Expr::Required));
        assert!(expr.validate(&Value::Null).is_ok());
        assert!(expr.validate(&Value::text("hello")).is_err());
    }

    #[test]
    fn test_if_then_else() {
        // If value is required (not null), then it must be at least 3 chars
        let expr = Expr::If {
            condition: Box::new(Expr::Required),
            then: Box::new(Expr::MinLength(3)),
            otherwise: None,
        };

        assert!(expr.validate(&Value::Null).is_ok()); // condition fails, no otherwise
        assert!(expr.validate(&Value::text("abc")).is_ok()); // condition passes, then passes
        assert!(expr.validate(&Value::text("ab")).is_err()); // condition passes, then fails
    }
}
