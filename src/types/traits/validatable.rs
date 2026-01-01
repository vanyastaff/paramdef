//! Validatable trait for nodes that can be validated.

#![cfg(feature = "validation")]

use crate::core::Value;
use crate::types::kind::NodeKind;
use crate::types::traits::Node;

/// Trait for nodes that can be validated.
///
/// Implemented by Container and Leaf nodes (11 out of 14 types) when the
/// `validation` feature is enabled. Group, Layout, and Decoration do not
/// have values to validate.
///
/// # Implementors
///
/// - **Container (6)**: Object, List, Mode, Routing, Expirable, Ref
/// - **Leaf (5)**: Text, Number, Boolean, Vector, Select
///
/// # Future Extensions
///
/// - `validate_async`: Async validation will be added when `ValidationConfig`
///   is implemented. Requires either `async_trait` crate or RPITIT (Rust 1.75+).
/// - `validation()`: Returns `Option<&ValidationConfig>` - deferred until
///   `ValidationConfig` type is implemented in the validation feature phase.
///
/// # Example
///
/// ```ignore
/// use paramdef::types::traits::Validatable;
/// use paramdef::types::leaf::Number;
/// use paramdef::core::Value;
///
/// let number = Number::builder("age")
///     .range(0.0..=150.0)
///     .build();
///
/// // Validate a value
/// assert!(number.validate_sync(&Value::Int(30)).is_ok());
/// assert!(number.validate_sync(&Value::Int(200)).is_err());
/// ```
pub trait Validatable: Node {
    /// Validates a value synchronously.
    ///
    /// Runs all synchronous validators and returns the first error, if any.
    ///
    /// # Errors
    ///
    /// Returns a validation error if the value fails any validator.
    fn validate_sync(&self, value: &Value) -> crate::core::Result<()>;

    /// Returns the expected `NodeKind` for values.
    fn expected_kind(&self) -> NodeKind {
        self.kind()
    }

    /// Returns whether the given value is considered empty.
    ///
    /// Used for required field validation.
    fn is_empty(&self, value: &Value) -> bool {
        match value {
            Value::Null => true,
            Value::Text(s) => s.is_empty(),
            Value::Array(arr) => arr.is_empty(),
            Value::Object(obj) => obj.is_empty(),
            _ => false,
        }
    }
}
