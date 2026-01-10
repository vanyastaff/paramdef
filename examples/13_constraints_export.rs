//! Example demonstrating export of built-in constraints to JSON Schema.
//!
//! Shows how Text subtypes export as format + pattern,
//! and Number subtypes export with minimum/maximum ranges.

use paramdef::export::JsonSchemaExporter;
use paramdef::schema::Schema;
use paramdef::types::leaf::{NumberBuilder, Text};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create schema with various constrained types
    let schema = Schema::builder()
        .parameter(
            Text::builder("email")
                .subtype(paramdef::subtype::text::Email)
                .label("Email Address")
                .required()
                .build(),
        )
        .parameter(
            Text::builder("website")
                .subtype(paramdef::subtype::text::Url)
                .label("Website URL")
                .build(),
        )
        .parameter(
            Text::builder("user_id")
                .subtype(paramdef::subtype::text::Uuid)
                .label("User ID")
                .build(),
        )
        .parameter(
            NumberBuilder::new("port", paramdef::subtype::number::Port)
                .label("Server Port")
                .required()
                .build(),
        )
        .parameter(
            NumberBuilder::new("rating", paramdef::subtype::number::Rating)
                .label("Product Rating")
                .build(),
        )
        .parameter(
            NumberBuilder::new("completion", paramdef::subtype::number::Percentage)
                .label("Completion Percentage")
                .build(),
        )
        .parameter(
            NumberBuilder::new("latitude", paramdef::subtype::number::Latitude)
                .label("Latitude")
                .build(),
        )
        .parameter(
            NumberBuilder::new("factor", paramdef::subtype::number::Factor)
                .label("Scale Factor")
                .build(),
        )
        .build();

    // Export to JSON Schema
    let exporter = JsonSchemaExporter::new()
        .with_title("Constrained Types Demo")
        .with_description("Demonstrates constraint export: format for Text, min/max for Number");

    let json_schema = exporter.export(&schema)?;

    println!("{}", serde_json::to_string_pretty(&json_schema)?);

    println!("\n=== Constraints Summary ===");
    println!("Text constraints:");
    println!("  - email: format=email, pattern");
    println!("  - website: format=uri, pattern");
    println!("  - user_id: format=uuid, pattern");
    println!("\nNumber constraints:");
    println!("  - port: minimum=1, maximum=65535");
    println!("  - rating: minimum=1, maximum=5");
    println!("  - completion: minimum=0, maximum=100");
    println!("  - latitude: minimum=-90, maximum=90");
    println!("  - factor: minimum=0, maximum=1");

    Ok(())
}
