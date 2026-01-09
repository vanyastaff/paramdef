//! Transformer trait and function wrapper.
//!
//! This module provides the core abstraction for value transformations.

use std::fmt;
use std::sync::Arc;

use crate::Value;

/// Trait for transforming values.
///
/// Transformers modify input values before validation and storage.
/// Unlike validators, transformers return a new value rather than
/// a validation result.
///
/// # Implementation Requirements
///
/// 1. **Idempotent**: `transform(transform(x)) == transform(x)`
/// 2. **Pure**: No side effects, no external state mutations
/// 3. **Type-preserving** (when possible): Transform Text to Text, Number to Number
///
/// # Example
///
/// ```
/// use paramdef::transform::Transformer;
/// use paramdef::Value;
///
/// struct Capitalize;
///
/// impl Transformer for Capitalize {
///     fn name(&self) -> &'static str { "capitalize" }
///
///     fn transform(&self, value: &Value) -> Value {
///         if let Some(s) = value.as_text() {
///             let capitalized = s
///                 .split_whitespace()
///                 .map(|word| {
///                     let mut chars = word.chars();
///                     match chars.next() {
///                         None => String::new(),
///                         Some(first) => {
///                             first.to_uppercase().chain(chars).collect()
///                         }
///                     }
///                 })
///                 .collect::<Vec<_>>()
///                 .join(" ");
///             Value::text(capitalized)
///         } else {
///             value.clone()
///         }
///     }
/// }
///
/// let transformer = Capitalize;
/// let value = Value::text("hello world");
/// let result = transformer.transform(&value);
/// assert_eq!(result.as_text(), Some("Hello World"));
/// ```
pub trait Transformer: Send + Sync {
    /// Returns the name of this transformer for debugging and serialization.
    fn name(&self) -> &'static str;

    /// Transforms the given value.
    ///
    /// Returns a new value with the transformation applied.
    /// If the transformation is not applicable to this value type,
    /// implementations should return the value unchanged (clone it).
    fn transform(&self, value: &Value) -> Value;

    /// Returns true if this transformer applies to the given value.
    ///
    /// Default implementation returns true for all values.
    /// Override to skip transformation for non-applicable types.
    fn applies_to(&self, _value: &Value) -> bool {
        true
    }
}

// Allow Arc<dyn Transformer> to be used as Transformer
impl Transformer for Arc<dyn Transformer> {
    fn name(&self) -> &'static str {
        (**self).name()
    }

    fn transform(&self, value: &Value) -> Value {
        (**self).transform(value)
    }

    fn applies_to(&self, value: &Value) -> bool {
        (**self).applies_to(value)
    }
}

/// A transformer that wraps a function.
///
/// This allows using closures as transformers without implementing
/// the trait manually.
///
/// # Example
///
/// ```
/// use paramdef::transform::{FnTransformer, Transformer};
/// use paramdef::Value;
///
/// let reverse = FnTransformer::new("reverse", |value: &Value| {
///     if let Some(s) = value.as_text() {
///         Value::text(s.chars().rev().collect::<String>())
///     } else {
///         value.clone()
///     }
/// });
///
/// let value = Value::text("hello");
/// let result = reverse.transform(&value);
/// assert_eq!(result.as_text(), Some("olleh"));
/// ```
pub struct FnTransformer<F>
where
    F: Fn(&Value) -> Value + Send + Sync,
{
    name: &'static str,
    func: F,
}

impl<F> FnTransformer<F>
where
    F: Fn(&Value) -> Value + Send + Sync,
{
    /// Creates a new function-based transformer.
    ///
    /// # Arguments
    ///
    /// * `name` - A static string identifying this transformer
    /// * `func` - The transformation function
    pub fn new(name: &'static str, func: F) -> Self {
        Self { name, func }
    }
}

impl<F> Transformer for FnTransformer<F>
where
    F: Fn(&Value) -> Value + Send + Sync,
{
    fn name(&self) -> &'static str {
        self.name
    }

    fn transform(&self, value: &Value) -> Value {
        (self.func)(value)
    }
}

impl<F> fmt::Debug for FnTransformer<F>
where
    F: Fn(&Value) -> Value + Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FnTransformer")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn_transformer() {
        let uppercase = FnTransformer::new("uppercase", |value: &Value| {
            if let Some(s) = value.as_text() {
                Value::text(s.to_uppercase())
            } else {
                value.clone()
            }
        });

        let value = Value::text("hello");
        let result = uppercase.transform(&value);
        assert_eq!(result.as_text(), Some("HELLO"));
    }

    #[test]
    fn test_fn_transformer_non_text() {
        let uppercase = FnTransformer::new("uppercase", |value: &Value| {
            if let Some(s) = value.as_text() {
                Value::text(s.to_uppercase())
            } else {
                value.clone()
            }
        });

        let value = Value::Int(42);
        let result = uppercase.transform(&value);
        assert_eq!(result, Value::Int(42));
    }

    #[test]
    fn test_fn_transformer_debug() {
        let t = FnTransformer::new("test", |v: &Value| v.clone());
        let debug = format!("{t:?}");
        assert!(debug.contains("test"));
    }
}
