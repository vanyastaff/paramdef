//! Expression target - where to apply the expression.

use crate::core::Key;

/// Target for expression evaluation.
///
/// Determines whether the expression applies to the current value being validated
/// or to another field's value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExprTarget {
    /// Apply to the local value (current parameter being validated).
    ///
    /// Used in validation rules to check the value being validated.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use paramdef::expr::{ExprTarget, Expr, Rule};
    ///
    /// // Validate that current value has minimum length of 8
    /// let rule = Rule::local(Expr::MinLength(8));
    /// ```
    Local,

    /// Apply to another field's value by key.
    ///
    /// Used for:
    /// - Visibility conditions (show field X if field Y meets condition)
    /// - Cross-field validation (field X must match field Y)
    ///
    /// # Example
    ///
    /// ```ignore
    /// use paramdef::expr::{ExprTarget, Expr, Rule};
    ///
    /// // Show current field only if "mode" field equals "advanced"
    /// let rule = Rule::field("mode", Expr::Eq(Value::text("advanced")));
    /// ```
    Field(Key),
}

impl ExprTarget {
    /// Create a local target.
    #[must_use]
    pub fn local() -> Self {
        Self::Local
    }

    /// Create a field target.
    #[must_use]
    pub fn field(key: impl Into<Key>) -> Self {
        Self::Field(key.into())
    }

    /// Returns true if this is a local target.
    #[must_use]
    pub fn is_local(&self) -> bool {
        matches!(self, Self::Local)
    }

    /// Returns true if this is a field target.
    #[must_use]
    pub fn is_field(&self) -> bool {
        matches!(self, Self::Field(_))
    }

    /// Get the field key if this is a field target.
    #[must_use]
    pub fn field_key(&self) -> Option<&Key> {
        match self {
            Self::Field(key) => Some(key),
            Self::Local => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local() {
        let target = ExprTarget::Local;
        assert!(target.is_local());
        assert!(!target.is_field());
        assert_eq!(target.field_key(), None);
    }

    #[test]
    fn test_field() {
        let target = ExprTarget::field("username");
        assert!(!target.is_local());
        assert!(target.is_field());
        assert_eq!(target.field_key(), Some(&Key::from("username")));
    }

    #[test]
    fn test_equality() {
        assert_eq!(ExprTarget::Local, ExprTarget::Local);
        assert_eq!(ExprTarget::field("name"), ExprTarget::field("name"));
        assert_ne!(ExprTarget::field("name"), ExprTarget::field("email"));
        assert_ne!(ExprTarget::Local, ExprTarget::field("name"));
    }
}
