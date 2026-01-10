//! Rule combining expression target and expression.

use super::{Expr, ExprTarget};
use crate::core::Key;

/// Rule combining a target and an expression.
///
/// A rule specifies:
/// - **Where** to apply the check (`ExprTarget`)
/// - **What** to check (`Expr`)
///
/// # Usage
///
/// ## Validation (local target)
///
/// ```ignore
/// use paramdef::expr::{Expr, Rule};
///
/// // Validate current value has minimum length of 8
/// let rule = Rule::local(Expr::MinLength(8));
///
/// // Or using convenience method
/// let rule = Expr::min_length(8);
/// ```
///
/// ## Visibility (field target)
///
/// ```ignore
/// use paramdef::expr::{Expr, Rule};
/// use paramdef::core::Value;
///
/// // Show current field only if "mode" equals "advanced"
/// let rule = Rule::field("mode", Expr::Eq(Value::text("advanced")));
///
/// // Or using on_field method
/// let rule = Expr::eq(Value::text("advanced")).on_field("mode");
/// ```
///
/// ## Cross-field Validation
///
/// ```ignore
/// use paramdef::expr::{Expr, Rule};
///
/// // Validate that "password_confirm" field matches "password" field
/// // This would be checked when validating the parent object
/// let rule = Rule::field("password_confirm", Expr::eq_field("password"));
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rule {
    /// Target for expression evaluation.
    pub target: ExprTarget,
    /// Expression to evaluate.
    pub expr: Expr,
}

impl Rule {
    /// Create a new rule.
    #[must_use]
    pub fn new(target: ExprTarget, expr: Expr) -> Self {
        Self { target, expr }
    }

    /// Create a rule targeting the local value.
    ///
    /// Used for validation rules that check the current value being validated.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use paramdef::expr::{Expr, Rule};
    ///
    /// let rule = Rule::local(Expr::MinLength(8));
    /// ```
    #[must_use]
    pub fn local(expr: Expr) -> Self {
        Self::new(ExprTarget::Local, expr)
    }

    /// Create a rule targeting another field.
    ///
    /// Used for:
    /// - Visibility conditions (show field X if field Y meets condition)
    /// - Cross-field validation (field X must match field Y)
    ///
    /// # Example
    ///
    /// ```ignore
    /// use paramdef::expr::{Expr, Rule};
    /// use paramdef::core::Value;
    ///
    /// // Show field if "mode" equals "advanced"
    /// let rule = Rule::field("mode", Expr::Eq(Value::text("advanced")));
    /// ```
    #[must_use]
    pub fn field(key: impl Into<Key>, expr: Expr) -> Self {
        Self::new(ExprTarget::field(key), expr)
    }

    /// Returns true if this rule targets the local value.
    #[must_use]
    pub fn is_local(&self) -> bool {
        self.target.is_local()
    }

    /// Returns true if this rule targets a field.
    #[must_use]
    pub fn is_field(&self) -> bool {
        self.target.is_field()
    }

    /// Get the field key if this rule targets a field.
    #[must_use]
    pub fn field_key(&self) -> Option<&Key> {
        self.target.field_key()
    }
}

impl Rule {
    /// Evaluate this rule against a context.
    ///
    /// For local targets, evaluates the expression against the provided value.
    /// For field targets, gets the field value from context and evaluates.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use paramdef::expr::{Expr, Rule};
    /// use paramdef::context::Context;
    /// use paramdef::core::Value;
    ///
    /// // Local rule
    /// let rule = Rule::local(Expr::MinLength(5));
    /// // Would need to pass value somehow
    ///
    /// // Field rule
    /// let rule = Rule::field("mode", Expr::Eq(Value::text("advanced")));
    /// // Evaluates against context
    /// let result = rule.eval(&ctx);
    /// ```
    #[cfg(feature = "visibility")]
    #[must_use]
    pub fn eval(&self, ctx: &crate::context::Context) -> bool {
        match &self.target {
            ExprTarget::Local => {
                // For local, we don't have a value here, return false
                // This should be used with validate() instead
                false
            }
            ExprTarget::Field(key) => {
                // Get the field value from context and evaluate
                if let Some(value) = ctx.get(key.as_str()) {
                    self.expr.eval(value)
                } else {
                    false
                }
            }
        }
    }

    /// Get all parameter keys this rule depends on.
    ///
    /// For local targets, returns empty vec.
    /// For field targets, returns the field key.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use paramdef::expr::{Expr, Rule};
    /// use paramdef::core::{Key, Value};
    ///
    /// let rule = Rule::field("mode", Expr::Eq(Value::text("advanced")));
    /// let deps = rule.dependencies();
    /// assert_eq!(deps, vec![Key::from("mode")]);
    /// ```
    #[must_use]
    pub fn dependencies(&self) -> Vec<Key> {
        match &self.target {
            ExprTarget::Local => vec![],
            ExprTarget::Field(key) => vec![key.clone()],
        }
    }
}

impl Expr {
    /// Apply this expression to the local value.
    ///
    /// Creates a `Rule` with `ExprTarget::Local`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use paramdef::expr::Expr;
    ///
    /// let rule = Expr::MinLength(8).on_local();
    /// ```
    #[must_use]
    pub fn on_local(self) -> Rule {
        Rule::local(self)
    }

    /// Apply this expression to another field.
    ///
    /// Creates a `Rule` with `ExprTarget::Field`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use paramdef::expr::Expr;
    ///
    /// let rule = Expr::MinLength(3).on_field("username");
    /// ```
    #[must_use]
    pub fn on_field(self, key: impl Into<Key>) -> Rule {
        Rule::field(key, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Value;

    #[test]
    fn test_rule_local() {
        let rule = Rule::local(Expr::MinLength(8));
        assert!(rule.is_local());
        assert!(!rule.is_field());
        assert_eq!(rule.field_key(), None);
    }

    #[test]
    fn test_rule_field() {
        let rule = Rule::field("username", Expr::MinLength(3));
        assert!(!rule.is_local());
        assert!(rule.is_field());
        assert_eq!(rule.field_key(), Some(&Key::from("username")));
    }

    #[test]
    fn test_expr_on_local() {
        let rule = Expr::min_length(8).on_local();
        assert!(rule.is_local());
        assert_eq!(rule.expr, Expr::MinLength(8));
    }

    #[test]
    fn test_expr_on_field() {
        let rule = Expr::min_length(3).on_field("username");
        assert!(rule.is_field());
        assert_eq!(rule.field_key(), Some(&Key::from("username")));
        assert_eq!(rule.expr, Expr::MinLength(3));
    }

    #[test]
    fn test_rule_equality() {
        let rule1 = Rule::local(Expr::Email);
        let rule2 = Rule::local(Expr::Email);
        assert_eq!(rule1, rule2);

        let rule3 = Rule::field("email", Expr::Email);
        let rule4 = Rule::field("email", Expr::Email);
        assert_eq!(rule3, rule4);

        assert_ne!(rule1, rule3);
    }

    #[test]
    fn test_complex_rule() {
        let rule = Rule::field(
            "password",
            Expr::and(vec![Expr::MinLength(8), Expr::contains(Value::text("!"))]),
        );

        assert!(rule.is_field());
        assert_eq!(rule.field_key(), Some(&Key::from("password")));
    }
}
