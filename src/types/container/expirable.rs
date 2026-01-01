//! Expirable container for TTL-wrapped values.
//!
//! Expirable wraps a child parameter with time-to-live expiration logic,
//! used for caching, sessions, and temporary data.

use std::any::Any;
use std::fmt;
use std::sync::Arc;

use crate::core::{Flags, Key, Metadata, SmartStr};
use crate::types::kind::NodeKind;
use crate::types::traits::{Container, Node};

/// Options for expirable values.
#[derive(Debug, Clone)]
pub struct ExpirableOptions {
    /// Time-to-live in seconds.
    pub ttl: u64,
    /// Whether to automatically refresh the TTL on access.
    pub auto_refresh: bool,
    /// Whether to automatically clear expired values.
    pub auto_clear_expired: bool,
    /// Seconds before expiry to show a warning (None = no warning).
    pub warning_threshold: Option<u64>,
}

impl Default for ExpirableOptions {
    fn default() -> Self {
        Self {
            ttl: 3600, // 1 hour default
            auto_refresh: false,
            auto_clear_expired: true,
            warning_threshold: None,
        }
    }
}

impl ExpirableOptions {
    /// Creates new expirable options with the given TTL in seconds.
    #[must_use]
    pub fn new(ttl: u64) -> Self {
        Self {
            ttl,
            ..Self::default()
        }
    }

    /// Creates options with TTL in minutes.
    ///
    /// Uses saturating multiplication to prevent overflow.
    #[must_use]
    pub fn minutes(minutes: u64) -> Self {
        Self::new(minutes.saturating_mul(60))
    }

    /// Creates options with TTL in hours.
    ///
    /// Uses saturating multiplication to prevent overflow.
    #[must_use]
    pub fn hours(hours: u64) -> Self {
        Self::new(hours.saturating_mul(3600))
    }

    /// Creates options with TTL in days.
    ///
    /// Uses saturating multiplication to prevent overflow.
    #[must_use]
    pub fn days(days: u64) -> Self {
        Self::new(days.saturating_mul(86400))
    }
}

/// A container for TTL-wrapped values.
///
/// Expirable is one of the six container types. It wraps a child node
/// with expiration metadata, producing `{ value, expires_at, created_at }`.
///
/// # Example
///
/// ```ignore
/// use paramdef::container::Expirable;
/// use paramdef::types::leaf::Text;
///
/// let token = Expirable::builder("cached_token")
///     .label("Cached Token")
///     .ttl_hours(1)
///     .auto_refresh(true)
///     .warning_threshold(300) // Warn 5 min before expiry
///     .child(Text::builder("token").build())
///     .build();
/// ```
#[derive(Clone)]
pub struct Expirable {
    metadata: Metadata,
    flags: Flags,
    child: Option<Arc<dyn Node>>,
    options: ExpirableOptions,
    /// Cached children for Container trait
    children_cache: Arc<[Arc<dyn Node>]>,
}

impl fmt::Debug for Expirable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Expirable")
            .field("metadata", &self.metadata)
            .field("flags", &self.flags)
            .field("has_child", &self.child.is_some())
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl Expirable {
    /// Creates a new builder for an Expirable container.
    #[must_use]
    pub fn builder(key: impl Into<Key>) -> ExpirableBuilder {
        ExpirableBuilder::new(key)
    }

    /// Returns the flags for this expirable.
    #[inline]
    #[must_use]
    pub fn flags(&self) -> Flags {
        self.flags
    }

    /// Returns the child node, if any.
    #[inline]
    #[must_use]
    pub fn child(&self) -> Option<&Arc<dyn Node>> {
        self.child.as_ref()
    }

    /// Returns the expirable options.
    #[inline]
    #[must_use]
    pub fn options(&self) -> &ExpirableOptions {
        &self.options
    }

    /// Returns the TTL in seconds.
    #[inline]
    #[must_use]
    pub fn ttl(&self) -> u64 {
        self.options.ttl
    }
}

impl Node for Expirable {
    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn key(&self) -> &Key {
        self.metadata.key()
    }

    fn kind(&self) -> NodeKind {
        NodeKind::Container
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Container for Expirable {
    fn children(&self) -> &[Arc<dyn Node>] {
        &self.children_cache
    }
}

// =============================================================================
// Builder
// =============================================================================

/// Builder for [`Expirable`].
pub struct ExpirableBuilder {
    key: Key,
    label: Option<SmartStr>,
    description: Option<SmartStr>,
    flags: Flags,
    child: Option<Arc<dyn Node>>,
    options: ExpirableOptions,
}

impl fmt::Debug for ExpirableBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExpirableBuilder")
            .field("key", &self.key)
            .field("label", &self.label)
            .field("description", &self.description)
            .field("flags", &self.flags)
            .field("has_child", &self.child.is_some())
            .field("options", &self.options)
            .finish()
    }
}

impl ExpirableBuilder {
    /// Creates a new builder with the given key.
    #[must_use]
    pub fn new(key: impl Into<Key>) -> Self {
        Self {
            key: key.into(),
            label: None,
            description: None,
            flags: Flags::empty(),
            child: None,
            options: ExpirableOptions::default(),
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

    /// Sets the child node.
    #[must_use]
    pub fn child(mut self, node: impl Node + 'static) -> Self {
        self.child = Some(Arc::new(node));
        self
    }

    /// Sets the TTL in seconds.
    #[must_use]
    pub fn ttl(mut self, seconds: u64) -> Self {
        self.options.ttl = seconds;
        self
    }

    /// Sets the TTL in minutes.
    ///
    /// Uses saturating multiplication to prevent overflow.
    #[must_use]
    pub fn ttl_minutes(mut self, minutes: u64) -> Self {
        self.options.ttl = minutes.saturating_mul(60);
        self
    }

    /// Sets the TTL in hours.
    ///
    /// Uses saturating multiplication to prevent overflow.
    #[must_use]
    pub fn ttl_hours(mut self, hours: u64) -> Self {
        self.options.ttl = hours.saturating_mul(3600);
        self
    }

    /// Sets the TTL in days.
    ///
    /// Uses saturating multiplication to prevent overflow.
    #[must_use]
    pub fn ttl_days(mut self, days: u64) -> Self {
        self.options.ttl = days.saturating_mul(86400);
        self
    }

    /// Sets whether to auto-refresh TTL on access.
    #[must_use]
    pub fn auto_refresh(mut self, auto_refresh: bool) -> Self {
        self.options.auto_refresh = auto_refresh;
        self
    }

    /// Sets whether to auto-clear expired values.
    #[must_use]
    pub fn auto_clear_expired(mut self, auto_clear: bool) -> Self {
        self.options.auto_clear_expired = auto_clear;
        self
    }

    /// Sets the warning threshold in seconds before expiry.
    #[must_use]
    pub fn warning_threshold(mut self, seconds: u64) -> Self {
        self.options.warning_threshold = Some(seconds);
        self
    }

    /// Builds the Expirable container.
    ///
    /// # Errors
    ///
    /// Returns an error if `warning_threshold` is greater than or equal to `ttl`.
    pub fn build(self) -> crate::core::Result<Expirable> {
        let mut metadata = Metadata::new(self.key);
        if let Some(label) = self.label {
            metadata = metadata.with_label(label);
        }
        if let Some(description) = self.description {
            metadata = metadata.with_description(description);
        }

        // Validate warning_threshold < ttl
        if let Some(threshold) = self.options.warning_threshold {
            if threshold >= self.options.ttl {
                return Err(crate::core::Error::validation(
                    "invalid_threshold",
                    format!(
                        "warning_threshold ({threshold}s) must be less than ttl ({}s)",
                        self.options.ttl
                    ),
                ));
            }
        }

        // Build children cache
        let children_cache: Arc<[Arc<dyn Node>]> = match &self.child {
            Some(child) => Arc::from([Arc::clone(child)]),
            None => Arc::from([]),
        };

        Ok(Expirable {
            metadata,
            flags: self.flags,
            child: self.child,
            options: self.options,
            children_cache,
        })
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::leaf::Text;

    #[test]
    fn test_expirable_basic() {
        let expirable = Expirable::builder("token")
            .label("Token")
            .ttl(3600)
            .build()
            .unwrap();

        assert_eq!(expirable.key().as_str(), "token");
        assert_eq!(expirable.metadata().label(), Some("Token"));
        assert_eq!(expirable.kind(), NodeKind::Container);
        assert_eq!(expirable.ttl(), 3600);
    }

    #[test]
    fn test_expirable_options() {
        let expirable = Expirable::builder("cache")
            .ttl_hours(2)
            .auto_refresh(true)
            .warning_threshold(300)
            .build()
            .unwrap();

        assert_eq!(expirable.options().ttl, 7200);
        assert!(expirable.options().auto_refresh);
        assert_eq!(expirable.options().warning_threshold, Some(300));
    }

    #[test]
    fn test_expirable_ttl_helpers() {
        let minutes = Expirable::builder("a").ttl_minutes(30).build().unwrap();
        assert_eq!(minutes.ttl(), 1800);

        let hours = Expirable::builder("b").ttl_hours(2).build().unwrap();
        assert_eq!(hours.ttl(), 7200);

        let days = Expirable::builder("c").ttl_days(1).build().unwrap();
        assert_eq!(days.ttl(), 86400);
    }

    #[test]
    fn test_expirable_with_child() {
        let expirable = Expirable::builder("cached_value")
            .child(Text::builder("value").build())
            .build()
            .unwrap();

        assert!(expirable.child().is_some());
        assert_eq!(expirable.child().unwrap().key().as_str(), "value");
    }

    #[test]
    fn test_expirable_options_constructors() {
        let opts = ExpirableOptions::minutes(30);
        assert_eq!(opts.ttl, 1800);

        let opts = ExpirableOptions::hours(2);
        assert_eq!(opts.ttl, 7200);

        let opts = ExpirableOptions::days(1);
        assert_eq!(opts.ttl, 86400);
    }

    #[test]
    fn test_expirable_warning_threshold_validation() {
        // Valid: warning_threshold < ttl
        let result = Expirable::builder("valid")
            .ttl(3600)
            .warning_threshold(300)
            .build();
        assert!(result.is_ok());

        // Invalid: warning_threshold == ttl
        let result = Expirable::builder("equal")
            .ttl(3600)
            .warning_threshold(3600)
            .build();
        assert!(result.is_err());

        // Invalid: warning_threshold > ttl
        let result = Expirable::builder("greater")
            .ttl(3600)
            .warning_threshold(7200)
            .build();
        assert!(result.is_err());
    }
}
