//! Ergonomics tests for Phase 4 (User Story 2)
//!
//! Tests for API improvements:
//! - Builder validation shortcuts
//! - Context error recovery
//! - ValidationError paths
//! - ObjectBuilder bulk operations
//! - ValueBuilder

#[cfg(feature = "validation")]
mod validation_shortcuts {
    use paramdef::prelude::*;

    /// T031: Test TextBuilder validation shortcuts
    #[test]
    fn test_text_builder_validate_required() {
        let text = Text::builder("email").validate_required().build();

        // Verify rule was added
        let rules = text.rules();
        assert_eq!(rules.len(), 1);
        // Rule checking requires context, but we can verify it was added
    }

    #[test]
    fn test_text_builder_validate_email() {
        let text = Text::builder("email").validate_email().build();

        let rules = text.rules();
        assert_eq!(rules.len(), 1);
    }

    #[test]
    fn test_text_builder_validate_min_length() {
        let text = Text::builder("username").validate_min_length(3).build();

        let rules = text.rules();
        assert_eq!(rules.len(), 1);
    }

    #[test]
    fn test_text_builder_validate_max_length() {
        let text = Text::builder("bio").validate_max_length(500).build();

        let rules = text.rules();
        assert_eq!(rules.len(), 1);
    }

    #[test]
    fn test_text_builder_validation_chaining() {
        let text = Text::builder("email")
            .validate_required()
            .validate_email()
            .validate_min_length(5)
            .build();

        let rules = text.rules();
        assert_eq!(rules.len(), 3);
    }

    /// T033: Test NumberBuilder validation shortcuts
    #[test]
    fn test_number_builder_validate_required() {
        let number = Number::builder("age").validate_required().build();

        let rules = number.rules();
        assert_eq!(rules.len(), 1);
    }

    #[test]
    fn test_number_builder_validate_min() {
        let number = Number::builder("age").validate_min(0.0).build();

        let rules = number.rules();
        assert_eq!(rules.len(), 1);
    }

    #[test]
    fn test_number_builder_validate_max() {
        let number = Number::builder("percentage").validate_max(100.0).build();

        let rules = number.rules();
        assert_eq!(rules.len(), 1);
    }

    #[test]
    fn test_number_builder_validation_chaining() {
        let number = Number::builder("age")
            .validate_required()
            .validate_min(0.0)
            .validate_max(120.0)
            .build();

        let rules = number.rules();
        assert_eq!(rules.len(), 3);
    }

    /// T035: Test BooleanBuilder validation shortcuts
    #[test]
    fn test_boolean_builder_validate_required() {
        let boolean = Boolean::builder("terms_accepted")
            .validate_required()
            .build();

        let rules = boolean.rules();
        assert_eq!(rules.len(), 1);
    }
}

/// T037: Test Context error recovery methods
mod error_recovery {
    use paramdef::prelude::*;

    #[test]
    fn test_context_get_text_or_with_existing_value() {
        let schema = Schema::builder()
            .node(Text::builder("name").default("Alice").build())
            .build();

        let ctx = Context::from_schema(schema);

        let name = ctx.get_text_or("name", "Unknown");
        assert_eq!(name, "Alice");
    }

    #[test]
    fn test_context_get_text_or_with_missing_key() {
        let schema = Schema::builder().build();
        let ctx = Context::from_schema(schema);

        let name = ctx.get_text_or("missing", "Default");
        assert_eq!(name, "Default");
    }

    #[test]
    fn test_context_get_text_or_with_wrong_type() {
        let schema = Schema::builder()
            .node(Number::builder("count").default(42.0).build())
            .build();

        let ctx = Context::from_schema(schema);

        // Wrong type (number instead of text) should return default
        let value = ctx.get_text_or("count", "Default");
        assert_eq!(value, "Default");
    }

    #[test]
    fn test_context_get_int_or() {
        let schema = Schema::builder()
            .node(Number::builder("count").default(42.0).build())
            .build();

        let ctx = Context::from_schema(schema);

        assert_eq!(ctx.get_int_or("count", 0), 42);
        assert_eq!(ctx.get_int_or("missing", 99), 99);
    }

    #[test]
    fn test_context_get_bool_or() {
        let schema = Schema::builder()
            .node(Boolean::builder("enabled").default(true).build())
            .build();

        let ctx = Context::from_schema(schema);

        assert_eq!(ctx.get_bool_or("enabled", false), true);
        assert_eq!(ctx.get_bool_or("missing", false), false);
    }

    #[test]
    fn test_context_get_float_or() {
        let schema = Schema::builder()
            .node(Number::builder("pi").default(3.14159).build())
            .build();

        let ctx = Context::from_schema(schema);

        assert_eq!(ctx.get_float_or("pi", 0.0), 3.14159);
        assert_eq!(ctx.get_float_or("missing", 1.0), 1.0);
    }
}

/// T039: Test ValidationError path support
#[cfg(feature = "validation")]
mod validation_error_paths {
    use paramdef::event::ValidationError;

    #[test]
    fn test_validation_error_with_path() {
        let err = ValidationError::new(
            "address.city".into(),
            "city".into(),
            "required".into(),
            "City is required".into(),
        );

        assert_eq!(err.path(), "address.city");
        assert_eq!(err.field(), "city");
    }

    #[test]
    fn test_validation_error_simple_top_level() {
        let err = ValidationError::simple(
            "email".into(),
            "email".into(),
            "Invalid email format".into(),
        );

        assert_eq!(err.path(), "email");
        assert_eq!(err.field(), "email");
    }

    #[test]
    fn test_validation_error_nested_object_path() {
        let err = ValidationError::new(
            "user.profile.bio".into(),
            "bio".into(),
            "max_length".into(),
            "Bio exceeds 500 characters".into(),
        );

        assert_eq!(err.path(), "user.profile.bio");
        assert_eq!(err.field(), "bio");
    }

    #[test]
    fn test_validation_error_constructors() {
        let required = ValidationError::required("name");
        assert_eq!(required.code(), "required");

        let min_length = ValidationError::min_length("username", 3);
        assert_eq!(min_length.code(), "min_length");

        let max_length = ValidationError::max_length("bio", 500);
        assert_eq!(max_length.code(), "max_length");
    }
}

/// T042: Test ObjectBuilder.fields() bulk method
mod object_builder {
    use paramdef::prelude::*;
    use std::sync::Arc;

    #[test]
    fn test_object_builder_fields_bulk_add() {
        use paramdef::types::Node;

        let fields: Vec<(&str, Arc<dyn Node>)> = vec![
            ("name", Arc::new(Text::builder("name").build())),
            ("age", Arc::new(Number::builder("age").build())),
            ("active", Arc::new(Boolean::builder("active").build())),
        ];

        let obj = Object::builder("user").fields(fields).build();

        assert_eq!(obj.children().len(), 3);
    }

    #[test]
    fn test_object_builder_fields_empty_iter() {
        use paramdef::types::Node;

        let fields: Vec<(&str, Arc<dyn Node>)> = vec![];

        let obj = Object::builder("empty").fields(fields).build();

        assert_eq!(obj.children().len(), 0);
    }

    #[test]
    fn test_object_builder_fields_mixed_types() {
        use paramdef::types::Node;

        let fields = vec![
            (
                "text",
                Arc::new(Text::builder("text").build()) as Arc<dyn Node>,
            ),
            (
                "num",
                Arc::new(Number::builder("num").build()) as Arc<dyn Node>,
            ),
            (
                "bool",
                Arc::new(Boolean::builder("bool").build()) as Arc<dyn Node>,
            ),
        ];

        let obj = Object::builder("mixed").fields(fields).build();

        assert_eq!(obj.children().len(), 3);
    }
}

/// T044: Test ValueBuilder (using existing ObjectBuilder API)
mod value_builder {
    use paramdef::core::{Key, Value};

    #[test]
    fn test_value_builder_empty_object() {
        let value = Value::build_object().build();

        match value {
            Value::Object(map) => assert_eq!(map.len(), 0),
            _ => panic!("Expected Object variant"),
        }
    }

    #[test]
    fn test_value_builder_single_field() {
        let value = Value::build_object()
            .field("name", Value::text("Alice"))
            .build();

        match value {
            Value::Object(map) => {
                assert_eq!(map.len(), 1);
                assert_eq!(map.get(&Key::from("name")), Some(&Value::text("Alice")));
            }
            _ => panic!("Expected Object variant"),
        }
    }

    #[test]
    fn test_value_builder_multiple_fields_chained() {
        let value = Value::build_object()
            .field("name", Value::text("Bob"))
            .field("age", Value::number(30.0))
            .field("active", Value::boolean(true))
            .build();

        match value {
            Value::Object(map) => {
                assert_eq!(map.len(), 3);
                assert_eq!(map.get(&Key::from("name")), Some(&Value::text("Bob")));
                assert_eq!(map.get(&Key::from("age")), Some(&Value::number(30.0)));
                assert_eq!(map.get(&Key::from("active")), Some(&Value::boolean(true)));
            }
            _ => panic!("Expected Object variant"),
        }
    }

    #[test]
    fn test_value_builder_fields_bulk() {
        use indexmap::IndexMap;

        let mut bulk = IndexMap::new();
        bulk.insert(Key::from("x"), Value::number(1.0));
        bulk.insert(Key::from("y"), Value::number(2.0));

        let value = Value::build_object().fields(bulk).build();

        match value {
            Value::Object(map) => {
                assert_eq!(map.len(), 2);
                assert_eq!(map.get(&Key::from("x")), Some(&Value::number(1.0)));
                assert_eq!(map.get(&Key::from("y")), Some(&Value::number(2.0)));
            }
            _ => panic!("Expected Object variant"),
        }
    }

    #[test]
    fn test_value_builder_field_if_conditional() {
        let include_age = true;
        let value = Value::build_object()
            .field("name", Value::text("Charlie"))
            .field_if(include_age, "age", Value::number(25.0))
            .build();

        match value {
            Value::Object(map) => {
                assert_eq!(map.len(), 2);
                assert!(map.contains_key(&Key::from("age")));
            }
            _ => panic!("Expected Object variant"),
        }

        // Test with condition false
        let include_age = false;
        let value = Value::build_object()
            .field("name", Value::text("Charlie"))
            .field_if(include_age, "age", Value::number(25.0))
            .build();

        match value {
            Value::Object(map) => {
                assert_eq!(map.len(), 1);
                assert!(!map.contains_key(&Key::from("age")));
            }
            _ => panic!("Expected Object variant"),
        }
    }

    #[test]
    fn test_value_builder_nested_objects() {
        let address = Value::build_object()
            .field("street", Value::text("123 Main St"))
            .field("city", Value::text("Springfield"))
            .build();

        let user = Value::build_object()
            .field("name", Value::text("Dave"))
            .field("address", address)
            .build();

        match user {
            Value::Object(map) => {
                assert_eq!(map.len(), 2);
                match map.get(&Key::from("address")) {
                    Some(Value::Object(addr_map)) => {
                        assert_eq!(addr_map.len(), 2);
                    }
                    _ => panic!("Expected nested Object"),
                }
            }
            _ => panic!("Expected Object variant"),
        }
    }
}
