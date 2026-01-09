//! Visibility trait for conditional display.

use crate::core::Key;

#[cfg(feature = "visibility")]
use crate::context::Context;
#[cfg(feature = "visibility")]
use crate::visibility::Expr;

/// Trait for visibility control.
///
/// All 23 node types implement this trait when the `visibility` feature is
/// enabled. Provides methods to set and evaluate conditional visibility based
/// on other parameter values.
///
/// # Design
///
/// - **Schema stores the expression**: `Option<Expr>` stored in each node
/// - **Evaluation requires Context**: `is_visible(&Context)` evaluates the expression
/// - **Dependencies tracked**: `dependencies()` returns keys this visibility depends on
///
/// # Example
///
/// ```
/// use paramdef::visibility::Expr;
/// use paramdef::types::leaf::Text;
/// use paramdef::context::Context;
/// use paramdef::schema::Schema;
/// use paramdef::core::Value;
/// use std::sync::Arc;
///
/// // Build schema with visibility condition
/// let schema = Arc::new(Schema::builder()
///     .parameter(
///         Text::builder("show_advanced")
///             .default("false")
///             .build()
///     )
///     .parameter(
///         Text::builder("advanced_option")
///             .visible_when(Expr::eq("show_advanced", Value::text("true")))
///             .build()
///     )
///     .build());
///
/// let mut ctx = Context::new(schema.clone());
///
/// // Check initial visibility (show_advanced is "false")
/// {
///     let node = ctx.schema().get("advanced_option").unwrap();
///     assert_eq!(node.is_visible(&ctx), false);
/// }
///
/// // Set show_advanced to true
/// let _ = ctx.set("show_advanced", Value::text("true"));
///
/// // Check visibility again (should be visible now)
/// {
///     let node = ctx.schema().get("advanced_option").unwrap();
///     assert_eq!(node.is_visible(&ctx), true);
/// }
/// ```
#[cfg(feature = "visibility")]
pub trait Visibility {
    /// Returns the visibility expression, if any.
    fn visibility_expr(&self) -> Option<&Expr>;

    /// Sets the visibility expression.
    ///
    /// This is typically used by builders, not at runtime.
    fn set_visibility_expr(&mut self, expr: Option<Expr>);

    /// Evaluates whether the node is currently visible in the given context.
    ///
    /// Returns `true` if:
    /// - No visibility expression is set (always visible)
    /// - The visibility expression evaluates to `true`
    ///
    /// Returns `false` if the visibility expression evaluates to `false`.
    fn is_visible(&self, ctx: &Context) -> bool {
        match self.visibility_expr() {
            Some(expr) => expr.eval(ctx),
            None => true, // No condition = always visible
        }
    }

    /// Returns the parameter keys that this node's visibility depends on.
    ///
    /// This is used for reactive updates - when a dependency changes,
    /// the visibility can be re-evaluated.
    fn dependencies(&self) -> Vec<Key> {
        match self.visibility_expr() {
            Some(expr) => expr.dependencies(),
            None => Vec::new(),
        }
    }
}
