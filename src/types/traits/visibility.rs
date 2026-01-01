//! Visibility trait for conditional display.

use crate::core::Value;
use crate::types::traits::Node;

/// Trait for visibility control.
///
/// All 14 node types implement this trait when the `visibility` feature is
/// enabled. Provides methods to evaluate conditional visibility based on
/// other parameter values.
///
/// # Example
///
/// ```ignore
/// use paramdef::types::traits::Visibility;
/// use paramdef::types::leaf::Text;
/// use paramdef::core::Value;
///
/// let mut text = Text::builder("advanced_option").build();
///
/// // Set visibility condition
/// text.set_visibility_expr(Some(Value::text("{{show_advanced}} == true")));
///
/// // Check if visible (would evaluate expression in real implementation)
/// assert!(text.is_visible());
/// ```
pub trait Visibility: Node {
    /// Returns the visibility expression, if any.
    fn visibility_expr(&self) -> Option<&Value>;

    /// Sets the visibility expression.
    fn set_visibility_expr(&mut self, expr: Option<Value>);

    /// Returns whether the node is currently visible.
    ///
    /// If no visibility expression is set, returns `true`.
    fn is_visible(&self) -> bool {
        true
    }

    /// Returns the keys that this node's visibility depends on.
    fn dependencies(&self) -> Vec<String> {
        Vec::new()
    }
}
