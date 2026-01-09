//! Value transformation system for paramdef.
//!
//! This module provides a flexible system for transforming values before validation
//! and storage. Transformers modify input values (like trimming whitespace, normalizing
//! case, or formatting phone numbers) to ensure consistent data storage.
//!
//! # Architecture
//!
//! The transformation system follows a hybrid approach similar to the validation system:
//!
//! - **[`Transform`]**: Declarative enum covering ~80% of common cases (Trim, Lowercase, etc.)
//! - **[`Transformer`]**: Trait for custom programmatic transformations (~20% complex cases)
//!
//! # Transformation Pipeline
//!
//! Transformations are applied in this order:
//!
//! 1. **Parse** (input → storage): Applied when setting values
//! 2. **Validate**: Applied after transformation (separate system)
//! 3. **Format** (storage → display): Applied when rendering (UI layer responsibility)
//!
//! This follows the React Final Form pattern where `parse` and `format` are inverses.
//!
//! # Usage
//!
//! ## Declarative Transformations
//!
//! ```
//! use paramdef::transform::{Transform, Transforms};
//!
//! // Single transformation
//! let trim = Transform::Trim;
//!
//! // Chained transformations (applied in order)
//! let normalize = Transforms::new()
//!     .push(Transform::Trim)
//!     .push(Transform::Lowercase);
//!
//! // Apply transformations
//! use paramdef::Value;
//! let value = Value::text("  HELLO  ");
//! let result = normalize.apply(&value);
//! assert_eq!(result.as_text(), Some("hello"));
//! ```
//!
//! ## Programmatic Transformations
//!
//! ```
//! use paramdef::transform::{Transformer, Transforms};
//! use paramdef::Value;
//!
//! struct PhoneFormatter;
//!
//! impl Transformer for PhoneFormatter {
//!     fn name(&self) -> &'static str { "phone_formatter" }
//!     fn transform(&self, value: &Value) -> Value {
//!         if let Some(s) = value.as_text() {
//!             // Keep only digits
//!             let digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
//!             if digits.len() == 10 {
//!                 Value::text(format!("({}) {}-{}",
//!                     &digits[0..3], &digits[3..6], &digits[6..10]))
//!             } else {
//!                 value.clone()
//!             }
//!         } else {
//!             value.clone()
//!         }
//!     }
//! }
//!
//! let transforms = Transforms::new().custom(PhoneFormatter);
//! let value = Value::text("5551234567");
//! let result = transforms.apply(&value);
//! assert_eq!(result.as_text(), Some("(555) 123-4567"));
//! ```
//!
//! # Best Practices
//!
//! 1. **Apply transformations BEFORE validation** - This is the OWASP-recommended pattern.
//!    Normalize input first, then validate the normalized form.
//!
//! 2. **Transformations should be idempotent** - Applying a transformation twice should
//!    give the same result as applying it once: `transform(transform(x)) == transform(x)`.
//!
//! 3. **Transformations should be pure** - No side effects, no external state.
//!
//! 4. **Keep transformations reversible when possible** - For display formatting,
//!    the UI layer can apply inverse transformations.

mod expr;
mod traits;
mod transforms;
mod transformers;

pub use expr::Transform;
pub use traits::{FnTransformer, Transformer};
pub use transforms::Transforms;
pub use transformers::{Clamp, Default, Replace, Round, Truncate};
