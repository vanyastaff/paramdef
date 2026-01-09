//! Visibility expression types and evaluation.

use crate::context::Context;
use crate::core::{Key, Value};
use std::sync::Arc;

/// A visibility expression that evaluates to a boolean.
///
/// Expressions are used to conditionally show or hide parameters based on
/// other parameter values in the context.
///
/// ## Expression Types
///
/// ### Value Comparisons
/// - `Eq` - Equals a specific value
/// - `Ne` - Not equals a specific value
/// - `Lt` - Less than (numeric comparison)
/// - `Gt` - Greater than (numeric comparison)
/// - `Lte` - Less than or equal (numeric comparison)
/// - `Gte` - Greater than or equal (numeric comparison)
/// - `Between` - Numeric value is between min and max (inclusive)
///
/// ### State Checks
/// - `IsSet` - Parameter has a value (not null/undefined)
/// - `IsEmpty` - Parameter is empty (null, empty string, empty array)
/// - `IsNull` - Parameter is explicitly null
/// - `IsNotEmpty` - Parameter is not empty (inverse of `IsEmpty`)
/// - `IsTrue` - Boolean parameter is true
/// - `IsFalse` - Boolean parameter is false
/// - `IsValid` - Parameter passes validation
///
/// ### String Operations
/// - `StartsWith` - String starts with a prefix
/// - `EndsWith` - String ends with a suffix
/// - `Matches` - String matches a regular expression pattern (requires `validation` feature)
///
/// ### Length Checks
/// - `LengthMin` - String/array length is at least min
/// - `LengthMax` - String/array length is at most max
/// - `LengthBetween` - String/array length is between min and max (inclusive)
///
/// ### Collection Operations
/// - `OneOf` - Value is in a list of allowed values
/// - `Contains` - String/array contains a value
///
/// ### Logical Operators
/// - `And` - All expressions must be true
/// - `Or` - At least one expression must be true
/// - `Not` - Inverts the expression result
///
/// ## Example
///
/// ```
/// use paramdef::visibility::Expr;
/// use paramdef::context::Context;
/// use paramdef::core::Value;
/// # use paramdef::schema::Schema;
/// # use paramdef::types::leaf::{Text, Number, Boolean};
/// # use std::sync::Arc;
///
/// # let schema = Arc::new(Schema::builder()
/// #     .parameter(Text::builder("mode").build())
/// #     .parameter(Number::builder("age").build())
/// #     .parameter(Boolean::builder("premium").build())
/// #     .build());
/// # let mut ctx = Context::new(schema);
/// # ctx.set("mode", Value::text("advanced"));
/// # ctx.set("age", Value::Int(25));
/// # ctx.set("premium", Value::Bool(true));
/// // Simple equality check
/// let expr = Expr::eq("mode", Value::text("advanced"));
/// assert_eq!(expr.eval(&ctx), true);
///
/// // Numeric comparison
/// let expr = Expr::gte("age", 18.0);
/// assert_eq!(expr.eval(&ctx), true);
///
/// // Compound logic: show if (mode == "advanced" AND premium == true)
/// let expr = Expr::and(vec![
///     Expr::eq("mode", Value::text("advanced")),
///     Expr::is_true("premium"),
/// ]);
/// assert_eq!(expr.eval(&ctx), true);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", allow(clippy::unsafe_derive_deserialize))]
pub enum Expr {
    /// Value equals the specified value.
    Eq(Key, Value),

    /// Value does not equal the specified value.
    Ne(Key, Value),

    /// Parameter has a value (not null).
    IsSet(Key),

    /// Parameter is empty (null, empty string, or empty array).
    IsEmpty(Key),

    /// Boolean parameter is true.
    IsTrue(Key),

    /// Boolean parameter is false.
    IsFalse(Key),

    /// Numeric value is less than the threshold.
    Lt(Key, f64),

    /// Numeric value is greater than the threshold.
    Gt(Key, f64),

    /// Numeric value is less than or equal to the threshold.
    Lte(Key, f64),

    /// Numeric value is greater than or equal to the threshold.
    Gte(Key, f64),

    /// Value is one of the specified values.
    OneOf(
        Key,
        #[cfg_attr(feature = "serde", serde(with = "arc_slice_serde"))] Arc<[Value]>,
    ),

    /// String or array contains the specified value.
    Contains(Key, Value),

    /// Parameter passes validation (has no errors).
    IsValid(Key),

    /// String starts with the specified prefix.
    StartsWith(Key, crate::core::SmartStr),

    /// String ends with the specified suffix.
    EndsWith(Key, crate::core::SmartStr),

    /// String matches the regular expression pattern.
    ///
    /// Requires the `validation` feature for regex support.
    #[cfg(feature = "validation")]
    Matches(Key, String),

    /// Numeric value is between min and max (inclusive).
    Between(Key, f64, f64),

    /// String or collection length is at least min.
    LengthMin(Key, usize),

    /// String or collection length is at most max.
    LengthMax(Key, usize),

    /// String or collection length is between min and max (inclusive).
    LengthBetween(Key, usize, usize),

    /// Parameter is explicitly null.
    IsNull(Key),

    /// Parameter is not empty (inverse of `IsEmpty`).
    IsNotEmpty(Key),

    /// All sub-expressions must be true.
    And(#[cfg_attr(feature = "serde", serde(with = "arc_slice_serde"))] Arc<[Expr]>),

    /// At least one sub-expression must be true.
    Or(#[cfg_attr(feature = "serde", serde(with = "arc_slice_serde"))] Arc<[Expr]>),

    /// Inverts the sub-expression result.
    Not(Box<Expr>),
}

impl Expr {
    /// Create an equality expression.
    #[must_use]
    pub fn eq(key: impl Into<Key>, value: Value) -> Self {
        Self::Eq(key.into(), value)
    }

    /// Create a not-equals expression.
    #[must_use]
    pub fn ne(key: impl Into<Key>, value: Value) -> Self {
        Self::Ne(key.into(), value)
    }

    /// Create an is-set expression (parameter has a value).
    #[must_use]
    pub fn is_set(key: impl Into<Key>) -> Self {
        Self::IsSet(key.into())
    }

    /// Create an is-empty expression.
    #[must_use]
    pub fn is_empty(key: impl Into<Key>) -> Self {
        Self::IsEmpty(key.into())
    }

    /// Create an is-true expression (for boolean parameters).
    #[must_use]
    pub fn is_true(key: impl Into<Key>) -> Self {
        Self::IsTrue(key.into())
    }

    /// Create an is-false expression (for boolean parameters).
    #[must_use]
    pub fn is_false(key: impl Into<Key>) -> Self {
        Self::IsFalse(key.into())
    }

    /// Create a less-than expression.
    #[must_use]
    pub fn lt(key: impl Into<Key>, threshold: f64) -> Self {
        Self::Lt(key.into(), threshold)
    }

    /// Create a greater-than expression.
    #[must_use]
    pub fn gt(key: impl Into<Key>, threshold: f64) -> Self {
        Self::Gt(key.into(), threshold)
    }

    /// Create a less-than-or-equal expression.
    #[must_use]
    pub fn lte(key: impl Into<Key>, threshold: f64) -> Self {
        Self::Lte(key.into(), threshold)
    }

    /// Create a greater-than-or-equal expression.
    #[must_use]
    pub fn gte(key: impl Into<Key>, threshold: f64) -> Self {
        Self::Gte(key.into(), threshold)
    }

    /// Create a one-of expression (value in list).
    #[must_use]
    pub fn one_of(key: impl Into<Key>, values: Vec<Value>) -> Self {
        Self::OneOf(key.into(), values.into())
    }

    /// Create a contains expression.
    #[must_use]
    pub fn contains(key: impl Into<Key>, value: Value) -> Self {
        Self::Contains(key.into(), value)
    }

    /// Create an is-valid expression (passes validation).
    #[must_use]
    pub fn is_valid(key: impl Into<Key>) -> Self {
        Self::IsValid(key.into())
    }

    /// Create a starts-with expression.
    #[must_use]
    pub fn starts_with(key: impl Into<Key>, prefix: impl Into<crate::core::SmartStr>) -> Self {
        Self::StartsWith(key.into(), prefix.into())
    }

    /// Create an ends-with expression.
    #[must_use]
    pub fn ends_with(key: impl Into<Key>, suffix: impl Into<crate::core::SmartStr>) -> Self {
        Self::EndsWith(key.into(), suffix.into())
    }

    /// Create a regex match expression.
    ///
    /// Requires the `validation` feature for regex support.
    #[cfg(feature = "validation")]
    #[must_use]
    pub fn matches(key: impl Into<Key>, pattern: impl Into<String>) -> Self {
        Self::Matches(key.into(), pattern.into())
    }

    /// Create a between expression (min <= value <= max).
    #[must_use]
    pub fn between(key: impl Into<Key>, min: f64, max: f64) -> Self {
        Self::Between(key.into(), min, max)
    }

    /// Create a length-min expression.
    #[must_use]
    pub fn length_min(key: impl Into<Key>, min: usize) -> Self {
        Self::LengthMin(key.into(), min)
    }

    /// Create a length-max expression.
    #[must_use]
    pub fn length_max(key: impl Into<Key>, max: usize) -> Self {
        Self::LengthMax(key.into(), max)
    }

    /// Create a length-between expression (min <= len <= max).
    #[must_use]
    pub fn length_between(key: impl Into<Key>, min: usize, max: usize) -> Self {
        Self::LengthBetween(key.into(), min, max)
    }

    /// Create an is-null expression.
    #[must_use]
    pub fn is_null(key: impl Into<Key>) -> Self {
        Self::IsNull(key.into())
    }

    /// Create an is-not-empty expression.
    #[must_use]
    pub fn is_not_empty(key: impl Into<Key>) -> Self {
        Self::IsNotEmpty(key.into())
    }

    /// Create an AND expression (all must be true).
    #[must_use]
    pub fn and(exprs: Vec<Expr>) -> Self {
        Self::And(exprs.into())
    }

    /// Create an OR expression (at least one must be true).
    #[must_use]
    pub fn or(exprs: Vec<Expr>) -> Self {
        Self::Or(exprs.into())
    }

    /// Create a NOT expression (inverts result).
    ///
    /// Note: This is named `negate` to avoid confusion with `std::ops::Not`.
    #[must_use]
    pub fn negate(expr: Expr) -> Self {
        Self::Not(Box::new(expr))
    }

    /// Evaluate the expression against a context.
    ///
    /// Returns `true` if the expression evaluates to true, `false` otherwise.
    /// If a referenced parameter doesn't exist or has the wrong type, returns `false`.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::visibility::Expr;
    /// use paramdef::context::Context;
    /// use paramdef::core::Value;
    /// # use paramdef::schema::Schema;
    /// # use paramdef::types::leaf::Boolean;
    /// # use std::sync::Arc;
    ///
    /// # let schema = Arc::new(Schema::builder()
    /// #     .parameter(Boolean::builder("enabled").build())
    /// #     .build());
    /// # let mut ctx = Context::new(schema);
    /// let expr = Expr::is_true("enabled");
    ///
    /// // Not set - returns false
    /// assert_eq!(expr.eval(&ctx), false);
    ///
    /// // Set to true - returns true
    /// ctx.set("enabled", Value::Bool(true));
    /// assert_eq!(expr.eval(&ctx), true);
    /// ```
    #[must_use]
    pub fn eval(&self, ctx: &Context) -> bool {
        match self {
            Self::Eq(key, expected) => ctx.get(key.as_str()) == Some(expected),

            Self::Ne(key, expected) => ctx.get(key.as_str()).is_some_and(|v| v != expected),

            Self::IsSet(key) => ctx.get(key.as_str()).is_some_and(|v| !v.is_null()),

            Self::IsEmpty(key) => ctx.get(key.as_str()).is_none_or(Value::is_empty),

            Self::IsTrue(key) => ctx
                .get(key.as_str())
                .and_then(Value::as_bool)
                .unwrap_or(false),

            Self::IsFalse(key) => ctx
                .get(key.as_str())
                .and_then(Value::as_bool)
                .is_some_and(|b| !b),

            Self::Lt(key, threshold) => ctx
                .get(key.as_str())
                .and_then(Value::as_f64)
                .is_some_and(|n| n < *threshold),

            Self::Gt(key, threshold) => ctx
                .get(key.as_str())
                .and_then(Value::as_f64)
                .is_some_and(|n| n > *threshold),

            Self::Lte(key, threshold) => ctx
                .get(key.as_str())
                .and_then(Value::as_f64)
                .is_some_and(|n| n <= *threshold),

            Self::Gte(key, threshold) => ctx
                .get(key.as_str())
                .and_then(Value::as_f64)
                .is_some_and(|n| n >= *threshold),

            Self::OneOf(key, values) => ctx
                .get(key.as_str())
                .is_some_and(|v| values.iter().any(|val| val == v)),

            Self::Contains(key, search) => ctx.get(key.as_str()).is_some_and(|v| match v {
                Value::Text(s) => search.as_text().is_some_and(|needle| s.contains(needle)),
                Value::Array(arr) => arr.iter().any(|item| item == search),
                _ => false,
            }),

            Self::IsValid(key) => {
                // For now, always return true
                // In a full implementation, this would check validation errors
                // stored in RuntimeNode or Context
                ctx.get(key.as_str()).is_some()
            }

            Self::StartsWith(key, prefix) => ctx
                .get(key.as_str())
                .and_then(Value::as_text)
                .is_some_and(|s| s.starts_with(prefix.as_str())),

            Self::EndsWith(key, suffix) => ctx
                .get(key.as_str())
                .and_then(Value::as_text)
                .is_some_and(|s| s.ends_with(suffix.as_str())),

            #[cfg(feature = "validation")]
            Self::Matches(key, pattern) => {
                use std::cell::RefCell;
                use std::collections::HashMap;

                thread_local! {
                    static REGEX_CACHE: RefCell<HashMap<String, Option<regex::Regex>>> =
                        RefCell::new(HashMap::new());
                }

                ctx.get(key.as_str())
                    .and_then(Value::as_text)
                    .is_some_and(|s| {
                        REGEX_CACHE.with(|cache| {
                            let mut cache = cache.borrow_mut();
                            let re = cache
                                .entry(pattern.clone())
                                .or_insert_with(|| regex::Regex::new(pattern).ok());
                            re.as_ref().is_some_and(|r| r.is_match(s))
                        })
                    })
            }

            Self::Between(key, min, max) => ctx
                .get(key.as_str())
                .and_then(Value::as_f64)
                .is_some_and(|n| n >= *min && n <= *max),

            Self::LengthMin(key, min) => ctx.get(key.as_str()).is_some_and(|v| match v {
                Value::Text(s) => s.len() >= *min,
                Value::Array(arr) => arr.len() >= *min,
                _ => false,
            }),

            Self::LengthMax(key, max) => ctx.get(key.as_str()).is_some_and(|v| match v {
                Value::Text(s) => s.len() <= *max,
                Value::Array(arr) => arr.len() <= *max,
                _ => false,
            }),

            Self::LengthBetween(key, min, max) => ctx.get(key.as_str()).is_some_and(|v| {
                let len = match v {
                    Value::Text(s) => s.len(),
                    Value::Array(arr) => arr.len(),
                    _ => return false,
                };
                len >= *min && len <= *max
            }),

            Self::IsNull(key) => ctx.get(key.as_str()).is_some_and(Value::is_null),

            Self::IsNotEmpty(key) => ctx.get(key.as_str()).is_some_and(|v| !v.is_empty()),

            Self::And(exprs) => exprs.iter().all(|e| e.eval(ctx)),

            Self::Or(exprs) => exprs.iter().any(|e| e.eval(ctx)),

            Self::Not(expr) => !expr.eval(ctx),
        }
    }

    /// Get all parameter keys that this expression depends on.
    ///
    /// This is used for reactive updates - when a dependency changes,
    /// the visibility can be re-evaluated.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::visibility::Expr;
    /// use paramdef::core::{Key, Value};
    ///
    /// let expr = Expr::and(vec![
    ///     Expr::is_true("show_advanced"),
    ///     Expr::eq("mode", Value::text("expert")),
    /// ]);
    ///
    /// let deps = expr.dependencies();
    /// assert_eq!(deps.len(), 2);
    /// assert!(deps.contains(&Key::from("show_advanced")));
    /// assert!(deps.contains(&Key::from("mode")));
    /// ```
    #[must_use]
    pub fn dependencies(&self) -> Vec<Key> {
        let mut deps = Vec::new();
        self.collect_dependencies(&mut deps);
        deps.sort();
        deps.dedup();
        deps
    }

    fn collect_dependencies(&self, deps: &mut Vec<Key>) {
        match self {
            Self::Eq(key, _)
            | Self::Ne(key, _)
            | Self::IsSet(key)
            | Self::IsEmpty(key)
            | Self::IsTrue(key)
            | Self::IsFalse(key)
            | Self::Lt(key, _)
            | Self::Gt(key, _)
            | Self::Lte(key, _)
            | Self::Gte(key, _)
            | Self::OneOf(key, _)
            | Self::Contains(key, _)
            | Self::IsValid(key)
            | Self::StartsWith(key, _)
            | Self::EndsWith(key, _)
            | Self::Between(key, _, _)
            | Self::LengthMin(key, _)
            | Self::LengthMax(key, _)
            | Self::LengthBetween(key, _, _)
            | Self::IsNull(key)
            | Self::IsNotEmpty(key) => {
                deps.push(key.clone());
            }

            #[cfg(feature = "validation")]
            Self::Matches(key, _) => {
                deps.push(key.clone());
            }

            Self::And(exprs) | Self::Or(exprs) => {
                for expr in exprs.iter() {
                    expr.collect_dependencies(deps);
                }
            }

            Self::Not(expr) => {
                expr.collect_dependencies(deps);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::Schema;
    use crate::types::leaf::{Boolean, Number, Text};
    use std::sync::Arc;

    fn create_test_context() -> Context {
        let schema = Arc::new(
            Schema::builder()
                .parameter(Text::builder("name").build())
                .parameter(Text::builder("mode").build())
                .parameter(Number::builder("age").build())
                .parameter(Boolean::builder("enabled").build())
                .parameter(Boolean::builder("premium").build())
                .build(),
        );
        Context::new(schema)
    }

    #[test]
    fn test_expr_eq() {
        let mut ctx = create_test_context();
        let expr = Expr::eq("name", Value::text("Alice"));

        assert_eq!(expr.eval(&ctx), false);

        let _ = ctx.set("name", Value::text("Alice"));
        assert_eq!(expr.eval(&ctx), true);

        let _ = ctx.set("name", Value::text("Bob"));
        assert_eq!(expr.eval(&ctx), false);
    }

    #[test]
    fn test_expr_ne() {
        let mut ctx = create_test_context();
        let _ = ctx.set("name", Value::text("Alice"));

        let expr = Expr::ne("name", Value::text("Bob"));
        assert_eq!(expr.eval(&ctx), true);

        let expr = Expr::ne("name", Value::text("Alice"));
        assert_eq!(expr.eval(&ctx), false);
    }

    #[test]
    fn test_expr_is_set() {
        let mut ctx = create_test_context();
        let expr = Expr::is_set("name");

        assert_eq!(expr.eval(&ctx), false);

        let _ = ctx.set("name", Value::text("Alice"));
        assert_eq!(expr.eval(&ctx), true);
    }

    #[test]
    fn test_expr_is_empty() {
        let mut ctx = create_test_context();
        let expr = Expr::is_empty("name");

        assert_eq!(expr.eval(&ctx), true);

        let _ = ctx.set("name", Value::text(""));
        assert_eq!(expr.eval(&ctx), true);

        let _ = ctx.set("name", Value::text("Alice"));
        assert_eq!(expr.eval(&ctx), false);
    }

    #[test]
    fn test_expr_is_true() {
        let mut ctx = create_test_context();
        let expr = Expr::is_true("enabled");

        assert_eq!(expr.eval(&ctx), false);

        let _ = ctx.set("enabled", Value::Bool(true));
        assert_eq!(expr.eval(&ctx), true);

        let _ = ctx.set("enabled", Value::Bool(false));
        assert_eq!(expr.eval(&ctx), false);
    }

    #[test]
    fn test_expr_is_false() {
        let mut ctx = create_test_context();
        let expr = Expr::is_false("enabled");

        assert_eq!(expr.eval(&ctx), false);

        let _ = ctx.set("enabled", Value::Bool(false));
        assert_eq!(expr.eval(&ctx), true);

        let _ = ctx.set("enabled", Value::Bool(true));
        assert_eq!(expr.eval(&ctx), false);
    }

    #[test]
    fn test_expr_numeric_comparisons() {
        let mut ctx = create_test_context();
        let _ = ctx.set("age", Value::Int(25));

        assert_eq!(Expr::lt("age", 30.0).eval(&ctx), true);
        assert_eq!(Expr::lt("age", 20.0).eval(&ctx), false);

        assert_eq!(Expr::gt("age", 20.0).eval(&ctx), true);
        assert_eq!(Expr::gt("age", 30.0).eval(&ctx), false);

        assert_eq!(Expr::lte("age", 25.0).eval(&ctx), true);
        assert_eq!(Expr::lte("age", 24.0).eval(&ctx), false);

        assert_eq!(Expr::gte("age", 25.0).eval(&ctx), true);
        assert_eq!(Expr::gte("age", 26.0).eval(&ctx), false);
    }

    #[test]
    fn test_expr_one_of() {
        let mut ctx = create_test_context();
        let _ = ctx.set("mode", Value::text("advanced"));

        let expr = Expr::one_of(
            "mode",
            vec![
                Value::text("basic"),
                Value::text("advanced"),
                Value::text("expert"),
            ],
        );
        assert_eq!(expr.eval(&ctx), true);

        let _ = ctx.set("mode", Value::text("custom"));
        assert_eq!(expr.eval(&ctx), false);
    }

    #[test]
    fn test_expr_and() {
        let mut ctx = create_test_context();
        let _ = ctx.set("enabled", Value::Bool(true));
        let _ = ctx.set("premium", Value::Bool(true));

        let expr = Expr::and(vec![Expr::is_true("enabled"), Expr::is_true("premium")]);
        assert_eq!(expr.eval(&ctx), true);

        let _ = ctx.set("premium", Value::Bool(false));
        assert_eq!(expr.eval(&ctx), false);
    }

    #[test]
    fn test_expr_or() {
        let mut ctx = create_test_context();
        let _ = ctx.set("enabled", Value::Bool(false));
        let _ = ctx.set("premium", Value::Bool(true));

        let expr = Expr::or(vec![Expr::is_true("enabled"), Expr::is_true("premium")]);
        assert_eq!(expr.eval(&ctx), true);

        let _ = ctx.set("premium", Value::Bool(false));
        assert_eq!(expr.eval(&ctx), false);
    }

    #[test]
    fn test_expr_not() {
        let mut ctx = create_test_context();
        let _ = ctx.set("enabled", Value::Bool(true));

        let expr = Expr::negate(Expr::is_true("enabled"));
        assert_eq!(expr.eval(&ctx), false);

        let _ = ctx.set("enabled", Value::Bool(false));
        assert_eq!(expr.eval(&ctx), true);
    }

    #[test]
    fn test_expr_dependencies() {
        let expr = Expr::and(vec![
            Expr::is_true("enabled"),
            Expr::eq("mode", Value::text("advanced")),
            Expr::gte("age", 18.0),
        ]);

        let deps = expr.dependencies();
        assert_eq!(deps.len(), 3);
        assert!(deps.contains(&Key::from("enabled")));
        assert!(deps.contains(&Key::from("mode")));
        assert!(deps.contains(&Key::from("age")));
    }

    #[test]
    fn test_expr_nested_dependencies() {
        let expr = Expr::or(vec![
            Expr::and(vec![Expr::is_true("enabled"), Expr::is_true("premium")]),
            Expr::is_true("admin"),
        ]);

        let deps = expr.dependencies();
        assert_eq!(deps.len(), 3);
    }

    #[test]
    fn test_expr_starts_with() {
        let mut ctx = create_test_context();
        let expr = Expr::starts_with("name", "Ali");

        assert_eq!(expr.eval(&ctx), false);

        let _ = ctx.set("name", Value::text("Alice"));
        assert_eq!(expr.eval(&ctx), true);

        let _ = ctx.set("name", Value::text("Bob"));
        assert_eq!(expr.eval(&ctx), false);

        let _ = ctx.set("name", Value::text("Alison"));
        assert_eq!(expr.eval(&ctx), true);
    }

    #[test]
    fn test_expr_ends_with() {
        let mut ctx = create_test_context();
        let expr = Expr::ends_with("name", "ice");

        assert_eq!(expr.eval(&ctx), false);

        let _ = ctx.set("name", Value::text("Alice"));
        assert_eq!(expr.eval(&ctx), true);

        let _ = ctx.set("name", Value::text("Bob"));
        assert_eq!(expr.eval(&ctx), false);

        let _ = ctx.set("name", Value::text("Janice"));
        assert_eq!(expr.eval(&ctx), true);
    }

    #[cfg(feature = "validation")]
    #[test]
    fn test_expr_matches() {
        let mut ctx = create_test_context();
        let expr = Expr::matches("name", r"^[A-Z][a-z]+$");

        assert_eq!(expr.eval(&ctx), false);

        let _ = ctx.set("name", Value::text("Alice"));
        assert_eq!(expr.eval(&ctx), true);

        let _ = ctx.set("name", Value::text("alice"));
        assert_eq!(expr.eval(&ctx), false);

        let _ = ctx.set("name", Value::text("ALICE"));
        assert_eq!(expr.eval(&ctx), false);

        let _ = ctx.set("name", Value::text("Bob"));
        assert_eq!(expr.eval(&ctx), true);
    }

    #[cfg(feature = "validation")]
    #[test]
    fn test_expr_matches_invalid_regex() {
        let mut ctx = create_test_context();
        let _ = ctx.set("name", Value::text("Alice"));

        // Invalid regex pattern should return false
        let expr = Expr::matches("name", "[invalid(");
        assert_eq!(expr.eval(&ctx), false);
    }

    #[test]
    fn test_expr_between() {
        let mut ctx = create_test_context();
        let expr = Expr::between("age", 18.0, 65.0);

        assert_eq!(expr.eval(&ctx), false);

        let _ = ctx.set("age", Value::Int(25));
        assert_eq!(expr.eval(&ctx), true);

        let _ = ctx.set("age", Value::Int(18));
        assert_eq!(expr.eval(&ctx), true);

        let _ = ctx.set("age", Value::Int(65));
        assert_eq!(expr.eval(&ctx), true);

        let _ = ctx.set("age", Value::Int(17));
        assert_eq!(expr.eval(&ctx), false);

        let _ = ctx.set("age", Value::Int(66));
        assert_eq!(expr.eval(&ctx), false);

        let _ = ctx.set("age", Value::Float(25.5));
        assert_eq!(expr.eval(&ctx), true);
    }

    #[test]
    fn test_expr_length_min() {
        let mut ctx = create_test_context();
        let expr = Expr::length_min("name", 3);

        assert_eq!(expr.eval(&ctx), false);

        let _ = ctx.set("name", Value::text("Al"));
        assert_eq!(expr.eval(&ctx), false);

        let _ = ctx.set("name", Value::text("Ali"));
        assert_eq!(expr.eval(&ctx), true);

        let _ = ctx.set("name", Value::text("Alice"));
        assert_eq!(expr.eval(&ctx), true);

        // Test with array
        let schema = Arc::new(
            Schema::builder()
                .parameter(Text::builder("items").build())
                .build(),
        );
        let mut ctx = Context::new(schema);
        let array_expr = Expr::length_min("items", 3);

        let _ = ctx.set("items", Value::array(vec![Value::Int(1), Value::Int(2)]));
        assert_eq!(array_expr.eval(&ctx), false);

        let _ = ctx.set(
            "items",
            Value::array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]),
        );
        assert_eq!(array_expr.eval(&ctx), true);
    }

    #[test]
    fn test_expr_length_max() {
        let mut ctx = create_test_context();
        let expr = Expr::length_max("name", 5);

        assert_eq!(expr.eval(&ctx), false);

        let _ = ctx.set("name", Value::text("Ali"));
        assert_eq!(expr.eval(&ctx), true);

        let _ = ctx.set("name", Value::text("Alice"));
        assert_eq!(expr.eval(&ctx), true);

        let _ = ctx.set("name", Value::text("Alison"));
        assert_eq!(expr.eval(&ctx), false);

        // Test with array
        let schema = Arc::new(
            Schema::builder()
                .parameter(Text::builder("items").build())
                .build(),
        );
        let mut ctx = Context::new(schema);
        let array_expr = Expr::length_max("items", 5);

        let _ = ctx.set(
            "items",
            Value::array(vec![
                Value::Int(1),
                Value::Int(2),
                Value::Int(3),
                Value::Int(4),
                Value::Int(5),
            ]),
        );
        assert_eq!(array_expr.eval(&ctx), true);

        let _ = ctx.set(
            "items",
            Value::array(vec![
                Value::Int(1),
                Value::Int(2),
                Value::Int(3),
                Value::Int(4),
                Value::Int(5),
                Value::Int(6),
            ]),
        );
        assert_eq!(array_expr.eval(&ctx), false);
    }

    #[test]
    fn test_expr_length_between() {
        let mut ctx = create_test_context();
        let expr = Expr::length_between("name", 3, 8);

        assert_eq!(expr.eval(&ctx), false);

        let _ = ctx.set("name", Value::text("Al"));
        assert_eq!(expr.eval(&ctx), false);

        let _ = ctx.set("name", Value::text("Ali"));
        assert_eq!(expr.eval(&ctx), true);

        let _ = ctx.set("name", Value::text("Alice"));
        assert_eq!(expr.eval(&ctx), true);

        let _ = ctx.set("name", Value::text("Alison"));
        assert_eq!(expr.eval(&ctx), true);

        let _ = ctx.set("name", Value::text("Alexandra"));
        assert_eq!(expr.eval(&ctx), false);

        // Test with array
        let schema = Arc::new(
            Schema::builder()
                .parameter(Text::builder("items").build())
                .build(),
        );
        let mut ctx = Context::new(schema);
        let array_expr = Expr::length_between("items", 3, 8);

        let _ = ctx.set("items", Value::array(vec![Value::Int(1), Value::Int(2)]));
        assert_eq!(array_expr.eval(&ctx), false);

        let _ = ctx.set(
            "items",
            Value::array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]),
        );
        assert_eq!(array_expr.eval(&ctx), true);

        let _ = ctx.set(
            "items",
            Value::array(vec![
                Value::Int(1),
                Value::Int(2),
                Value::Int(3),
                Value::Int(4),
                Value::Int(5),
                Value::Int(6),
                Value::Int(7),
                Value::Int(8),
            ]),
        );
        assert_eq!(array_expr.eval(&ctx), true);

        let _ = ctx.set(
            "items",
            Value::array(vec![
                Value::Int(1),
                Value::Int(2),
                Value::Int(3),
                Value::Int(4),
                Value::Int(5),
                Value::Int(6),
                Value::Int(7),
                Value::Int(8),
                Value::Int(9),
            ]),
        );
        assert_eq!(array_expr.eval(&ctx), false);
    }

    #[test]
    fn test_expr_is_null() {
        let mut ctx = create_test_context();
        let expr = Expr::is_null("name");

        // Not set - is_null returns false (no value)
        assert_eq!(expr.eval(&ctx), false);

        let _ = ctx.set("name", Value::Null);
        assert_eq!(expr.eval(&ctx), true);

        let _ = ctx.set("name", Value::text(""));
        assert_eq!(expr.eval(&ctx), false);

        let _ = ctx.set("name", Value::text("Alice"));
        assert_eq!(expr.eval(&ctx), false);
    }

    #[test]
    fn test_expr_is_not_empty() {
        let mut ctx = create_test_context();
        let expr = Expr::is_not_empty("name");

        assert_eq!(expr.eval(&ctx), false);

        let _ = ctx.set("name", Value::text(""));
        assert_eq!(expr.eval(&ctx), false);

        let _ = ctx.set("name", Value::Null);
        assert_eq!(expr.eval(&ctx), false);

        let _ = ctx.set("name", Value::text("Alice"));
        assert_eq!(expr.eval(&ctx), true);

        let _ = ctx.set("name", Value::array(vec![]));
        assert_eq!(expr.eval(&ctx), false);

        let _ = ctx.set("name", Value::array(vec![Value::Int(1)]));
        assert_eq!(expr.eval(&ctx), true);
    }

    #[test]
    fn test_expr_new_variants_dependencies() {
        // Test StartsWith
        let expr = Expr::starts_with("name", "Ali");
        let deps = expr.dependencies();
        assert_eq!(deps.len(), 1);
        assert!(deps.contains(&Key::from("name")));

        // Test Between
        let expr = Expr::between("age", 18.0, 65.0);
        let deps = expr.dependencies();
        assert_eq!(deps.len(), 1);
        assert!(deps.contains(&Key::from("age")));

        // Test LengthBetween
        let expr = Expr::length_between("name", 3, 8);
        let deps = expr.dependencies();
        assert_eq!(deps.len(), 1);
        assert!(deps.contains(&Key::from("name")));

        // Test compound with new variants
        let expr = Expr::and(vec![
            Expr::starts_with("name", "A"),
            Expr::length_min("name", 3),
            Expr::between("age", 18.0, 65.0),
        ]);
        let deps = expr.dependencies();
        assert_eq!(deps.len(), 2);
        assert!(deps.contains(&Key::from("name")));
        assert!(deps.contains(&Key::from("age")));
    }

    #[test]
    fn test_expr_length_with_wrong_type() {
        let mut ctx = create_test_context();

        // Set age to a number
        let _ = ctx.set("age", Value::Int(42));

        // Length checks should return false for non-text, non-array types
        assert_eq!(Expr::length_min("age", 1).eval(&ctx), false);
        assert_eq!(Expr::length_max("age", 10).eval(&ctx), false);
        assert_eq!(Expr::length_between("age", 1, 10).eval(&ctx), false);
    }

    #[test]
    fn test_expr_between_with_wrong_type() {
        let mut ctx = create_test_context();

        // Set name to a string
        let _ = ctx.set("name", Value::text("Alice"));

        // Between should return false for non-numeric types
        assert_eq!(Expr::between("name", 1.0, 10.0).eval(&ctx), false);
    }

    #[test]
    fn test_expr_string_ops_with_wrong_type() {
        let mut ctx = create_test_context();

        // Set age to a number
        let _ = ctx.set("age", Value::Int(42));

        // String operations should return false for non-text types
        assert_eq!(Expr::starts_with("age", "4").eval(&ctx), false);
        assert_eq!(Expr::ends_with("age", "2").eval(&ctx), false);
    }
}

/// Serde helper for Arc<[T]> serialization/deserialization.
#[cfg(feature = "serde")]
mod arc_slice_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::sync::Arc;

    pub fn serialize<S, T>(value: &Arc<[T]>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        value.as_ref().serialize(serializer)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Arc<[T]>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        let vec = Vec::<T>::deserialize(deserializer)?;
        Ok(Arc::from(vec.into_boxed_slice()))
    }
}
