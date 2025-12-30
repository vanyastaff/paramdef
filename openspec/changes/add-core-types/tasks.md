## 1. Project Structure Setup

- [ ] 1.1 Create `src/core/mod.rs` module file
- [ ] 1.2 Create `src/core/key.rs` file
- [ ] 1.3 Create `src/core/metadata.rs` file
- [ ] 1.4 Create `src/core/flags.rs` file
- [ ] 1.5 Create `src/core/value.rs` file
- [ ] 1.6 Create `src/core/error.rs` file
- [ ] 1.7 Update `src/lib.rs` to export core module
- [ ] 1.8 Run `cargo check` to verify structure compiles

## 2. Key Type Implementation

- [ ] 2.1 Write failing test: `test_key_from_str`
- [ ] 2.2 Run test to verify it fails
- [ ] 2.3 Implement `pub type Key = SmartString<LazyCompact>;`
- [ ] 2.4 Run test to verify it passes
- [ ] 2.5 Write failing test: `test_key_equality`
- [ ] 2.6 Run test to verify it fails
- [ ] 2.7 Implement equality (automatic via SmartString)
- [ ] 2.8 Run test to verify it passes
- [ ] 2.9 Write failing test: `test_key_display`
- [ ] 2.10 Run test to verify it fails
- [ ] 2.11 Implement Display (automatic via SmartString)
- [ ] 2.12 Run test to verify it passes
- [ ] 2.13 Add documentation for Key type
- [ ] 2.14 Commit: `feat(core): add Key type alias`

## 3. Flags Implementation

- [ ] 3.1 Write failing test: `test_flags_required`
- [ ] 3.2 Run test to verify it fails
- [ ] 3.3 Implement Flags bitflags with REQUIRED
- [ ] 3.4 Run test to verify it passes
- [ ] 3.5 Write failing test: `test_flags_readonly`
- [ ] 3.6 Implement READONLY flag
- [ ] 3.7 Run test to verify it passes
- [ ] 3.8 Write failing test: `test_flags_hidden`
- [ ] 3.9 Implement HIDDEN flag
- [ ] 3.10 Run test to verify it passes
- [ ] 3.11 Write failing test: `test_flags_sensitive`
- [ ] 3.12 Implement SENSITIVE flag
- [ ] 3.13 Run test to verify it passes
- [ ] 3.14 Write failing test: `test_flags_combination`
- [ ] 3.15 Run test to verify it fails
- [ ] 3.16 Verify flag combinations work (bitwise OR)
- [ ] 3.17 Run test to verify it passes
- [ ] 3.18 Implement remaining flags (ADVANCED, ANIMATABLE, DEPRECATED, etc.)
- [ ] 3.19 Add convenience methods (is_required, is_readonly, etc.)
- [ ] 3.20 Add documentation for all flags
- [ ] 3.21 Commit: `feat(core): add Flags bitflags`

## 4. StateFlags Implementation

- [ ] 4.1 Write failing test: `test_state_flags_dirty`
- [ ] 4.2 Run test to verify it fails
- [ ] 4.3 Implement StateFlags bitflags with DIRTY
- [ ] 4.4 Run test to verify it passes
- [ ] 4.5 Write failing test: `test_state_flags_touched`
- [ ] 4.6 Implement TOUCHED flag
- [ ] 4.7 Run test to verify it passes
- [ ] 4.8 Write failing test: `test_state_flags_valid`
- [ ] 4.9 Implement VALID flag
- [ ] 4.10 Run test to verify it passes
- [ ] 4.11 Implement remaining state flags (VISIBLE, ENABLED, READONLY)
- [ ] 4.12 Add convenience methods
- [ ] 4.13 Add documentation
- [ ] 4.14 Commit: `feat(core): add StateFlags bitflags`

## 5. Metadata Implementation

- [ ] 5.1 Write failing test: `test_metadata_minimal`
- [ ] 5.2 Run test to verify it fails
- [ ] 5.3 Implement Metadata struct with key field
- [ ] 5.4 Run test to verify it passes
- [ ] 5.5 Write failing test: `test_metadata_with_label`
- [ ] 5.6 Implement label field
- [ ] 5.7 Run test to verify it passes
- [ ] 5.8 Write failing test: `test_metadata_with_description`
- [ ] 5.9 Implement description field
- [ ] 5.10 Run test to verify it passes
- [ ] 5.11 Write failing test: `test_metadata_with_group`
- [ ] 5.12 Implement group field
- [ ] 5.13 Run test to verify it passes
- [ ] 5.14 Write failing test: `test_metadata_with_tags`
- [ ] 5.15 Implement tags field (Vec<SmartString>)
- [ ] 5.16 Run test to verify it passes
- [ ] 5.17 Write failing test: `test_metadata_builder`
- [ ] 5.18 Implement MetadataBuilder
- [ ] 5.19 Run test to verify it passes
- [ ] 5.20 Add documentation
- [ ] 5.21 Commit: `feat(core): add Metadata struct with builder`

## 6. Value Enum Implementation

- [ ] 6.1 Write failing test: `test_value_null`
- [ ] 6.2 Run test to verify it fails
- [ ] 6.3 Implement Value::Null variant
- [ ] 6.4 Run test to verify it passes
- [ ] 6.5 Write failing test: `test_value_bool`
- [ ] 6.6 Implement Value::Bool variant
- [ ] 6.7 Run test to verify it passes
- [ ] 6.8 Write failing test: `test_value_int`
- [ ] 6.9 Implement Value::Int variant
- [ ] 6.10 Run test to verify it passes
- [ ] 6.11 Write failing test: `test_value_float`
- [ ] 6.12 Implement Value::Float variant
- [ ] 6.13 Run test to verify it passes
- [ ] 6.14 Write failing test: `test_value_text`
- [ ] 6.15 Implement Value::Text variant
- [ ] 6.16 Run test to verify it passes
- [ ] 6.17 Write failing test: `test_value_array`
- [ ] 6.18 Implement Value::Array variant with Arc<[Value]>
- [ ] 6.19 Run test to verify it passes
- [ ] 6.20 Write failing test: `test_value_object`
- [ ] 6.21 Implement Value::Object variant with Arc<HashMap>
- [ ] 6.22 Run test to verify it passes
- [ ] 6.23 Write failing test: `test_value_binary`
- [ ] 6.24 Implement Value::Binary variant
- [ ] 6.25 Run test to verify it passes
- [ ] 6.26 Commit: `feat(core): add Value enum variants`

## 7. Value Helper Methods

- [ ] 7.1 Write failing test: `test_value_is_null`
- [ ] 7.2 Implement is_null() method
- [ ] 7.3 Run test to verify it passes
- [ ] 7.4 Write failing test: `test_value_is_bool`
- [ ] 7.5 Implement is_bool() method
- [ ] 7.6 Run test to verify it passes
- [ ] 7.7 Write failing tests for remaining is_* methods
- [ ] 7.8 Implement is_int, is_float, is_text, is_array, is_object, is_binary
- [ ] 7.9 Run tests to verify they pass
- [ ] 7.10 Write failing test: `test_value_as_bool`
- [ ] 7.11 Implement as_bool() -> Option<bool>
- [ ] 7.12 Run test to verify it passes
- [ ] 7.13 Implement remaining as_* methods (as_int, as_float, as_text, etc.)
- [ ] 7.14 Run tests to verify they pass
- [ ] 7.15 Add documentation for all methods
- [ ] 7.16 Commit: `feat(core): add Value helper methods`

## 8. Value JSON Conversion (serde feature)

- [ ] 8.1 Write failing test: `test_value_to_json` (cfg feature = "serde")
- [ ] 8.2 Run test to verify it fails
- [ ] 8.3 Implement From<Value> for serde_json::Value
- [ ] 8.4 Run test to verify it passes
- [ ] 8.5 Write failing test: `test_json_to_value`
- [ ] 8.6 Implement From<serde_json::Value> for Value
- [ ] 8.7 Run test to verify it passes
- [ ] 8.8 Write failing test: `test_value_from_str`
- [ ] 8.9 Implement FromStr for Value
- [ ] 8.10 Run test to verify it passes
- [ ] 8.11 Write failing test: `test_value_display`
- [ ] 8.12 Implement Display for Value (compact JSON)
- [ ] 8.13 Run test to verify it passes
- [ ] 8.14 Write failing test: `test_value_display_pretty`
- [ ] 8.15 Implement alternate Display for Value (pretty JSON)
- [ ] 8.16 Run test to verify it passes
- [ ] 8.17 Add Serialize/Deserialize derives
- [ ] 8.18 Add documentation
- [ ] 8.19 Commit: `feat(core): add Value JSON conversion (serde feature)`

## 9. Error Types Implementation

- [ ] 9.1 Write failing test: `test_type_mismatch_error`
- [ ] 9.2 Run test to verify it fails
- [ ] 9.3 Implement TypeMismatch error with thiserror
- [ ] 9.4 Run test to verify it passes
- [ ] 9.5 Write failing test: `test_validation_error`
- [ ] 9.6 Implement ValidationError
- [ ] 9.7 Run test to verify it passes
- [ ] 9.8 Write failing test: `test_error_display`
- [ ] 9.9 Verify Display trait works via thiserror
- [ ] 9.10 Run test to verify it passes
- [ ] 9.11 Add documentation
- [ ] 9.12 Commit: `feat(core): add Error types`

## 10. Final Verification

- [ ] 10.1 Run `cargo fmt --all`
- [ ] 10.2 Run `cargo clippy --workspace --all-features -- -D warnings`
- [ ] 10.3 Run `cargo test --workspace`
- [ ] 10.4 Run `cargo test --workspace --features serde`
- [ ] 10.5 Run `cargo doc --no-deps --all-features`
- [ ] 10.6 Run `cargo +1.85 check --workspace` (MSRV)
- [ ] 10.7 Verify test coverage is 95%+
- [ ] 10.8 Commit: `chore: verify Phase 1 complete`
