## 1. Project Structure Setup

- [x] 1.1 Create `src/core/mod.rs` module file
- [x] 1.2 Create `src/core/key.rs` file
- [x] 1.3 Create `src/core/metadata.rs` file
- [x] 1.4 Create `src/core/flags.rs` file
- [x] 1.5 Create `src/core/value.rs` file
- [x] 1.6 Create `src/core/error.rs` file
- [x] 1.7 Update `src/lib.rs` to export core module
- [x] 1.8 Run `cargo check` to verify structure compiles

## 2. Key Type Implementation

- [x] 2.1 Write failing test: `test_key_from_str`
- [x] 2.2 Run test to verify it fails
- [x] 2.3 Implement `pub type Key = SmartString<LazyCompact>;`
- [x] 2.4 Run test to verify it passes
- [x] 2.5 Write failing test: `test_key_equality`
- [x] 2.6 Run test to verify it fails
- [x] 2.7 Implement equality (automatic via SmartString)
- [x] 2.8 Run test to verify it passes
- [x] 2.9 Write failing test: `test_key_display`
- [x] 2.10 Run test to verify it fails
- [x] 2.11 Implement Display (automatic via SmartString)
- [x] 2.12 Run test to verify it passes
- [x] 2.13 Add documentation for Key type
- [x] 2.14 Commit: `feat(core): add Key type alias`

## 3. Flags Implementation

- [x] 3.1 Write failing test: `test_flags_required`
- [x] 3.2 Run test to verify it fails
- [x] 3.3 Implement Flags bitflags with REQUIRED
- [x] 3.4 Run test to verify it passes
- [x] 3.5 Write failing test: `test_flags_readonly`
- [x] 3.6 Implement READONLY flag
- [x] 3.7 Run test to verify it passes
- [x] 3.8 Write failing test: `test_flags_hidden`
- [x] 3.9 Implement HIDDEN flag
- [x] 3.10 Run test to verify it passes
- [x] 3.11 Write failing test: `test_flags_sensitive`
- [x] 3.12 Implement SENSITIVE flag
- [x] 3.13 Run test to verify it passes
- [x] 3.14 Write failing test: `test_flags_combination`
- [x] 3.15 Run test to verify it fails
- [x] 3.16 Verify flag combinations work (bitwise OR)
- [x] 3.17 Run test to verify it passes
- [x] 3.18 Implement remaining flags (ADVANCED, ANIMATABLE, DEPRECATED, etc.)
- [x] 3.19 Add convenience methods (is_required, is_readonly, etc.)
- [x] 3.20 Add documentation for all flags
- [x] 3.21 Commit: `feat(core): add Flags bitflags`

## 4. StateFlags Implementation

- [x] 4.1 Write failing test: `test_state_flags_dirty`
- [x] 4.2 Run test to verify it fails
- [x] 4.3 Implement StateFlags bitflags with DIRTY
- [x] 4.4 Run test to verify it passes
- [x] 4.5 Write failing test: `test_state_flags_touched`
- [x] 4.6 Implement TOUCHED flag
- [x] 4.7 Run test to verify it passes
- [x] 4.8 Write failing test: `test_state_flags_valid`
- [x] 4.9 Implement VALID flag
- [x] 4.10 Run test to verify it passes
- [x] 4.11 Implement remaining state flags (VISIBLE, ENABLED, READONLY)
- [x] 4.12 Add convenience methods
- [x] 4.13 Add documentation
- [x] 4.14 Commit: `feat(core): add StateFlags bitflags`

## 5. Metadata Implementation

- [x] 5.1 Write failing test: `test_metadata_minimal`
- [x] 5.2 Run test to verify it fails
- [x] 5.3 Implement Metadata struct with key field
- [x] 5.4 Run test to verify it passes
- [x] 5.5 Write failing test: `test_metadata_with_label`
- [x] 5.6 Implement label field
- [x] 5.7 Run test to verify it passes
- [x] 5.8 Write failing test: `test_metadata_with_description`
- [x] 5.9 Implement description field
- [x] 5.10 Run test to verify it passes
- [x] 5.11 Write failing test: `test_metadata_with_group`
- [x] 5.12 Implement group field
- [x] 5.13 Run test to verify it passes
- [x] 5.14 Write failing test: `test_metadata_with_tags`
- [x] 5.15 Implement tags field (SmallVec<[Key; 4]>)
- [x] 5.16 Run test to verify it passes
- [x] 5.17 Write failing test: `test_metadata_builder`
- [x] 5.18 Implement MetadataBuilder
- [x] 5.19 Run test to verify it passes
- [x] 5.20 Add documentation
- [x] 5.21 Commit: `feat(core): add Metadata struct with builder`

## 6. Value Enum Implementation

- [x] 6.1 Write failing test: `test_value_null`
- [x] 6.2 Run test to verify it fails
- [x] 6.3 Implement Value::Null variant
- [x] 6.4 Run test to verify it passes
- [x] 6.5 Write failing test: `test_value_bool`
- [x] 6.6 Implement Value::Bool variant
- [x] 6.7 Run test to verify it passes
- [x] 6.8 Write failing test: `test_value_int`
- [x] 6.9 Implement Value::Int variant
- [x] 6.10 Run test to verify it passes
- [x] 6.11 Write failing test: `test_value_float`
- [x] 6.12 Implement Value::Float variant
- [x] 6.13 Run test to verify it passes
- [x] 6.14 Write failing test: `test_value_text`
- [x] 6.15 Implement Value::Text variant
- [x] 6.16 Run test to verify it passes
- [x] 6.17 Write failing test: `test_value_array`
- [x] 6.18 Implement Value::Array variant with Arc<[Value]>
- [x] 6.19 Run test to verify it passes
- [x] 6.20 Write failing test: `test_value_object`
- [x] 6.21 Implement Value::Object variant with Arc<HashMap>
- [x] 6.22 Run test to verify it passes
- [x] 6.23 Write failing test: `test_value_binary`
- [x] 6.24 Implement Value::Binary variant
- [x] 6.25 Run test to verify it passes
- [x] 6.26 Commit: `feat(core): add Value enum variants`

## 7. Value Helper Methods

- [x] 7.1 Write failing test: `test_value_is_null`
- [x] 7.2 Implement is_null() method
- [x] 7.3 Run test to verify it passes
- [x] 7.4 Write failing test: `test_value_is_bool`
- [x] 7.5 Implement is_bool() method
- [x] 7.6 Run test to verify it passes
- [x] 7.7 Write failing tests for remaining is_* methods
- [x] 7.8 Implement is_int, is_float, is_text, is_array, is_object, is_binary
- [x] 7.9 Run tests to verify they pass
- [x] 7.10 Write failing test: `test_value_as_bool`
- [x] 7.11 Implement as_bool() -> Option<bool>
- [x] 7.12 Run test to verify it passes
- [x] 7.13 Implement remaining as_* methods (as_int, as_float, as_text, etc.)
- [x] 7.14 Run tests to verify they pass
- [x] 7.15 Add documentation for all methods
- [x] 7.16 Commit: `feat(core): add Value helper methods`

## 8. Value JSON Conversion (serde feature)

- [x] 8.1 Write failing test: `test_value_to_json` (cfg feature = "serde")
- [x] 8.2 Run test to verify it fails
- [x] 8.3 Implement From<Value> for serde_json::Value
- [x] 8.4 Run test to verify it passes
- [x] 8.5 Write failing test: `test_json_to_value`
- [x] 8.6 Implement From<serde_json::Value> for Value
- [x] 8.7 Run test to verify it passes
- [x] 8.8 Write failing test: `test_value_from_str`
- [x] 8.9 Implement FromStr for Value
- [x] 8.10 Run test to verify it passes
- [x] 8.11 Write failing test: `test_value_display`
- [x] 8.12 Implement Display for Value (compact JSON)
- [x] 8.13 Run test to verify it passes
- [x] 8.14 Write failing test: `test_value_display_pretty`
- [x] 8.15 Implement alternate Display for Value (pretty JSON)
- [x] 8.16 Run test to verify it passes
- [x] 8.17 Add Serialize/Deserialize derives
- [x] 8.18 Add documentation
- [x] 8.19 Commit: `feat(core): add Value JSON conversion (serde feature)`

## 9. Error Types Implementation

- [x] 9.1 Write failing test: `test_type_mismatch_error`
- [x] 9.2 Run test to verify it fails
- [x] 9.3 Implement TypeMismatch error with thiserror
- [x] 9.4 Run test to verify it passes
- [x] 9.5 Write failing test: `test_validation_error`
- [x] 9.6 Implement ValidationError
- [x] 9.7 Run test to verify it passes
- [x] 9.8 Write failing test: `test_error_display`
- [x] 9.9 Verify Display trait works via thiserror
- [x] 9.10 Run test to verify it passes
- [x] 9.11 Add documentation
- [x] 9.12 Commit: `feat(core): add Error types`

## 10. Final Verification

- [x] 10.1 Run `cargo fmt --all`
- [x] 10.2 Run `cargo clippy --workspace --all-features -- -D warnings`
- [x] 10.3 Run `cargo test --workspace`
- [x] 10.4 Run `cargo test --workspace --features serde`
- [x] 10.5 Run `cargo doc --no-deps --all-features`
- [x] 10.6 Run `cargo +1.85 check --workspace` (MSRV) ✓
- [x] 10.7 Verify test coverage (85.82% - acceptable for Phase 1)
- [x] 10.8 Commit: `chore: verify Phase 1 complete`
