# Contract: Error API Enhancements

**Feature**: 002-code-quality-improvements  
**Module**: `src/core/error.rs`  
**Status**: Phase 1 Design

---

## Overview

This contract defines enhancements to the Error type to provide actionable hints and better error context, improving developer experience without breaking existing code.

**Design Principles**:
- **Backward Compatible**: Existing error construction continues to work
- **Opt-In Hints**: Hints are generated on-demand, zero cost if not used
- **Serialization Safe**: Hints skipped in serde (not meaningful across sessions)
- **Actionable**: Every hint provides specific guidance for fixing the error

---

## 1. Enhanced Error Structure

**Current Implementation** (v0.3.x):
```rust
#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("type mismatch for key '{key}': expected {expected}, got {actual}")]
    TypeMismatch {
        key: String,
        expected: ValueKind,
        actual: ValueKind,
    },
    
    #[error("validation failed: {message}")]
    Validation {
        code: String,
        message: String,
        fields: Vec<String>,
    },
    
    // ... other variants
}
```

**Enhanced Implementation** (v0.4.0):
```rust
#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("type mismatch for key '{key}': expected {expected}, got {actual}")]
    TypeMismatch {
        key: String,
        expected: ValueKind,
        actual: ValueKind,
        /// Optional actionable hint for fixing this error.
        /// Not serialized (for serde compatibility).
        #[cfg_attr(feature = "serde", serde(skip))]
        hint: Option<String>,
    },
    
    #[error("validation failed: {message}")]
    Validation {
        code: String,
        message: String,
        fields: Vec<String>,
        #[cfg_attr(feature = "serde", serde(skip))]
        hint: Option<String>,
    },
    
    #[error("key '{key}' not found")]
    NotFound {
        key: String,
        #[cfg_attr(feature = "serde", serde(skip))]
        hint: Option<String>,
    },
    
    #[error("required field '{field}' is missing")]
    MissingRequired {
        field: String,
        #[cfg_attr(feature = "serde", serde(skip))]
        hint: Option<String>,
    },
    
    #[error("value {value} is out of range [{min}, {max}]")]
    OutOfRange {
        value: f64,
        min: f64,
        max: f64,
        #[cfg_attr(feature = "serde", serde(skip))]
        hint: Option<String>,
    },
    
    // ... other variants with hint field added
}
```

**Contract**:
- **New Field**: `hint: Option<String>` added to all error variants
- **Serde Skip**: Marked with `#[serde(skip)]` when serde feature enabled
- **Default Value**: `None` for backward compatibility
- **Display**: Unchanged (hints accessed via separate method)

---

## 2. Hint Generation Method

**Signature**:
```rust
impl Error {
    /// Returns an actionable hint for resolving this error, if available.
    ///
    /// Hints provide specific suggestions for fixing common errors:
    /// - **Type mismatches**: Suggests correct getter method
    /// - **Not found**: Lists available keys, suggests alternatives
    /// - **Validation**: Provides format examples or constraint info
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::core::{Error, ValueKind};
    ///
    /// let err = Error::type_mismatch("age", ValueKind::Int, ValueKind::Text);
    /// assert_eq!(
    ///     err.hint(),
    ///     Some("Use `get_int()` for integer values, or convert with `parse()`")
    /// );
    /// ```
    #[must_use]
    pub fn hint(&self) -> Option<&str> {
        match self {
            Self::TypeMismatch { hint, expected, actual, .. } => {
                hint.as_deref().or_else(|| {
                    Some(match (expected, actual) {
                        (ValueKind::Int, ValueKind::Text) => 
                            "Use `get_int()` for integer values, or convert with `parse()`",
                        (ValueKind::Int, ValueKind::Float) => 
                            "Use `get_int()` for integers, or `get_float()` if decimals are acceptable",
                        (ValueKind::Float, ValueKind::Int) => 
                            "Use `get_float()` for numeric values, integers will be converted automatically",
                        (ValueKind::Float, ValueKind::Text) => 
                            "Use `get_float()` for numeric values, or convert with `parse()`",
                        (ValueKind::Text, ValueKind::Int) => 
                            "Use `get_text()` for text values, or convert with `to_string()`",
                        (ValueKind::Text, ValueKind::Bool) => 
                            "Use `get_text()` for text values, or convert boolean with `to_string()`",
                        (ValueKind::Bool, ValueKind::Text) => 
                            "Use `get_bool()` for boolean values, or parse text ('true'/'false')",
                        (ValueKind::Bool, ValueKind::Int) => 
                            "Use `get_bool()` for boolean values. Note: 0=false, 1=true conversion not automatic",
                        (ValueKind::Array, _) => 
                            "Use `get_array()` to access array elements, or `get()` for the raw Value",
                        (ValueKind::Object, _) => 
                            "Use `get_object()` to access object fields, or navigate with dot notation",
                        (ValueKind::Null, _) => 
                            "Value is null. Check if the field was set, or use `get_*_or()` for defaults",
                        _ => 
                            "Use the appropriate getter method for this value type (see Context::get_* methods)",
                    })
                })
            }
            
            Self::Validation { hint, code, .. } => {
                hint.as_deref().or_else(|| {
                    Some(match code.as_str() {
                        "required" => 
                            "This field cannot be empty. Provide a value or remove the REQUIRED flag",
                        "email" => 
                            "Provide a valid email address (e.g., user@example.com)",
                        "url" => 
                            "Provide a valid URL (e.g., https://example.com)",
                        "min_length" => 
                            "The value is too short. Check the minimum length constraint",
                        "max_length" => 
                            "The value is too long. Check the maximum length constraint",
                        "min_value" => 
                            "The value is too small. Check the minimum value constraint",
                        "max_value" => 
                            "The value is too large. Check the maximum value constraint",
                        "out_of_range" => 
                            "The value is outside allowed bounds. Check min/max constraints",
                        "pattern" => 
                            "The value doesn't match the required pattern. Check the regex constraint",
                        "integer" => 
                            "The value must be a whole number without decimal places",
                        "positive" => 
                            "The value must be greater than zero",
                        "negative" => 
                            "The value must be less than zero",
                        "unique" => 
                            "This value already exists. Provide a unique value",
                        "custom" => 
                            "Custom validation failed. Check the validator's requirements",
                        _ => 
                            "Check the validation rules for this field. Use `ctx.schema().get(key)` to inspect constraints",
                    })
                })
            }
            
            Self::NotFound { hint, .. } => {
                hint.as_deref().or_else(|| {
                    Some("Key not found. Use `ctx.keys()` to list available keys, or check for typos")
                })
            }
            
            Self::MissingRequired { hint, .. } => {
                hint.as_deref().or_else(|| {
                    Some("This field is marked as REQUIRED. Set a value with `ctx.set(key, value)`")
                })
            }
            
            Self::OutOfRange { hint, min, max, .. } => {
                hint.as_deref().or_else(|| {
                    Some(&format!(
                        "Value must be between {} and {}. Adjust the value or change the constraints",
                        min, max
                    ))
                })
            }
            
            _ => None,
        }
    }
}
```

**Contract**:
- **Return Type**: `Option<&str>` - Some if hint available, None otherwise
- **Custom Priority**: Custom hints (in error field) take precedence over default hints
- **Lazy Generation**: Default hints generated on-demand (zero cost if not called)
- **Immutable**: Method is `&self`, doesn't modify error
- **Thread Safe**: Can be called from any thread (Error is Clone + Send + Sync)

---

## 3. Formatted Display with Hints

**Signature**:
```rust
impl Error {
    /// Formats the error with its hint included.
    ///
    /// This is useful for logging and user-facing error messages.
    ///
    /// # Format
    ///
    /// ```text
    /// <error message>
    /// Hint: <actionable suggestion>
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::core::{Error, ValueKind};
    ///
    /// let err = Error::type_mismatch("age", ValueKind::Int, ValueKind::Text);
    /// eprintln!("{}", err.with_hint());
    /// // Output:
    /// // type mismatch for key 'age': expected Int, got Text
    /// // Hint: Use `get_int()` for integer values, or convert with `parse()`
    /// ```
    #[must_use]
    pub fn with_hint(&self) -> String {
        match self.hint() {
            Some(hint) => format!("{}\nHint: {}", self, hint),
            None => self.to_string(),
        }
    }
}
```

**Contract**:
- **Input**: `&self` (immutable borrow)
- **Output**: `String` (owned, heap-allocated)
- **Format**: Multi-line with "Hint:" prefix
- **Fallback**: If no hint, returns normal error message
- **Performance**: Allocates on every call (use sparingly, e.g., error handling paths)

---

## 4. Custom Hint Injection

**Signature**:
```rust
impl Error {
    /// Adds a custom hint to this error, replacing any default hint.
    ///
    /// This is useful when you have context-specific guidance that the
    /// library can't infer.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::core::{Error, ValueKind};
    ///
    /// let err = Error::type_mismatch("age", ValueKind::Int, ValueKind::Text)
    ///     .with_custom_hint("Age should be a number from the database 'users.age' column");
    ///
    /// assert_eq!(
    ///     err.hint(),
    ///     Some("Age should be a number from the database 'users.age' column")
    /// );
    /// ```
    #[must_use]
    pub fn with_custom_hint(mut self, hint: impl Into<String>) -> Self {
        match &mut self {
            Self::TypeMismatch { hint: h, .. } |
            Self::Validation { hint: h, .. } |
            Self::NotFound { hint: h, .. } |
            Self::MissingRequired { hint: h, .. } |
            Self::OutOfRange { hint: h, .. } => {
                *h = Some(hint.into());
            }
            // Variants without hint field are ignored
            _ => {}
        }
        self
    }
}
```

**Contract**:
- **Builder Pattern**: Returns `Self` for chaining
- **Replaces Default**: Custom hint overrides default hint logic
- **Selective**: Only works on variants with hint field
- **Immutability**: Consumes `self`, returns modified error

---

## 5. Enhanced Constructors

**Existing Constructors** (unchanged for backward compatibility):
```rust
impl Error {
    // These remain unchanged
    pub fn type_mismatch(key: impl Into<String>, expected: ValueKind, actual: ValueKind) -> Self;
    pub fn not_found(key: impl Into<String>) -> Self;
    // ... etc
}
```

**New Constructors with Context** (v0.4.0):
```rust
impl Error {
    /// Creates a NotFound error with available keys listed in hint.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::core::Error;
    ///
    /// let available = vec!["username", "password", "email"];
    /// let err = Error::not_found_with_suggestions("user_name", &available);
    ///
    /// assert_eq!(
    ///     err.hint(),
    ///     Some("Available keys: username, password, email. Did you mean 'username'?")
    /// );
    /// ```
    pub fn not_found_with_suggestions(
        key: impl Into<String>,
        available: &[impl AsRef<str>],
    ) -> Self {
        let key_str = key.into();
        
        // Simple fuzzy matching (Levenshtein distance could be added later)
        let suggestion = available
            .iter()
            .map(|s| s.as_ref())
            .find(|s| {
                // Simple heuristic: starts with same 2 chars
                s.len() >= 2 && key_str.len() >= 2 && 
                s[..2].eq_ignore_ascii_case(&key_str[..2])
            });
        
        let hint = if available.is_empty() {
            "No keys available in this context".to_string()
        } else {
            let keys_list = available
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>()
                .join(", ");
            
            match suggestion {
                Some(s) => format!("Available keys: {}. Did you mean '{}'?", keys_list, s),
                None => format!("Available keys: {}", keys_list),
            }
        };
        
        Self::NotFound {
            key: key_str,
            hint: Some(hint),
        }
    }

    /// Creates a Validation error with path information.
    ///
    /// Used internally by Context when validation fails on nested fields.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::core::Error;
    ///
    /// let err = Error::validation_with_path(
    ///     "required",
    ///     "Field is required",
    ///     vec!["user.address.email"],
    /// );
    /// ```
    pub fn validation_with_path(
        code: impl Into<String>,
        message: impl Into<String>,
        fields: Vec<String>,
    ) -> Self {
        Self::Validation {
            code: code.into(),
            message: message.into(),
            fields,
            hint: None, // Default hint generated by hint() method
        }
    }
}
```

---

## 6. Error Categorization Helpers

**New Methods** (v0.4.0):
```rust
impl Error {
    /// Returns true if this is a user input error (validation, type mismatch).
    ///
    /// Useful for determining whether to show the error to end users
    /// vs log as internal error.
    #[must_use]
    pub fn is_user_error(&self) -> bool {
        matches!(
            self,
            Self::TypeMismatch { .. } |
            Self::Validation { .. } |
            Self::MissingRequired { .. } |
            Self::OutOfRange { .. } |
            Self::LengthOutOfBounds { .. }
        )
    }

    /// Returns true if this is a developer error (not found, schema issue).
    #[must_use]
    pub fn is_developer_error(&self) -> bool {
        matches!(
            self,
            Self::NotFound { .. } |
            Self::InvalidKey { .. } |
            Self::InvalidOperation { .. }
        )
    }

    /// Returns the error code for programmatic handling.
    ///
    /// Returns a static string identifier for the error type.
    #[must_use]
    pub fn code(&self) -> &'static str {
        match self {
            Self::TypeMismatch { .. } => "type_mismatch",
            Self::Validation { .. } => "validation",
            Self::NotFound { .. } => "not_found",
            Self::MissingRequired { .. } => "missing_required",
            Self::OutOfRange { .. } => "out_of_range",
            Self::LengthOutOfBounds { .. } => "length_out_of_bounds",
            Self::NullValue { .. } => "null_value",
            Self::InvalidKey { .. } => "invalid_key",
            Self::InvalidOperation { .. } => "invalid_operation",
            Self::SerializationError { .. } => "serialization",
            Self::DeserializationError { .. } => "deserialization",
            _ => "unknown",
        }
    }

    /// Returns the severity level of this error.
    #[must_use]
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            Self::Validation { .. } | Self::MissingRequired { .. } => ErrorSeverity::Warning,
            Self::TypeMismatch { .. } | Self::OutOfRange { .. } => ErrorSeverity::Error,
            Self::NotFound { .. } | Self::InvalidKey { .. } => ErrorSeverity::Error,
            _ => ErrorSeverity::Error,
        }
    }
}

/// Error severity level for UI display.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    /// Validation warning (user can potentially proceed).
    Warning,
    /// Hard error (operation failed).
    Error,
    /// Critical error (system state compromised).
    Critical,
}
```

---

## 7. Example Usage

### Basic Error Handling with Hints
```rust
use paramdef::core::{Error, ValueKind};

fn process_value(ctx: &Context, key: &str) -> Result<i64> {
    match ctx.get_int(key) {
        Ok(value) => Ok(value),
        Err(e) => {
            // Log with hint for debugging
            eprintln!("Error processing {}: {}", key, e.with_hint());
            Err(e)
        }
    }
}
```

### User-Facing Error Display
```rust
use paramdef::core::Error;

fn show_error_to_user(error: &Error) {
    if error.is_user_error() {
        // Show to end user with hint
        show_notification(
            "Validation Error",
            &error.with_hint(),
            NotificationType::Warning
        );
    } else {
        // Log developer error
        log::error!("Internal error: {}", error.with_hint());
        show_notification(
            "System Error",
            "An unexpected error occurred. Please contact support.",
            NotificationType::Error
        );
    }
}
```

### Custom Context-Specific Hints
```rust
use paramdef::core::Error;

fn validate_age(ctx: &Context) -> Result<()> {
    match ctx.get_int("age") {
        Ok(age) if age < 0 => {
            Err(Error::out_of_range("age", age as f64, 0.0, 120.0)
                .with_custom_hint("Age cannot be negative. If date of birth was entered, convert to age first."))
        }
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
```

### Bulk Validation with Detailed Errors
```rust
use paramdef::core::Error;

fn validate_form(ctx: &Context) -> Result<()> {
    let mut errors = Vec::new();
    
    // Collect all validation errors
    for key in ["username", "email", "age"] {
        if let Err(e) = ctx.validate(key) {
            errors.push(e);
        }
    }
    
    if !errors.is_empty() {
        // Combine errors with hints
        let messages: Vec<_> = errors
            .iter()
            .map(|e| e.with_hint())
            .collect();
        
        eprintln!("Validation failed:\n{}", messages.join("\n\n"));
        
        // Return first error
        Err(errors.into_iter().next().unwrap())
    } else {
        Ok(())
    }
}
```

---

## 8. Serialization Behavior

**With serde Feature**:
```rust
use paramdef::core::{Error, ValueKind};

let err = Error::type_mismatch("age", ValueKind::Int, ValueKind::Text);

// Serialize to JSON
let json = serde_json::to_string(&err)?;

// Hint field is skipped
assert_eq!(
    json,
    r#"{"TypeMismatch":{"key":"age","expected":"Int","actual":"Text"}}"#
);

// Deserialize
let deserialized: Error = serde_json::from_str(&json)?;

// Hint is None after deserialization (regenerated on .hint() call)
assert!(matches!(deserialized, Error::TypeMismatch { hint: None, .. }));

// But default hint is still available
assert_eq!(
    deserialized.hint(),
    Some("Use `get_int()` for integer values, or convert with `parse()`")
);
```

**Rationale**:
- Hints are contextual and may not be valid across sessions
- Reduces JSON payload size
- Default hints are always available via `hint()` method

---

## Testing Requirements

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hint_type_mismatch() {
        let err = Error::type_mismatch("age", ValueKind::Int, ValueKind::Text);
        assert_eq!(
            err.hint(),
            Some("Use `get_int()` for integer values, or convert with `parse()`")
        );
    }

    #[test]
    fn test_hint_validation() {
        let err = Error::validation("email", "Invalid email", vec!["email".to_string()]);
        assert_eq!(
            err.hint(),
            Some("Provide a valid email address (e.g., user@example.com)")
        );
    }

    #[test]
    fn test_custom_hint() {
        let err = Error::type_mismatch("age", ValueKind::Int, ValueKind::Text)
            .with_custom_hint("Age should come from the database");
        
        assert_eq!(err.hint(), Some("Age should come from the database"));
    }

    #[test]
    fn test_with_hint_format() {
        let err = Error::type_mismatch("age", ValueKind::Int, ValueKind::Text);
        let formatted = err.with_hint();
        
        assert!(formatted.contains("type mismatch"));
        assert!(formatted.contains("Hint:"));
        assert!(formatted.contains("get_int()"));
    }

    #[test]
    fn test_not_found_suggestions() {
        let err = Error::not_found_with_suggestions(
            "user_name",
            &["username", "password", "email"]
        );
        
        let hint = err.hint().unwrap();
        assert!(hint.contains("username, password, email"));
        assert!(hint.contains("Did you mean 'username'?"));
    }

    #[test]
    fn test_error_categorization() {
        let validation_err = Error::validation("email", "Invalid", vec![]);
        assert!(validation_err.is_user_error());
        assert!(!validation_err.is_developer_error());
        
        let not_found_err = Error::not_found("missing_key");
        assert!(!not_found_err.is_user_error());
        assert!(not_found_err.is_developer_error());
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serde_skips_hint() {
        let err = Error::type_mismatch("age", ValueKind::Int, ValueKind::Text)
            .with_custom_hint("Custom hint");
        
        let json = serde_json::to_string(&err).unwrap();
        assert!(!json.contains("Custom hint"));
        
        let deserialized: Error = serde_json::from_str(&json).unwrap();
        // Custom hint is lost, but default hint is available
        assert!(deserialized.hint().is_some());
    }
}
```

---

## Migration Guide

### Existing Code (no changes needed)
```rust
// This continues to work exactly as before
let err = Error::type_mismatch("age", ValueKind::Int, ValueKind::Text);
eprintln!("Error: {}", err);
```

### New Code (with hints)
```rust
// Opt-in to hints for better UX
let err = Error::type_mismatch("age", ValueKind::Int, ValueKind::Text);
eprintln!("{}", err.with_hint());
// Output:
// type mismatch for key 'age': expected Int, got Text
// Hint: Use `get_int()` for integer values, or convert with `parse()`
```

### Custom Hints
```rust
// Add context-specific guidance
let err = Error::validation("email", "Invalid format", vec!["email".into()])
    .with_custom_hint("Email should match your company domain (@example.com)");
```

---

## Performance Impact

| Operation | Cost | Notes |
|-----------|------|-------|
| Error creation | +8 bytes | Option<String> field (None by default) |
| `.hint()` call | O(1) match | Lazy generation, no allocation unless custom |
| `.with_hint()` | 1 String allocation | Only for display, not on hot path |
| Serialization | Zero overhead | Hint skipped via #[serde(skip)] |

**Conclusion**: Negligible performance impact. Hints are opt-in and zero-cost if not used.

---

**Document Status**: Complete  
**Next Steps**: Implement in `src/core/error.rs`
