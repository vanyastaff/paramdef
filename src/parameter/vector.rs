//! Vector parameter type for fixed-size numeric arrays.

use crate::core::{Flags, Key, Metadata, Value};
use crate::node::{Leaf, Node, NodeKind};
use crate::subtypes::{Numeric, NumericKind};

/// A vector parameter schema for fixed-size numeric arrays.
///
/// Vector parameters store fixed-size arrays of numeric values.
/// The element type and size are stored at runtime, but the builder
/// provides compile-time type safety.
///
/// This is the **schema** definition - it does not hold runtime values.
///
/// # Example
///
/// ```
/// use paramdef::parameter::Vector;
///
/// // 3D position vector
/// let position = Vector::builder::<f64, 3>("position")
///     .label("Position")
///     .default([0.0, 0.0, 0.0])
///     .build();
///
/// assert_eq!(position.size(), 3);
/// ```
#[derive(Debug, Clone)]
pub struct Vector {
    metadata: Metadata,
    flags: Flags,
    element_type: NumericKind,
    size: usize,
    default: Option<Vec<f64>>,
}

impl Vector {
    /// Creates a vector builder with compile-time type safety.
    ///
    /// The generic parameters provide type safety at construction time,
    /// while the resulting `Vector` stores the information at runtime.
    pub fn builder<T: Numeric, const N: usize>(key: impl Into<Key>) -> VectorBuilder<T, N> {
        VectorBuilder::new(key)
    }

    /// Returns the element type.
    #[must_use]
    pub fn element_type(&self) -> NumericKind {
        self.element_type
    }

    /// Returns the vector size (number of components).
    #[must_use]
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the default value, if set.
    #[must_use]
    pub fn default_vec(&self) -> Option<&[f64]> {
        self.default.as_deref()
    }

    /// Returns the flags.
    #[must_use]
    pub fn flags(&self) -> Flags {
        self.flags
    }
}

impl Node for Vector {
    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn key(&self) -> &Key {
        self.metadata.key()
    }

    fn kind(&self) -> NodeKind {
        NodeKind::Leaf
    }
}

impl Leaf for Vector {
    fn default_value(&self) -> Option<Value> {
        self.default
            .as_ref()
            .map(|v| Value::array(v.iter().copied().map(Value::Float).collect::<Vec<_>>()))
    }
}

/// Builder for [`Vector`] parameters with compile-time type safety.
///
/// The generic parameters `T` (element type) and `N` (size) provide
/// compile-time safety, while the built `Vector` stores this information
/// at runtime for uniform schema storage.
#[derive(Debug, Clone)]
pub struct VectorBuilder<T: Numeric, const N: usize> {
    key: Key,
    label: Option<Key>,
    description: Option<Key>,
    group: Option<Key>,
    flags: Flags,
    default: Option<[T; N]>,
}

impl<T: Numeric, const N: usize> VectorBuilder<T, N> {
    /// Creates a new vector builder.
    pub fn new(key: impl Into<Key>) -> Self {
        Self {
            key: key.into(),
            label: None,
            description: None,
            group: None,
            flags: Flags::empty(),
            default: None,
        }
    }

    /// Sets the display label.
    #[must_use]
    pub fn label(mut self, label: impl Into<Key>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the description.
    #[must_use]
    pub fn description(mut self, description: impl Into<Key>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the group.
    #[must_use]
    pub fn group(mut self, group: impl Into<Key>) -> Self {
        self.group = Some(group.into());
        self
    }

    /// Sets the default value with compile-time size checking.
    #[must_use]
    pub fn default(mut self, value: [T; N]) -> Self {
        self.default = Some(value);
        self
    }

    /// Marks the parameter as required.
    #[must_use]
    pub fn required(mut self) -> Self {
        self.flags |= Flags::REQUIRED;
        self
    }

    /// Marks the parameter as readonly.
    #[must_use]
    pub fn readonly(mut self) -> Self {
        self.flags |= Flags::READONLY;
        self
    }

    /// Marks the parameter as hidden.
    #[must_use]
    pub fn hidden(mut self) -> Self {
        self.flags |= Flags::HIDDEN;
        self
    }

    /// Builds the vector parameter.
    #[must_use]
    pub fn build(self) -> Vector {
        let mut metadata_builder = Metadata::builder(self.key);

        if let Some(label) = self.label {
            metadata_builder = metadata_builder.label(label);
        }
        if let Some(description) = self.description {
            metadata_builder = metadata_builder.description(description);
        }
        if let Some(group) = self.group {
            metadata_builder = metadata_builder.group(group);
        }

        Vector {
            metadata: metadata_builder.build(),
            flags: self.flags,
            element_type: T::kind(),
            size: N,
            default: self
                .default
                .map(|arr| arr.iter().map(|v| v.to_f64()).collect()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_minimal() {
        let vec = Vector::builder::<f64, 3>("position").build();

        assert_eq!(vec.key(), "position");
        assert_eq!(vec.kind(), NodeKind::Leaf);
        assert_eq!(vec.size(), 3);
        assert_eq!(vec.element_type(), NumericKind::F64);
        assert!(vec.default_value().is_none());
    }

    #[test]
    fn test_vector_with_default() {
        let vec = Vector::builder::<f64, 3>("position")
            .label("Position")
            .default([1.0, 2.0, 3.0])
            .build();

        assert_eq!(vec.key(), "position");
        assert_eq!(vec.metadata().label(), Some("Position"));
        assert_eq!(vec.default_vec(), Some([1.0, 2.0, 3.0].as_slice()));
    }

    #[test]
    fn test_vector_size_2() {
        let vec = Vector::builder::<f64, 2>("uv").default([0.0, 0.0]).build();

        assert_eq!(vec.size(), 2);
    }

    #[test]
    fn test_vector_size_4() {
        let vec = Vector::builder::<f64, 4>("color")
            .default([1.0, 1.0, 1.0, 1.0])
            .build();

        assert_eq!(vec.size(), 4);
    }

    #[test]
    fn test_vector_i32_elements() {
        let vec = Vector::builder::<i32, 3>("grid_pos")
            .default([0, 0, 0])
            .build();

        assert_eq!(vec.element_type(), NumericKind::I32);
        assert_eq!(vec.size(), 3);
    }

    #[test]
    fn test_vector_default_value_as_value() {
        let vec = Vector::builder::<f64, 3>("pos")
            .default([1.0, 2.0, 3.0])
            .build();

        let value = vec.default_value();
        assert!(value.is_some());

        let expected = Value::array(vec![
            Value::Float(1.0),
            Value::Float(2.0),
            Value::Float(3.0),
        ]);
        assert_eq!(value.unwrap(), expected);
    }

    #[test]
    fn test_numeric_kind() {
        assert_eq!(NumericKind::I32.name(), "i32");
        assert_eq!(NumericKind::I64.name(), "i64");
        assert_eq!(NumericKind::F32.name(), "f32");
        assert_eq!(NumericKind::F64.name(), "f64");

        assert!(NumericKind::I32.is_integer());
        assert!(NumericKind::I64.is_integer());
        assert!(!NumericKind::F32.is_integer());
        assert!(!NumericKind::F64.is_integer());

        assert!(!NumericKind::I32.is_float());
        assert!(NumericKind::F32.is_float());
        assert!(NumericKind::F64.is_float());
    }
}
