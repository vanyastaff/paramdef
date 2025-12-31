//! Text parameter type for string values.

use crate::core::{Flags, Key, Metadata};
use crate::node::{Leaf, Node, NodeKind};
use crate::subtypes::TextSubtype;
use smartstring::{LazyCompact, SmartString};

/// A text parameter schema for string values.
///
/// Text parameters support various string types through [`TextSubtype`].
/// This is the **schema** definition - it does not hold runtime values.
///
/// # Example
///
/// ```
/// use paramdef::parameter::Text;
/// use paramdef::subtypes::Email;
///
/// // Using builder
/// let username = Text::builder("username")
///     .label("Username")
///     .build();
///
/// // Using convenience constructor
/// let email = Text::email("contact_email");
/// ```
#[derive(Debug, Clone)]
pub struct Text<S: TextSubtype = crate::subtypes::Plain> {
    metadata: Metadata,
    flags: Flags,
    subtype: S,
    default: Option<SmartString<LazyCompact>>,
}

impl<S: TextSubtype> Text<S> {
    /// Returns the text subtype.
    #[must_use]
    pub fn subtype(&self) -> &S {
        &self.subtype
    }

    /// Returns the default string value, if set.
    #[must_use]
    pub fn default_str(&self) -> Option<&str> {
        self.default.as_deref()
    }

    /// Returns the flags.
    #[must_use]
    pub fn flags(&self) -> Flags {
        self.flags
    }
}

impl Text<crate::subtypes::Plain> {
    /// Creates a new builder for a text parameter.
    pub fn builder(key: impl Into<Key>) -> TextBuilder<crate::subtypes::Plain> {
        TextBuilder::new(key)
    }
}

// Convenience constructors for common subtypes
impl Text<crate::subtypes::Email> {
    /// Creates an email text parameter.
    #[must_use]
    pub fn email(key: impl Into<Key>) -> Self {
        TextBuilder::new(key)
            .subtype(crate::subtypes::Email)
            .build()
    }
}

impl Text<crate::subtypes::Url> {
    /// Creates a URL text parameter.
    #[must_use]
    pub fn url(key: impl Into<Key>) -> Self {
        TextBuilder::new(key).subtype(crate::subtypes::Url).build()
    }
}

impl Text<crate::subtypes::Password> {
    /// Creates a password text parameter.
    #[must_use]
    pub fn password(key: impl Into<Key>) -> Self {
        TextBuilder::new(key)
            .subtype(crate::subtypes::Password)
            .sensitive()
            .build()
    }
}

impl Text<crate::subtypes::MultiLine> {
    /// Creates a multiline text parameter.
    #[must_use]
    pub fn multiline(key: impl Into<Key>) -> Self {
        TextBuilder::new(key)
            .subtype(crate::subtypes::MultiLine)
            .build()
    }
}

impl Text<crate::subtypes::Json> {
    /// Creates a JSON text parameter.
    #[must_use]
    pub fn json(key: impl Into<Key>) -> Self {
        TextBuilder::new(key).subtype(crate::subtypes::Json).build()
    }
}

impl<S: TextSubtype + 'static> Node for Text<S> {
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
}

impl<S: TextSubtype> Leaf for Text<S> {
    fn default_value(&self) -> Option<crate::core::Value> {
        self.default.clone().map(crate::core::Value::Text)
    }
}

/// Builder for [`Text`] parameters.
#[derive(Debug, Clone)]
pub struct TextBuilder<S: TextSubtype = crate::subtypes::Plain> {
    key: Key,
    label: Option<Key>,
    description: Option<Key>,
    group: Option<Key>,
    flags: Flags,
    subtype: S,
    default: Option<SmartString<LazyCompact>>,
}

impl TextBuilder<crate::subtypes::Plain> {
    /// Creates a new text builder.
    pub fn new(key: impl Into<Key>) -> Self {
        Self {
            key: key.into(),
            label: None,
            description: None,
            group: None,
            flags: Flags::empty(),
            subtype: crate::subtypes::Plain,
            default: None,
        }
    }
}

impl<S: TextSubtype> TextBuilder<S> {
    /// Sets the subtype, returning a builder with the new type.
    pub fn subtype<T: TextSubtype>(self, subtype: T) -> TextBuilder<T> {
        TextBuilder {
            key: self.key,
            label: self.label,
            description: self.description,
            group: self.group,
            flags: self.flags,
            subtype,
            default: self.default,
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
    pub fn default(mut self, value: impl Into<SmartString<LazyCompact>>) -> Self {
        self.default = Some(value.into());
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

    /// Marks the parameter as sensitive.
    #[must_use]
    pub fn sensitive(mut self) -> Self {
        self.flags |= Flags::SENSITIVE;
        self
    }

    /// Builds the text parameter.
    #[must_use]
    pub fn build(self) -> Text<S> {
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

        Text {
            metadata: metadata_builder.build(),
            flags: self.flags,
            subtype: self.subtype,
            default: self.default,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::subtypes::{Email, Password};

    #[test]
    fn test_text_minimal() {
        let text = Text::builder("name").build();

        assert_eq!(text.key(), "name");
        assert_eq!(text.kind(), NodeKind::Leaf);
        assert!(text.default_value().is_none());
    }

    #[test]
    fn test_text_builder() {
        let text = Text::builder("username")
            .label("Username")
            .description("Your username")
            .default("guest")
            .required()
            .build();

        assert_eq!(text.key(), "username");
        assert_eq!(text.metadata().label(), Some("Username"));
        assert_eq!(text.metadata().description(), Some("Your username"));
        assert_eq!(text.default_str(), Some("guest"));
        assert!(text.flags().contains(Flags::REQUIRED));
    }

    #[test]
    fn test_text_email_convenience() {
        let email: Text<Email> = Text::email("contact");

        assert_eq!(email.key(), "contact");
        // Email is not sensitive by default
        assert!(!Email::is_sensitive());
    }

    #[test]
    fn test_text_password_convenience() {
        let password: Text<Password> = Text::password("secret");

        assert_eq!(password.key(), "secret");
        assert!(Password::is_sensitive());
        assert!(password.flags().contains(Flags::SENSITIVE));
    }

    #[test]
    fn test_text_subtype_change() {
        use crate::subtypes::Json;

        let builder = Text::builder("data").label("Data");
        let json_text = builder.subtype(Json).build();

        assert_eq!(json_text.key(), "data");
        // Json is multiline
        assert!(Json::is_multiline());
    }

    #[test]
    fn test_text_default_value_as_value() {
        let text = Text::builder("name").default("hello").build();

        let value = text.default_value();
        assert!(value.is_some());
        assert_eq!(value, Some(crate::core::Value::text("hello")));
    }
}
