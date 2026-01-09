//! Unified expression system for validation and visibility.

use crate::core::{SmartStr, Value};
use std::sync::Arc;

/// Expression for validation and visibility checks.
///
/// Expressions define conditions that can be evaluated against values.
/// They are used for:
/// - **Validation**: Check if a value meets certain criteria
/// - **Visibility**: Conditionally show/hide parameters based on other values
///
/// # Expression Categories
///
/// ## Value Comparisons
/// - `Eq` - Equals a specific value
/// - `Ne` - Not equals a specific value
/// - `Lt` - Less than (numeric)
/// - `Gt` - Greater than (numeric)
/// - `Lte` - Less than or equal (numeric)
/// - `Gte` - Greater than or equal (numeric)
/// - `Between` - Between min and max (inclusive)
///
/// ## String Operations
/// - `StartsWith` - String starts with prefix
/// - `EndsWith` - String ends with suffix
/// - `Contains` - String/array contains value
/// - `Matches` - String matches regex pattern (requires `validation` feature)
///
/// ## String Validation
/// - `Email` - Valid email format
/// - `Url` - Valid URL format
/// - `Uuid` - Valid UUID format
///
/// ## Length Checks
/// - `MinLength` - Minimum string/array length
/// - `MaxLength` - Maximum string/array length
/// - `Length` - Exact string/array length
/// - `LengthBetween` - Length between min and max
///
/// ## Numeric Constraints
/// - `Min` - Minimum value (inclusive)
/// - `Max` - Maximum value (inclusive)
/// - `ExclusiveMin` - Minimum value (exclusive)
/// - `ExclusiveMax` - Maximum value (exclusive)
/// - `MultipleOf` - Value is multiple of given number
/// - `Positive` - Value > 0
/// - `Negative` - Value < 0
/// - `NonNegative` - Value >= 0
/// - `Integer` - Value is integer (no fractional part)
///
/// ## Collection Constraints
/// - `MinItems` - Minimum array length
/// - `MaxItems` - Maximum array length
/// - `ItemCount` - Exact array length
/// - `UniqueItems` - All array items are unique
///
/// ## State Checks
/// - `Required` - Value must not be null/empty
/// - `IsSet` - Value is not null
/// - `IsEmpty` - Value is empty
/// - `IsNull` - Value is null
/// - `IsNotEmpty` - Value is not empty
/// - `IsTrue` - Boolean is true
/// - `IsFalse` - Boolean is false
/// - `IsValid` - Passes validation (for visibility)
///
/// ## Set Operations
/// - `OneOf` - Value is in allowed set
/// - `Const` - Value equals constant
///
/// ## Logical Operators
/// - `And` - All expressions must pass
/// - `Or` - At least one expression must pass
/// - `Not` - Inverts expression result
/// - `If` - Conditional expression (if-then-else)
///
/// # Example
///
/// ```ignore
/// use paramdef::expr::{Expr, Rule};
/// use paramdef::core::Value;
///
/// // Validation: current value must be email with min length 5
/// let rule = Rule::local(Expr::And(vec![
///     Expr::Email,
///     Expr::MinLength(5),
/// ]));
///
/// // Visibility: show field if "mode" equals "advanced"
/// let rule = Rule::field("mode", Expr::Eq(Value::text("advanced")));
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", allow(clippy::unsafe_derive_deserialize))]
#[non_exhaustive]
pub enum Expr {
    // === Value Comparisons ===
    /// Value equals the specified value.
    Eq(Value),

    /// Value does not equal the specified value.
    Ne(Value),

    /// Numeric value is less than threshold.
    Lt(f64),

    /// Numeric value is greater than threshold.
    Gt(f64),

    /// Numeric value is less than or equal to threshold.
    Lte(f64),

    /// Numeric value is greater than or equal to threshold.
    Gte(f64),

    /// Numeric value is between min and max (inclusive).
    Between(f64, f64),

    // === String Operations ===
    /// String starts with the specified prefix.
    StartsWith(SmartStr),

    /// String ends with the specified suffix.
    EndsWith(SmartStr),

    /// String or array contains the specified value.
    Contains(Value),

    /// String matches the regular expression pattern.
    ///
    /// Requires the `validation` feature for regex support.
    #[cfg(feature = "validation")]
    Matches(String),

    // === String Validation ===
    /// Valid email format (HTML5 spec).
    Email,

    /// Valid URL format.
    Url,

    /// Valid UUID format.
    Uuid,

    // === Length Checks ===
    /// Minimum string/array length (inclusive).
    MinLength(usize),

    /// Maximum string/array length (inclusive).
    MaxLength(usize),

    /// Exact string/array length.
    Length(usize),

    /// String/array length is between min and max (inclusive).
    LengthBetween(usize, usize),

    // === Numeric Constraints ===
    /// Minimum value (inclusive).
    Min(f64),

    /// Maximum value (inclusive).
    Max(f64),

    /// Minimum value (exclusive).
    ExclusiveMin(f64),

    /// Maximum value (exclusive).
    ExclusiveMax(f64),

    /// Value must be a multiple of the given number.
    MultipleOf(f64),

    /// Value must be positive (> 0).
    Positive,

    /// Value must be negative (< 0).
    Negative,

    /// Value must be non-negative (>= 0).
    NonNegative,

    /// Value must be an integer (no fractional part).
    Integer,

    // === Collection Constraints ===
    /// Minimum number of items in array (inclusive).
    MinItems(usize),

    /// Maximum number of items in array (inclusive).
    MaxItems(usize),

    /// Exact number of items in array.
    ItemCount(usize),

    /// All items in array must be unique.
    UniqueItems,

    // === State Checks ===
    /// Value must not be null or empty.
    Required,

    /// Value is not null.
    IsSet,

    /// Value is empty (null, empty string, or empty array).
    IsEmpty,

    /// Value is explicitly null.
    IsNull,

    /// Value is not empty (inverse of `IsEmpty`).
    IsNotEmpty,

    /// Boolean value is true.
    IsTrue,

    /// Boolean value is false.
    IsFalse,

    /// Parameter passes validation (for visibility conditions).
    IsValid,

    // === Set Operations ===
    /// Value is one of the allowed values.
    OneOf(Arc<[Value]>),

    /// Value equals the constant value.
    Const(Value),

    // === Logical Operators ===
    /// All sub-expressions must pass.
    And(Arc<[Expr]>),

    /// At least one sub-expression must pass.
    Or(Arc<[Expr]>),

    /// Inverts the sub-expression result.
    Not(Box<Expr>),

    /// Conditional expression (if-then-else).
    ///
    /// If condition passes, then consequent must pass.
    /// If condition fails and otherwise is Some, otherwise must pass.
    /// If condition fails and otherwise is None, passes automatically.
    If {
        /// Condition expression.
        condition: Box<Expr>,
        /// Expression to evaluate if condition passes.
        then: Box<Expr>,
        /// Optional expression to evaluate if condition fails.
        otherwise: Option<Box<Expr>>,
    },
}

impl Expr {
    // === Value Comparisons ===

    /// Create an equality expression.
    #[must_use]
    pub fn eq(value: Value) -> Self {
        Self::Eq(value)
    }

    /// Create a not-equals expression.
    #[must_use]
    pub fn ne(value: Value) -> Self {
        Self::Ne(value)
    }

    /// Create a less-than expression.
    #[must_use]
    pub fn lt(threshold: f64) -> Self {
        Self::Lt(threshold)
    }

    /// Create a greater-than expression.
    #[must_use]
    pub fn gt(threshold: f64) -> Self {
        Self::Gt(threshold)
    }

    /// Create a less-than-or-equal expression.
    #[must_use]
    pub fn lte(threshold: f64) -> Self {
        Self::Lte(threshold)
    }

    /// Create a greater-than-or-equal expression.
    #[must_use]
    pub fn gte(threshold: f64) -> Self {
        Self::Gte(threshold)
    }

    /// Create a between expression (inclusive).
    #[must_use]
    pub fn between(min: f64, max: f64) -> Self {
        Self::Between(min, max)
    }

    // === String Operations ===

    /// Create a starts-with expression.
    #[must_use]
    pub fn starts_with(prefix: impl Into<SmartStr>) -> Self {
        Self::StartsWith(prefix.into())
    }

    /// Create an ends-with expression.
    #[must_use]
    pub fn ends_with(suffix: impl Into<SmartStr>) -> Self {
        Self::EndsWith(suffix.into())
    }

    /// Create a contains expression.
    #[must_use]
    pub fn contains(value: Value) -> Self {
        Self::Contains(value)
    }

    /// Create a regex match expression.
    ///
    /// Requires the `validation` feature.
    #[cfg(feature = "validation")]
    #[must_use]
    pub fn matches(pattern: impl Into<String>) -> Self {
        Self::Matches(pattern.into())
    }

    // === String Validation ===

    /// Create an email validation expression.
    #[must_use]
    pub fn email() -> Self {
        Self::Email
    }

    /// Create a URL validation expression.
    #[must_use]
    pub fn url() -> Self {
        Self::Url
    }

    /// Create a UUID validation expression.
    #[must_use]
    pub fn uuid() -> Self {
        Self::Uuid
    }

    // === Length Checks ===

    /// Create a minimum length expression.
    #[must_use]
    pub fn min_length(len: usize) -> Self {
        Self::MinLength(len)
    }

    /// Create a maximum length expression.
    #[must_use]
    pub fn max_length(len: usize) -> Self {
        Self::MaxLength(len)
    }

    /// Create an exact length expression.
    #[must_use]
    pub fn length(len: usize) -> Self {
        Self::Length(len)
    }

    /// Create a length-between expression (inclusive).
    #[must_use]
    pub fn length_between(min: usize, max: usize) -> Self {
        Self::LengthBetween(min, max)
    }

    // === Numeric Constraints ===

    /// Create a minimum value expression (inclusive).
    #[must_use]
    pub fn min(value: f64) -> Self {
        Self::Min(value)
    }

    /// Create a maximum value expression (inclusive).
    #[must_use]
    pub fn max(value: f64) -> Self {
        Self::Max(value)
    }

    /// Create an exclusive minimum expression.
    #[must_use]
    pub fn exclusive_min(value: f64) -> Self {
        Self::ExclusiveMin(value)
    }

    /// Create an exclusive maximum expression.
    #[must_use]
    pub fn exclusive_max(value: f64) -> Self {
        Self::ExclusiveMax(value)
    }

    /// Create a multiple-of expression.
    #[must_use]
    pub fn multiple_of(value: f64) -> Self {
        Self::MultipleOf(value)
    }

    /// Create a positive value expression (> 0).
    #[must_use]
    pub fn positive() -> Self {
        Self::Positive
    }

    /// Create a negative value expression (< 0).
    #[must_use]
    pub fn negative() -> Self {
        Self::Negative
    }

    /// Create a non-negative value expression (>= 0).
    #[must_use]
    pub fn non_negative() -> Self {
        Self::NonNegative
    }

    /// Create an integer value expression.
    #[must_use]
    pub fn integer() -> Self {
        Self::Integer
    }

    // === Collection Constraints ===

    /// Create a minimum items expression.
    #[must_use]
    pub fn min_items(count: usize) -> Self {
        Self::MinItems(count)
    }

    /// Create a maximum items expression.
    #[must_use]
    pub fn max_items(count: usize) -> Self {
        Self::MaxItems(count)
    }

    /// Create an exact item count expression.
    #[must_use]
    pub fn item_count(count: usize) -> Self {
        Self::ItemCount(count)
    }

    /// Create a unique items expression.
    #[must_use]
    pub fn unique_items() -> Self {
        Self::UniqueItems
    }

    // === State Checks ===

    /// Create a required expression.
    #[must_use]
    pub fn required() -> Self {
        Self::Required
    }

    /// Create an is-set expression.
    #[must_use]
    pub fn is_set() -> Self {
        Self::IsSet
    }

    /// Create an is-empty expression.
    #[must_use]
    pub fn is_empty() -> Self {
        Self::IsEmpty
    }

    /// Create an is-null expression.
    #[must_use]
    pub fn is_null() -> Self {
        Self::IsNull
    }

    /// Create an is-not-empty expression.
    #[must_use]
    pub fn is_not_empty() -> Self {
        Self::IsNotEmpty
    }

    /// Create an is-true expression.
    #[must_use]
    pub fn is_true() -> Self {
        Self::IsTrue
    }

    /// Create an is-false expression.
    #[must_use]
    pub fn is_false() -> Self {
        Self::IsFalse
    }

    /// Create an is-valid expression.
    #[must_use]
    pub fn is_valid() -> Self {
        Self::IsValid
    }

    // === Set Operations ===

    /// Create a one-of expression.
    #[must_use]
    pub fn one_of(values: Vec<Value>) -> Self {
        Self::OneOf(values.into())
    }

    /// Create a const expression.
    #[must_use]
    pub fn const_value(value: Value) -> Self {
        Self::Const(value)
    }

    // === Logical Operators ===

    /// Create an AND expression (all must pass).
    #[must_use]
    pub fn and(exprs: Vec<Expr>) -> Self {
        Self::And(exprs.into())
    }

    /// Create an OR expression (at least one must pass).
    #[must_use]
    pub fn or(exprs: Vec<Expr>) -> Self {
        Self::Or(exprs.into())
    }

    /// Create a NOT expression (inverts result).
    #[must_use]
    pub fn not(expr: Expr) -> Self {
        Self::Not(Box::new(expr))
    }

    /// Create an IF expression (conditional).
    #[must_use]
    pub fn if_then_else(condition: Expr, then: Expr, otherwise: Option<Expr>) -> Self {
        Self::If {
            condition: Box::new(condition),
            then: Box::new(then),
            otherwise: otherwise.map(Box::new),
        }
    }

    /// Create an IF-THEN expression (no else branch).
    #[must_use]
    pub fn if_then(condition: Expr, then: Expr) -> Self {
        Self::if_then_else(condition, then, None)
    }

    // === Cross-field Validation (for compatibility) ===
    // TODO: These will be replaced by ExprTarget::Field mechanism

    /// Create an equal-to expression for cross-field validation.
    ///
    /// Note: This is a temporary compatibility helper. Use `ExprTarget::Field` instead.
    #[must_use]
    #[cfg(feature = "validation")]
    pub fn equal_to(field: impl Into<crate::core::SmartStr>) -> Self {
        // For now, create a custom variant that will be handled specially
        Self::Const(crate::core::Value::text(format!(
            "__equal_to__{}",
            field.into()
        )))
    }

    /// Create a not-equal-to expression for cross-field validation.
    ///
    /// Note: This is a temporary compatibility helper. Use `ExprTarget::Field` instead.
    #[must_use]
    #[cfg(feature = "validation")]
    pub fn not_equal_to(field: impl Into<crate::core::SmartStr>) -> Self {
        // For now, create a custom variant that will be handled specially
        Self::Const(crate::core::Value::text(format!(
            "__not_equal_to__{}",
            field.into()
        )))
    }
}
