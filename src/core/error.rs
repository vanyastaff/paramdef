//! Error types for parameter operations.
//!
//! This module provides error types using [`thiserror`] for ergonomic error handling.

use thiserror::Error;

/// Result type alias using the paramdef [`enum@Error`] type.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during parameter operations.
#[derive(Debug, Error)]
pub enum Error {
    /// Type mismatch when accessing a value.
    #[error("type mismatch: expected {expected}, got {actual}")]
    TypeMismatch {
        /// Expected type name.
        expected: &'static str,
        /// Actual type name.
        actual: &'static str,
    },

    /// Validation failed for a parameter value.
    #[error("validation failed: {message}")]
    Validation {
        /// Error code for programmatic handling.
        code: String,
        /// Human-readable error message.
        message: String,
        /// Fields involved in the validation error.
        fields: Vec<String>,
    },

    /// Required value is missing.
    #[error("required field '{field}' is missing")]
    MissingRequired {
        /// Name of the missing field.
        field: String,
    },

    /// Value is out of allowed range.
    #[error("value {value} is out of range [{min}, {max}]")]
    OutOfRange {
        /// The actual value.
        value: f64,
        /// Minimum allowed value.
        min: f64,
        /// Maximum allowed value.
        max: f64,
    },

    /// String length is out of bounds.
    #[error("length {length} is out of bounds [{min}, {max}]")]
    LengthOutOfBounds {
        /// Actual length.
        length: usize,
        /// Minimum allowed length.
        min: usize,
        /// Maximum allowed length.
        max: usize,
    },

    /// Pattern match failed.
    #[error("value does not match pattern: {pattern}")]
    PatternMismatch {
        /// The pattern that wasn't matched.
        pattern: String,
    },

    /// Value not in allowed set.
    #[error("value '{value}' is not in allowed values")]
    NotInAllowedValues {
        /// The invalid value.
        value: String,
    },

    /// Parameter not found.
    #[error("parameter '{key}' not found")]
    NotFound {
        /// The key that wasn't found.
        key: String,
    },

    /// Generic error with custom message.
    #[error("{0}")]
    Custom(String),
}

impl Error {
    /// Creates a type mismatch error.
    #[must_use]
    pub const fn type_mismatch(expected: &'static str, actual: &'static str) -> Self {
        Self::TypeMismatch { expected, actual }
    }

    /// Creates a validation error.
    #[must_use]
    pub fn validation(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Validation {
            code: code.into(),
            message: message.into(),
            fields: Vec::new(),
        }
    }

    /// Creates a validation error with fields.
    #[must_use]
    pub fn validation_with_fields(
        code: impl Into<String>,
        message: impl Into<String>,
        fields: Vec<String>,
    ) -> Self {
        Self::Validation {
            code: code.into(),
            message: message.into(),
            fields,
        }
    }

    /// Creates a missing required field error.
    #[must_use]
    pub fn missing_required(field: impl Into<String>) -> Self {
        Self::MissingRequired {
            field: field.into(),
        }
    }

    /// Creates an out of range error.
    #[must_use]
    pub const fn out_of_range(value: f64, min: f64, max: f64) -> Self {
        Self::OutOfRange { value, min, max }
    }

    /// Creates a length out of bounds error.
    #[must_use]
    pub const fn length_out_of_bounds(length: usize, min: usize, max: usize) -> Self {
        Self::LengthOutOfBounds { length, min, max }
    }

    /// Creates a pattern mismatch error.
    #[must_use]
    pub fn pattern_mismatch(pattern: impl Into<String>) -> Self {
        Self::PatternMismatch {
            pattern: pattern.into(),
        }
    }

    /// Creates a not in allowed values error.
    #[must_use]
    pub fn not_in_allowed_values(value: impl Into<String>) -> Self {
        Self::NotInAllowedValues {
            value: value.into(),
        }
    }

    /// Creates a not found error.
    #[must_use]
    pub fn not_found(key: impl Into<String>) -> Self {
        Self::NotFound { key: key.into() }
    }

    /// Creates a custom error.
    #[must_use]
    pub fn custom(message: impl Into<String>) -> Self {
        Self::Custom(message.into())
    }

    /// Returns the error code if this is a validation error.
    #[must_use]
    pub fn code(&self) -> Option<&str> {
        match self {
            Self::Validation { code, .. } => Some(code),
            _ => None,
        }
    }

    /// Returns the fields involved if this is a validation error.
    #[must_use]
    pub fn fields(&self) -> &[String] {
        match self {
            Self::Validation { fields, .. } => fields,
            _ => &[],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_mismatch_error() {
        let err = Error::type_mismatch("int", "text");
        assert!(matches!(err, Error::TypeMismatch { .. }));

        let msg = err.to_string();
        assert!(msg.contains("int"));
        assert!(msg.contains("text"));
    }

    #[test]
    fn test_validation_error() {
        let err = Error::validation("required", "This field is required");
        assert!(matches!(err, Error::Validation { .. }));
        assert_eq!(err.code(), Some("required"));
    }

    #[test]
    fn test_validation_error_with_fields() {
        let err = Error::validation_with_fields(
            "mismatch",
            "Passwords don't match",
            vec!["password".into(), "confirm".into()],
        );

        assert_eq!(err.fields(), &["password", "confirm"]);
    }

    #[test]
    fn test_missing_required_error() {
        let err = Error::missing_required("username");
        let msg = err.to_string();
        assert!(msg.contains("username"));
        assert!(msg.contains("required"));
    }

    #[test]
    fn test_out_of_range_error() {
        let err = Error::out_of_range(150.0, 0.0, 100.0);
        let msg = err.to_string();
        assert!(msg.contains("150"));
        assert!(msg.contains("0"));
        assert!(msg.contains("100"));
    }

    #[test]
    fn test_length_out_of_bounds_error() {
        let err = Error::length_out_of_bounds(2, 3, 100);
        let msg = err.to_string();
        assert!(msg.contains("2"));
        assert!(msg.contains("3"));
        assert!(msg.contains("100"));
    }

    #[test]
    fn test_pattern_mismatch_error() {
        let err = Error::pattern_mismatch(r"^\d+$");
        let msg = err.to_string();
        assert!(msg.contains("pattern"));
    }

    #[test]
    fn test_not_in_allowed_values_error() {
        let err = Error::not_in_allowed_values("invalid");
        let msg = err.to_string();
        assert!(msg.contains("invalid"));
        assert!(msg.contains("allowed"));
    }

    #[test]
    fn test_not_found_error() {
        let err = Error::not_found("missing_key");
        let msg = err.to_string();
        assert!(msg.contains("missing_key"));
        assert!(msg.contains("not found"));
    }

    #[test]
    fn test_custom_error() {
        let err = Error::custom("Something went wrong");
        assert_eq!(err.to_string(), "Something went wrong");
    }

    #[test]
    fn test_error_display() {
        // All errors should implement Display via thiserror
        let errors = [
            Error::type_mismatch("int", "text"),
            Error::validation("code", "message"),
            Error::missing_required("field"),
            Error::out_of_range(5.0, 0.0, 3.0),
            Error::length_out_of_bounds(10, 0, 5),
            Error::pattern_mismatch("pattern"),
            Error::not_in_allowed_values("value"),
            Error::not_found("key"),
            Error::custom("custom"),
        ];

        for err in errors {
            // Should not panic
            let _ = err.to_string();
        }
    }

    #[test]
    fn test_error_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Error>();
    }
}
