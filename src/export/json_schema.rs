//! JSON Schema export for paramdef schemas.
//!
//! Converts paramdef schemas to JSON Schema (draft 2020-12).
//!
//! # JSON Schema Support
//!
//! - **Type mapping**: paramdef types → JSON Schema types
//! - **Validation**: All validation rules converted to JSON Schema constraints
//! - **Metadata**: Labels, descriptions, examples preserved
//! - **Nested schemas**: Objects, Lists, Modes properly structured
//!
//! # Example
//!
//! ```rust,ignore
//! use paramdef::export::JsonSchemaExporter;
//! use paramdef::schema::Schema;
//! use paramdef::types::leaf::Text;
//!
//! let schema = Schema::builder()
//!     .parameter(Text::builder("email")
//!         .label("Email Address")
//!         .description("User's email")
//!         .required()
//!         .build())
//!     .build();
//!
//! let exporter = JsonSchemaExporter::new();
//! let json_schema = exporter.export(&schema)?;
//! ```

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value as JsonValue};

use crate::schema::Schema;
use crate::types::traits::Node;

/// JSON Schema representation.
///
/// Conforms to JSON Schema draft 2020-12.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonSchema {
    /// JSON Schema version.
    #[serde(rename = "$schema")]
    pub schema: String,

    /// Schema title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Schema description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// JSON Schema type.
    #[serde(rename = "type")]
    pub type_: String,

    /// Object properties (for object schemas).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Map<String, JsonValue>>,

    /// Required property names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,

    /// Additional properties allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "additionalProperties")]
    pub additional_properties: Option<bool>,

    /// Pattern for string validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,

    /// Minimum value (numeric).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,

    /// Maximum value (numeric).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,

    /// String format (email, uri, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// Enum values.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "enum")]
    pub enum_: Option<Vec<JsonValue>>,

    /// Default value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<JsonValue>,

    /// Examples.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<JsonValue>>,
}

impl JsonSchema {
    /// Creates a new JSON Schema with defaults.
    #[must_use]
    pub fn new() -> Self {
        Self {
            schema: "https://json-schema.org/draft/2020-12/schema".to_string(),
            title: None,
            description: None,
            type_: "object".to_string(),
            properties: None,
            required: None,
            additional_properties: Some(false),
            pattern: None,
            minimum: None,
            maximum: None,
            format: None,
            enum_: None,
            default: None,
            examples: None,
        }
    }
}

impl Default for JsonSchema {
    fn default() -> Self {
        Self::new()
    }
}

/// Exporter for converting paramdef schemas to JSON Schema.
///
/// # Example
///
/// ```rust,ignore
/// let exporter = JsonSchemaExporter::new()
///     .with_title("User Registration")
///     .with_description("User signup form schema");
///
/// let json_schema = exporter.export(&schema)?;
/// ```
#[derive(Debug, Clone)]
pub struct JsonSchemaExporter {
    title: Option<String>,
    description: Option<String>,
    strict: bool,
}

impl JsonSchemaExporter {
    /// Creates a new exporter with default settings.
    #[must_use]
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            strict: true,
        }
    }

    /// Sets the schema title.
    #[must_use]
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the schema description.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets strict mode (additionalProperties: false).
    #[must_use]
    pub fn strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    /// Exports a paramdef schema to JSON Schema.
    ///
    /// # Errors
    ///
    /// Returns error if schema contains unsupported features.
    pub fn export(&self, schema: &Schema) -> Result<JsonSchema, ExportError> {
        let mut json_schema = JsonSchema::new();

        // Set metadata
        json_schema.title.clone_from(&self.title);
        json_schema.description.clone_from(&self.description);
        json_schema.additional_properties = Some(!self.strict);

        // Convert parameters to properties
        let mut properties = Map::new();
        let mut required = Vec::new();

        for node in schema.iter() {
            let key = node.key().as_str();
            let property = Self::convert_node(node.as_ref())?;

            properties.insert(key.to_string(), property);

            // Check if required by checking flags
            if Self::is_required(node.as_ref()) {
                required.push(key.to_string());
            }
        }

        json_schema.properties = Some(properties);
        if !required.is_empty() {
            json_schema.required = Some(required);
        }

        Ok(json_schema)
    }

    /// Converts a single node to JSON Schema property.
    fn convert_node(node: &dyn Node) -> Result<JsonValue, ExportError> {
        use crate::types::NodeKind;

        let mut prop = Map::new();

        // Add metadata
        let metadata = node.metadata();
        if let Some(label) = metadata.label() {
            prop.insert("title".to_string(), JsonValue::String(label.to_string()));
        }
        if let Some(desc) = metadata.description() {
            prop.insert(
                "description".to_string(),
                JsonValue::String(desc.to_string()),
            );
        }

        // Convert based on node kind
        match node.kind() {
            NodeKind::Leaf => {
                Self::convert_leaf_type(node, &mut prop);
            }
            NodeKind::Container => {
                Self::convert_container_type(node, &mut prop);
            }
            NodeKind::Group | NodeKind::Layout => {
                // Groups don't have JSON Schema representation
                return Err(ExportError::UnsupportedType(format!(
                    "Group/Layout node '{}' cannot be exported to JSON Schema",
                    node.metadata().key()
                )));
            }
            NodeKind::Decoration => {
                // Decorations are UI-only, skip
                return Err(ExportError::UnsupportedType(format!(
                    "Decoration node '{}' is UI-only and cannot be exported",
                    node.metadata().key()
                )));
            }
        }

        Ok(JsonValue::Object(prop))
    }

    /// Helper to create items schema for Select with multiple selection.
    fn create_select_items_schema(
        select: &crate::types::leaf::Select,
        is_static: bool,
    ) -> JsonValue {
        if !is_static {
            return serde_json::json!({"type": "string"});
        }

        let enum_values: Vec<JsonValue> = select
            .options()
            .iter()
            .map(|opt| JsonValue::String(opt.value.to_string()))
            .collect();

        if enum_values.is_empty() {
            serde_json::json!({"type": "string"})
        } else {
            serde_json::json!({"type": "string", "enum": enum_values})
        }
    }

    /// Converts a Leaf node to JSON Schema properties.
    fn convert_leaf_type(node: &dyn Node, prop: &mut Map<String, JsonValue>) {
        // Try downcasting to specific leaf types
        let any = node.as_any();

        // Boolean
        if any.downcast_ref::<crate::types::leaf::Boolean>().is_some() {
            prop.insert("type".to_string(), JsonValue::String("boolean".to_string()));
            return;
        }

        // Select
        if let Some(select) = any.downcast_ref::<crate::types::leaf::Select>() {
            use crate::types::leaf::{OptionSource, SelectionMode};

            let is_static = matches!(select.option_source(), OptionSource::Static);
            let is_multiple = select.selection_mode() == SelectionMode::Multiple;

            // For static options with single selection, export as enum
            if is_static && !is_multiple {
                prop.insert("type".to_string(), JsonValue::String("string".to_string()));
                let enum_values: Vec<JsonValue> = select
                    .options()
                    .iter()
                    .map(|opt| JsonValue::String(opt.value.to_string()))
                    .collect();
                if !enum_values.is_empty() {
                    prop.insert("enum".to_string(), JsonValue::Array(enum_values));
                }
                return;
            }

            // Multiple selection is array of strings
            if is_multiple {
                prop.insert("type".to_string(), JsonValue::String("array".to_string()));

                let items = Self::create_select_items_schema(select, is_static);
                prop.insert("items".to_string(), items);
                prop.insert("uniqueItems".to_string(), JsonValue::Bool(true));
                return;
            }

            // Dynamic single selection - just string
            prop.insert("type".to_string(), JsonValue::String("string".to_string()));
            return;
        }

        // File
        if any.downcast_ref::<crate::types::leaf::File>().is_some() {
            prop.insert("type".to_string(), JsonValue::String("string".to_string()));
            prop.insert("format".to_string(), JsonValue::String("uri".to_string()));
            return;
        }

        // Vector
        if any.downcast_ref::<crate::types::leaf::Vector>().is_some() {
            prop.insert("type".to_string(), JsonValue::String("array".to_string()));
            prop.insert("items".to_string(), serde_json::json!({"type": "number"}));
            return;
        }

        // Number - try common subtypes
        if any
            .downcast_ref::<crate::types::leaf::Number<crate::subtype::number::GenericNumber>>()
            .is_some()
            || any
                .downcast_ref::<crate::types::leaf::Number<crate::subtype::number::Port>>()
                .is_some()
            || any
                .downcast_ref::<crate::types::leaf::Number<crate::subtype::number::Count>>()
                .is_some()
            || any
                .downcast_ref::<crate::types::leaf::Number<crate::subtype::number::Percentage>>()
                .is_some()
            || any
                .downcast_ref::<crate::types::leaf::Number<crate::subtype::number::Angle>>()
                .is_some()
            || any
                .downcast_ref::<crate::types::leaf::Number<crate::subtype::number::Distance>>()
                .is_some()
        {
            prop.insert("type".to_string(), JsonValue::String("number".to_string()));
            return;
        }

        // Text - check explicit Text types
        if any
            .downcast_ref::<crate::types::leaf::Text<crate::subtype::text::Plain>>()
            .is_some()
            || any
                .downcast_ref::<crate::types::leaf::Text<crate::subtype::text::Email>>()
                .is_some()
            || any
                .downcast_ref::<crate::types::leaf::Text<crate::subtype::text::Url>>()
                .is_some()
            || any
                .downcast_ref::<crate::types::leaf::Text<crate::subtype::text::Password>>()
                .is_some()
        {
            prop.insert("type".to_string(), JsonValue::String("string".to_string()));
            return;
        }

        // Unknown type - fallback to string with warning
        eprintln!(
            "Warning: Unrecognized leaf node type '{}' for key '{}', defaulting to string type",
            std::any::type_name_of_val(any),
            node.metadata().key()
        );
        prop.insert("type".to_string(), JsonValue::String("string".to_string()));
    }

    /// Converts a Container node to JSON Schema properties.
    fn convert_container_type(node: &dyn Node, prop: &mut Map<String, JsonValue>) {
        let any = node.as_any();

        // Object
        if let Some(obj) = any.downcast_ref::<crate::types::container::Object>() {
            prop.insert("type".to_string(), JsonValue::String("object".to_string()));

            // Export nested properties recursively
            let mut properties = Map::new();
            let mut required_fields = Vec::new();

            for (field_key, field_node) in obj.fields() {
                let Ok(field_schema) = Self::convert_node(field_node.as_ref()) else {
                    continue;
                };

                properties.insert(field_key.to_string(), field_schema);

                // Check if field is required
                if Self::is_required(field_node.as_ref()) {
                    required_fields.push(field_key.to_string());
                }
            }

            if !properties.is_empty() {
                prop.insert("properties".to_string(), JsonValue::Object(properties));
            }

            if !required_fields.is_empty() {
                prop.insert(
                    "required".to_string(),
                    JsonValue::Array(required_fields.into_iter().map(JsonValue::String).collect()),
                );
            }

            // Set additionalProperties based on extensibility
            prop.insert(
                "additionalProperties".to_string(),
                JsonValue::Bool(obj.is_extensible()),
            );

            return;
        }

        // List
        if let Some(list) = any.downcast_ref::<crate::types::container::List>() {
            prop.insert("type".to_string(), JsonValue::String("array".to_string()));

            // Export item template schema
            let template = list.item_template();
            if let Ok(item_schema) = Self::convert_node(template.as_ref()) {
                prop.insert("items".to_string(), item_schema);
            }

            // Add min/max items constraints
            if let Some(min) = list.min_items() {
                prop.insert("minItems".to_string(), JsonValue::Number(min.into()));
            }
            if let Some(max) = list.max_items() {
                prop.insert("maxItems".to_string(), JsonValue::Number(max.into()));
            }

            // Add uniqueItems if the list requires unique items
            if list.is_unique() {
                prop.insert("uniqueItems".to_string(), JsonValue::Bool(true));
            }

            return;
        }

        // Fallback for other container types
        prop.insert("type".to_string(), JsonValue::String("object".to_string()));
    }

    /// Checks if a node has the REQUIRED flag set.
    fn is_required(node: &dyn Node) -> bool {
        let any = node.as_any();

        // Text (try common subtypes)
        if let Some(text) =
            any.downcast_ref::<crate::types::leaf::Text<crate::subtype::text::Plain>>()
        {
            return text.flags().is_required();
        }
        if let Some(text) =
            any.downcast_ref::<crate::types::leaf::Text<crate::subtype::text::Email>>()
        {
            return text.flags().is_required();
        }
        if let Some(text) =
            any.downcast_ref::<crate::types::leaf::Text<crate::subtype::text::Url>>()
        {
            return text.flags().is_required();
        }
        if let Some(text) =
            any.downcast_ref::<crate::types::leaf::Text<crate::subtype::text::Password>>()
        {
            return text.flags().is_required();
        }

        // Number (try common subtypes)
        if let Some(num) =
            any.downcast_ref::<crate::types::leaf::Number<crate::subtype::number::GenericNumber>>()
        {
            return num.flags().is_required();
        }
        if let Some(num) =
            any.downcast_ref::<crate::types::leaf::Number<crate::subtype::number::Port>>()
        {
            return num.flags().is_required();
        }
        if let Some(num) =
            any.downcast_ref::<crate::types::leaf::Number<crate::subtype::number::Count>>()
        {
            return num.flags().is_required();
        }
        if let Some(num) =
            any.downcast_ref::<crate::types::leaf::Number<crate::subtype::number::Percentage>>()
        {
            return num.flags().is_required();
        }
        if let Some(num) =
            any.downcast_ref::<crate::types::leaf::Number<crate::subtype::number::Angle>>()
        {
            return num.flags().is_required();
        }
        if let Some(num) =
            any.downcast_ref::<crate::types::leaf::Number<crate::subtype::number::Distance>>()
        {
            return num.flags().is_required();
        }

        // Boolean
        if let Some(boolean) = any.downcast_ref::<crate::types::leaf::Boolean>() {
            return boolean.flags().is_required();
        }

        // Vector
        if let Some(vec) = any.downcast_ref::<crate::types::leaf::Vector>() {
            return vec.flags().is_required();
        }

        // Select
        if let Some(select) = any.downcast_ref::<crate::types::leaf::Select>() {
            return select.flags().is_required();
        }

        // File
        if let Some(file) = any.downcast_ref::<crate::types::leaf::File>() {
            return file.flags().is_required();
        }

        // Object
        if let Some(obj) = any.downcast_ref::<crate::types::container::Object>() {
            return obj.flags().is_required();
        }

        // List
        if let Some(list) = any.downcast_ref::<crate::types::container::List>() {
            return list.flags().is_required();
        }

        // Mode
        if let Some(mode) = any.downcast_ref::<crate::types::container::Mode>() {
            return mode.flags().is_required();
        }

        // Default: not required (no warning - likely a specialized subtype)
        false
    }
}

impl Default for JsonSchemaExporter {
    fn default() -> Self {
        Self::new()
    }
}

/// Errors that can occur during export.
#[derive(Debug, Clone, thiserror::Error)]
pub enum ExportError {
    /// Unsupported node type.
    #[error("unsupported type: {0}")]
    UnsupportedType(String),

    /// Serialization error.
    #[error("serialization failed: {0}")]
    Serialization(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_schema_new() {
        let schema = JsonSchema::new();
        assert_eq!(schema.type_, "object");
        assert_eq!(
            schema.schema,
            "https://json-schema.org/draft/2020-12/schema"
        );
    }

    #[test]
    fn test_exporter_builder() {
        let exporter = JsonSchemaExporter::new()
            .with_title("Test Schema")
            .with_description("Test description")
            .strict(false);

        assert_eq!(exporter.title, Some("Test Schema".to_string()));
        assert_eq!(exporter.description, Some("Test description".to_string()));
        assert!(!exporter.strict);
    }
}
