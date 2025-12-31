//! Routing container for workflow connections.
//!
//! Routing wraps a child parameter with workflow connection capabilities,
//! used in visual programming and node-based tools.

use std::any::Any;
use std::fmt;
use std::sync::Arc;

use crate::core::{Flags, Key, Metadata, SmartStr};
use crate::node::{Container, Node, NodeKind};

/// Options for routing connections.
#[derive(Debug, Clone, Default)]
pub struct RoutingOptions {
    /// Label for the connection point.
    pub connection_label: Option<SmartStr>,
    /// Whether a connection is required.
    pub connection_required: bool,
    /// Maximum number of connections (None = unlimited).
    pub max_connections: Option<usize>,
}

impl RoutingOptions {
    /// Creates new routing options.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the connection label.
    #[must_use]
    pub fn connection_label(mut self, label: impl Into<SmartStr>) -> Self {
        self.connection_label = Some(label.into());
        self
    }

    /// Sets whether a connection is required.
    #[must_use]
    pub fn connection_required(mut self, required: bool) -> Self {
        self.connection_required = required;
        self
    }

    /// Sets the maximum number of connections.
    #[must_use]
    pub fn max_connections(mut self, max: usize) -> Self {
        self.max_connections = Some(max);
        self
    }
}

/// A container for workflow connections.
///
/// Routing is one of the six container types. It wraps a child node
/// with connection capabilities for workflow/node-based tools.
///
/// # Example
///
/// ```ignore
/// use paramdef::container::{Routing, Object};
/// use paramdef::parameter::{Text, Number};
///
/// let input = Routing::builder("input_data")
///     .label("Data Input")
///     .connection_label("Data In")
///     .connection_required(true)
///     .max_connections(1)
///     .child(Object::builder("payload")
///         .field("id", Text::builder("id").build())
///         .field("value", Number::float("value").build())
///         .build())
///     .build();
/// ```
#[derive(Clone)]
pub struct Routing {
    metadata: Metadata,
    flags: Flags,
    child: Option<Arc<dyn Node>>,
    options: RoutingOptions,
    /// Cached children for Container trait
    children_cache: Arc<[Arc<dyn Node>]>,
}

impl fmt::Debug for Routing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Routing")
            .field("metadata", &self.metadata)
            .field("flags", &self.flags)
            .field("has_child", &self.child.is_some())
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl Routing {
    /// Creates a new builder for a Routing container.
    #[must_use]
    pub fn builder(key: impl Into<Key>) -> RoutingBuilder {
        RoutingBuilder::new(key)
    }

    /// Returns the flags for this routing.
    #[must_use]
    pub fn flags(&self) -> Flags {
        self.flags
    }

    /// Returns the child node, if any.
    #[must_use]
    pub fn child(&self) -> Option<&Arc<dyn Node>> {
        self.child.as_ref()
    }

    /// Returns the routing options.
    #[must_use]
    pub fn options(&self) -> &RoutingOptions {
        &self.options
    }
}

impl Node for Routing {
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

impl Container for Routing {
    fn children(&self) -> &[Arc<dyn Node>] {
        &self.children_cache
    }
}

// =============================================================================
// Builder
// =============================================================================

/// Builder for [`Routing`].
pub struct RoutingBuilder {
    key: Key,
    label: Option<SmartStr>,
    description: Option<SmartStr>,
    flags: Flags,
    child: Option<Arc<dyn Node>>,
    options: RoutingOptions,
}

impl fmt::Debug for RoutingBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RoutingBuilder")
            .field("key", &self.key)
            .field("label", &self.label)
            .field("description", &self.description)
            .field("flags", &self.flags)
            .field("has_child", &self.child.is_some())
            .field("options", &self.options)
            .finish()
    }
}

impl RoutingBuilder {
    /// Creates a new builder with the given key.
    #[must_use]
    pub fn new(key: impl Into<Key>) -> Self {
        Self {
            key: key.into(),
            label: None,
            description: None,
            flags: Flags::empty(),
            child: None,
            options: RoutingOptions::default(),
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

    /// Sets the connection label.
    #[must_use]
    pub fn connection_label(mut self, label: impl Into<SmartStr>) -> Self {
        self.options.connection_label = Some(label.into());
        self
    }

    /// Sets whether a connection is required.
    #[must_use]
    pub fn connection_required(mut self, required: bool) -> Self {
        self.options.connection_required = required;
        self
    }

    /// Sets the maximum number of connections.
    #[must_use]
    pub fn max_connections(mut self, max: usize) -> Self {
        self.options.max_connections = Some(max);
        self
    }

    /// Builds the Routing container.
    #[must_use]
    pub fn build(self) -> Routing {
        let mut metadata = Metadata::new(self.key);
        if let Some(label) = self.label {
            metadata = metadata.with_label(label);
        }
        if let Some(description) = self.description {
            metadata = metadata.with_description(description);
        }

        // Build children cache
        let children_cache: Arc<[Arc<dyn Node>]> = match &self.child {
            Some(child) => Arc::from([Arc::clone(child)]),
            None => Arc::from([]),
        };

        Routing {
            metadata,
            flags: self.flags,
            child: self.child,
            options: self.options,
            children_cache,
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameter::Text;

    #[test]
    fn test_routing_basic() {
        let routing = Routing::builder("input")
            .label("Input")
            .connection_label("Data In")
            .build();

        assert_eq!(routing.key().as_str(), "input");
        assert_eq!(routing.metadata().label(), Some("Input"));
        assert_eq!(routing.kind(), NodeKind::Container);
    }

    #[test]
    fn test_routing_options() {
        let routing = Routing::builder("input")
            .connection_label("In")
            .connection_required(true)
            .max_connections(1)
            .build();

        assert_eq!(routing.options().connection_label.as_deref(), Some("In"));
        assert!(routing.options().connection_required);
        assert_eq!(routing.options().max_connections, Some(1));
    }

    #[test]
    fn test_routing_with_child() {
        let routing = Routing::builder("input")
            .child(Text::builder("data").build())
            .build();

        assert!(routing.child().is_some());
        assert_eq!(routing.child().unwrap().key().as_str(), "data");
    }
}
