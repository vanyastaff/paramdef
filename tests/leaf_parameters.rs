//! Integration tests for leaf parameter types.

use paramdef::core::{Flags, Value};
use paramdef::subtype::{Email, NumericKind, Password, Percentage, Port, TextSubtype};
use paramdef::types::kind::NodeKind;
use paramdef::types::leaf::{Boolean, Number, Select, SelectOption, SelectionMode, Text, Vector};
use paramdef::types::traits::{Leaf, Node};

#[test]
fn test_text_parameter_lifecycle() {
    // Create a text parameter with all options
    let username = Text::builder("username")
        .label("Username")
        .description("Enter your username")
        .default("guest")
        .required()
        .build();

    // Verify Node trait
    assert_eq!(username.key(), "username");
    assert_eq!(username.kind(), NodeKind::Leaf);
    assert_eq!(username.metadata().label(), Some("Username"));
    assert_eq!(
        username.metadata().description(),
        Some("Enter your username")
    );

    // Verify Leaf trait
    assert_eq!(username.default_value(), Some(Value::text("guest")));

    // Verify flags
    assert!(username.flags().contains(Flags::REQUIRED));
}

#[test]
fn test_text_subtypes() {
    // Email subtype
    let email: Text<Email> = Text::email("contact");
    assert_eq!(email.key(), "contact");
    assert!(!Email::is_sensitive());

    // Password subtype with sensitive flag
    let password: Text<Password> = Text::password("secret");
    assert!(password.flags().contains(Flags::SENSITIVE));
    assert!(Password::is_sensitive());
}

#[test]
fn test_number_parameter_lifecycle() {
    // Create a number parameter
    let age = Number::builder("age")
        .label("Age")
        .default(25.0)
        .required()
        .build();

    // Verify Node trait
    assert_eq!(age.key(), "age");
    assert_eq!(age.kind(), NodeKind::Leaf);

    // Verify Leaf trait
    assert_eq!(age.default_value(), Some(Value::Float(25.0)));

    // Verify accessors
    assert_eq!(age.default_f64(), Some(25.0));
    assert_eq!(age.default_i64(), Some(25));
}

#[test]
fn test_number_subtypes() {
    // Port subtype (integer-only)
    let port: Number<Port> = Number::port("server_port").build();
    assert_eq!(port.key(), "server_port");

    // Percentage subtype (float-only)
    let opacity: Number<Percentage> = Number::percentage("opacity").build();
    assert_eq!(opacity.key(), "opacity");
}

#[test]
fn test_boolean_parameter_lifecycle() {
    // Create a boolean parameter
    let enabled = Boolean::builder("enabled")
        .label("Enable Feature")
        .default(true)
        .build();

    // Verify Node trait
    assert_eq!(enabled.key(), "enabled");
    assert_eq!(enabled.kind(), NodeKind::Leaf);

    // Verify Leaf trait
    assert_eq!(enabled.default_value(), Some(Value::Bool(true)));

    // Verify accessor
    assert_eq!(enabled.default_bool(), Some(true));
}

#[test]
fn test_vector_parameter_lifecycle() {
    // Create a 3D vector parameter
    let position = Vector::builder::<f64, 3>("position")
        .label("Position")
        .default([1.0, 2.0, 3.0])
        .build();

    // Verify Node trait
    assert_eq!(position.key(), "position");
    assert_eq!(position.kind(), NodeKind::Leaf);

    // Verify vector-specific properties
    assert_eq!(position.size(), 3);
    assert_eq!(position.element_type(), NumericKind::F64);
    assert_eq!(position.default_vec(), Some([1.0, 2.0, 3.0].as_slice()));

    // Verify Leaf trait returns array
    let value = position.default_value().unwrap();
    assert_eq!(
        value,
        Value::array(vec![
            Value::Float(1.0),
            Value::Float(2.0),
            Value::Float(3.0)
        ])
    );
}

#[test]
fn test_vector_different_sizes() {
    // 2D vector
    let uv = Vector::builder::<f64, 2>("uv").default([0.5, 0.5]).build();
    assert_eq!(uv.size(), 2);

    // 4D vector (e.g., quaternion or RGBA)
    let color = Vector::builder::<f64, 4>("color")
        .default([1.0, 0.0, 0.0, 1.0])
        .build();
    assert_eq!(color.size(), 4);
}

#[test]
fn test_vector_integer_elements() {
    // Integer vector
    let grid_pos = Vector::builder::<i32, 3>("grid_pos")
        .default([10, 20, 30])
        .build();

    assert_eq!(grid_pos.element_type(), NumericKind::I32);
    assert_eq!(grid_pos.size(), 3);
}

#[test]
fn test_select_single_selection() {
    // Single selection
    let method = Select::single("method")
        .label("HTTP Method")
        .options(vec![
            SelectOption::simple("GET"),
            SelectOption::simple("POST"),
            SelectOption::new("PUT", "Update"),
            SelectOption::new("DELETE", "Remove"),
        ])
        .default_single("GET")
        .build();

    // Verify Node trait
    assert_eq!(method.key(), "method");
    assert_eq!(method.kind(), NodeKind::Leaf);

    // Verify selection mode
    assert_eq!(method.selection_mode(), SelectionMode::Single);

    // Verify options
    assert_eq!(method.options().len(), 4);

    // Verify Leaf trait
    assert_eq!(method.default_value(), Some(Value::text("GET")));
}

#[test]
fn test_select_multiple_selection() {
    // Multiple selection
    let tags = Select::multiple("tags")
        .label("Tags")
        .options(vec![
            SelectOption::simple("urgent"),
            SelectOption::simple("bug"),
            SelectOption::simple("feature"),
            SelectOption::simple("docs"),
        ])
        .default_multiple(["urgent", "bug"])
        .searchable()
        .build();

    // Verify selection mode
    assert_eq!(tags.selection_mode(), SelectionMode::Multiple);
    assert!(tags.is_searchable());

    // Verify Leaf trait returns array
    let value = tags.default_value().unwrap();
    assert_eq!(
        value,
        Value::array(vec![Value::text("urgent"), Value::text("bug")])
    );
}

#[test]
fn test_select_with_rich_options() {
    let country = Select::single("country")
        .options(vec![
            SelectOption::new("us", "United States")
                .with_description("USA")
                .with_icon("flag-us")
                .with_group("North America"),
            SelectOption::new("ca", "Canada")
                .with_description("CAN")
                .with_icon("flag-ca")
                .with_group("North America"),
            SelectOption::new("uk", "United Kingdom")
                .with_description("UK")
                .with_icon("flag-uk")
                .with_group("Europe"),
        ])
        .searchable()
        .build();

    let options = country.options();
    assert_eq!(options.len(), 3);

    // Check first option has all metadata
    assert_eq!(options[0].value.as_str(), "us");
    assert_eq!(options[0].label.as_str(), "United States");
    assert_eq!(options[0].description.as_deref(), Some("USA"));
    assert_eq!(options[0].icon.as_deref(), Some("flag-us"));
    assert_eq!(options[0].group.as_deref(), Some("North America"));
}

#[test]
fn test_parameter_flags() {
    // Test various flag combinations
    let readonly_text = Text::builder("id").readonly().build();
    assert!(readonly_text.flags().contains(Flags::READONLY));

    let hidden_number = Number::builder("internal_id").hidden().build();
    assert!(hidden_number.flags().contains(Flags::HIDDEN));

    let required_bool = Boolean::builder("agree").required().build();
    assert!(required_bool.flags().contains(Flags::REQUIRED));
}

#[test]
fn test_parameter_without_defaults() {
    // Parameters without default values
    let text = Text::builder("name").build();
    assert!(text.default_value().is_none());

    let number = Number::builder("count").build();
    assert!(number.default_value().is_none());

    let boolean = Boolean::builder("flag").build();
    assert!(boolean.default_value().is_none());

    let vector = Vector::builder::<f64, 3>("pos").build();
    assert!(vector.default_value().is_none());

    let select = Select::single("choice").build();
    assert!(select.default_value().is_none());
}

#[test]
fn test_metadata_propagation() {
    // Verify metadata is correctly set through builders
    let param = Text::builder("test")
        .label("Test Label")
        .description("Test Description")
        .group("Test Group")
        .build();

    let meta = param.metadata();
    assert_eq!(meta.key(), "test");
    assert_eq!(meta.label(), Some("Test Label"));
    assert_eq!(meta.description(), Some("Test Description"));
    assert_eq!(meta.group(), Some("Test Group"));
}
