# Change: Add Visibility and Validation Systems

## Why
The paramdef library needs conditional visibility (Expr) for dynamic forms and a validation system that integrates with any validation library. Visibility uses a single expression that evaluates to bool. Validation provides traits for sync/async validators without bundling specific libraries.

## What Changes
- Add `Visibility` trait (feature = "visibility")
- Add `Expr` enum for visibility expressions (Eq, Ne, IsTrue, And, Or, Not, etc.)
- Add `VisibilityObserver` for reactive visibility updates
- Add `Validatable` trait (feature = "validation")
- Add `Validator` and `AsyncValidator` traits
- Add `ValidationConfig` for per-parameter validation setup
- Add `CrossValidator` trait for multi-parameter validation
- Add built-in validators (Required, MinLength, MaxLength, Range, Pattern, Email, Url)

## Impact
- Affected specs: visibility-validation (new capability)
- Affected code: new `src/visibility/` and `src/validation/` modules
- Depends on: add-schema-runtime (Phase 4), add-reactive-system (Phase 5)
- Feature-gated: `visibility` and `validation` features
