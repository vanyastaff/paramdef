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
    /// use paramdef::validation::NoValues;
    ///
    /// let expr = Expr::MinLength(5);
    /// let ctx = NoValues;
    /// let result = expr.validate(&Value::text("hi"), &ctx);
    /// assert!(result.is_err());
    /// ```
    pub fn validate(&self, value: &Value, _ctx: &ValidationContext<'_>) -> ValidationResult {
        // For now, use simple eval and convert to ValidationResult
        // TODO: Add detailed error messages for each variant
        if self.eval(value) {
            Ok(())
        } else {
            Err(self.error_for_failed_validation(value))
        }
    }

    /// Generate an appropriate error for a failed validation.
    fn error_for_failed_validation(&self, _value: &Value) -> crate::validation::ValidationOutcome {
        use crate::validation::ValidationOutcome;

        let error = match self {
            Self::Required => Error::required(),
            Self::MinLength(min) => {
                let actual = match _value {
                    Value::Text(s) => s.len(),
                    Value::Array(a) => a.len(),
                    _ => 0,
                };
                Error::min_length(*min, actual)
            }
            Self::MaxLength(max) => {
                let actual = match _value {
                    Value::Text(s) => s.len(),
                    Value::Array(a) => a.len(),
                    _ => 0,
                };
                Error::max_length(*max, actual)
            }
            Self::Min(min) => {
                let actual = _value.as_f64().unwrap_or(0.0);
                Error::min_value(*min, actual)
            }
            Self::Max(max) => {
                let actual = _value.as_f64().unwrap_or(0.0);
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
