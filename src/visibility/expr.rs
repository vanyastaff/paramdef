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
///
/// ### State Checks
/// - `IsSet` - Parameter has a value (not null/undefined)
/// - `IsEmpty` - Parameter is empty (null, empty string, empty array)
/// - `IsTrue` - Boolean parameter is true
/// - `IsFalse` - Boolean parameter is false
/// - `IsValid` - Parameter passes validation
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
            | Self::IsValid(key) => {
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

        ctx.set("name", Value::text("Alice"));
        assert_eq!(expr.eval(&ctx), true);

        ctx.set("name", Value::text("Bob"));
        assert_eq!(expr.eval(&ctx), false);
    }

    #[test]
    fn test_expr_ne() {
        let mut ctx = create_test_context();
        ctx.set("name", Value::text("Alice"));

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

        ctx.set("name", Value::text("Alice"));
        assert_eq!(expr.eval(&ctx), true);
    }

    #[test]
    fn test_expr_is_empty() {
        let mut ctx = create_test_context();
        let expr = Expr::is_empty("name");

        assert_eq!(expr.eval(&ctx), true);

        ctx.set("name", Value::text(""));
        assert_eq!(expr.eval(&ctx), true);

        ctx.set("name", Value::text("Alice"));
        assert_eq!(expr.eval(&ctx), false);
    }

    #[test]
    fn test_expr_is_true() {
        let mut ctx = create_test_context();
        let expr = Expr::is_true("enabled");

        assert_eq!(expr.eval(&ctx), false);

        ctx.set("enabled", Value::Bool(true));
        assert_eq!(expr.eval(&ctx), true);

        ctx.set("enabled", Value::Bool(false));
        assert_eq!(expr.eval(&ctx), false);
    }

    #[test]
    fn test_expr_is_false() {
        let mut ctx = create_test_context();
        let expr = Expr::is_false("enabled");

        assert_eq!(expr.eval(&ctx), false);

        ctx.set("enabled", Value::Bool(false));
        assert_eq!(expr.eval(&ctx), true);

        ctx.set("enabled", Value::Bool(true));
        assert_eq!(expr.eval(&ctx), false);
    }

    #[test]
    fn test_expr_numeric_comparisons() {
        let mut ctx = create_test_context();
        ctx.set("age", Value::Int(25));

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
        ctx.set("mode", Value::text("advanced"));

        let expr = Expr::one_of(
            "mode",
            vec![
                Value::text("basic"),
                Value::text("advanced"),
                Value::text("expert"),
            ],
        );
        assert_eq!(expr.eval(&ctx), true);

        ctx.set("mode", Value::text("custom"));
        assert_eq!(expr.eval(&ctx), false);
    }

    #[test]
    fn test_expr_and() {
        let mut ctx = create_test_context();
        ctx.set("enabled", Value::Bool(true));
        ctx.set("premium", Value::Bool(true));

        let expr = Expr::and(vec![Expr::is_true("enabled"), Expr::is_true("premium")]);
        assert_eq!(expr.eval(&ctx), true);

        ctx.set("premium", Value::Bool(false));
        assert_eq!(expr.eval(&ctx), false);
    }

    #[test]
    fn test_expr_or() {
        let mut ctx = create_test_context();
        ctx.set("enabled", Value::Bool(false));
        ctx.set("premium", Value::Bool(true));

        let expr = Expr::or(vec![Expr::is_true("enabled"), Expr::is_true("premium")]);
        assert_eq!(expr.eval(&ctx), true);

        ctx.set("premium", Value::Bool(false));
        assert_eq!(expr.eval(&ctx), false);
    }

    #[test]
    fn test_expr_not() {
        let mut ctx = create_test_context();
        ctx.set("enabled", Value::Bool(true));

        let expr = Expr::negate(Expr::is_true("enabled"));
        assert_eq!(expr.eval(&ctx), false);

        ctx.set("enabled", Value::Bool(false));
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
