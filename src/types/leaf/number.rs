//! Number parameter type for numeric values.

use crate::core::{Flags, Key, Metadata, Value};
use crate::types::kind::NodeKind;
use crate::types::traits::{Leaf, Node, };
use crate::subtype::{NumberSubtype, NumberUnit};

/// A number parameter schema for numeric values.
///
/// Number parameters support various numeric types through [`NumberSubtype`],
/// with optional unit. This is the **schema** definition - it does not hold runtime values.
///
/// # Example
///
/// ```
/// use paramdef::types::leaf::Number;
/// use paramdef::subtype::NumberUnit;
///
/// // Integer parameter
/// let count = Number::integer("count")
///     .label("Count")
///     .build();
///
/// // Float parameter with unit
/// let temperature = Number::float("temperature")
///     .unit(NumberUnit::Celsius)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct Number<S: NumberSubtype> {
    metadata: Metadata,
    flags: Flags,
    subtype: S,
    unit: Option<NumberUnit>,
    default: Option<f64>,
}

impl<S: NumberSubtype> Number<S> {
    /// Returns the number subtype.
    #[must_use]
    pub fn subtype(&self) -> &S {
        &self.subtype
    }

    /// Returns the unit, if set.
    #[must_use]
    pub fn unit(&self) -> Option<NumberUnit> {
        self.unit
    }

    /// Returns the default value as f64, if set.
    #[must_use]
    pub fn default_f64(&self) -> Option<f64> {
        self.default
    }

    /// Returns the default value as i64, if set.
    #[must_use]
    pub fn default_i64(&self) -> Option<i64> {
        #[allow(clippy::cast_possible_truncation)]
        self.default.map(|v| v as i64)
    }

    /// Returns the flags.
    #[must_use]
    pub fn flags(&self) -> Flags {
        self.flags
    }
}

// Convenience constructors

impl Number<crate::subtype::GenericNumber> {
    /// Creates a builder for a generic number parameter.
    pub fn builder(key: impl Into<Key>) -> NumberBuilder<crate::subtype::GenericNumber> {
        NumberBuilder::new(key, crate::subtype::GenericNumber)
    }

    /// Creates an integer number parameter builder.
    pub fn integer(key: impl Into<Key>) -> NumberBuilder<crate::subtype::GenericNumber> {
        NumberBuilder::new(key, crate::subtype::GenericNumber)
    }

    /// Creates a float number parameter builder.
    pub fn float(key: impl Into<Key>) -> NumberBuilder<crate::subtype::GenericNumber> {
        NumberBuilder::new(key, crate::subtype::GenericNumber)
    }
}

impl Number<crate::subtype::Percentage> {
    /// Creates a percentage number parameter builder.
    pub fn percentage(key: impl Into<Key>) -> NumberBuilder<crate::subtype::Percentage> {
        NumberBuilder::new(key, crate::subtype::Percentage)
    }
}

impl Number<crate::subtype::Port> {
    /// Creates a port number parameter builder.
    pub fn port(key: impl Into<Key>) -> NumberBuilder<crate::subtype::Port> {
        NumberBuilder::new(key, crate::subtype::Port)
    }
}

impl Number<crate::subtype::Factor> {
    /// Creates a factor/multiplier parameter builder.
    pub fn factor(key: impl Into<Key>) -> NumberBuilder<crate::subtype::Factor> {
        NumberBuilder::new(key, crate::subtype::Factor)
    }
}

impl<S: NumberSubtype + 'static> Node for Number<S> {
    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn key(&self) -> &Key {
        self.metadata.key()
    }

    fn kind(&self) -> NodeKind {
        NodeKind::Leaf
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl<S: NumberSubtype> Leaf for Number<S> {
    fn default_value(&self) -> Option<Value> {
        self.default.map(Value::Float)
    }
}

/// Builder for [`Number`] parameters.
#[derive(Debug, Clone)]
pub struct NumberBuilder<S: NumberSubtype> {
    key: Key,
    label: Option<Key>,
    description: Option<Key>,
    group: Option<Key>,
    flags: Flags,
    subtype: S,
    unit: Option<NumberUnit>,
    default: Option<f64>,
}

impl<S: NumberSubtype> NumberBuilder<S> {
    /// Creates a new number builder.
    pub fn new(key: impl Into<Key>, subtype: S) -> Self {
        Self {
            key: key.into(),
            label: None,
            description: None,
            group: None,
            flags: Flags::empty(),
            subtype,
            unit: None,
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

    /// Sets the unit.
    #[must_use]
    pub fn unit(mut self, unit: NumberUnit) -> Self {
        self.unit = Some(unit);
        self
    }

    /// Sets the default value.
    #[must_use]
    pub fn default(mut self, value: f64) -> Self {
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

    /// Builds the number parameter.
    #[must_use]
    pub fn build(self) -> Number<S> {
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

        Number {
            metadata: metadata_builder.build(),
            flags: self.flags,
            subtype: self.subtype,
            unit: self.unit,
            default: self.default,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_minimal() {
        let num = Number::builder("value").build();

        assert_eq!(num.key(), "value");
        assert_eq!(num.kind(), NodeKind::Leaf);
        assert!(num.default_value().is_none());
    }

    #[test]
    fn test_number_builder() {
        let num = Number::float("temperature")
            .label("Temperature")
            .unit(NumberUnit::Celsius)
            .default(20.0)
            .build();

        assert_eq!(num.key(), "temperature");
        assert_eq!(num.metadata().label(), Some("Temperature"));
        assert_eq!(num.unit(), Some(NumberUnit::Celsius));
        assert_eq!(num.default_f64(), Some(20.0));
    }

    #[test]
    fn test_number_integer() {
        let num = Number::integer("count").default(42.0).build();

        assert_eq!(num.default_i64(), Some(42));
    }

    #[test]
    fn test_number_percentage() {
        let pct = Number::percentage("opacity").default(100.0).build();

        assert_eq!(pct.default_f64(), Some(100.0));
    }

    #[test]
    fn test_number_port() {
        let port = Number::port("http_port").default(8080.0).build();

        assert_eq!(port.default_i64(), Some(8080));
    }

    #[test]
    fn test_number_default_value_as_value() {
        let num = Number::float("x").default(3.14).build();

        let value = num.default_value();
        assert!(value.is_some());
        assert_eq!(value.unwrap(), Value::Float(3.14));
    }
}
