//! Text parameter type for string values.

use crate::core::{Flags, Key, Metadata, SmartStr};
use crate::subtype::TextSubtype;
use crate::types::kind::NodeKind;
use crate::types::traits::{Leaf, Node};

/// A text parameter schema for string values.
///
/// Text parameters support various string types through [`TextSubtype`].
/// This is the **schema** definition - it does not hold runtime values.
///
/// # Example
///
/// ```
/// use paramdef::types::leaf::Text;
/// use paramdef::subtype::Email;
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
pub struct Text<S: TextSubtype = crate::subtype::Plain> {
    metadata: Metadata,
    flags: Flags,
    subtype: S,
    default: Option<SmartStr>,
    #[cfg(feature = "visibility")]
    visibility: Option<crate::expr::Rule>,
    #[cfg(feature = "validation")]
    rules: crate::validation::Rules,
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

    /// Returns the validation rules.
    #[cfg(feature = "validation")]
    #[must_use]
    pub fn rules(&self) -> &crate::validation::Rules {
        &self.rules
    }
}

impl Text<crate::subtype::Plain> {
    /// Creates a new builder for a text parameter.
    pub fn builder(key: impl Into<Key>) -> TextBuilder<crate::subtype::Plain> {
        TextBuilder::new(key)
    }

    /// Creates a required text field with a label (1-liner convenience).
    ///
    /// This is a shorthand that combines builder construction, label, and required flag.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::types::leaf::Text;
    ///
    /// // Before (4 lines):
    /// let name = Text::builder("name")
    ///     .label("Full Name")
    ///     .required()
    ///     .build();
    ///
    /// // After (1 line):
    /// let name = Text::required("name", "Full Name");
    /// ```
    #[must_use]
    pub fn required(
        key: impl Into<Key>,
        label: impl Into<crate::core::SmartStr>,
    ) -> Text<crate::subtype::Plain> {
        Text::builder(key).label(label.into()).required().build()
    }

    /// Creates a required email field builder with validation.
    ///
    /// Returns a builder configured with:
    /// - `Email` subtype
    /// - Required flag
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::types::leaf::Text;
    ///
    /// let email = Text::required_email("contact")
    ///     .label("Contact Email")
    ///     .build();
    /// ```
    pub fn required_email(key: impl Into<Key>) -> TextBuilder<crate::subtype::Email> {
        TextBuilder::new(key)
            .subtype(crate::subtype::Email)
            .required()
    }

    /// Creates a multiline text area builder.
    ///
    /// Returns a builder configured with:
    /// - `MultiLine` subtype
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::types::leaf::Text;
    ///
    /// let description = Text::textarea("description")
    ///     .label("Description")
    ///     .build();
    /// ```
    pub fn textarea(key: impl Into<Key>) -> TextBuilder<crate::subtype::MultiLine> {
        TextBuilder::new(key).subtype(crate::subtype::MultiLine)
    }

    /// Creates a URL slug field builder.
    ///
    /// Returns a builder configured with:
    /// - `Slug` subtype
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::types::leaf::Text;
    ///
    /// let slug = Text::slug("url_slug")
    ///     .label("URL Slug")
    ///     .build();
    /// ```
    pub fn slug(key: impl Into<Key>) -> TextBuilder<crate::subtype::Slug> {
        TextBuilder::new(key).subtype(crate::subtype::Slug)
    }

    /// Creates a phone number field builder.
    ///
    /// Returns a builder configured with:
    /// - `PhoneNumber` subtype
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::types::leaf::Text;
    ///
    /// let phone = Text::phone("contact_phone")
    ///     .label("Phone Number")
    ///     .build();
    /// ```
    pub fn phone(key: impl Into<Key>) -> TextBuilder<crate::subtype::PhoneNumber> {
        TextBuilder::new(key).subtype(crate::subtype::PhoneNumber)
    }

    /// Creates a UUID field builder.
    ///
    /// Returns a builder configured with:
    /// - `Uuid` subtype
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::types::leaf::Text;
    ///
    /// let id = Text::uuid("resource_id")
    ///     .label("Resource ID")
    ///     .readonly()
    ///     .build();
    /// ```
    pub fn uuid(key: impl Into<Key>) -> TextBuilder<crate::subtype::Uuid> {
        TextBuilder::new(key).subtype(crate::subtype::Uuid)
    }
}

// Convenience constructors for common subtypes
impl Text<crate::subtype::Email> {
    /// Creates an email text parameter.
    #[must_use]
    pub fn email(key: impl Into<Key>) -> Self {
        TextBuilder::new(key).subtype(crate::subtype::Email).build()
    }
}

impl Text<crate::subtype::Url> {
    /// Creates a URL text parameter.
    #[must_use]
    pub fn url(key: impl Into<Key>) -> Self {
        TextBuilder::new(key).subtype(crate::subtype::Url).build()
    }
}

impl Text<crate::subtype::Password> {
    /// Creates a password text parameter.
    #[must_use]
    pub fn password(key: impl Into<Key>) -> Self {
        TextBuilder::new(key)
            .subtype(crate::subtype::Password)
            .sensitive()
            .build()
    }
}

impl Text<crate::subtype::MultiLine> {
    /// Creates a multiline text parameter.
    #[must_use]
    pub fn multiline(key: impl Into<Key>) -> Self {
        TextBuilder::new(key)
            .subtype(crate::subtype::MultiLine)
            .build()
    }
}

impl Text<crate::subtype::Json> {
    /// Creates a JSON text parameter.
    #[must_use]
    pub fn json(key: impl Into<Key>) -> Self {
        TextBuilder::new(key).subtype(crate::subtype::Json).build()
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

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
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
pub struct TextBuilder<S: TextSubtype = crate::subtype::Plain> {
    key: Key,
    label: Option<SmartStr>,
    description: Option<SmartStr>,
    group: Option<Key>,
    flags: Flags,
    subtype: S,
    default: Option<SmartStr>,
    #[cfg(feature = "visibility")]
    visibility: Option<crate::expr::Rule>,
    #[cfg(feature = "validation")]
    rules: crate::validation::Rules,
}

impl TextBuilder<crate::subtype::Plain> {
    /// Creates a new text builder.
    pub fn new(key: impl Into<Key>) -> Self {
        Self {
            key: key.into(),
            label: None,
            description: None,
            group: None,
            flags: Flags::empty(),
            subtype: crate::subtype::Plain,
            default: None,
            #[cfg(feature = "visibility")]
            visibility: None,
            #[cfg(feature = "validation")]
            rules: crate::validation::Rules::new(),
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
            #[cfg(feature = "visibility")]
            visibility: self.visibility,
            #[cfg(feature = "validation")]
            rules: self.rules,
        }
    }

    /// Sets the display label.
    #[must_use]
    pub fn label(mut self, label: impl Into<SmartStr>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the description.
    #[must_use]
    pub fn description(mut self, description: impl Into<SmartStr>) -> Self {
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
    pub fn default(mut self, value: impl Into<SmartStr>) -> Self {
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

    /// Sets a visibility condition.
    ///
    /// The parameter will only be visible when the rule evaluates to true.
    #[cfg(feature = "visibility")]
    #[must_use]
    pub fn visible_when(mut self, rule: crate::expr::Rule) -> Self {
        self.visibility = Some(rule);
        self
    }

    /// Adds a validation rule requiring the field to have a value.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::types::leaf::Text;
    ///
    /// let field = Text::builder("name")
    ///     .validate_required()
    ///     .build();
    /// ```
    #[cfg(feature = "validation")]
    #[must_use]
    pub fn validate_required(mut self) -> Self {
        self.rules.push(crate::validation::Rule::required());
        self
    }

    /// Adds a validation rule for email format.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::types::leaf::Text;
    ///
    /// let field = Text::builder("email")
    ///     .validate_email()
    ///     .build();
    /// ```
    #[cfg(feature = "validation")]
    #[must_use]
    pub fn validate_email(mut self) -> Self {
        self.rules.push(crate::validation::Rule::email());
        self
    }

    /// Adds a validation rule for minimum string length.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::types::leaf::Text;
    ///
    /// let field = Text::builder("username")
    ///     .validate_min_length(3)
    ///     .build();
    /// ```
    #[cfg(feature = "validation")]
    #[must_use]
    pub fn validate_min_length(mut self, min: usize) -> Self {
        self.rules.push(crate::validation::Rule::min_length(min));
        self
    }

    /// Adds a validation rule for maximum string length.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::types::leaf::Text;
    ///
    /// let field = Text::builder("bio")
    ///     .validate_max_length(500)
    ///     .build();
    /// ```
    #[cfg(feature = "validation")]
    #[must_use]
    pub fn validate_max_length(mut self, max: usize) -> Self {
        self.rules.push(crate::validation::Rule::max_length(max));
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
            #[cfg(feature = "visibility")]
            visibility: self.visibility,
            #[cfg(feature = "validation")]
            rules: self.rules,
        }
    }
}

// Visibility trait implementation
#[cfg(feature = "visibility")]
impl<S: TextSubtype> crate::types::traits::Visibility for Text<S> {
    fn visibility_rule(&self) -> Option<&crate::expr::Rule> {
        self.visibility.as_ref()
    }

    fn set_visibility_rule(&mut self, rule: Option<crate::expr::Rule>) {
        self.visibility = rule;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::subtype::{Email, Password};

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
        use crate::subtype::Json;

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
