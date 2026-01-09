//! Visibility system for conditional display of parameters.
//!
//! The visibility system enables dynamic show/hide behavior based on other parameter
//! values. This is essential for building adaptive UIs that only show relevant options.
//!
//! ## Design Principles
//!
//! 1. **Simple expressions for common cases**: Most visibility needs are simple
//!    comparisons like "show if X is true" or "hide if Y is empty"
//! 2. **Composable logic**: Complex conditions built from simple expressions using
//!    And, Or, Not operators
//! 3. **Dependency tracking**: Automatically track which parameters affect visibility
//!    for efficient reactive updates
//! 4. **Type-safe evaluation**: Expressions check types at evaluation time and handle
//!    mismatches gracefully
//!
//! ## Industry Patterns
//!
//! | Source | Pattern Adopted |
//! |--------|-----------------|
//! | JSON Schema | Conditional schemas (`if`/`then`/`else`) |
//! | React Hook Form | Field dependencies with `watch()` |
//! | Formik | Conditional rendering based on values |
//! | Angular Forms | Dynamic form controls |
//!
//! ## Example
//!
//! ```
//! use paramdef::visibility::Expr;
//! use paramdef::context::Context;
//! use paramdef::core::{Key, Value};
//! # use paramdef::schema::Schema;
//! # use paramdef::types::leaf::{Text, Boolean};
//! # use std::sync::Arc;
//!
//! # let schema = Arc::new(Schema::builder()
//! #     .parameter(Boolean::builder("show_advanced").default(false).build())
//! #     .parameter(Text::builder("advanced_option").build())
//! #     .build());
//! # let mut ctx = Context::new(schema);
//! // Create visibility condition: show only when checkbox is true
//! let expr = Expr::is_true("show_advanced");
//!
//! // Evaluate visibility
//! assert_eq!(expr.eval(&ctx), false); // Checkbox not set
//!
//! ctx.set("show_advanced", Value::Bool(true));
//! assert_eq!(expr.eval(&ctx), true); // Now visible
//!
//! // Get dependencies for reactive updates
//! let deps = expr.dependencies();
//! assert_eq!(deps, vec![Key::from("show_advanced")]);
//! ```

mod expr;

pub use expr::Expr;
