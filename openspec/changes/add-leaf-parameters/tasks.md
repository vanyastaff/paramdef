## 1. Project Structure

- [ ] 1.1 Create `src/parameter/mod.rs` module file
- [ ] 1.2 Create `src/parameter/text.rs` file
- [ ] 1.3 Create `src/parameter/number.rs` file
- [ ] 1.4 Create `src/parameter/boolean.rs` file
- [ ] 1.5 Create `src/parameter/vector.rs` file
- [ ] 1.6 Create `src/parameter/select.rs` file
- [ ] 1.7 Update `src/lib.rs` to export parameter module
- [ ] 1.8 Run `cargo check` to verify structure compiles

## 2. Text Parameter Implementation

- [ ] 2.1 Write failing test: `test_text_minimal`
- [ ] 2.2 Implement Text struct with metadata, subtype, value
- [ ] 2.3 Run test to verify it passes
- [ ] 2.4 Write failing test: `test_text_builder`
- [ ] 2.5 Implement TextBuilder with chainable methods
- [ ] 2.6 Run test to verify it passes
- [ ] 2.7 Add min_length, max_length, pattern fields
- [ ] 2.8 Add validators and transformers fields
- [ ] 2.9 Implement Leaf trait for Text
- [ ] 2.10 Write failing test: `test_text_email_convenience`
- [ ] 2.11 Implement Text::email() convenience constructor
- [ ] 2.12 Run test to verify it passes
- [ ] 2.13 Implement Text::url(), Text::password(), etc.
- [ ] 2.14 Add documentation
- [ ] 2.15 Commit: `feat(parameter): add Text type`

## 3. Number Parameter Implementation

- [ ] 3.1 Write failing test: `test_number_minimal`
- [ ] 3.2 Implement Number struct with metadata, subtype, unit, value
- [ ] 3.3 Run test to verify it passes
- [ ] 3.4 Write failing test: `test_number_builder`
- [ ] 3.5 Implement NumberBuilder with chainable methods
- [ ] 3.6 Run test to verify it passes
- [ ] 3.7 Add min, max, soft_min, soft_max, step fields
- [ ] 3.8 Add validators and transformers fields
- [ ] 3.9 Implement Leaf trait for Number
- [ ] 3.10 Write failing test: `test_number_integer_convenience`
- [ ] 3.11 Implement Number::integer() convenience constructor
- [ ] 3.12 Run test to verify it passes
- [ ] 3.13 Implement Number::float(), Number::percentage(), etc.
- [ ] 3.14 Add documentation
- [ ] 3.15 Commit: `feat(parameter): add Number type`

## 4. Boolean Parameter Implementation

- [ ] 4.1 Write failing test: `test_boolean_minimal`
- [ ] 4.2 Implement Boolean struct with metadata, value, default
- [ ] 4.3 Run test to verify it passes
- [ ] 4.4 Write failing test: `test_boolean_builder`
- [ ] 4.5 Implement BooleanBuilder
- [ ] 4.6 Run test to verify it passes
- [ ] 4.7 Implement Leaf trait for Boolean
- [ ] 4.8 Add documentation
- [ ] 4.9 Commit: `feat(parameter): add Boolean type`

## 5. Vector Parameter Implementation

- [ ] 5.1 Write failing test: `test_vector_minimal`
- [ ] 5.2 Implement Vector struct with metadata, subtype, value
- [ ] 5.3 Run test to verify it passes
- [ ] 5.4 Write failing test: `test_vector_builder`
- [ ] 5.5 Implement VectorBuilder with type-safe component setters
- [ ] 5.6 Run test to verify it passes
- [ ] 5.7 Add validators field
- [ ] 5.8 Implement Leaf trait for Vector
- [ ] 5.9 Write failing test: `test_vector_vector3_convenience`
- [ ] 5.10 Implement Vector::vector3() with default_vec3([f64; 3])
- [ ] 5.11 Run test to verify it passes
- [ ] 5.12 Implement Vector::color_rgba(), Vector::quaternion(), etc.
- [ ] 5.13 Add documentation
- [ ] 5.14 Commit: `feat(parameter): add Vector type`

## 6. Select Parameter Implementation

- [ ] 6.1 Write failing test: `test_select_single_static`
- [ ] 6.2 Implement Select struct with selection_mode, option_source
- [ ] 6.3 Implement SelectOption struct (value, label, icon, enabled)
- [ ] 6.4 Run test to verify it passes
- [ ] 6.5 Write failing test: `test_select_multiple_static`
- [ ] 6.6 Implement SelectionMode enum (Single, Multiple)
- [ ] 6.7 Run test to verify it passes
- [ ] 6.8 Write failing test: `test_select_dynamic_loader`
- [ ] 6.9 Implement OptionSource enum (Static, Dynamic)
- [ ] 6.10 Run test to verify it passes
- [ ] 6.11 Implement SelectBuilder
- [ ] 6.12 Implement Leaf trait for Select
- [ ] 6.13 Implement Select::single(), Select::multiple()
- [ ] 6.14 Add documentation
- [ ] 6.15 Commit: `feat(parameter): add Select type (unified)`

## 7. Validation Integration

- [ ] 7.1 Write failing test: `test_text_validation_sync`
- [ ] 7.2 Implement validate_sync() for Text
- [ ] 7.3 Run test to verify it passes
- [ ] 7.4 Write failing test: `test_text_validation_async`
- [ ] 7.5 Implement validate_async() for Text
- [ ] 7.6 Run test to verify it passes
- [ ] 7.7 Implement Validatable trait for all leaf types
- [ ] 7.8 Write tests for Number, Boolean, Vector, Select validation
- [ ] 7.9 Run tests to verify they pass
- [ ] 7.10 Commit: `feat(parameter): add validation integration`

## 8. Transformation Integration

- [ ] 8.1 Write failing test: `test_text_trim_transformer`
- [ ] 8.2 Implement transform() method for Text
- [ ] 8.3 Run test to verify it passes
- [ ] 8.4 Write failing test: `test_number_clamp_transformer`
- [ ] 8.5 Implement transform() method for Number
- [ ] 8.6 Run test to verify it passes
- [ ] 8.7 Add tests for transformer pipeline
- [ ] 8.8 Commit: `feat(parameter): add transformation support`

## 9. Type Safety Tests

- [ ] 9.1 Write failing test: `test_text_type_safe_getter`
- [ ] 9.2 Verify get_value() returns Option<&String>
- [ ] 9.3 Run test to verify it passes
- [ ] 9.4 Write failing test: `test_number_type_safe_setter`
- [ ] 9.5 Verify set_value(i64) works for integer Number
- [ ] 9.6 Run test to verify it passes
- [ ] 9.7 Write failing test: `test_vector_type_mismatch`
- [ ] 9.8 Verify from_value(Value::Text) fails on Vector
- [ ] 9.9 Run test to verify it passes
- [ ] 9.10 Commit: `test: add type safety tests for all leaf types`

## 10. Integration Tests

- [ ] 10.1 Write integration test: `test_complete_text_workflow`
- [ ] 10.2 Test create → set → transform → validate → get cycle
- [ ] 10.3 Run test to verify it passes
- [ ] 10.4 Write integration test: `test_complete_number_workflow`
- [ ] 10.5 Test with unit conversion
- [ ] 10.6 Run test to verify it passes
- [ ] 10.7 Write integration test: `test_complete_select_workflow`
- [ ] 10.8 Test single and multiple selection modes
- [ ] 10.9 Run test to verify it passes
- [ ] 10.10 Commit: `test: add integration tests for leaf parameters`

## 11. Documentation

- [ ] 11.1 Document Text API with examples
- [ ] 11.2 Document Number API with unit conversion examples
- [ ] 11.3 Document Boolean API
- [ ] 11.4 Document Vector API with component access
- [ ] 11.5 Document Select API with static/dynamic options
- [ ] 11.6 Add builder pattern examples
- [ ] 11.7 Commit: `docs: add leaf parameter documentation`

## 12. Final Verification

- [ ] 12.1 Run `cargo fmt --all`
- [ ] 12.2 Run `cargo clippy --workspace --all-features -- -D warnings`
- [ ] 12.3 Run `cargo test --workspace`
- [ ] 12.4 Run `cargo test --workspace --features validation`
- [ ] 12.5 Run `cargo doc --no-deps --all-features`
- [ ] 12.6 Run `cargo +1.85 check --workspace` (MSRV)
- [ ] 12.7 Verify test coverage is 90%+
- [ ] 12.8 Commit: `chore: verify leaf parameters complete`
