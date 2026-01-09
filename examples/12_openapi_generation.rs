//! Example demonstrating OpenAPI 3.0 specification generation.
//!
//! Shows how to:
//! - Generate OpenAPI 3.0 specs from paramdef schemas
//! - Create API endpoints with request/response schemas
//! - Use schema components for reusability

use paramdef::export::OpenApiGenerator;
use paramdef::schema::Schema;
use paramdef::types::leaf::{Boolean, Number, Text};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a schema representing a user
    let schema = Schema::builder()
        .parameter(
            Text::builder("email")
                .subtype(paramdef::subtype::Email)
                .label("Email Address")
                .description("User's email address")
                .required()
                .build(),
        )
        .parameter(
            Text::builder("username")
                .label("Username")
                .description("Unique username (3-20 characters)")
                .required()
                .build(),
        )
        .parameter(
            Text::builder("password")
                .subtype(paramdef::subtype::Password)
                .label("Password")
                .description("Account password (min 8 characters)")
                .required()
                .sensitive()
                .build(),
        )
        .parameter(
            Text::builder("full_name")
                .label("Full Name")
                .description("User's full name")
                .build(),
        )
        .parameter(
            Number::integer("age")
                .label("Age")
                .description("User's age in years")
                .build(),
        )
        .parameter(
            Boolean::builder("newsletter")
                .label("Subscribe to Newsletter")
                .description("Receive email updates")
                .default(false)
                .build(),
        )
        .build();

    // Generate OpenAPI 3.0 specification
    let generator = OpenApiGenerator::new()
        .with_title("User")
        .with_version("1.0.0")
        .with_description("User management API");

    let spec = generator.generate(&schema)?;

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&spec)?;
    println!("{json}");

    // Verify structure
    println!("\n=== OpenAPI Specification Info ===");
    println!("OpenAPI version: {}", spec.openapi);
    println!("API title: {}", spec.info.title);
    println!("API version: {}", spec.info.version);

    if let Some(paths) = &spec.paths {
        println!("Paths defined: {}", paths.len());
        for path in paths.keys() {
            println!("  - {}", path);
        }
    }

    if let Some(components) = &spec.components {
        if let Some(schemas) = &components.schemas {
            println!("Schema components: {}", schemas.len());
            for schema_name in schemas.keys() {
                println!("  - {}", schema_name);
            }
        }
    }

    Ok(())
}
