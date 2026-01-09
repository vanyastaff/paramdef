//! Schema export functionality for JSON Schema and `OpenAPI`.
//!
//! This module provides conversion from paramdef schemas to standard formats
//! like JSON Schema (draft 2020-12) and `OpenAPI` 3.0.
//!
//! # Features
//!
//! Requires the `serde` feature flag.
//!
//! # Example
//!
//! ```rust,ignore
//! use paramdef::schema::Schema;
//! use paramdef::export::JsonSchemaExporter;
//!
//! let schema = Schema::builder()
//!     .parameter(Text::builder("name").required().build())
//!     .build();
//!
//! let json_schema = JsonSchemaExporter::new().export(&schema)?;
//! println!("{}", serde_json::to_string_pretty(&json_schema)?);
//! ```

#[cfg(feature = "serde")]
mod json_schema;

#[cfg(feature = "serde")]
mod openapi;

#[cfg(feature = "serde")]
pub use json_schema::{ExportError, JsonSchema, JsonSchemaExporter};

#[cfg(feature = "serde")]
pub use openapi::{OpenApiGenerator, OpenApiSpec};
