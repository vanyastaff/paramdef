//! `OpenAPI` 3.0 specification generation.
//!
//! Generates `OpenAPI` 3.0 specifications from paramdef schemas.
//!
//! # Example
//!
//! ```rust,ignore
//! use paramdef::export::OpenApiGenerator;
//! use paramdef::schema::Schema;
//!
//! let schema = Schema::builder()
//!     .parameter(Text::builder("name").build())
//!     .build();
//!
//! let generator = OpenApiGenerator::new()
//!     .with_title("User API")
//!     .with_version("1.0.0");
//!
//! let spec = generator.generate(&schema)?;
//! ```

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

use crate::schema::Schema;

/// `OpenAPI` 3.0 specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiSpec {
    /// `OpenAPI` version.
    pub openapi: String,

    /// API metadata.
    pub info: ApiInfo,

    /// API paths.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<HashMap<String, PathItem>>,

    /// Reusable schema components.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Components>,
}

/// API information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiInfo {
    /// API title.
    pub title: String,

    /// API version.
    pub version: String,

    /// API description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Path item in `OpenAPI`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathItem {
    /// GET operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<Operation>,

    /// POST operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Operation>,

    /// PUT operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<Operation>,

    /// DELETE operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Operation>,
}

/// HTTP operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::struct_field_names)]
pub struct Operation {
    /// Operation summary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// Operation ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "operationId")]
    pub operation_id: Option<String>,

    /// Request body.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "requestBody")]
    pub request_body: Option<RequestBody>,

    /// Responses.
    pub responses: HashMap<String, Response>,
}

/// Request body.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestBody {
    /// Required flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    /// Content types.
    pub content: HashMap<String, MediaType>,
}

/// Media type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaType {
    /// Schema reference.
    pub schema: JsonValue,
}

/// Response object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    /// Response description.
    pub description: String,

    /// Response content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<HashMap<String, MediaType>>,
}

/// Reusable components.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Components {
    /// Schema definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<HashMap<String, JsonValue>>,
}

/// `OpenAPI` specification generator.
#[derive(Debug, Clone)]
pub struct OpenApiGenerator {
    title: String,
    version: String,
    description: Option<String>,
}

impl OpenApiGenerator {
    /// Creates a new generator.
    #[must_use]
    pub fn new() -> Self {
        Self {
            title: "API".to_string(),
            version: "1.0.0".to_string(),
            description: None,
        }
    }

    /// Sets the API title.
    #[must_use]
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Sets the API version.
    #[must_use]
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    /// Sets the API description.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Generates `OpenAPI` specification from schema.
    ///
    /// Converts paramdef Schema into `OpenAPI` 3.0 with:
    /// - POST endpoint for creating resources
    /// - Schema component with all parameters
    ///
    /// # Errors
    ///
    /// Returns error if schema cannot be converted.
    pub fn generate(
        &self,
        schema: &Schema,
    ) -> Result<OpenApiSpec, super::json_schema::ExportError> {
        use super::json_schema::JsonSchemaExporter;

        // Generate JSON Schema for the request body
        let json_exporter = JsonSchemaExporter::new()
            .with_title(&self.title)
            .with_description(self.description.as_deref().unwrap_or(""));

        let json_schema = json_exporter.export(schema)?;

        // Convert to JSON Value for OpenAPI components
        let schema_json = serde_json::to_value(&json_schema)
            .map_err(|e| super::json_schema::ExportError::Serialization(e.to_string()))?;

        // Create components with schema
        let mut schemas = HashMap::new();
        schemas.insert(self.title.clone(), schema_json);

        let components = Components {
            schemas: Some(schemas),
        };

        // Create POST operation for resource creation
        let post_op = Operation {
            summary: Some(format!("Create {}", self.title)),
            operation_id: Some(format!("create{}", self.title.replace(' ', ""))),
            request_body: Some(RequestBody {
                required: Some(true),
                content: {
                    let mut content = HashMap::new();
                    content.insert(
                        "application/json".to_string(),
                        MediaType {
                            schema: serde_json::json!({
                                "$ref": format!("#/components/schemas/{}", self.title)
                            }),
                        },
                    );
                    content
                },
            }),
            responses: {
                let mut responses = HashMap::new();
                responses.insert(
                    "201".to_string(),
                    Response {
                        description: "Created successfully".to_string(),
                        content: Some({
                            let mut content = HashMap::new();
                            content.insert(
                                "application/json".to_string(),
                                MediaType {
                                    schema: serde_json::json!({
                                        "$ref": format!("#/components/schemas/{}", self.title)
                                    }),
                                },
                            );
                            content
                        }),
                    },
                );
                responses.insert(
                    "400".to_string(),
                    Response {
                        description: "Bad request".to_string(),
                        content: None,
                    },
                );
                responses
            },
        };

        // Create path item
        let path_item = PathItem {
            get: None,
            post: Some(post_op),
            put: None,
            delete: None,
        };

        // Create paths
        let mut paths = HashMap::new();
        let path_key = format!("/{}", self.title.to_lowercase().replace(' ', "-"));
        paths.insert(path_key, path_item);

        Ok(OpenApiSpec {
            openapi: "3.0.3".to_string(),
            info: ApiInfo {
                title: self.title.clone(),
                version: self.version.clone(),
                description: self.description.clone(),
            },
            paths: Some(paths),
            components: Some(components),
        })
    }
}

impl Default for OpenApiGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_builder() {
        let generator = OpenApiGenerator::new()
            .with_title("User API")
            .with_version("2.0.0")
            .with_description("API for user management");

        assert_eq!(generator.title, "User API");
        assert_eq!(generator.version, "2.0.0");
        assert_eq!(
            generator.description,
            Some("API for user management".to_string())
        );
    }
}
