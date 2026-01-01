//! Progress decoration for displaying progress indicators.
//!
//! Progress displays completion status, loading indicators, or step progress
//! as a display-only element in the UI.

use std::any::Any;

use crate::core::{Flags, Key, Metadata, SmartStr};
use crate::types::kind::NodeKind;
use crate::types::traits::{Decoration, Node};

/// Visual style for progress display.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressStyle {
    /// Horizontal progress bar (default).
    #[default]
    Bar,
    /// Circular progress indicator.
    Circle,
    /// Step-based progress (1/5, 2/5, etc.).
    Steps,
    /// Percentage text only.
    Text,
    /// Indeterminate spinner (for unknown duration).
    Spinner,
}

impl ProgressStyle {
    /// Returns the name of this style.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::Bar => "bar",
            Self::Circle => "circle",
            Self::Steps => "steps",
            Self::Text => "text",
            Self::Spinner => "spinner",
        }
    }

    /// Returns true if this style shows a determinate progress.
    #[must_use]
    pub fn is_determinate(&self) -> bool {
        !matches!(self, Self::Spinner)
    }
}

/// Source for progress value.
///
/// Progress can display a static value, bind to a parameter,
/// or compute from an expression.
#[derive(Debug, Clone, PartialEq)]
pub enum ProgressSource {
    /// Static value (0.0 to 1.0 or 0 to 100 depending on context).
    Static(f64),
    /// Bind to a parameter key (reads value from context).
    Parameter(Key),
    /// Expression to compute progress (e.g., `filled / total`).
    /// Note: Expression evaluation requires the visibility feature.
    Expression(SmartStr),
}

impl ProgressSource {
    /// Creates a static progress source.
    #[must_use]
    pub fn static_value(value: f64) -> Self {
        Self::Static(value.clamp(0.0, 1.0))
    }

    /// Creates a parameter binding source.
    #[must_use]
    pub fn parameter(key: impl Into<Key>) -> Self {
        Self::Parameter(key.into())
    }

    /// Creates an expression source.
    #[must_use]
    pub fn expression(expr: impl Into<SmartStr>) -> Self {
        Self::Expression(expr.into())
    }
}

/// Progress display options packed into a single struct.
#[derive(Debug, Clone, Copy, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct ProgressOptions {
    /// Show percentage text.
    pub show_percentage: bool,
    /// Show value text (e.g., "75/100").
    pub show_value: bool,
    /// Whether progress is animated.
    pub animated: bool,
    /// Whether progress bar has stripes.
    pub striped: bool,
}

/// A display-only progress indicator decoration.
///
/// Progress displays completion status or loading state. It has no value
/// and cannot contain children.
///
/// # Example
///
/// ```ignore
/// use paramdef::decoration::{Progress, ProgressStyle, ProgressSource};
///
/// // Simple progress bar with static value
/// let loading = Progress::bar("loading", 0.75)
///     .label("Loading...");
///
/// // Circular progress bound to a parameter
/// let completion = Progress::builder("completion")
///     .style(ProgressStyle::Circle)
///     .bind_to("progress_value")
///     .show_percentage(true)
///     .build();
///
/// // Step-based progress
/// let wizard = Progress::steps("wizard_progress", 3, 5)
///     .label("Step 3 of 5");
///
/// // Form completion progress from expression
/// let form_progress = Progress::builder("form_completion")
///     .expression("filled_fields / total_fields")
///     .label("Form Completion")
///     .show_percentage(true)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct Progress {
    metadata: Metadata,
    flags: Flags,
    source: ProgressSource,
    style: ProgressStyle,
    /// For step-based progress: total number of steps.
    total_steps: Option<u32>,
    /// Display options.
    options: ProgressOptions,
    /// Color or theme variant.
    color: Option<SmartStr>,
    /// Size variant (small, medium, large).
    size: Option<SmartStr>,
}

impl Progress {
    /// Creates a new builder for a Progress.
    #[must_use]
    pub fn builder(key: impl Into<Key>) -> ProgressBuilder {
        ProgressBuilder::new(key)
    }

    /// Creates a simple progress bar with a static value (0.0 to 1.0).
    #[must_use]
    pub fn bar(key: impl Into<Key>, value: f64) -> ProgressBuilder {
        ProgressBuilder::new(key)
            .source(ProgressSource::static_value(value))
            .style(ProgressStyle::Bar)
    }

    /// Creates a circular progress indicator.
    #[must_use]
    pub fn circle(key: impl Into<Key>, value: f64) -> ProgressBuilder {
        ProgressBuilder::new(key)
            .source(ProgressSource::static_value(value))
            .style(ProgressStyle::Circle)
    }

    /// Creates a step-based progress indicator.
    #[must_use]
    pub fn steps(key: impl Into<Key>, current: u32, total: u32) -> ProgressBuilder {
        let value = if total > 0 {
            f64::from(current) / f64::from(total)
        } else {
            0.0
        };
        ProgressBuilder::new(key)
            .source(ProgressSource::static_value(value))
            .style(ProgressStyle::Steps)
            .total_steps(total)
    }

    /// Creates an indeterminate spinner.
    #[must_use]
    pub fn spinner(key: impl Into<Key>) -> ProgressBuilder {
        ProgressBuilder::new(key)
            .source(ProgressSource::static_value(0.0))
            .style(ProgressStyle::Spinner)
    }

    /// Returns the flags for this progress.
    #[must_use]
    pub fn flags(&self) -> Flags {
        self.flags
    }

    /// Returns the progress source.
    #[must_use]
    pub fn source(&self) -> &ProgressSource {
        &self.source
    }

    /// Returns the progress style.
    #[must_use]
    pub fn style(&self) -> ProgressStyle {
        self.style
    }

    /// Returns the total number of steps (for step-based progress).
    #[must_use]
    pub fn total_steps(&self) -> Option<u32> {
        self.total_steps
    }

    /// Returns true if percentage should be shown.
    #[must_use]
    pub fn show_percentage(&self) -> bool {
        self.options.show_percentage
    }

    /// Returns true if value should be shown.
    #[must_use]
    pub fn show_value(&self) -> bool {
        self.options.show_value
    }

    /// Returns the color variant, if set.
    #[must_use]
    pub fn color(&self) -> Option<&str> {
        self.color.as_deref()
    }

    /// Returns the size variant, if set.
    #[must_use]
    pub fn size(&self) -> Option<&str> {
        self.size.as_deref()
    }

    /// Returns true if the progress is animated.
    #[must_use]
    pub fn animated(&self) -> bool {
        self.options.animated
    }

    /// Returns true if the progress bar has stripes.
    #[must_use]
    pub fn striped(&self) -> bool {
        self.options.striped
    }

    /// Returns true if this is an indeterminate progress.
    #[must_use]
    pub fn is_indeterminate(&self) -> bool {
        matches!(self.style, ProgressStyle::Spinner)
    }
}

impl Node for Progress {
    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn key(&self) -> &Key {
        self.metadata.key()
    }

    fn kind(&self) -> NodeKind {
        NodeKind::Decoration
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Decoration for Progress {}

// =============================================================================
// Builder
// =============================================================================

/// Builder for [`Progress`].
#[derive(Debug)]
pub struct ProgressBuilder {
    key: Key,
    label: Option<SmartStr>,
    description: Option<SmartStr>,
    flags: Flags,
    source: ProgressSource,
    style: ProgressStyle,
    total_steps: Option<u32>,
    options: ProgressOptions,
    color: Option<SmartStr>,
    size: Option<SmartStr>,
}

impl ProgressBuilder {
    /// Creates a new builder with the given key.
    #[must_use]
    pub fn new(key: impl Into<Key>) -> Self {
        Self {
            key: key.into(),
            label: None,
            description: None,
            flags: Flags::empty(),
            source: ProgressSource::Static(0.0),
            style: ProgressStyle::default(),
            total_steps: None,
            options: ProgressOptions::default(),
            color: None,
            size: None,
        }
    }

    /// Sets the label.
    #[must_use]
    pub fn label(mut self, label: impl Into<SmartStr>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the description.
    #[must_use]
    pub fn description(mut self, description: impl Into<SmartStr>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the flags.
    #[must_use]
    pub fn flags(mut self, flags: Flags) -> Self {
        self.flags = flags;
        self
    }

    /// Sets the progress source.
    #[must_use]
    pub fn source(mut self, source: ProgressSource) -> Self {
        self.source = source;
        self
    }

    /// Sets a static progress value (0.0 to 1.0).
    #[must_use]
    pub fn value(mut self, value: f64) -> Self {
        self.source = ProgressSource::static_value(value);
        self
    }

    /// Binds progress to a parameter key.
    #[must_use]
    pub fn bind_to(mut self, key: impl Into<Key>) -> Self {
        self.source = ProgressSource::parameter(key);
        self
    }

    /// Sets an expression for computing progress.
    #[must_use]
    pub fn expression(mut self, expr: impl Into<SmartStr>) -> Self {
        self.source = ProgressSource::expression(expr);
        self
    }

    /// Sets the progress style.
    #[must_use]
    pub fn style(mut self, style: ProgressStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets the total number of steps (for step-based progress).
    #[must_use]
    pub fn total_steps(mut self, total: u32) -> Self {
        self.total_steps = Some(total);
        self
    }

    /// Shows percentage text.
    #[must_use]
    pub fn show_percentage(mut self, show: bool) -> Self {
        self.options.show_percentage = show;
        self
    }

    /// Shows value text.
    #[must_use]
    pub fn show_value(mut self, show: bool) -> Self {
        self.options.show_value = show;
        self
    }

    /// Sets the color variant.
    #[must_use]
    pub fn color(mut self, color: impl Into<SmartStr>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Sets the size variant.
    #[must_use]
    pub fn size_variant(mut self, size: impl Into<SmartStr>) -> Self {
        self.size = Some(size.into());
        self
    }

    /// Enables animation.
    #[must_use]
    pub fn animated(mut self, animated: bool) -> Self {
        self.options.animated = animated;
        self
    }

    /// Enables striped styling.
    #[must_use]
    pub fn striped(mut self, striped: bool) -> Self {
        self.options.striped = striped;
        self
    }

    /// Builds the Progress decoration.
    #[must_use]
    pub fn build(self) -> Progress {
        let mut metadata = Metadata::new(self.key);
        if let Some(label) = self.label {
            metadata = metadata.with_label(label);
        }
        if let Some(description) = self.description {
            metadata = metadata.with_description(description);
        }

        Progress {
            metadata,
            flags: self.flags,
            source: self.source,
            style: self.style,
            total_steps: self.total_steps,
            options: self.options,
            color: self.color,
            size: self.size,
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_bar() {
        let progress = Progress::bar("loading", 0.75)
            .label("Loading...")
            .build();

        assert_eq!(progress.key().as_str(), "loading");
        assert_eq!(progress.metadata().label(), Some("Loading..."));
        assert_eq!(progress.style(), ProgressStyle::Bar);
        assert!(matches!(progress.source(), ProgressSource::Static(v) if (*v - 0.75).abs() < f64::EPSILON));
    }

    #[test]
    fn test_progress_circle() {
        let progress = Progress::circle("completion", 0.5).build();

        assert_eq!(progress.style(), ProgressStyle::Circle);
    }

    #[test]
    fn test_progress_steps() {
        let progress = Progress::steps("wizard", 3, 5)
            .label("Step 3 of 5")
            .build();

        assert_eq!(progress.style(), ProgressStyle::Steps);
        assert_eq!(progress.total_steps(), Some(5));
        // 3/5 = 0.6
        assert!(matches!(progress.source(), ProgressSource::Static(v) if (*v - 0.6).abs() < f64::EPSILON));
    }

    #[test]
    fn test_progress_spinner() {
        let progress = Progress::spinner("loading").build();

        assert_eq!(progress.style(), ProgressStyle::Spinner);
        assert!(progress.is_indeterminate());
        assert!(!progress.style().is_determinate());
    }

    #[test]
    fn test_progress_bind_to_parameter() {
        let progress = Progress::builder("bound")
            .bind_to("progress_value")
            .build();

        assert!(matches!(
            progress.source(),
            ProgressSource::Parameter(k) if k.as_str() == "progress_value"
        ));
    }

    #[test]
    fn test_progress_expression() {
        let progress = Progress::builder("form")
            .expression("filled_fields / total_fields")
            .build();

        assert!(matches!(
            progress.source(),
            ProgressSource::Expression(e) if e == "filled_fields / total_fields"
        ));
    }

    #[test]
    fn test_progress_builder() {
        let progress = Progress::builder("custom")
            .value(0.8)
            .style(ProgressStyle::Bar)
            .show_percentage(true)
            .show_value(true)
            .color("primary")
            .size_variant("large")
            .animated(true)
            .striped(true)
            .build();

        assert!(progress.show_percentage());
        assert!(progress.show_value());
        assert_eq!(progress.color(), Some("primary"));
        assert_eq!(progress.size(), Some("large"));
        assert!(progress.animated());
        assert!(progress.striped());
    }

    #[test]
    fn test_progress_value_clamping() {
        // Value should be clamped to 0.0-1.0
        let source = ProgressSource::static_value(1.5);
        assert!(matches!(source, ProgressSource::Static(v) if (v - 1.0).abs() < f64::EPSILON));

        let source = ProgressSource::static_value(-0.5);
        assert!(matches!(source, ProgressSource::Static(v) if v.abs() < f64::EPSILON));
    }

    #[test]
    fn test_progress_kind() {
        let progress = Progress::bar("test", 0.5).build();

        assert_eq!(progress.kind(), NodeKind::Decoration);
    }

    #[test]
    fn test_progress_invariants() {
        let progress = Progress::bar("test", 0.5).build();

        // Decoration has NO own value
        assert!(!progress.kind().has_own_value());

        // Decoration has NO ValueAccess
        assert!(!progress.kind().has_value_access());

        // Decoration CANNOT have children
        assert!(!progress.kind().can_have_children());
    }

    #[test]
    fn test_progress_style_names() {
        assert_eq!(ProgressStyle::Bar.name(), "bar");
        assert_eq!(ProgressStyle::Circle.name(), "circle");
        assert_eq!(ProgressStyle::Steps.name(), "steps");
        assert_eq!(ProgressStyle::Text.name(), "text");
        assert_eq!(ProgressStyle::Spinner.name(), "spinner");
    }

    #[test]
    fn test_progress_style_determinate() {
        assert!(ProgressStyle::Bar.is_determinate());
        assert!(ProgressStyle::Circle.is_determinate());
        assert!(ProgressStyle::Steps.is_determinate());
        assert!(ProgressStyle::Text.is_determinate());
        assert!(!ProgressStyle::Spinner.is_determinate());
    }
}
