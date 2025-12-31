//! Reference container for template nodes.
//!
//! Reference (Ref in the type system docs) points to a template node,
//! allowing structure reuse without duplication.

use std::any::Any;
use std::sync::Arc;

use crate::core::{Flags, Key, Metadata, SmartStr};
use crate::node::{Container, Node, NodeKind};

/// A reference to a template node.
///
/// Reference is one of the six container types. It points to another
/// node's structure without duplicating it. Each reference has its own
/// metadata and visibility but delegates structure to the target.
///
/// # Example
///
/// ```ignore
/// use paramdef::container::{Reference, Object};
/// use paramdef::parameter::Text;
///
/// // Define a reusable address template
/// let address_template = Object::builder("address_template")
///     .field("street", Text::builder("street").required().build())
///     .field("city", Text::builder("city").required().build())
///     .field("zip", Text::builder("zip").build())
///     .build();
///
/// // Reference it multiple times with different metadata
/// let billing = Reference::builder("billing_address")
///     .label("Billing Address")
///     .target("address_template")
///     .build();
///
/// let shipping = Reference::builder("shipping_address")
///     .label("Shipping Address")
///     .target("address_template")
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct Reference {
    metadata: Metadata,
    flags: Flags,
    target: Key,
}

impl Reference {
    /// Creates a new builder for a Reference.
    #[must_use]
    pub fn builder(key: impl Into<Key>) -> ReferenceBuilder {
        ReferenceBuilder::new(key)
    }

    /// Returns the flags for this reference.
    #[must_use]
    pub fn flags(&self) -> Flags {
        self.flags
    }

    /// Returns the target key (the template being referenced).
    #[must_use]
    pub fn target(&self) -> &Key {
        &self.target
    }
}

impl Node for Reference {
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

impl Container for Reference {
    fn children(&self) -> &[Arc<dyn Node>] {
        // Reference is special: it has no schema-level children.
        // Children are resolved from the target template at runtime
        // by the Context or RuntimeParameter.
        &[]
    }
}

// =============================================================================
// Builder
// =============================================================================

/// Builder for [`Reference`].
#[derive(Debug)]
pub struct ReferenceBuilder {
    key: Key,
    label: Option<SmartStr>,
    description: Option<SmartStr>,
    flags: Flags,
    target: Option<Key>,
}

impl ReferenceBuilder {
    /// Creates a new builder with the given key.
    #[must_use]
    pub fn new(key: impl Into<Key>) -> Self {
        Self {
            key: key.into(),
            label: None,
            description: None,
            flags: Flags::empty(),
            target: None,
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

    /// Sets the target key (the template to reference).
    #[must_use]
    pub fn target(mut self, target: impl Into<Key>) -> Self {
        self.target = Some(target.into());
        self
    }

    /// Builds the Reference.
    ///
    /// # Errors
    ///
    /// Returns an error if no target was specified.
    pub fn build(self) -> crate::core::Result<Reference> {
        let mut metadata = Metadata::new(self.key);
        if let Some(label) = self.label {
            metadata = metadata.with_label(label);
        }
        if let Some(description) = self.description {
            metadata = metadata.with_description(description);
        }

        let target = self
            .target
            .ok_or_else(|| crate::core::Error::missing_required("target"))?;

        Ok(Reference {
            metadata,
            flags: self.flags,
            target,
        })
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reference_basic() {
        let reference = Reference::builder("billing")
            .label("Billing Address")
            .target("address_template")
            .build()
            .unwrap();

        assert_eq!(reference.key().as_str(), "billing");
        assert_eq!(reference.metadata().label(), Some("Billing Address"));
        assert_eq!(reference.target().as_str(), "address_template");
        assert_eq!(reference.kind(), NodeKind::Container);
    }

    #[test]
    fn test_reference_flags() {
        let reference = Reference::builder("ref")
            .target("template")
            .flags(Flags::REQUIRED)
            .build()
            .unwrap();

        assert!(reference.flags().contains(Flags::REQUIRED));
    }

    #[test]
    fn test_reference_requires_target() {
        let result = Reference::builder("no_target").build();
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_references_same_target() {
        let billing = Reference::builder("billing")
            .label("Billing")
            .target("address")
            .build()
            .unwrap();

        let shipping = Reference::builder("shipping")
            .label("Shipping")
            .target("address")
            .build()
            .unwrap();

        // Both reference the same target but have different keys
        assert_eq!(billing.target(), shipping.target());
        assert_ne!(billing.key(), shipping.key());
    }
}
