## 1. Module Structure Setup

- [ ] 1.1 Create `src/subtypes/mod.rs`
- [ ] 1.2 Create `src/subtypes/traits.rs` for trait definitions
- [ ] 1.3 Create `src/subtypes/macros.rs` for definition macros
- [ ] 1.4 Create `src/subtypes/number.rs` for NumberSubtype implementations
- [ ] 1.5 Create `src/subtypes/vector.rs` for VectorSubtype implementations
- [ ] 1.6 Create `src/subtypes/text.rs` for TextSubtype implementations
- [ ] 1.7 Create `src/subtypes/unit.rs` for NumberUnit
- [ ] 1.8 Update `src/lib.rs` to export subtypes module
- [ ] 1.9 Run `cargo check` to verify structure compiles

## 2. Numeric Trait Definition

- [ ] 2.1 Write failing test: `test_numeric_trait_bounds`
- [ ] 2.2 Run test to verify it fails
- [ ] 2.3 Define `Numeric` trait with bounds (Copy + PartialOrd + num traits)
- [ ] 2.4 Run test to verify it passes
- [ ] 2.5 Implement Numeric for all integer and float types
- [ ] 2.6 Add documentation
- [ ] 2.7 Commit: `feat(subtypes): add Numeric trait`

## 3. NumberSubtype Trait

- [ ] 3.1 Write failing test: `test_number_subtype_trait`
- [ ] 3.2 Run test to verify it fails
- [ ] 3.3 Define `NumberSubtype<T: Numeric>` trait
- [ ] 3.4 Run test to verify it passes
- [ ] 3.5 Add `default_range()` method
- [ ] 3.6 Add `name()` method
- [ ] 3.7 Add documentation
- [ ] 3.8 Commit: `feat(subtypes): add NumberSubtype trait`

## 4. VectorSubtype Trait

- [ ] 4.1 Write failing test: `test_vector_subtype_trait`
- [ ] 4.2 Run test to verify it fails
- [ ] 4.3 Define `VectorSubtype<const N: usize>` trait
- [ ] 4.4 Run test to verify it passes
- [ ] 4.5 Add `name()` and `default_range()` methods
- [ ] 4.6 Add documentation
- [ ] 4.7 Commit: `feat(subtypes): add VectorSubtype trait`

## 5. TextSubtype Trait

- [ ] 5.1 Write failing test: `test_text_subtype_trait`
- [ ] 5.2 Run test to verify it fails
- [ ] 5.3 Define `TextSubtype` trait
- [ ] 5.4 Run test to verify it passes
- [ ] 5.5 Add `pattern()` and `placeholder()` methods
- [ ] 5.6 Add documentation
- [ ] 5.7 Commit: `feat(subtypes): add TextSubtype trait`

## 6. Definition Macros

- [ ] 6.1 Write failing test: `test_define_number_subtype_int_only`
- [ ] 6.2 Implement `define_number_subtype!` macro for int_only
- [ ] 6.3 Run test to verify it passes
- [ ] 6.4 Write failing test: `test_define_number_subtype_float_only`
- [ ] 6.5 Extend macro for float_only
- [ ] 6.6 Run test to verify it passes
- [ ] 6.7 Write failing test: `test_define_number_subtype_any`
- [ ] 6.8 Extend macro for any
- [ ] 6.9 Run test to verify it passes
- [ ] 6.10 Write failing test: `test_define_vector_subtype`
- [ ] 6.11 Implement `define_vector_subtype!` macro
- [ ] 6.12 Run test to verify it passes
- [ ] 6.13 Write failing test: `test_define_text_subtype`
- [ ] 6.14 Implement `define_text_subtype!` macro
- [ ] 6.15 Run test to verify it passes
- [ ] 6.16 Add documentation
- [ ] 6.17 Commit: `feat(subtypes): add definition macros`

## 7. Standard Number Subtypes

- [ ] 7.1 Define integer-only subtypes: Port, Count, Rating, ByteCount, Index
- [ ] 7.2 Write tests for integer-only subtypes
- [ ] 7.3 Define float-only subtypes: Factor, Percentage, Angle, AngleRadians
- [ ] 7.4 Write tests for float-only subtypes
- [ ] 7.5 Define universal subtypes: Distance, Duration, Temperature, Currency, Speed, Mass, Generic
- [ ] 7.6 Write tests for universal subtypes
- [ ] 7.7 Add documentation
- [ ] 7.8 Commit: `feat(subtypes): add standard NumberSubtype implementations`

## 8. Standard Vector Subtypes

- [ ] 8.1 Define size-2 subtypes: Position2D, Size2D, Uv, LatLong, MinMax, Direction2D, Scale2D, Vector2
- [ ] 8.2 Write tests for size-2 subtypes
- [ ] 8.3 Define size-3 subtypes: Position3D, Direction3D, Normal, Scale3D, Euler, ColorRgb, ColorHsv, Vector3
- [ ] 8.4 Write tests for size-3 subtypes
- [ ] 8.5 Define size-4 subtypes: Quaternion, AxisAngle, ColorRgba, Bounds2D, Vector4
- [ ] 8.6 Write tests for size-4 subtypes
- [ ] 8.7 Define size-6, size-9, size-16 subtypes: Bounds3D, Matrix3x3, Matrix4x4
- [ ] 8.8 Write tests for larger subtypes
- [ ] 8.9 Add documentation
- [ ] 8.10 Commit: `feat(subtypes): add standard VectorSubtype implementations`

## 9. Standard Text Subtypes

- [ ] 9.1 Define basic subtypes: Plain, MultiLine
- [ ] 9.2 Define network subtypes: Email, Url, Domain, IpAddressV4, IpAddressV6, Hostname
- [ ] 9.3 Define path subtypes: FilePath, DirPath, FileName
- [ ] 9.4 Define security subtypes: Secret, Password, ApiKey, BearerToken
- [ ] 9.5 Define identifier subtypes: Uuid, Slug
- [ ] 9.6 Define date/time subtypes: DateTime, Date, Time
- [ ] 9.7 Define structured data subtypes: Json, Yaml, Toml, Xml
- [ ] 9.8 Define code subtypes: Code(CodeLanguage), Sql, Regex, Expression
- [ ] 9.9 Write tests for all text subtypes
- [ ] 9.10 Add documentation
- [ ] 9.11 Commit: `feat(subtypes): add standard TextSubtype implementations`

## 10. NumberUnit Implementation

- [ ] 10.1 Write failing test: `test_number_unit_length`
- [ ] 10.2 Implement NumberUnit enum with Length variants
- [ ] 10.3 Run test to verify it passes
- [ ] 10.4 Write failing test: `test_unit_conversion`
- [ ] 10.5 Implement from_base() and to_base() methods
- [ ] 10.6 Run test to verify it passes
- [ ] 10.7 Add Temperature, Time, Rotation, Data unit categories
- [ ] 10.8 Write tests for all unit categories
- [ ] 10.9 Add display_suffix() method
- [ ] 10.10 Add documentation
- [ ] 10.11 Commit: `feat(subtypes): add NumberUnit with conversions`

## 11. IntoBuilder Trait

- [ ] 11.1 Write failing test: `test_into_builder_trait`
- [ ] 11.2 Define `IntoBuilder` trait
- [ ] 11.3 Run test to verify it passes
- [ ] 11.4 Write failing test: `test_port_into_builder`
- [ ] 11.5 Implement IntoBuilder for Port
- [ ] 11.6 Run test to verify it passes
- [ ] 11.7 Implement IntoBuilder for Factor, Percentage, Rating
- [ ] 11.8 Implement IntoBuilder for Position3D, ColorRgba, Quaternion
- [ ] 11.9 Implement IntoBuilder for Email, Url, Secret
- [ ] 11.10 Write tests for all IntoBuilder implementations
- [ ] 11.11 Add documentation
- [ ] 11.12 Commit: `feat(subtypes): add IntoBuilder trait and implementations`

## 12. Final Verification

- [ ] 12.1 Run `cargo fmt --all`
- [ ] 12.2 Run `cargo clippy --workspace --all-features -- -D warnings`
- [ ] 12.3 Run `cargo test --workspace`
- [ ] 12.4 Run `cargo doc --no-deps --all-features`
- [ ] 12.5 Run `cargo +1.85 check --workspace` (MSRV)
- [ ] 12.6 Verify test coverage is 90%+
- [ ] 12.7 Commit: `chore: verify Phase 2 complete`
