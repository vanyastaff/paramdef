//! Visibility trait for conditional display.

use crate::core::Key;

#[cfg(feature = "visibility")]
use crate::context::Context;
#[cfg(feature = "visibility")]
use crate::expr::Rule;

/// Trait for visibility control.
///
/// All 23 node types implement this trait when the `visibility` feature is
/// enabled. Provides methods to set and evaluate conditional visibility based
/// on other parameter values.
///
/// # Design
///
/// - **Schema stores the rule**: `Option<Rule>` stored in each node
/// - **Evaluation requires Context**: `is_visible(&Context)` evaluates the rule
/// - **Dependencies tracked**: `dependencies()` returns keys this visibility depends on
///
/// # Example
///
/// ```
/// use paramdef::visibility::when;
/// use paramdef::types::leaf::Text;
/// use paramdef::context::Context;
/// use paramdef::schema::Schema;
/// use paramdef::core::Value;
/// use std::sync::Arc;
///
/// // Build schema with visibility condition using fluent API
/// let schema = Arc::new(Schema::builder()
///     .parameter(
///         Text::builder("show_advanced")
///             .default("false")
///             .build()
///     )
///     .parameter(
///         Text::builder("advanced_option")
///             .visible_when(when("show_advanced").eq(Value::text("true")))
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
/// ctx.set("show_advanced", Value::text("true")).expect("Failed to set value");
///
/// // Check visibility again (should be visible now)
/// {
///     let node = ctx.schema().get("advanced_option").unwrap();
///     assert_eq!(node.is_visible(&ctx), true);
/// }
/// ```
#[cfg(feature = "visibility")]
pub trait Visibility {
    /// Returns the visibility rule, if any.
    fn visibility_rule(&self) -> Option<&Rule>;

    /// Sets the visibility rule.
    ///
    /// **Deprecated**: Visibility rules should be set during construction via builder methods.
    /// Schema types should be immutable after construction to maintain architectural invariants.
    ///
    /// Use `.visible_when(rule)` on the builder instead:
    ///
    /// ```ignore
    /// // ❌ Old way (deprecated):
    /// let mut node = Text::builder("field").build();
    /// node.set_visibility_rule(Some(rule));
    ///
    /// // ✅ New way:
    /// let node = Text::builder("field")
    ///     .visible_when(rule)
    ///     .build();
    /// ```
    ///
    /// This method will be removed in version 0.5.0.
    #[deprecated(
        since = "0.4.0",
        note = "Set visibility via builder. Use .visible_when() during construction. Schema should be immutable after build()."
    )]
    fn set_visibility_rule(&mut self, rule: Option<Rule>);

    /// Evaluates whether the node is currently visible in the given context.
    ///
    /// Returns `true` if:
    /// - No visibility rule is set (always visible)
    /// - The visibility rule evaluates to `true`
    ///
    /// Returns `false` if the visibility rule evaluates to `false`.
    fn is_visible(&self, ctx: &Context) -> bool {
        match self.visibility_rule() {
            Some(rule) => rule.eval(ctx),
            None => true, // No condition = always visible
        }
    }

    /// Returns the parameter keys that this node's visibility depends on.
    ///
    /// This is used for reactive updates - when a dependency changes,
    /// the visibility can be re-evaluated.
    fn dependencies(&self) -> Vec<Key> {
        match self.visibility_rule() {
            Some(rule) => rule.dependencies(),
            None => Vec::new(),
        }
    }
}
