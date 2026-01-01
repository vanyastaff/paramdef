//! Notice decoration for displaying messages.
//!
//! Notice displays info, warning, error, success, or tip messages.

use std::any::Any;

use crate::core::{Flags, Key, Metadata, SmartStr};
use crate::types::kind::NoticeType;
use crate::types::kind::NodeKind;
use crate::types::traits::{Decoration, Node};

/// A display-only message decoration.
///
/// Notice displays informational messages, warnings, errors, success messages,
/// or tips. It has no value and cannot contain children.
///
/// # Example
///
/// ```ignore
/// use paramdef::decoration::Notice;
/// use paramdef::node::NoticeType;
///
/// // Info message
/// let info = Notice::info("welcome", "Configure your settings below.");
///
/// // Warning with dismissible option
/// let warning = Notice::builder("deprecation")
///     .notice_type(NoticeType::Warning)
///     .message("This feature will be removed in v2.0.")
///     .dismissible(true)
///     .build();
///
/// // Error message
/// let error = Notice::error("connection", "Unable to connect to database.");
///
/// // Success message
/// let success = Notice::success("saved", "Settings saved successfully.");
///
/// // Tip message
/// let tip = Notice::tip("hint", "Press Ctrl+S to save quickly.");
/// ```
#[derive(Debug, Clone)]
pub struct Notice {
    metadata: Metadata,
    flags: Flags,
    kind: NoticeType,
    message: SmartStr,
    dismissible: bool,
}

impl Notice {
    /// Creates a new builder for a Notice.
    #[must_use]
    pub fn builder(key: impl Into<Key>) -> NoticeBuilder {
        NoticeBuilder::new(key)
    }

    /// Creates an info notice.
    #[must_use]
    pub fn info(key: impl Into<Key>, message: impl Into<SmartStr>) -> Self {
        Self::builder(key)
            .notice_type(NoticeType::Info)
            .message(message)
            .build()
    }

    /// Creates a warning notice.
    #[must_use]
    pub fn warning(key: impl Into<Key>, message: impl Into<SmartStr>) -> Self {
        Self::builder(key)
            .notice_type(NoticeType::Warning)
            .message(message)
            .build()
    }

    /// Creates an error notice.
    #[must_use]
    pub fn error(key: impl Into<Key>, message: impl Into<SmartStr>) -> Self {
        Self::builder(key)
            .notice_type(NoticeType::Error)
            .message(message)
            .build()
    }

    /// Creates a success notice.
    #[must_use]
    pub fn success(key: impl Into<Key>, message: impl Into<SmartStr>) -> Self {
        Self::builder(key)
            .notice_type(NoticeType::Success)
            .message(message)
            .build()
    }

    /// Creates a tip notice.
    #[must_use]
    pub fn tip(key: impl Into<Key>, message: impl Into<SmartStr>) -> Self {
        Self::builder(key)
            .notice_type(NoticeType::Tip)
            .message(message)
            .build()
    }

    /// Returns the flags for this notice.
    #[must_use]
    pub fn flags(&self) -> Flags {
        self.flags
    }

    /// Returns the notice type.
    #[must_use]
    pub fn notice_type(&self) -> NoticeType {
        self.kind
    }

    /// Returns the message content.
    #[must_use]
    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    /// Returns whether the notice can be dismissed.
    #[must_use]
    pub fn is_dismissible(&self) -> bool {
        self.dismissible
    }
}

impl Node for Notice {
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

impl Decoration for Notice {}

// =============================================================================
// Builder
// =============================================================================

/// Builder for [`Notice`].
#[derive(Debug)]
pub struct NoticeBuilder {
    key: Key,
    label: Option<SmartStr>,
    description: Option<SmartStr>,
    flags: Flags,
    notice_type: NoticeType,
    message: SmartStr,
    dismissible: bool,
}

impl NoticeBuilder {
    /// Creates a new builder with the given key.
    #[must_use]
    pub fn new(key: impl Into<Key>) -> Self {
        Self {
            key: key.into(),
            label: None,
            description: None,
            flags: Flags::empty(),
            notice_type: NoticeType::Info,
            message: SmartStr::new(),
            dismissible: false,
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

    /// Sets the notice type.
    #[must_use]
    pub fn notice_type(mut self, notice_type: NoticeType) -> Self {
        self.notice_type = notice_type;
        self
    }

    /// Sets the message.
    #[must_use]
    pub fn message(mut self, message: impl Into<SmartStr>) -> Self {
        self.message = message.into();
        self
    }

    /// Sets whether the notice can be dismissed.
    #[must_use]
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    /// Builds the Notice.
    #[must_use]
    pub fn build(self) -> Notice {
        let mut metadata = Metadata::new(self.key);
        if let Some(label) = self.label {
            metadata = metadata.with_label(label);
        }
        if let Some(description) = self.description {
            metadata = metadata.with_description(description);
        }

        Notice {
            metadata,
            flags: self.flags,
            kind: self.notice_type,
            message: self.message,
            dismissible: self.dismissible,
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
    fn test_notice_info() {
        let notice = Notice::info("info", "Hello world");

        assert_eq!(notice.key().as_str(), "info");
        assert_eq!(notice.notice_type(), NoticeType::Info);
        assert_eq!(notice.message(), "Hello world");
        assert!(!notice.is_dismissible());
    }

    #[test]
    fn test_notice_warning() {
        let notice = Notice::warning("warn", "Be careful!");

        assert_eq!(notice.notice_type(), NoticeType::Warning);
        assert_eq!(notice.message(), "Be careful!");
    }

    #[test]
    fn test_notice_error() {
        let notice = Notice::error("err", "Something went wrong");

        assert_eq!(notice.notice_type(), NoticeType::Error);
        assert_eq!(notice.message(), "Something went wrong");
    }

    #[test]
    fn test_notice_success() {
        let notice = Notice::success("ok", "Operation completed");

        assert_eq!(notice.notice_type(), NoticeType::Success);
        assert_eq!(notice.message(), "Operation completed");
    }

    #[test]
    fn test_notice_tip() {
        let notice = Notice::tip("hint", "Pro tip here");

        assert_eq!(notice.notice_type(), NoticeType::Tip);
        assert_eq!(notice.message(), "Pro tip here");
    }

    #[test]
    fn test_notice_builder() {
        let notice = Notice::builder("custom")
            .label("Custom Notice")
            .notice_type(NoticeType::Warning)
            .message("Custom message")
            .dismissible(true)
            .build();

        assert_eq!(notice.metadata().label(), Some("Custom Notice"));
        assert_eq!(notice.notice_type(), NoticeType::Warning);
        assert_eq!(notice.message(), "Custom message");
        assert!(notice.is_dismissible());
    }

    #[test]
    fn test_notice_kind() {
        let notice = Notice::info("test", "Test");

        assert_eq!(notice.kind(), NodeKind::Decoration);
    }

    #[test]
    fn test_notice_invariants() {
        let notice = Notice::info("test", "Test");

        // Decoration has NO own value
        assert!(!notice.kind().has_own_value());

        // Decoration has NO ValueAccess
        assert!(!notice.kind().has_value_access());

        // Decoration CANNOT have children
        assert!(!notice.kind().can_have_children());
    }
}
