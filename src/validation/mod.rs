//! Validation system for parameter values.
//!
//! This module provides a hybrid validation approach combining:
//! - **Declarative rules** via [`Expr`] - serializable, covers 80% of cases
//! - **Programmatic validators** via [`Validator`] trait - flexible, Rust-native
//!
//! # Architecture
//!
//! The validation system follows the adapter pattern (like React Hook Form resolvers),
//! allowing integration with any validation library while providing built-in validators
//! for common cases.
//!
//! ```text
//! Rule (validation definition)
//!   ├── Expr - Declarative, JSON-serializable expressions
//!   └── Fn   - Programmatic Validator trait implementations
//!
//! ValidationContext (cross-field access)
//!   ├── Current value being validated
//!   ├── Schema reference for metadata
//!   └── ValueAccess for sibling values
//! ```
//!
//! # Example
//!
//! ```ignore
//! use paramdef::validation::{Rule, Expr, ValidationContext};
//!
//! // Declarative validation (80% of cases)
//! let rules = vec![
//!     Rule::required(),
//!     Rule::min_length(3),
//!     Rule::max_length(50),
//!     Rule::pattern(r"^[a-zA-Z]+$"),
//! ];
//!
//! // Programmatic validation (complex cases)
//! let custom = Rule::custom(|value, ctx| {
//!     // Cross-field validation
//!     if let Some(other) = ctx.get("other_field") {
//!         // ... validate against other field
//!     }
//!     Ok(())
//! });
//! ```
//!
//! # Industry Patterns
//!
//! This design is inspired by:
//! - **JSON Schema** - Declarative constraints (required, minLength, pattern)
//! - **Zod/Yup** - Chainable validation with cross-field support
//! - **React Hook Form** - Resolver pattern for library integration
//! - **CEL (Kubernetes)** - Expression language for policy validation
//! - **garde/nutype** - Rust validation with derive macros

mod context;
mod result;
mod rule;
mod traits;
mod validators;

// Re-export unified Expr from crate::expr
pub use crate::expr::Expr;

pub use context::{NoValues, ValidationContext, ValueAccess};
pub use result::{Error, ValidationOutcome, ValidationResult};
pub use rule::{Rule, Rules};
pub use traits::{FnValidator, Validator};
pub use validators::{Length, Match, PasswordStrength, Range, Required, When};
