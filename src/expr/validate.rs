//! Validation logic for expressions with detailed error reporting.

#[cfg(feature = "validation")]
use super::Expr;
#[cfg(feature = "validation")]
use crate::core::Value;
#[cfg(feature = "validation")]
use crate::validation::{Error, ValidationContext, ValidationResult};

#[cfg(feature = "validation")]
impl Expr {
    /// Validate a value with detailed error reporting.
    ///
    /// This method provides validation with detailed error messages.
    /// For cross-field validation, use `validate_with_context`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use paramdef::expr::Expr;
    /// use paramdef::core::Value;
    ///
    /// let expr = Expr::MinLength(5);
    /// let result = expr.validate(&Value::text("hi"));
    /// assert!(result.is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `ValidationOutcome` error when the expression evaluation fails.
    pub fn validate(&self, value: &Value) -> ValidationResult {
        // Use simple eval and convert to ValidationResult
        if self.eval(value) {
            Ok(())
        } else {
            Err(self.error_for_failed_validation(value))
        }
    }

    /// Validate a value with cross-field context.
    ///
    /// This method provides full validation with:
    /// - Detailed error messages
    /// - Cross-field validation support via context
    /// - Field-specific error reporting
    ///
    /// # Example
    ///
    /// ```ignore
    /// use paramdef::expr::Expr;
    /// use paramdef::core::Value;
    /// use paramdef::validation::ValidationContext;
    ///
    /// let expr = Expr::MinLength(5);
    /// // ctx would be a real ValidationContext in practice
    /// let result = expr.validate_with_context(&Value::text("hi"), &ctx);
    /// assert!(result.is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `ValidationOutcome` error when the expression evaluation fails.
    pub fn validate_with_context(
        &self,
        value: &Value,
        _ctx: &ValidationContext<'_>,
    ) -> ValidationResult {
        // For now, just delegate to validate()
        // TODO: Add cross-field validation support
        self.validate(value)
    }

    /// Generate an appropriate error for a failed validation.
    fn error_for_failed_validation(&self, value: &Value) -> crate::validation::ValidationOutcome {
        use crate::validation::ValidationOutcome;

        let error = match self {
            Self::Required => Error::required(),
            Self::MinLength(min) => {
                let actual = match value {
                    Value::Text(s) => s.len(),
                    Value::Array(a) => a.len(),
                    _ => 0,
                };
                Error::min_length(*min, actual)
            }
            Self::MaxLength(max) => {
                let actual = match value {
                    Value::Text(s) => s.len(),
                    Value::Array(a) => a.len(),
                    _ => 0,
                };
                Error::max_length(*max, actual)
            }
            Self::Min(min) => {
                let actual = value.as_f64().unwrap_or(0.0);
                Error::min_value(*min, actual)
            }
            Self::Max(max) => {
                let actual = value.as_f64().unwrap_or(0.0);
                Error::max_value(*max, actual)
            }
            Self::Email => Error::email(),
            Self::Url => Error::url(),
            Self::Uuid => Error::custom("invalid_uuid", "Invalid UUID format"),
            _ => Error::custom("validation_failed", "Validation failed"),
        };

        ValidationOutcome::single(error)
    }
}
