//! Unified expression system for validation and visibility.
//!
//! This module provides a unified way to express conditions that can be used for:
//! - **Validation**: Check if a value meets certain criteria
//! - **Visibility**: Conditionally show/hide parameters based on other values
//! - **Cross-field validation**: Check relationships between multiple fields
//!
//! # Architecture
//!
//! The system consists of three main components:
//!
//! - [`ExprTarget`]: Specifies **where** to apply the check (local value or another field)
//! - [`Expr`]: Specifies **what** to check (email, min length, equals, etc.)
//! - [`Rule`]: Combines a target and an expression into a complete rule
//!
//! # Examples
//!
//! ## Validation (Local Target)
//!
//! ```ignore
//! use paramdef::expr::{Expr, Rule};
//!
//! // Validate that current value is an email with minimum length of 5
//! let rule = Rule::local(Expr::and(vec![
//!     Expr::Email,
//!     Expr::MinLength(5),
//! ]));
//!
//! // Or using convenience methods
//! let rule = Expr::email().on_local();
//! ```
//!
//! ## Visibility (Field Target)
//!
//! ```ignore
//! use paramdef::expr::{Expr, Rule};
//! use paramdef::core::Value;
//!
//! // Show current field only if "mode" equals "advanced"
//! let rule = Rule::field("mode", Expr::Eq(Value::text("advanced")));
//!
//! // Or using on_field method
//! let rule = Expr::eq(Value::text("advanced")).on_field("mode");
//! ```
//!
//! ## Cross-field Validation
//!
//! ```ignore
//! use paramdef::expr::{Expr, Rule};
//!
//! // Validate that "password" field has minimum length of 8
//! // This would be used in a parent object's validation rules
//! let rule = Rule::field("password", Expr::MinLength(8));
//! ```
//!
//! # Design Inspiration
//!
//! This system is inspired by:
//! - **JSON Schema**: Declarative validation with `required`, `minLength`, `pattern`, etc.
//! - **Zod/Yup**: Chainable validation with cross-field support
//! - **garde/validator**: Rust validation libraries
//! - **React Hook Form**: Field-based and cross-field validation

mod eval;
mod expr;
mod rule;
mod target;

#[cfg(feature = "validation")]
mod validate;

pub use expr::Expr;
pub use rule::Rule;
pub use target::ExprTarget;
