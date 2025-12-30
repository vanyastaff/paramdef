//! Boolean parameter type for true/false values.

use crate::core::{Flags, Key, Metadata, Value};
use crate::node::{Leaf, Node, NodeKind};

/// A boolean parameter schema for true/false values.
///
/// Boolean parameters are simple toggles without subtypes.
/// This is the **schema** definition - it does not hold runtime values.
///
/// # Example
///
/// ```
/// use paramdef::parameter::Boolean;
///
/// let enabled = Boolean::builder("enabled")
///     .label("Enable Feature")
///     .default(true)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct Boolean {
    metadata: Metadata,
    flags: Flags,
    default: Option<bool>,
}

impl Boolean {
    /// Creates a new builder for a boolean parameter.
    pub fn builder(key: impl Into<Key>) -> BooleanBuilder {
        BooleanBuilder::new(key)
    }

    /// Returns the default value, if set.
    #[must_use]
    pub fn default_bool(&self) -> Option<bool> {
        self.default
    }

    /// Returns the flags.
    #[must_use]
    pub fn flags(&self) -> Flags {
        self.flags
    }
}

impl Node for Boolean {
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

impl Leaf for Boolean {
    fn default_value(&self) -> Option<Value> {
        self.default.map(Value::Bool)
    }
}

/// Builder for [`Boolean`] parameters.
#[derive(Debug, Clone)]
pub struct BooleanBuilder {
    key: Key,
    label: Option<Key>,
    description: Option<Key>,
    group: Option<Key>,
    flags: Flags,
    default: Option<bool>,
}

impl BooleanBuilder {
    /// Creates a new boolean builder.
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

    /// Sets the default value.
    #[must_use]
    pub fn default(mut self, value: bool) -> Self {
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

    /// Builds the boolean parameter.
    #[must_use]
    pub fn build(self) -> Boolean {
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

        Boolean {
            metadata: metadata_builder.build(),
            flags: self.flags,
            default: self.default,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean_minimal() {
        let bool_param = Boolean::builder("enabled").build();

        assert_eq!(bool_param.key(), "enabled");
        assert_eq!(bool_param.kind(), NodeKind::Leaf);
        assert!(bool_param.default_value().is_none());
    }

    #[test]
    fn test_boolean_builder() {
        let bool_param = Boolean::builder("dark_mode")
            .label("Dark Mode")
            .description("Enable dark theme")
            .default(true)
            .build();

        assert_eq!(bool_param.key(), "dark_mode");
        assert_eq!(bool_param.metadata().label(), Some("Dark Mode"));
        assert_eq!(
            bool_param.metadata().description(),
            Some("Enable dark theme")
        );
        assert_eq!(bool_param.default_bool(), Some(true));
    }

    #[test]
    fn test_boolean_default_false() {
        let bool_param = Boolean::builder("disabled").default(false).build();

        assert_eq!(bool_param.default_bool(), Some(false));
        assert_eq!(bool_param.default_value(), Some(Value::Bool(false)));
    }

    #[test]
    fn test_boolean_default_value_as_value() {
        let bool_param = Boolean::builder("flag").default(true).build();

        let value = bool_param.default_value();
        assert!(value.is_some());
        assert_eq!(value.unwrap(), Value::Bool(true));
    }
}
