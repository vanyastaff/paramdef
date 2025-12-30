## 1. Module Structure Setup

- [x] 1.1 Create `src/subtypes/mod.rs`
- [x] 1.2 Create `src/subtypes/traits.rs` for trait definitions
- [x] 1.3 Create `src/subtypes/macros.rs` for definition macros
- [x] 1.4 Create `src/subtypes/number.rs` for NumberSubtype implementations
- [x] 1.5 Create `src/subtypes/vector.rs` for VectorSubtype implementations
- [x] 1.6 Create `src/subtypes/text.rs` for TextSubtype implementations
- [x] 1.7 Create `src/subtypes/unit.rs` for NumberUnit
- [x] 1.8 Update `src/lib.rs` to export subtypes module
- [x] 1.9 Run `cargo check` to verify structure compiles

## 2. Numeric Trait Definition

- [x] 2.1 Write failing test: `test_numeric_trait_bounds`
- [x] 2.2 Run test to verify it fails
- [x] 2.3 Define `Numeric` trait with bounds (Copy + PartialOrd + num traits)
- [x] 2.4 Run test to verify it passes
- [x] 2.5 Implement Numeric for all integer and float types
- [x] 2.6 Add documentation
- [x] 2.7 Commit: `feat(subtypes): add Numeric trait`

## 3. NumberSubtype Trait

- [x] 3.1 Write failing test: `test_number_subtype_trait`
- [x] 3.2 Run test to verify it fails
- [x] 3.3 Define `NumberSubtype<T: Numeric>` trait
- [x] 3.4 Run test to verify it passes
- [x] 3.5 Add `default_range()` method
- [x] 3.6 Add `name()` method
- [x] 3.7 Add documentation
- [x] 3.8 Commit: `feat(subtypes): add NumberSubtype trait`

## 4. VectorSubtype Trait

- [x] 4.1 Write failing test: `test_vector_subtype_trait`
- [x] 4.2 Run test to verify it fails
- [x] 4.3 Define `VectorSubtype<const N: usize>` trait
- [x] 4.4 Run test to verify it passes
- [x] 4.5 Add `name()` and `default_range()` methods
- [x] 4.6 Add documentation
- [x] 4.7 Commit: `feat(subtypes): add VectorSubtype trait`

## 5. TextSubtype Trait

- [x] 5.1 Write failing test: `test_text_subtype_trait`
- [x] 5.2 Run test to verify it fails
- [x] 5.3 Define `TextSubtype` trait
- [x] 5.4 Run test to verify it passes
- [x] 5.5 Add `pattern()` and `placeholder()` methods
- [x] 5.6 Add documentation
- [x] 5.7 Commit: `feat(subtypes): add TextSubtype trait`

## 6. Definition Macros

- [x] 6.1 Write failing test: `test_define_number_subtype_int_only`
- [x] 6.2 Implement `define_number_subtype!` macro for int_only
- [x] 6.3 Run test to verify it passes
- [x] 6.4 Write failing test: `test_define_number_subtype_float_only`
- [x] 6.5 Extend macro for float_only
- [x] 6.6 Run test to verify it passes
- [x] 6.7 Write failing test: `test_define_number_subtype_any`
- [x] 6.8 Extend macro for any
- [x] 6.9 Run test to verify it passes
- [x] 6.10 Write failing test: `test_define_vector_subtype`
- [x] 6.11 Implement `define_vector_subtype!` macro
- [x] 6.12 Run test to verify it passes
- [x] 6.13 Write failing test: `test_define_text_subtype`
- [x] 6.14 Implement `define_text_subtype!` macro
- [x] 6.15 Run test to verify it passes
- [x] 6.16 Add documentation
- [x] 6.17 Commit: `feat(subtypes): add definition macros`

## 7. Standard Number Subtypes

- [x] 7.1 Define integer-only subtypes: Port, Count, Rating, ByteCount, Index
- [x] 7.2 Write tests for integer-only subtypes
- [x] 7.3 Define float-only subtypes: Factor, Percentage, Angle, AngleRadians
- [x] 7.4 Write tests for float-only subtypes
- [x] 7.5 Define universal subtypes: Distance, Duration, Temperature, Currency, Speed, Mass, Generic
- [x] 7.6 Write tests for universal subtypes
- [x] 7.7 Add documentation
- [x] 7.8 Commit: `feat(subtypes): add standard NumberSubtype implementations`

## 8. Standard Vector Subtypes

- [x] 8.1 Define size-2 subtypes: Position2D, Size2D, Uv, LatLong, MinMax, Direction2D, Scale2D, Vector2
- [x] 8.2 Write tests for size-2 subtypes
- [x] 8.3 Define size-3 subtypes: Position3D, Direction3D, Normal, Scale3D, Euler, ColorRgb, ColorHsv, Vector3
- [x] 8.4 Write tests for size-3 subtypes
- [x] 8.5 Define size-4 subtypes: Quaternion, AxisAngle, ColorRgba, Bounds2D, Vector4
- [x] 8.6 Write tests for size-4 subtypes
- [x] 8.7 Define size-6, size-9, size-16 subtypes: Bounds3D, Matrix3x3, Matrix4x4
- [x] 8.8 Write tests for larger subtypes
- [x] 8.9 Add documentation
- [x] 8.10 Commit: `feat(subtypes): add standard VectorSubtype implementations`

## 9. Standard Text Subtypes

- [x] 9.1 Define basic subtypes: Plain, MultiLine
- [x] 9.2 Define network subtypes: Email, Url, Domain, IpAddressV4, IpAddressV6, Hostname
- [x] 9.3 Define path subtypes: FilePath, DirPath, FileName
- [x] 9.4 Define security subtypes: Secret, Password, ApiKey, BearerToken
- [x] 9.5 Define identifier subtypes: Uuid, Slug
- [x] 9.6 Define date/time subtypes: DateTime, Date, Time
- [x] 9.7 Define structured data subtypes: Json, Yaml, Toml, Xml
- [x] 9.8 Define code subtypes: Sql, Regex, Expression, JavaScript, Python, Rust
- [x] 9.9 Write tests for all text subtypes
- [x] 9.10 Add documentation
- [x] 9.11 Commit: `feat(subtypes): add standard TextSubtype implementations`

## 10. NumberUnit Implementation

- [x] 10.1 Write failing test: `test_number_unit_length`
- [x] 10.2 Implement NumberUnit enum with Length variants
- [x] 10.3 Run test to verify it passes
- [x] 10.4 Write failing test: `test_unit_conversion`
- [x] 10.5 Implement from_base() and to_base() methods
- [x] 10.6 Run test to verify it passes
- [x] 10.7 Add Temperature, Time, Rotation, Data unit categories
- [x] 10.8 Write tests for all unit categories
- [x] 10.9 Add display_suffix() method
- [x] 10.10 Add documentation
- [x] 10.11 Commit: `feat(subtypes): add NumberUnit with conversions`

## 11. IntoBuilder Trait

- [x] 11.1 Define `IntoBuilder` trait
- [x] 11.2 Add documentation
- [ ] 11.3-11.12 IntoBuilder implementations deferred to Phase 3 (requires parameter builders)

## 12. Final Verification

- [x] 12.1 Run `cargo fmt --all`
- [x] 12.2 Run `cargo clippy --workspace --all-features -- -D warnings`
- [x] 12.3 Run `cargo test --workspace`
- [x] 12.4 Run `cargo doc --no-deps --all-features`
- [ ] 12.5 Run `cargo +1.85 check --workspace` (MSRV)
- [ ] 12.6 Verify test coverage is 90%+
- [ ] 12.7 Commit: `feat(subtypes): add Phase 2 subtypes system`
