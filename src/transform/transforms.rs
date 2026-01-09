//! Collection of transformations.
//!
//! This module provides the [`Transforms`] type for combining multiple
//! transformations into a single unit.

use std::sync::Arc;

use crate::Value;

use super::{FnTransformer, Transform, Transformer};

/// A collection of transformations to apply in sequence.
///
/// `Transforms` combines both declarative [`Transform`] expressions and
/// programmatic [`Transformer`] implementations into a single pipeline.
///
/// # Example
///
/// ```
/// use paramdef::transform::{Transform, Transforms};
/// use paramdef::Value;
///
/// let transforms = Transforms::new()
///     .push(Transform::Trim)
///     .push(Transform::Lowercase)
///     .push(Transform::Capitalize);
///
/// let value = Value::text("  HELLO WORLD  ");
/// let result = transforms.apply(&value);
/// assert_eq!(result.as_text(), Some("Hello World"));
/// ```
#[derive(Clone, Default)]
pub struct Transforms {
    transforms: Vec<TransformItem>,
}

/// An item in the transforms collection.
#[derive(Clone)]
enum TransformItem {
    /// Declarative transformation expression.
    Expr(Transform),
    /// Programmatic transformer.
    Fn(Arc<dyn Transformer>),
}

impl Transforms {
    /// Creates an empty transforms collection.
    #[must_use]
    pub fn new() -> Self {
        Self {
            transforms: Vec::new(),
        }
    }

    /// Creates a transforms collection with a single declarative transformation.
    #[must_use]
    pub fn single(transform: Transform) -> Self {
        Self {
            transforms: vec![TransformItem::Expr(transform)],
        }
    }

    /// Returns true if no transformations are configured.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.transforms.is_empty()
    }

    /// Returns the number of transformations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.transforms.len()
    }

    /// Adds a declarative transformation to the pipeline.
    #[must_use]
    pub fn push(mut self, transform: Transform) -> Self {
        self.transforms.push(TransformItem::Expr(transform));
        self
    }

    /// Adds a programmatic transformer to the pipeline.
    #[must_use]
    pub fn custom<T: Transformer + 'static>(mut self, transformer: T) -> Self {
        self.transforms
            .push(TransformItem::Fn(Arc::new(transformer)));
        self
    }

    /// Adds a function-based transformer to the pipeline.
    #[must_use]
    pub fn func<F>(mut self, name: &'static str, f: F) -> Self
    where
        F: Fn(&Value) -> Value + Send + Sync + 'static,
    {
        self.transforms
            .push(TransformItem::Fn(Arc::new(FnTransformer::new(name, f))));
        self
    }

    /// Applies all transformations to a value in sequence.
    ///
    /// Transformations are applied in the order they were added.
    /// Each transformation receives the output of the previous one.
    #[must_use]
    pub fn apply(&self, value: &Value) -> Value {
        self.transforms
            .iter()
            .fold(value.clone(), |v, item| match item {
                TransformItem::Expr(t) => t.apply(&v),
                TransformItem::Fn(t) => t.transform(&v),
            })
    }

    // === Convenience Methods ===

    /// Adds a trim transformation.
    #[must_use]
    pub fn trim(self) -> Self {
        self.push(Transform::Trim)
    }

    /// Adds a lowercase transformation.
    #[must_use]
    pub fn lowercase(self) -> Self {
        self.push(Transform::Lowercase)
    }

    /// Adds an uppercase transformation.
    #[must_use]
    pub fn uppercase(self) -> Self {
        self.push(Transform::Uppercase)
    }

    /// Adds a capitalize transformation.
    #[must_use]
    pub fn capitalize(self) -> Self {
        self.push(Transform::Capitalize)
    }

    /// Adds a clamp transformation.
    #[must_use]
    pub fn clamp(self, min: f64, max: f64) -> Self {
        self.push(Transform::Clamp { min, max })
    }

    /// Adds a round-to-decimals transformation.
    #[must_use]
    pub fn round_to(self, decimals: u32) -> Self {
        self.push(Transform::RoundTo { decimals })
    }

    /// Adds a default value transformation.
    #[must_use]
    pub fn default_to(self, value: Value) -> Self {
        self.push(Transform::DefaultTo { value })
    }

    /// Adds a truncate transformation.
    #[must_use]
    pub fn truncate(self, max_length: usize) -> Self {
        self.push(Transform::Truncate {
            max_length,
            suffix: None,
        })
    }

    /// Adds a truncate with ellipsis transformation.
    #[must_use]
    pub fn truncate_with_ellipsis(self, max_length: usize) -> Self {
        self.push(Transform::Truncate {
            max_length,
            suffix: Some("...".to_string()),
        })
    }

    /// Adds a replace transformation.
    #[must_use]
    pub fn replace(self, from: impl Into<String>, to: impl Into<String>) -> Self {
        self.push(Transform::Replace {
            from: from.into(),
            to: to.into(),
        })
    }

    /// Adds a collapse whitespace transformation.
    #[must_use]
    pub fn collapse_whitespace(self) -> Self {
        self.push(Transform::CollapseWhitespace)
    }

    /// Adds a null-if-empty transformation.
    #[must_use]
    pub fn null_if_empty(self) -> Self {
        self.push(Transform::NullIfEmpty)
    }
}

impl std::fmt::Debug for Transforms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Transforms")
            .field("count", &self.transforms.len())
            .finish_non_exhaustive()
    }
}

impl From<Transform> for Transforms {
    fn from(transform: Transform) -> Self {
        Self::single(transform)
    }
}

impl<T: Transformer + 'static> From<T> for Transforms {
    fn from(transformer: T) -> Self {
        Self::new().custom(transformer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let transforms = Transforms::new();
        assert!(transforms.is_empty());
        assert_eq!(transforms.len(), 0);

        let value = Value::text("hello");
        assert_eq!(transforms.apply(&value), value);
    }

    #[test]
    fn test_single() {
        let transforms = Transforms::single(Transform::Uppercase);
        assert!(!transforms.is_empty());
        assert_eq!(transforms.len(), 1);

        let value = Value::text("hello");
        assert_eq!(transforms.apply(&value).as_text(), Some("HELLO"));
    }

    #[test]
    fn test_chain() {
        let transforms = Transforms::new()
            .push(Transform::Trim)
            .push(Transform::Lowercase);

        let value = Value::text("  HELLO  ");
        assert_eq!(transforms.apply(&value).as_text(), Some("hello"));
    }

    #[test]
    fn test_custom_transformer() {
        struct Reverse;
        impl Transformer for Reverse {
            fn name(&self) -> &'static str {
                "reverse"
            }
            fn transform(&self, value: &Value) -> Value {
                if let Some(s) = value.as_text() {
                    Value::text(s.chars().rev().collect::<String>())
                } else {
                    value.clone()
                }
            }
        }

        let transforms = Transforms::new().custom(Reverse);
        let value = Value::text("hello");
        assert_eq!(transforms.apply(&value).as_text(), Some("olleh"));
    }

    #[test]
    fn test_func_transformer() {
        let transforms = Transforms::new().func("double", |v| {
            if let Value::Int(n) = v {
                Value::Int(n * 2)
            } else {
                v.clone()
            }
        });

        let value = Value::Int(21);
        assert_eq!(transforms.apply(&value), Value::Int(42));
    }

    #[test]
    fn test_mixed_transforms() {
        struct Double;
        impl Transformer for Double {
            fn name(&self) -> &'static str {
                "double"
            }
            fn transform(&self, value: &Value) -> Value {
                if let Value::Int(n) = value {
                    Value::Int(n * 2)
                } else {
                    value.clone()
                }
            }
        }

        let transforms = Transforms::new()
            .push(Transform::Clamp {
                min: 0.0,
                max: 100.0,
            })
            .custom(Double);

        // 150 -> clamped to 100 -> doubled to 200
        let value = Value::Int(150);
        assert_eq!(transforms.apply(&value), Value::Int(200));
    }

    #[test]
    fn test_convenience_methods() {
        let transforms = Transforms::new().trim().lowercase().capitalize();

        let value = Value::text("  HELLO WORLD  ");
        assert_eq!(transforms.apply(&value).as_text(), Some("Hello World"));
    }

    #[test]
    fn test_from_transform() {
        let transforms: Transforms = Transform::Trim.into();
        assert_eq!(transforms.len(), 1);
    }

    #[test]
    fn test_debug() {
        let transforms = Transforms::new().trim().lowercase();
        let debug = format!("{transforms:?}");
        assert!(debug.contains("Transforms"));
        assert!(debug.contains("count"));
    }
}
