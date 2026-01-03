//! Validator trait for programmatic validation.
//!
//! The [`Validator`] trait enables custom validation logic that can't be
//! expressed declaratively. Use this for:
//!
//! - Complex business rules
//! - Cross-field validation
//! - Async validation (database lookups, API calls)
//! - Integration with external validation libraries
//!
//! # Example
//!
//! ```ignore
//! use paramdef::validation::{Validator, ValidationContext, ValidationResult};
//! use paramdef::core::Value;
//!
//! /// Validates that password and confirmation match.
//! struct PasswordMatch;
//!
//! impl Validator for PasswordMatch {
//!     fn validate(&self, value: &Value, ctx: &ValidationContext<'_>) -> ValidationResult {
//!         let password = value.as_text().unwrap_or("");
//!         let confirm = ctx.get("password_confirm")
//!             .and_then(|v| v.as_text())
//!             .unwrap_or("");
//!
//!         if password != confirm {
//!             return Err(Error::custom("password_mismatch", "Passwords do not match").into());
//!         }
//!         Ok(())
//!     }
//! }
//! ```

use crate::core::Value;
use super::context::ValidationContext;
use super::result::ValidationResult;

/// Trait for custom validation logic.
///
/// Implement this trait for validators that require:
/// - Complex business logic
/// - Cross-field validation via [`ValidationContext`]
/// - State or configuration
///
/// # Thread Safety
///
/// Validators must be `Send + Sync` to support concurrent validation
/// and schema sharing across threads.
///
/// # Example: Range Validator
///
/// ```ignore
/// use paramdef::validation::{Validator, ValidationContext, ValidationResult};
/// use paramdef::core::Value;
///
/// struct Range {
///     min: f64,
///     max: f64,
/// }
///
/// impl Validator for Range {
///     fn validate(&self, value: &Value, _ctx: &ValidationContext<'_>) -> ValidationResult {
///         let num = match value {
///             Value::Int(n) => *n as f64,
///             Value::Float(n) => *n,
///             _ => return Ok(()),
///         };
///
///         if num < self.min {
///             return Err(Error::min_value(self.min, num).into());
///         }
///         if num > self.max {
///             return Err(Error::max_value(self.max, num).into());
///         }
///         Ok(())
///     }
/// }
/// ```
pub trait Validator: Send + Sync + std::fmt::Debug {
    /// Validates a value within the given context.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to validate
    /// * `ctx` - Context providing access to sibling values and schema
    ///
    /// # Errors
    ///
    /// Returns `Err(ValidationOutcome)` if validation fails with error details.
    fn validate(&self, value: &Value, ctx: &ValidationContext<'_>) -> ValidationResult;

    /// Returns a human-readable name for this validator.
    ///
    /// Used for debugging and error messages.
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

/// A validator implemented as a function.
///
/// Enables creating validators from closures without defining a struct.
///
/// # Example
///
/// ```ignore
/// use paramdef::validation::{FnValidator, Rule};
///
/// let validator = FnValidator::new("is_even", |value, _ctx| {
///     if let Value::Int(n) = value {
///         if n % 2 != 0 {
///             return Err(Error::custom("even", "Value must be even").into());
///         }
///     }
///     Ok(())
/// });
///
/// let rule = Rule::Fn(Arc::new(validator));
/// ```
pub struct FnValidator<F>
where
    F: Fn(&Value, &ValidationContext<'_>) -> ValidationResult + Send + Sync,
{
    name: &'static str,
    func: F,
}

impl<F> FnValidator<F>
where
    F: Fn(&Value, &ValidationContext<'_>) -> ValidationResult + Send + Sync,
{
    /// Creates a new function-based validator.
    pub const fn new(name: &'static str, func: F) -> Self {
        Self { name, func }
    }
}

impl<F> std::fmt::Debug for FnValidator<F>
where
    F: Fn(&Value, &ValidationContext<'_>) -> ValidationResult + Send + Sync,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FnValidator")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

impl<F> Validator for FnValidator<F>
where
    F: Fn(&Value, &ValidationContext<'_>) -> ValidationResult + Send + Sync,
{
    fn validate(&self, value: &Value, ctx: &ValidationContext<'_>) -> ValidationResult {
        (self.func)(value, ctx)
    }

    fn name(&self) -> &str {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::Schema;
    use crate::types::leaf::Text;
    use crate::validation::context::NoValues;
    use crate::validation::result::Error;
    use std::sync::Arc;

    #[derive(Debug)]
    struct AlwaysValid;

    impl Validator for AlwaysValid {
        fn validate(&self, _value: &Value, _ctx: &ValidationContext<'_>) -> ValidationResult {
            Ok(())
        }
    }

    #[derive(Debug)]
    struct AlwaysInvalid;

    impl Validator for AlwaysInvalid {
        fn validate(&self, _value: &Value, _ctx: &ValidationContext<'_>) -> ValidationResult {
            Err(Error::custom("always_invalid", "Always fails").into())
        }
    }

    fn create_test_context<'a>(
        key: &'a crate::core::Key,
        schema: &'a Arc<Schema>,
        values: &'a NoValues,
    ) -> ValidationContext<'a> {
        ValidationContext::new(key, schema, values)
    }

    #[test]
    fn test_always_valid() {
        let schema = Arc::new(Schema::builder().parameter(Text::builder("test").build()).build());
        let key = "test".into();
        let values = NoValues;
        let ctx = create_test_context(&key, &schema, &values);

        let validator = AlwaysValid;
        assert!(validator.validate(&Value::Null, &ctx).is_ok());
    }

    #[test]
    fn test_always_invalid() {
        let schema = Arc::new(Schema::builder().parameter(Text::builder("test").build()).build());
        let key = "test".into();
        let values = NoValues;
        let ctx = create_test_context(&key, &schema, &values);

        let validator = AlwaysInvalid;
        assert!(validator.validate(&Value::Null, &ctx).is_err());
    }

    #[test]
    fn test_fn_validator() {
        let schema = Arc::new(Schema::builder().parameter(Text::builder("test").build()).build());
        let key = "test".into();
        let values = NoValues;
        let ctx = create_test_context(&key, &schema, &values);

        let validator = FnValidator::new("is_positive", |value, _ctx| {
            if let Value::Int(n) = value {
                if *n <= 0 {
                    return Err(Error::custom("positive", "Must be positive").into());
                }
            }
            Ok(())
        });

        assert!(validator.validate(&Value::Int(1), &ctx).is_ok());
        assert!(validator.validate(&Value::Int(0), &ctx).is_err());
        assert!(validator.validate(&Value::Int(-1), &ctx).is_err());
    }

    #[test]
    fn test_validator_name() {
        let validator = AlwaysValid;
        assert!(validator.name().contains("AlwaysValid"));

        let fn_validator = FnValidator::new("custom_name", |_, _| Ok(()));
        assert_eq!(fn_validator.name(), "custom_name");
    }

    #[test]
    fn test_validator_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<AlwaysValid>();
        assert_send_sync::<FnValidator<fn(&Value, &ValidationContext<'_>) -> ValidationResult>>();
    }
}
