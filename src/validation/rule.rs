//! Validation rule combining declarative and programmatic approaches.
//!
//! [`Rule`] is the primary unit of validation, supporting:
//! - Declarative expressions via [`Expr`] (~80% of cases)
//! - Programmatic validators via [`Validator`] trait (~20% complex cases)
//!
//! # Design Philosophy
//!
//! The hybrid approach provides:
//! - **Serializability**: Expr rules can be stored in JSON/YAML
//! - **Flexibility**: Custom validators for complex business logic
//! - **Composability**: Rules can be combined into lists
//! - **Performance**: Expr rules are evaluated without dynamic dispatch
//!
//! # Example
//!
//! ```ignore
//! use paramdef::validation::{Rule, Expr};
//! use std::sync::Arc;
//!
//! // Declarative rules (most common)
//! let rules = vec![
//!     Rule::required(),
//!     Rule::min_length(3),
//!     Rule::max_length(100),
//!     Rule::email(),
//! ];
//!
//! // Custom validator for complex logic
//! let custom = Rule::custom("unique_username", |value, ctx| {
//!     // Check against database, etc.
//!     Ok(())
//! });
//! ```

use std::sync::Arc;

use super::context::ValidationContext;
use super::expr::Expr;
use super::result::ValidationResult;
use super::traits::{FnValidator, Validator};
use crate::core::Value;

/// A validation rule that can be either declarative or programmatic.
///
/// Rules are the building blocks of parameter validation. Each parameter
/// can have multiple rules that are evaluated in order.
#[derive(Clone)]
pub enum Rule {
    /// Declarative validation expression.
    ///
    /// Serializable, covers common validation cases.
    Expr(Expr),

    /// Programmatic validator.
    ///
    /// Uses `Arc` for cheap cloning and schema sharing.
    Fn(Arc<dyn Validator>),
}

impl std::fmt::Debug for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expr(expr) => f.debug_tuple("Expr").field(expr).finish(),
            Self::Fn(v) => f.debug_tuple("Fn").field(&v.name()).finish(),
        }
    }
}

impl Rule {
    /// Validates a value with this rule.
    ///
    /// For rules without cross-field dependencies, context provides access to other fields.
    ///
    /// # Errors
    ///
    /// Returns `Err(ValidationOutcome)` if the value fails validation.
    pub fn validate(&self, value: &Value, ctx: &ValidationContext<'_>) -> ValidationResult {
        match self {
            Self::Expr(expr) => expr.validate_with_context(value, ctx),
            Self::Fn(validator) => validator.validate(value, ctx),
        }
    }

    /// Validates a value without context.
    ///
    /// This is a simplified validation method for cases where cross-field
    /// validation is not needed. Function-based validators (`Rule::Fn`) are
    /// skipped and return `Ok(())` since they require context.
    ///
    /// Use [`validate`](Self::validate) with a `ValidationContext` for complete
    /// validation including function-based rules.
    ///
    /// # Errors
    ///
    /// Returns `Err(ValidationOutcome)` if the value fails validation.
    pub fn check(&self, value: &Value) -> ValidationResult {
        match self {
            Self::Expr(expr) => expr.validate(value),
            Self::Fn(_) => {
                // Cannot validate Fn rules without context
                // Return Ok to allow standalone validation of Expr rules
                Ok(())
            }
        }
    }

    // === Convenience Constructors ===

    /// Creates a required rule.
    #[must_use]
    pub const fn required() -> Self {
        Self::Expr(Expr::Required)
    }

    /// Creates a minimum length rule.
    #[must_use]
    pub const fn min_length(min: usize) -> Self {
        Self::Expr(Expr::MinLength(min))
    }

    /// Creates a maximum length rule.
    #[must_use]
    pub const fn max_length(max: usize) -> Self {
        Self::Expr(Expr::MaxLength(max))
    }

    /// Creates an exact length rule.
    #[must_use]
    pub const fn length(len: usize) -> Self {
        Self::Expr(Expr::Length(len))
    }

    /// Creates a pattern (regex) rule.
    #[must_use]
    pub fn pattern(pattern: impl Into<crate::core::SmartStr>) -> Self {
        Self::Expr(Expr::Pattern(pattern.into()))
    }

    /// Creates an email validation rule.
    #[must_use]
    pub const fn email() -> Self {
        Self::Expr(Expr::Email)
    }

    /// Creates a URL validation rule.
    #[must_use]
    pub const fn url() -> Self {
        Self::Expr(Expr::Url)
    }

    /// Creates a UUID validation rule.
    #[must_use]
    pub const fn uuid() -> Self {
        Self::Expr(Expr::Uuid)
    }

    /// Creates a minimum value rule.
    #[must_use]
    pub const fn min(min: f64) -> Self {
        Self::Expr(Expr::Min(min))
    }

    /// Creates a maximum value rule.
    #[must_use]
    pub const fn max(max: f64) -> Self {
        Self::Expr(Expr::Max(max))
    }

    /// Creates an exclusive minimum rule.
    #[must_use]
    pub const fn exclusive_min(min: f64) -> Self {
        Self::Expr(Expr::ExclusiveMin(min))
    }

    /// Creates an exclusive maximum rule.
    #[must_use]
    pub const fn exclusive_max(max: f64) -> Self {
        Self::Expr(Expr::ExclusiveMax(max))
    }

    /// Creates a range rule (min <= value <= max).
    #[must_use]
    pub fn range(min: f64, max: f64) -> Self {
        Self::Expr(Expr::And(vec![Expr::Min(min), Expr::Max(max)]))
    }

    /// Creates a positive number rule.
    #[must_use]
    pub const fn positive() -> Self {
        Self::Expr(Expr::Positive)
    }

    /// Creates a negative number rule.
    #[must_use]
    pub const fn negative() -> Self {
        Self::Expr(Expr::Negative)
    }

    /// Creates a non-negative number rule.
    #[must_use]
    pub const fn non_negative() -> Self {
        Self::Expr(Expr::NonNegative)
    }

    /// Creates an integer rule.
    #[must_use]
    pub const fn integer() -> Self {
        Self::Expr(Expr::Integer)
    }

    /// Creates a multiple-of rule.
    #[must_use]
    pub const fn multiple_of(divisor: f64) -> Self {
        Self::Expr(Expr::MultipleOf(divisor))
    }

    /// Creates a minimum items rule for arrays.
    #[must_use]
    pub const fn min_items(min: usize) -> Self {
        Self::Expr(Expr::MinItems(min))
    }

    /// Creates a maximum items rule for arrays.
    #[must_use]
    pub const fn max_items(max: usize) -> Self {
        Self::Expr(Expr::MaxItems(max))
    }

    /// Creates a unique items rule for arrays.
    #[must_use]
    pub const fn unique_items() -> Self {
        Self::Expr(Expr::UniqueItems)
    }

    /// Creates a one-of (enum) rule.
    #[must_use]
    pub fn one_of(values: impl IntoIterator<Item = Value>) -> Self {
        Self::Expr(Expr::OneOf(values.into_iter().collect()))
    }

    /// Creates a string enum rule.
    #[must_use]
    pub fn string_enum(values: impl IntoIterator<Item = impl Into<crate::core::SmartStr>>) -> Self {
        Self::Expr(Expr::OneOf(
            values.into_iter().map(|s| Value::text(s)).collect(),
        ))
    }

    /// Creates a constant value rule.
    #[must_use]
    pub fn constant(value: Value) -> Self {
        Self::Expr(Expr::Const(value))
    }

    /// Creates an equal-to rule for cross-field validation.
    #[must_use]
    pub fn equal_to(other_key: impl Into<crate::core::SmartStr>) -> Self {
        Self::Expr(Expr::EqualTo(other_key.into()))
    }

    /// Creates a not-equal-to rule for cross-field validation.
    #[must_use]
    pub fn not_equal_to(other_key: impl Into<crate::core::SmartStr>) -> Self {
        Self::Expr(Expr::NotEqualTo(other_key.into()))
    }

    /// Creates an AND rule combining multiple rules.
    #[must_use]
    pub fn all(rules: impl IntoIterator<Item = Rule>) -> Self {
        let exprs: Vec<Expr> = rules
            .into_iter()
            .filter_map(|r| match r {
                Rule::Expr(e) => Some(e),
                Rule::Fn(_) => None, // Can't combine Fn rules in Expr::And
            })
            .collect();
        Self::Expr(Expr::And(exprs))
    }

    /// Creates an OR rule for alternative validation.
    #[must_use]
    pub fn any(rules: impl IntoIterator<Item = Rule>) -> Self {
        let exprs: Vec<Expr> = rules
            .into_iter()
            .filter_map(|r| match r {
                Rule::Expr(e) => Some(e),
                Rule::Fn(_) => None,
            })
            .collect();
        Self::Expr(Expr::Or(exprs))
    }

    /// Creates a NOT rule for negation.
    #[must_use]
    pub fn negate(rule: Rule) -> Self {
        match rule {
            Rule::Expr(e) => Self::Expr(Expr::Not(Box::new(e))),
            Rule::Fn(_) => rule, // Can't negate Fn rules
        }
    }

    /// Creates a custom validator from a function.
    #[must_use]
    pub fn custom<F>(name: &'static str, f: F) -> Self
    where
        F: Fn(&Value, &ValidationContext<'_>) -> ValidationResult + Send + Sync + 'static,
    {
        Self::Fn(Arc::new(FnValidator::new(name, f)))
    }

    /// Creates a rule from a validator.
    #[must_use]
    pub fn validator(v: impl Validator + 'static) -> Self {
        Self::Fn(Arc::new(v))
    }

    /// Returns `true` if this is an expression rule.
    #[must_use]
    pub const fn is_expr(&self) -> bool {
        matches!(self, Self::Expr(_))
    }

    /// Returns `true` if this is a function rule.
    #[must_use]
    pub const fn is_fn(&self) -> bool {
        matches!(self, Self::Fn(_))
    }
}

impl From<Expr> for Rule {
    fn from(expr: Expr) -> Self {
        Self::Expr(expr)
    }
}

impl<F> From<FnValidator<F>> for Rule
where
    F: Fn(&Value, &ValidationContext<'_>) -> ValidationResult + Send + Sync + 'static,
{
    fn from(validator: FnValidator<F>) -> Self {
        Self::Fn(Arc::new(validator))
    }
}

/// A collection of validation rules for a parameter.
#[derive(Debug, Clone, Default)]
pub struct Rules {
    rules: Vec<Rule>,
}

impl Rules {
    /// Creates an empty rule collection.
    #[must_use]
    pub const fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Creates a rule collection with a single rule.
    #[must_use]
    pub fn single(rule: Rule) -> Self {
        Self { rules: vec![rule] }
    }

    /// Creates a rule collection from multiple rules.
    #[must_use]
    pub fn from_rules(rules: impl IntoIterator<Item = Rule>) -> Self {
        Self {
            rules: rules.into_iter().collect(),
        }
    }

    /// Adds a rule to the collection.
    pub fn push(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    /// Returns `true` if there are no rules.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    /// Returns the number of rules.
    #[must_use]
    pub fn len(&self) -> usize {
        self.rules.len()
    }

    /// Validates a value against all rules.
    ///
    /// Returns the first error encountered (fail-fast).
    ///
    /// # Errors
    ///
    /// Returns `Err(ValidationOutcome)` if any rule fails.
    pub fn validate(&self, value: &Value, ctx: &ValidationContext<'_>) -> ValidationResult {
        for rule in &self.rules {
            rule.validate(value, ctx)?;
        }
        Ok(())
    }

    /// Validates a value and collects all errors.
    ///
    /// Unlike [`validate`](Self::validate), this doesn't short-circuit on first error.
    #[must_use]
    pub fn validate_all(
        &self,
        value: &Value,
        ctx: &ValidationContext<'_>,
    ) -> Vec<super::result::Error> {
        let mut errors = Vec::new();
        for rule in &self.rules {
            if let Err(outcome) = rule.validate(value, ctx) {
                errors.extend(outcome.errors().iter().cloned());
            }
        }
        errors
    }

    /// Returns an iterator over the rules.
    pub fn iter(&self) -> impl Iterator<Item = &Rule> {
        self.rules.iter()
    }
}

impl IntoIterator for Rules {
    type Item = Rule;
    type IntoIter = std::vec::IntoIter<Rule>;

    fn into_iter(self) -> Self::IntoIter {
        self.rules.into_iter()
    }
}

impl<'a> IntoIterator for &'a Rules {
    type Item = &'a Rule;
    type IntoIter = std::slice::Iter<'a, Rule>;

    fn into_iter(self) -> Self::IntoIter {
        self.rules.iter()
    }
}

impl Extend<Rule> for Rules {
    fn extend<T: IntoIterator<Item = Rule>>(&mut self, iter: T) {
        self.rules.extend(iter);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::Schema;
    use crate::types::leaf::Text;
    use crate::validation::context::NoValues;

    fn create_test_context<'a>(
        key: &'a crate::core::Key,
        schema: &'a Arc<Schema>,
        values: &'a NoValues,
    ) -> ValidationContext<'a> {
        ValidationContext::new(key, schema, values)
    }

    #[test]
    fn test_rule_required() {
        let schema = Arc::new(
            Schema::builder()
                .parameter(Text::builder("test").build())
                .build(),
        );
        let key = "test".into();
        let values = NoValues;
        let ctx = create_test_context(&key, &schema, &values);

        let rule = Rule::required();
        assert!(rule.validate(&Value::text("hello"), &ctx).is_ok());
        assert!(rule.validate(&Value::Null, &ctx).is_err());
    }

    #[test]
    fn test_rule_min_max_length() {
        let schema = Arc::new(
            Schema::builder()
                .parameter(Text::builder("test").build())
                .build(),
        );
        let key = "test".into();
        let values = NoValues;
        let ctx = create_test_context(&key, &schema, &values);

        let min = Rule::min_length(3);
        let max = Rule::max_length(10);

        assert!(min.validate(&Value::text("abc"), &ctx).is_ok());
        assert!(min.validate(&Value::text("ab"), &ctx).is_err());

        assert!(max.validate(&Value::text("short"), &ctx).is_ok());
        assert!(
            max.validate(&Value::text("this is too long"), &ctx)
                .is_err()
        );
    }

    #[test]
    fn test_rule_range() {
        let schema = Arc::new(
            Schema::builder()
                .parameter(Text::builder("test").build())
                .build(),
        );
        let key = "test".into();
        let values = NoValues;
        let ctx = create_test_context(&key, &schema, &values);

        let rule = Rule::range(0.0, 100.0);

        assert!(rule.validate(&Value::Int(50), &ctx).is_ok());
        assert!(rule.validate(&Value::Int(0), &ctx).is_ok());
        assert!(rule.validate(&Value::Int(100), &ctx).is_ok());
        assert!(rule.validate(&Value::Int(-1), &ctx).is_err());
        assert!(rule.validate(&Value::Int(101), &ctx).is_err());
    }

    #[test]
    fn test_rule_email() {
        let schema = Arc::new(
            Schema::builder()
                .parameter(Text::builder("test").build())
                .build(),
        );
        let key = "test".into();
        let values = NoValues;
        let ctx = create_test_context(&key, &schema, &values);

        let rule = Rule::email();

        assert!(
            rule.validate(&Value::text("test@example.com"), &ctx)
                .is_ok()
        );
        assert!(rule.validate(&Value::text("invalid"), &ctx).is_err());
    }

    #[test]
    fn test_rule_string_enum() {
        let schema = Arc::new(
            Schema::builder()
                .parameter(Text::builder("test").build())
                .build(),
        );
        let key = "test".into();
        let values = NoValues;
        let ctx = create_test_context(&key, &schema, &values);

        let rule = Rule::string_enum(["active", "inactive", "pending"]);

        assert!(rule.validate(&Value::text("active"), &ctx).is_ok());
        assert!(rule.validate(&Value::text("unknown"), &ctx).is_err());
    }

    #[test]
    fn test_rule_custom() {
        let schema = Arc::new(
            Schema::builder()
                .parameter(Text::builder("test").build())
                .build(),
        );
        let key = "test".into();
        let values = NoValues;
        let ctx = create_test_context(&key, &schema, &values);

        let rule = Rule::custom("is_even", |value, _ctx| {
            if let Value::Int(n) = value {
                if n % 2 != 0 {
                    return Err(
                        super::super::result::Error::custom("even", "Value must be even").into(),
                    );
                }
            }
            Ok(())
        });

        assert!(rule.validate(&Value::Int(2), &ctx).is_ok());
        assert!(rule.validate(&Value::Int(3), &ctx).is_err());
    }

    #[test]
    fn test_rules_collection() {
        let schema = Arc::new(
            Schema::builder()
                .parameter(Text::builder("test").build())
                .build(),
        );
        let key = "test".into();
        let values = NoValues;
        let ctx = create_test_context(&key, &schema, &values);

        let mut rules = Rules::new();
        rules.push(Rule::required());
        rules.push(Rule::min_length(3));
        rules.push(Rule::max_length(10));

        assert!(rules.validate(&Value::text("hello"), &ctx).is_ok());
        assert!(rules.validate(&Value::text("ab"), &ctx).is_err());
        assert!(rules.validate(&Value::Null, &ctx).is_err());
    }

    #[test]
    fn test_rules_validate_all() {
        let schema = Arc::new(
            Schema::builder()
                .parameter(Text::builder("test").build())
                .build(),
        );
        let key = "test".into();
        let values = NoValues;
        let ctx = create_test_context(&key, &schema, &values);

        let rules = Rules::from_rules([Rule::min_length(10), Rule::max_length(5)]);

        // Both rules will fail for "abc"
        let errors = rules.validate_all(&Value::text("abc"), &ctx);
        // min_length fails, max_length passes for "abc"
        assert_eq!(errors.len(), 1);

        // Empty string fails both required (if we add it) and min_length
        let rules = Rules::from_rules([Rule::required(), Rule::min_length(5)]);
        let errors = rules.validate_all(&Value::text(""), &ctx);
        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn test_rule_debug() {
        let expr_rule = Rule::required();
        let debug_str = format!("{:?}", expr_rule);
        assert!(debug_str.contains("Expr"));
        assert!(debug_str.contains("Required"));

        let fn_rule = Rule::custom("test_validator", |_, _| Ok(()));
        let debug_str = format!("{:?}", fn_rule);
        assert!(debug_str.contains("Fn"));
        assert!(debug_str.contains("test_validator"));
    }

    #[test]
    fn test_rule_is_expr_is_fn() {
        let expr_rule = Rule::required();
        assert!(expr_rule.is_expr());
        assert!(!expr_rule.is_fn());

        let fn_rule = Rule::custom("test", |_, _| Ok(()));
        assert!(!fn_rule.is_expr());
        assert!(fn_rule.is_fn());
    }
}
