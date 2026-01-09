//! Validation result types.

use std::sync::Arc;

use crate::core::SmartStr;

/// Result of a validation operation.
pub type ValidationResult = Result<(), ValidationOutcome>;

/// Validation failure with one or more errors.
#[derive(Debug, Clone)]
pub struct ValidationOutcome {
    /// List of validation errors.
    errors: Arc<[Error]>,
}

impl ValidationOutcome {
    /// Creates a new validation outcome with a single error.
    #[must_use]
    pub fn single(error: Error) -> Self {
        Self {
            errors: Arc::from([error]),
        }
    }

    /// Creates a new validation outcome with multiple errors.
    #[must_use]
    pub fn multiple(errors: impl IntoIterator<Item = Error>) -> Self {
        Self {
            errors: errors.into_iter().collect(),
        }
    }

    /// Returns the validation errors.
    #[must_use]
    pub fn errors(&self) -> &[Error] {
        &self.errors
    }

    /// Returns `true` if there are no errors.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Returns the number of errors.
    #[must_use]
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Converts to event system's `ValidationError` type.
    #[cfg(feature = "events")]
    #[must_use]
    pub fn to_event_errors(&self) -> Arc<[crate::event::ValidationError]> {
        self.errors
            .iter()
            .map(|e| crate::event::ValidationError::new(e.code.clone(), e.message.clone()))
            .collect()
    }
}

impl From<Error> for ValidationOutcome {
    fn from(error: Error) -> Self {
        Self::single(error)
    }
}

impl From<Vec<Error>> for ValidationOutcome {
    fn from(errors: Vec<Error>) -> Self {
        Self::multiple(errors)
    }
}

/// A single validation error.
///
/// Contains a machine-readable code and human-readable message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    /// Machine-readable error code (e.g., `required`, `min_length`).
    pub code: SmartStr,
    /// Human-readable error message.
    pub message: SmartStr,
}

impl Error {
    /// Creates a new validation error.
    #[must_use]
    pub fn new(code: impl Into<SmartStr>, message: impl Into<SmartStr>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }

    // === Built-in error constructors ===

    /// Creates a "required" error.
    #[must_use]
    pub fn required() -> Self {
        Self::new("required", "This field is required")
    }

    /// Creates a "type" error for type mismatch.
    #[must_use]
    pub fn type_mismatch(expected: &str, got: &str) -> Self {
        Self::new("type", format!("Expected {expected}, got {got}"))
    }

    /// Creates a `min_length` error.
    #[must_use]
    pub fn min_length(min: usize, actual: usize) -> Self {
        Self::new(
            "min_length",
            format!("Minimum length is {min}, got {actual}"),
        )
    }

    /// Creates a `max_length` error.
    #[must_use]
    pub fn max_length(max: usize, actual: usize) -> Self {
        Self::new(
            "max_length",
            format!("Maximum length is {max}, got {actual}"),
        )
    }

    /// Creates a "min" error for numeric values.
    #[must_use]
    pub fn min_value(min: f64, actual: f64) -> Self {
        Self::new("min", format!("Minimum value is {min}, got {actual}"))
    }

    /// Creates a "max" error for numeric values.
    #[must_use]
    pub fn max_value(max: f64, actual: f64) -> Self {
        Self::new("max", format!("Maximum value is {max}, got {actual}"))
    }

    /// Creates an `exclusive_min` error.
    #[must_use]
    pub fn exclusive_min(min: f64, actual: f64) -> Self {
        Self::new(
            "exclusive_min",
            format!("Value must be greater than {min}, got {actual}"),
        )
    }

    /// Creates an `exclusive_max` error.
    #[must_use]
    pub fn exclusive_max(max: f64, actual: f64) -> Self {
        Self::new(
            "exclusive_max",
            format!("Value must be less than {max}, got {actual}"),
        )
    }

    /// Creates a "pattern" error.
    #[must_use]
    pub fn pattern(pattern: &str) -> Self {
        Self::new(
            "pattern",
            format!("Value does not match pattern: {pattern}"),
        )
    }

    /// Creates an "email" error.
    #[must_use]
    pub fn email() -> Self {
        Self::new("email", "Invalid email address")
    }

    /// Creates a "url" error.
    #[must_use]
    pub fn url() -> Self {
        Self::new("url", "Invalid URL")
    }

    /// Creates a `min_items` error for arrays.
    #[must_use]
    pub fn min_items(min: usize, actual: usize) -> Self {
        Self::new("min_items", format!("Minimum items is {min}, got {actual}"))
    }

    /// Creates a `max_items` error for arrays.
    #[must_use]
    pub fn max_items(max: usize, actual: usize) -> Self {
        Self::new("max_items", format!("Maximum items is {max}, got {actual}"))
    }

    /// Creates a `unique_items` error.
    #[must_use]
    pub fn unique_items() -> Self {
        Self::new("unique_items", "Array items must be unique")
    }

    /// Creates an "enum" error for value not in allowed set.
    #[must_use]
    pub fn not_in_enum(allowed: &[&str]) -> Self {
        Self::new(
            "enum",
            format!("Value must be one of: {}", allowed.join(", ")),
        )
    }

    /// Creates a "const" error for value not matching constant.
    #[must_use]
    pub fn not_const(expected: &str) -> Self {
        Self::new("const", format!("Value must be exactly: {expected}"))
    }

    /// Creates a `multiple_of` error.
    #[must_use]
    pub fn multiple_of(divisor: f64, actual: f64) -> Self {
        Self::new(
            "multiple_of",
            format!("Value must be a multiple of {divisor}, got {actual}"),
        )
    }

    /// Creates a custom error.
    #[must_use]
    pub fn custom(code: impl Into<SmartStr>, message: impl Into<SmartStr>) -> Self {
        Self::new(code, message)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_constructors() {
        let err = Error::required();
        assert_eq!(err.code.as_str(), "required");

        let err = Error::min_length(5, 3);
        assert_eq!(err.code.as_str(), "min_length");
        assert!(err.message.contains("5"));
        assert!(err.message.contains("3"));
    }

    #[test]
    fn test_validation_outcome_single() {
        let outcome = ValidationOutcome::single(Error::required());
        assert_eq!(outcome.len(), 1);
        assert!(!outcome.is_empty());
    }

    #[test]
    fn test_validation_outcome_multiple() {
        let outcome = ValidationOutcome::multiple([Error::required(), Error::min_length(5, 3)]);
        assert_eq!(outcome.len(), 2);
    }

    #[test]
    fn test_validation_result() {
        let ok: ValidationResult = Ok(());
        assert!(ok.is_ok());

        let err: ValidationResult = Err(Error::required().into());
        assert!(err.is_err());
    }

    #[test]
    fn test_error_display() {
        let err = Error::required();
        assert_eq!(err.to_string(), "[required] This field is required");
    }
}
