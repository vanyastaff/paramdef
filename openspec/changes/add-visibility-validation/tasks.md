## 1. Module Structure Setup

- [ ] 1.1 Create `src/visibility/mod.rs`
- [ ] 1.2 Create `src/visibility/expr.rs` (Expr enum)
- [ ] 1.3 Create `src/visibility/traits.rs` (Visibility trait)
- [ ] 1.4 Create `src/visibility/observer.rs` (VisibilityObserver)
- [ ] 1.5 Create `src/validation/mod.rs`
- [ ] 1.6 Create `src/validation/traits.rs` (Validator, AsyncValidator, Validatable)
- [ ] 1.7 Create `src/validation/config.rs` (ValidationConfig)
- [ ] 1.8 Create `src/validation/error.rs` (ValidationError)
- [ ] 1.9 Create `src/validation/validators/mod.rs` (built-in validators)
- [ ] 1.10 Create `src/validation/cross.rs` (CrossValidator)
- [ ] 1.11 Update `src/lib.rs` with feature gates
- [ ] 1.12 Run `cargo check` to verify structure compiles

## 2. Expr Enum Implementation

- [ ] 2.1 Write failing test: `test_expr_eq`
- [ ] 2.2 Implement Expr enum with Eq variant
- [ ] 2.3 Run test to verify it passes
- [ ] 2.4 Write failing test: `test_expr_ne`
- [ ] 2.5 Add Ne variant
- [ ] 2.6 Run test to verify it passes
- [ ] 2.7 Add IsSet, IsEmpty, IsTrue variants
- [ ] 2.8 Add Lt, Le, Gt, Ge variants
- [ ] 2.9 Add OneOf variant
- [ ] 2.10 Add IsValid variant
- [ ] 2.11 Add And, Or, Not combinators
- [ ] 2.12 Write tests for all variants
- [ ] 2.13 Add documentation
- [ ] 2.14 Commit: `feat(visibility): add Expr enum`

## 3. Expr Evaluation

- [ ] 3.1 Write failing test: `test_expr_eval_eq`
- [ ] 3.2 Implement eval() method for Eq
- [ ] 3.3 Run test to verify it passes
- [ ] 3.4 Implement eval() for all comparison variants
- [ ] 3.5 Write failing test: `test_expr_eval_and`
- [ ] 3.6 Implement eval() for And, Or, Not
- [ ] 3.7 Run test to verify it passes
- [ ] 3.8 Add documentation
- [ ] 3.9 Commit: `feat(visibility): add Expr evaluation`

## 4. Expr Dependencies

- [ ] 4.1 Write failing test: `test_expr_dependencies_simple`
- [ ] 4.2 Implement dependencies() method
- [ ] 4.3 Run test to verify it passes
- [ ] 4.4 Write failing test: `test_expr_dependencies_compound`
- [ ] 4.5 Verify compound expressions return all dependencies
- [ ] 4.6 Run test to verify it passes
- [ ] 4.7 Write failing test: `test_expr_depends_on`
- [ ] 4.8 Implement depends_on(key) helper
- [ ] 4.9 Run test to verify it passes
- [ ] 4.10 Add documentation
- [ ] 4.11 Commit: `feat(visibility): add Expr dependencies`

## 5. Visibility Trait

- [ ] 5.1 Write failing test: `test_visibility_trait`
- [ ] 5.2 Define Visibility trait with visibility(), set_visibility(), is_visible()
- [ ] 5.3 Run test to verify it passes
- [ ] 5.4 Implement Visibility for all 14 node types
- [ ] 5.5 Write tests for each node type
- [ ] 5.6 Add documentation
- [ ] 5.7 Commit: `feat(visibility): add Visibility trait`

## 6. VisibilityObserver

- [ ] 6.1 Write failing test: `test_visibility_observer_register`
- [ ] 6.2 Implement VisibilityObserver struct
- [ ] 6.3 Run test to verify it passes
- [ ] 6.4 Write failing test: `test_visibility_observer_evaluate`
- [ ] 6.5 Implement evaluation on dependency change
- [ ] 6.6 Run test to verify it passes
- [ ] 6.7 Write failing test: `test_visibility_observer_events`
- [ ] 6.8 Verify VisibilityChanged events are emitted
- [ ] 6.9 Run test to verify it passes
- [ ] 6.10 Add documentation
- [ ] 6.11 Commit: `feat(visibility): add VisibilityObserver`

## 7. Validator Traits

- [ ] 7.1 Write failing test: `test_validator_trait`
- [ ] 7.2 Define Validator trait
- [ ] 7.3 Run test to verify it passes
- [ ] 7.4 Write failing test: `test_validator_closure`
- [ ] 7.5 Implement blanket impl for closures
- [ ] 7.6 Run test to verify it passes
- [ ] 7.7 Write failing test: `test_async_validator_trait`
- [ ] 7.8 Define AsyncValidator trait with async_trait
- [ ] 7.9 Run test to verify it passes
- [ ] 7.10 Add documentation
- [ ] 7.11 Commit: `feat(validation): add Validator traits`

## 8. Validatable Trait

- [ ] 8.1 Write failing test: `test_validatable_trait`
- [ ] 8.2 Define Validatable trait
- [ ] 8.3 Run test to verify it passes
- [ ] 8.4 Implement Validatable for Text
- [ ] 8.5 Implement Validatable for Number
- [ ] 8.6 Implement Validatable for Boolean
- [ ] 8.7 Implement Validatable for Vector
- [ ] 8.8 Implement Validatable for Select
- [ ] 8.9 Implement Validatable for Object
- [ ] 8.10 Implement Validatable for List
- [ ] 8.11 Implement Validatable for Mode
- [ ] 8.12 Implement Validatable for Routing
- [ ] 8.13 Implement Validatable for Expirable
- [ ] 8.14 Write test verifying Group/Panel/Notice/Ref do NOT implement
- [ ] 8.15 Add documentation
- [ ] 8.16 Commit: `feat(validation): add Validatable trait`

## 9. ValidationConfig

- [ ] 9.1 Write failing test: `test_validation_config_creation`
- [ ] 9.2 Implement ValidationConfig struct
- [ ] 9.3 Run test to verify it passes
- [ ] 9.4 Write failing test: `test_validation_config_sync`
- [ ] 9.5 Add sync_validators field and methods
- [ ] 9.6 Run test to verify it passes
- [ ] 9.7 Write failing test: `test_validation_config_async`
- [ ] 9.8 Add async_validators field and methods
- [ ] 9.9 Run test to verify it passes
- [ ] 9.10 Add debounce_ms field
- [ ] 9.11 Add error_messages customization
- [ ] 9.12 Add documentation
- [ ] 9.13 Commit: `feat(validation): add ValidationConfig`

## 10. Built-in Validators

- [ ] 10.1 Write failing test: `test_required_validator`
- [ ] 10.2 Implement RequiredValidator
- [ ] 10.3 Run test to verify it passes
- [ ] 10.4 Write failing test: `test_min_length_validator`
- [ ] 10.5 Implement MinLengthValidator
- [ ] 10.6 Run test to verify it passes
- [ ] 10.7 Write failing test: `test_max_length_validator`
- [ ] 10.8 Implement MaxLengthValidator
- [ ] 10.9 Run test to verify it passes
- [ ] 10.10 Write failing test: `test_range_validator`
- [ ] 10.11 Implement RangeValidator
- [ ] 10.12 Run test to verify it passes
- [ ] 10.13 Write failing test: `test_pattern_validator`
- [ ] 10.14 Implement PatternValidator
- [ ] 10.15 Run test to verify it passes
- [ ] 10.16 Write failing test: `test_email_validator`
- [ ] 10.17 Implement EmailValidator
- [ ] 10.18 Run test to verify it passes
- [ ] 10.19 Write failing test: `test_url_validator`
- [ ] 10.20 Implement UrlValidator
- [ ] 10.21 Run test to verify it passes
- [ ] 10.22 Add helper functions (required(), min_length(), etc.)
- [ ] 10.23 Add documentation
- [ ] 10.24 Commit: `feat(validation): add built-in validators`

## 11. CrossValidator

- [ ] 11.1 Write failing test: `test_cross_validator_trait`
- [ ] 11.2 Define CrossValidator trait
- [ ] 11.3 Run test to verify it passes
- [ ] 11.4 Write failing test: `test_cross_validator_date_range`
- [ ] 11.5 Implement DateRangeValidator example
- [ ] 11.6 Run test to verify it passes
- [ ] 11.7 Add documentation
- [ ] 11.8 Commit: `feat(validation): add CrossValidator trait`

## 12. Validation Integration

- [ ] 12.1 Write failing test: `test_context_validates_on_set`
- [ ] 12.2 Integrate validation into Context.set_value()
- [ ] 12.3 Run test to verify it passes
- [ ] 12.4 Write failing test: `test_validation_events`
- [ ] 12.5 Verify validation events are emitted
- [ ] 12.6 Run test to verify it passes
- [ ] 12.7 Write failing test: `test_valid_state_flag`
- [ ] 12.8 Verify StateFlags::VALID is set/cleared
- [ ] 12.9 Run test to verify it passes
- [ ] 12.10 Add documentation
- [ ] 12.11 Commit: `feat(validation): integrate with Context`

## 13. Final Verification

- [ ] 13.1 Run `cargo fmt --all`
- [ ] 13.2 Run `cargo clippy --workspace --all-features -- -D warnings`
- [ ] 13.3 Run `cargo test --workspace`
- [ ] 13.4 Run `cargo test --workspace --features visibility`
- [ ] 13.5 Run `cargo test --workspace --features validation`
- [ ] 13.6 Run `cargo test --workspace --features full`
- [ ] 13.7 Run `cargo doc --no-deps --all-features`
- [ ] 13.8 Run `cargo +1.85 check --workspace` (MSRV)
- [ ] 13.9 Verify test coverage is 90%+
- [ ] 13.10 Commit: `chore: verify Phase 6 complete`
