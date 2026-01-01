//! Select parameter type for single/multiple selection.

use crate::core::{Flags, Key, Metadata, SmartStr, Value};
use crate::types::kind::NodeKind;
use crate::types::traits::{Leaf, Node, };

/// Selection mode for the select parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectionMode {
    /// Single selection (dropdown, radio buttons).
    #[default]
    Single,
    /// Multiple selection (checkboxes, multi-select).
    Multiple,
}

/// Source of options for the select parameter.
#[derive(Debug, Clone, Default)]
pub enum OptionSource {
    /// Static list of options defined at schema time.
    #[default]
    Static,
    /// Dynamic options loaded at runtime (via loader).
    Dynamic,
}

/// A single option in a select parameter.
#[derive(Debug, Clone)]
pub struct SelectOption {
    /// Unique identifier for the option.
    pub value: Key,
    /// Display label for the option.
    pub label: SmartStr,
    /// Optional description/tooltip.
    pub description: Option<SmartStr>,
    /// Optional icon identifier.
    pub icon: Option<Key>,
    /// Optional group for categorization.
    pub group: Option<Key>,
}

impl SelectOption {
    /// Creates a new select option with the given value and label.
    #[must_use]
    pub fn new(value: impl Into<Key>, label: impl Into<SmartStr>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            description: None,
            icon: None,
            group: None,
        }
    }

    /// Creates a simple option where value and label are the same.
    #[must_use]
    pub fn simple(value: impl Into<Key>) -> Self {
        let v: Key = value.into();
        let label: SmartStr = v.as_str().into();
        Self {
            value: v,
            label,
            description: None,
            icon: None,
            group: None,
        }
    }

    /// Sets the description for this option.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<SmartStr>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the icon for this option.
    #[must_use]
    pub fn with_icon(mut self, icon: impl Into<Key>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Sets the group for this option.
    #[must_use]
    pub fn with_group(mut self, group: impl Into<Key>) -> Self {
        self.group = Some(group.into());
        self
    }
}

/// A select parameter schema for single or multiple selection.
///
/// Select parameters support both static and dynamic option sources,
/// and can be configured for single or multiple selection.
/// This is the **schema** definition - it does not hold runtime values.
///
/// # Four Use Cases
///
/// | Selection | Source | Value | Example |
/// |-----------|--------|-------|---------|
/// | Single | Static | `Value::Text` | Dropdown, radio |
/// | Multiple | Static | `Value::Array` | Multi-select, checkboxes |
/// | Single | Dynamic | `Value::Text` | Resource picker |
/// | Multiple | Dynamic | `Value::Array` | Multi-resource |
///
/// # Example
///
/// ```
/// use paramdef::types::leaf::{Select, SelectOption, SelectionMode};
///
/// let method = Select::single("method")
///     .label("HTTP Method")
///     .options(vec![
///         SelectOption::simple("GET"),
///         SelectOption::simple("POST"),
///         SelectOption::simple("PUT"),
///         SelectOption::simple("DELETE"),
///     ])
///     .default_single("GET")
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct Select {
    metadata: Metadata,
    flags: Flags,
    selection_mode: SelectionMode,
    option_source: OptionSource,
    /// Static options (when `option_source` is `Static`).
    options: Vec<SelectOption>,
    /// Default value for single selection.
    default_single: Option<Key>,
    /// Default values for multiple selection.
    default_multiple: Option<Vec<Key>>,
    /// Whether options can be searched/filtered.
    searchable: bool,
    /// Whether new options can be created by the user.
    creatable: bool,
}

impl Select {
    /// Creates a single-selection select parameter builder.
    pub fn single(key: impl Into<Key>) -> SelectBuilder {
        SelectBuilder::new(key, SelectionMode::Single)
    }

    /// Creates a multiple-selection select parameter builder.
    pub fn multiple(key: impl Into<Key>) -> SelectBuilder {
        SelectBuilder::new(key, SelectionMode::Multiple)
    }

    /// Returns the selection mode.
    #[must_use]
    pub fn selection_mode(&self) -> SelectionMode {
        self.selection_mode
    }

    /// Returns the option source.
    #[must_use]
    pub fn option_source(&self) -> &OptionSource {
        &self.option_source
    }

    /// Returns the static options.
    #[must_use]
    pub fn options(&self) -> &[SelectOption] {
        &self.options
    }

    /// Returns the default value for single selection.
    #[must_use]
    pub fn default_single(&self) -> Option<&Key> {
        self.default_single.as_ref()
    }

    /// Returns the default values for multiple selection.
    #[must_use]
    pub fn default_multiple(&self) -> Option<&[Key]> {
        self.default_multiple.as_deref()
    }

    /// Returns whether options are searchable.
    #[must_use]
    pub fn is_searchable(&self) -> bool {
        self.searchable
    }

    /// Returns whether new options can be created.
    #[must_use]
    pub fn is_creatable(&self) -> bool {
        self.creatable
    }

    /// Returns the flags.
    #[must_use]
    pub fn flags(&self) -> Flags {
        self.flags
    }
}

impl Node for Select {
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

impl Leaf for Select {
    fn default_value(&self) -> Option<Value> {
        match self.selection_mode {
            SelectionMode::Single => self
                .default_single
                .as_ref()
                .map(|v| Value::text(v.as_str())),
            SelectionMode::Multiple => self
                .default_multiple
                .as_ref()
                .map(|values| Value::array(values.iter().map(|v| Value::text(v.as_str())))),
        }
    }
}

/// Builder for [`Select`] parameters.
#[derive(Debug, Clone)]
pub struct SelectBuilder {
    key: Key,
    label: Option<Key>,
    description: Option<Key>,
    group: Option<Key>,
    flags: Flags,
    selection_mode: SelectionMode,
    option_source: OptionSource,
    options: Vec<SelectOption>,
    default_single: Option<Key>,
    default_multiple: Option<Vec<Key>>,
    searchable: bool,
    creatable: bool,
}

impl SelectBuilder {
    /// Creates a new select builder.
    pub fn new(key: impl Into<Key>, selection_mode: SelectionMode) -> Self {
        Self {
            key: key.into(),
            label: None,
            description: None,
            group: None,
            flags: Flags::empty(),
            selection_mode,
            option_source: OptionSource::Static,
            options: Vec::new(),
            default_single: None,
            default_multiple: None,
            searchable: false,
            creatable: false,
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

    /// Sets the static options.
    #[must_use]
    pub fn options(mut self, options: Vec<SelectOption>) -> Self {
        self.options = options;
        self.option_source = OptionSource::Static;
        self
    }

    /// Marks this select as having dynamic options (loaded at runtime).
    #[must_use]
    pub fn dynamic(mut self) -> Self {
        self.option_source = OptionSource::Dynamic;
        self
    }

    /// Sets the default value for single selection.
    #[must_use]
    pub fn default_single(mut self, value: impl Into<Key>) -> Self {
        self.default_single = Some(value.into());
        self
    }

    /// Sets the default values for multiple selection.
    #[must_use]
    pub fn default_multiple(mut self, values: impl IntoIterator<Item = impl Into<Key>>) -> Self {
        self.default_multiple = Some(values.into_iter().map(Into::into).collect());
        self
    }

    /// Enables search/filter for options.
    #[must_use]
    pub fn searchable(mut self) -> Self {
        self.searchable = true;
        self
    }

    /// Allows creation of new options by the user.
    #[must_use]
    pub fn creatable(mut self) -> Self {
        self.creatable = true;
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

    /// Builds the select parameter.
    #[must_use]
    pub fn build(self) -> Select {
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

        Select {
            metadata: metadata_builder.build(),
            flags: self.flags,
            selection_mode: self.selection_mode,
            option_source: self.option_source,
            options: self.options,
            default_single: self.default_single,
            default_multiple: self.default_multiple,
            searchable: self.searchable,
            creatable: self.creatable,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_single_minimal() {
        let select = Select::single("method").build();

        assert_eq!(select.key(), "method");
        assert_eq!(select.kind(), NodeKind::Leaf);
        assert_eq!(select.selection_mode(), SelectionMode::Single);
        assert!(select.default_value().is_none());
    }

    #[test]
    fn test_select_single_with_options() {
        let select = Select::single("method")
            .label("HTTP Method")
            .options(vec![
                SelectOption::simple("GET"),
                SelectOption::simple("POST"),
                SelectOption::new("PUT", "Update"),
            ])
            .default_single("GET")
            .build();

        assert_eq!(select.key(), "method");
        assert_eq!(select.metadata().label(), Some("HTTP Method"));
        assert_eq!(select.options().len(), 3);
        assert_eq!(select.default_single(), Some(&Key::from("GET")));
        assert_eq!(select.default_value(), Some(Value::text("GET")));
    }

    #[test]
    fn test_select_multiple() {
        let select = Select::multiple("tags")
            .label("Tags")
            .options(vec![
                SelectOption::simple("urgent"),
                SelectOption::simple("bug"),
                SelectOption::simple("feature"),
            ])
            .default_multiple(["urgent", "bug"])
            .build();

        assert_eq!(select.selection_mode(), SelectionMode::Multiple);
        assert_eq!(
            select.default_multiple(),
            Some(["urgent", "bug"].map(Key::from).as_slice())
        );

        let default = select.default_value().unwrap();
        let expected = Value::array(vec![Value::text("urgent"), Value::text("bug")]);
        assert_eq!(default, expected);
    }

    #[test]
    fn test_select_option_builder() {
        let option = SelectOption::new("us", "United States")
            .with_description("USA")
            .with_icon("flag-us")
            .with_group("North America");

        assert_eq!(option.value.as_str(), "us");
        assert_eq!(option.label.as_str(), "United States");
        assert_eq!(option.description.as_deref(), Some("USA"));
        assert_eq!(option.icon.as_deref(), Some("flag-us"));
        assert_eq!(option.group.as_deref(), Some("North America"));
    }

    #[test]
    fn test_select_dynamic() {
        let select = Select::single("database")
            .label("Database")
            .dynamic()
            .searchable()
            .build();

        assert!(matches!(select.option_source(), OptionSource::Dynamic));
        assert!(select.is_searchable());
        assert!(select.options().is_empty());
    }

    #[test]
    fn test_select_creatable() {
        let select = Select::multiple("custom_tags")
            .creatable()
            .searchable()
            .build();

        assert!(select.is_creatable());
        assert!(select.is_searchable());
    }
}
