## 1. Project Structure

- [x] 1.1 Create `src/parameter/mod.rs` module file
- [x] 1.2 Create `src/parameter/text.rs` file
- [x] 1.3 Create `src/parameter/number.rs` file
- [x] 1.4 Create `src/parameter/boolean.rs` file
- [x] 1.5 Create `src/parameter/vector.rs` file
- [x] 1.6 Create `src/parameter/select.rs` file
- [x] 1.7 Update `src/lib.rs` to export parameter module
- [x] 1.8 Run `cargo check` to verify structure compiles

## 2. Text Parameter Implementation

- [x] 2.1 Write failing test: `test_text_minimal`
- [x] 2.2 Implement Text struct with metadata, subtype, value
- [x] 2.3 Run test to verify it passes
- [x] 2.4 Write failing test: `test_text_builder`
- [x] 2.5 Implement TextBuilder with chainable methods
- [x] 2.6 Run test to verify it passes
- [x] 2.7 Add min_length, max_length, pattern fields
- [x] 2.8 Add validators and transformers fields
- [x] 2.9 Implement Leaf trait for Text
- [x] 2.10 Write failing test: `test_text_email_convenience`
- [x] 2.11 Implement Text::email() convenience constructor
- [x] 2.12 Run test to verify it passes
- [x] 2.13 Implement Text::url(), Text::password(), etc.
- [x] 2.14 Add documentation
- [x] 2.15 Commit: `feat(parameter): add Text type`

## 3. Number Parameter Implementation

- [x] 3.1 Write failing test: `test_number_minimal`
- [x] 3.2 Implement Number struct with metadata, subtype, unit, value
- [x] 3.3 Run test to verify it passes
- [x] 3.4 Write failing test: `test_number_builder`
- [x] 3.5 Implement NumberBuilder with chainable methods
- [x] 3.6 Run test to verify it passes
- [x] 3.7 Add min, max, soft_min, soft_max, step fields
- [x] 3.8 Add validators and transformers fields
- [x] 3.9 Implement Leaf trait for Number
- [x] 3.10 Write failing test: `test_number_integer_convenience`
- [x] 3.11 Implement Number::integer() convenience constructor
- [x] 3.12 Run test to verify it passes
- [x] 3.13 Implement Number::float(), Number::percentage(), etc.
- [x] 3.14 Add documentation
- [x] 3.15 Commit: `feat(parameter): add Number type`

## 4. Boolean Parameter Implementation

- [x] 4.1 Write failing test: `test_boolean_minimal`
- [x] 4.2 Implement Boolean struct with metadata, value, default
- [x] 4.3 Run test to verify it passes
- [x] 4.4 Write failing test: `test_boolean_builder`
- [x] 4.5 Implement BooleanBuilder
- [x] 4.6 Run test to verify it passes
- [x] 4.7 Implement Leaf trait for Boolean
- [x] 4.8 Add documentation
- [x] 4.9 Commit: `feat(parameter): add Boolean type`

## 5. Vector Parameter Implementation

- [x] 5.1 Write failing test: `test_vector_minimal`
- [x] 5.2 Implement Vector struct with metadata, subtype, value
- [x] 5.3 Run test to verify it passes
- [x] 5.4 Write failing test: `test_vector_builder`
- [x] 5.5 Implement VectorBuilder with type-safe component setters
- [x] 5.6 Run test to verify it passes
- [x] 5.7 Add validators field
- [x] 5.8 Implement Leaf trait for Vector
- [x] 5.9 Write failing test: `test_vector_vector3_convenience`
- [x] 5.10 Implement Vector::vector3() with default_vec3([f64; 3])
- [x] 5.11 Run test to verify it passes
- [x] 5.12 Implement Vector::color_rgba(), Vector::quaternion(), etc.
- [x] 5.13 Add documentation
- [x] 5.14 Commit: `feat(parameter): add Vector type`

## 6. Select Parameter Implementation

- [x] 6.1 Write failing test: `test_select_single_static`
- [x] 6.2 Implement Select struct with selection_mode, option_source
- [x] 6.3 Implement SelectOption struct (value, label, icon, enabled)
- [x] 6.4 Run test to verify it passes
- [x] 6.5 Write failing test: `test_select_multiple_static`
- [x] 6.6 Implement SelectionMode enum (Single, Multiple)
- [x] 6.7 Run test to verify it passes
- [x] 6.8 Write failing test: `test_select_dynamic_loader`
- [x] 6.9 Implement OptionSource enum (Static, Dynamic)
- [x] 6.10 Run test to verify it passes
- [x] 6.11 Implement SelectBuilder
- [x] 6.12 Implement Leaf trait for Select
- [x] 6.13 Implement Select::single(), Select::multiple()
- [x] 6.14 Add documentation
- [x] 6.15 Commit: `feat(parameter): add Select type (unified)`

## 7. Validation Integration

- [x] 7.1-7.10 Validation integration deferred to add-visibility-validation change

## 8. Transformation Integration

- [x] 8.1-8.8 Transformation integration deferred to add-visibility-validation change

## 9. Type Safety Tests

- [x] 9.1 Write failing test: `test_text_type_safe_getter`
- [x] 9.2 Verify get_value() returns Option<&String>
- [x] 9.3 Run test to verify it passes
- [x] 9.4 Write failing test: `test_number_type_safe_setter`
- [x] 9.5 Verify set_value(i64) works for integer Number
- [x] 9.6 Run test to verify it passes
- [x] 9.7 Write failing test: `test_vector_type_mismatch`
- [x] 9.8 Verify from_value(Value::Text) fails on Vector
- [x] 9.9 Run test to verify it passes
- [x] 9.10 Commit: `test: add type safety tests for all leaf types`

## 10. Integration Tests

- [x] 10.1 Write integration test: `test_complete_text_workflow`
- [x] 10.2 Test create → set → transform → validate → get cycle
- [x] 10.3 Run test to verify it passes
- [x] 10.4 Write integration test: `test_complete_number_workflow`
- [x] 10.5 Test with unit conversion
- [x] 10.6 Run test to verify it passes
- [x] 10.7 Write integration test: `test_complete_select_workflow`
- [x] 10.8 Test single and multiple selection modes
- [x] 10.9 Run test to verify it passes
- [x] 10.10 Commit: `test: add integration tests for leaf parameters`

## 11. Documentation

- [x] 11.1 Document Text API with examples
- [x] 11.2 Document Number API with unit conversion examples
- [x] 11.3 Document Boolean API
- [x] 11.4 Document Vector API with component access
- [x] 11.5 Document Select API with static/dynamic options
- [x] 11.6 Add builder pattern examples
- [x] 11.7 Commit: `docs: add leaf parameter documentation`

## 12. Final Verification

- [x] 12.1 Run `cargo fmt --all`
- [x] 12.2 Run `cargo clippy --workspace --all-features -- -D warnings`
- [x] 12.3 Run `cargo test --workspace`
- [x] 12.4 Run `cargo test --workspace --features validation`
- [x] 12.5 Run `cargo doc --no-deps --all-features`
- [x] 12.6 Run `cargo +1.85 check --workspace` (MSRV)
- [x] 12.7 Verify test coverage is 90%+
- [x] 12.8 Commit: `chore: verify leaf parameters complete`
