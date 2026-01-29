//! Event types for the reactive system.
//!
//! This module defines all events that can be emitted by the parameter system.
//! Events follow the observer pattern and are broadcast to all subscribers.
//!
//! # Event Categories
//!
//! - **Value events**: `ValueChanging`, `ValueChanged` - track value modifications
//! - **State events**: `Touched`, `Dirtied`, `Reset` - track state changes
//! - **Validation events**: `Validated` - track validation results
//! - **Batch events**: `BatchBegin`, `BatchEnd` - group related changes
//!
//! # Example
//!
//! ```ignore
//! use paramdef::event::{Event, EventBus};
//!
//! let bus = EventBus::new(64);
//! let mut rx = bus.subscribe();
//!
//! // Events are received by all subscribers
//! bus.emit(Event::Touched { key: "username".into() });
//! ```

use std::sync::Arc;

use crate::core::{Key, SmartStr, Value};

/// Events emitted by the parameter system.
///
/// All events are `Clone` and `Send + Sync` for safe concurrent access.
/// Events use `Arc` for data that may be large (arrays, errors) to avoid
/// expensive clones when broadcasting to multiple subscribers.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Event {
    /// Emitted before a value changes.
    ///
    /// This is a notification event - the change has already been validated
    /// and will proceed. Use this for logging, debugging, or preparing
    /// dependent updates.
    ///
    /// For cancellable changes, use middleware or validators instead.
    ValueChanging {
        /// Key of the parameter being changed.
        key: Key,
        /// Current value before the change.
        old_value: Option<Value>,
        /// New value being set.
        new_value: Value,
    },

    /// Emitted after a value has changed.
    ///
    /// The change is complete and the new value is now active.
    /// Use this for:
    /// - Updating dependent values
    /// - Triggering side effects
    /// - Syncing with external systems
    ValueChanged {
        /// Key of the parameter that changed.
        key: Key,
        /// Previous value.
        old_value: Option<Value>,
        /// New current value.
        new_value: Value,
    },

    /// Emitted when a value is cleared.
    ///
    /// The parameter no longer has a value set.
    ValueCleared {
        /// Key of the parameter that was cleared.
        key: Key,
        /// The value that was removed.
        old_value: Value,
    },

    /// Emitted when validation completes for a parameter.
    ///
    /// Contains the validation result and any errors.
    Validated {
        /// Key of the validated parameter.
        key: Key,
        /// Whether validation passed.
        is_valid: bool,
        /// Validation errors (empty if valid).
        /// Uses `Arc` to avoid cloning error details to each subscriber.
        errors: Arc<[ValidationError]>,
    },

    /// Emitted when a parameter is first interacted with.
    ///
    /// "Touched" typically means the user focused and then blurred the field,
    /// or programmatically marked it as touched. Used for showing validation
    /// errors only after user interaction.
    Touched {
        /// Key of the touched parameter.
        key: Key,
    },

    /// Emitted when a parameter value becomes dirty.
    ///
    /// A parameter is "dirty" when its value differs from the initial/saved state.
    Dirtied {
        /// Key of the dirtied parameter.
        key: Key,
    },

    /// Emitted when a parameter is marked clean.
    ///
    /// Typically after saving or explicitly clearing the dirty flag.
    Cleaned {
        /// Key of the cleaned parameter.
        key: Key,
    },

    /// Emitted when a parameter is reset to initial state.
    ///
    /// Value, dirty flag, touched flag, and validation state are all reset.
    Reset {
        /// Key of the reset parameter.
        key: Key,
    },

    /// Emitted when a batch operation begins.
    ///
    /// Multiple changes within a batch are logically grouped.
    /// Observers may defer processing until `BatchEnd`.
    BatchBegin {
        /// Unique identifier for this batch.
        id: u64,
        /// Optional description of the batch operation.
        description: Option<SmartStr>,
    },

    /// Emitted when a batch operation ends.
    ///
    /// All changes in the batch are now complete.
    BatchEnd {
        /// Identifier matching the `BatchBegin`.
        id: u64,
        /// Whether the batch operation succeeded completely.
        success: bool,
        /// True if some operations succeeded and some failed (only relevant when success=false).
        partial: bool,
    },

    /// Emitted when a set operation fails within a batch.
    ///
    /// This event is emitted during partial batch operations when an individual
    /// value cannot be set due to an error.
    SetFailed {
        /// Key that failed to be set.
        key: Key,
        /// The error that occurred.
        error: SmartStr,
    },

    /// Emitted when a value is reverted due to transaction rollback.
    ///
    /// This event is emitted during transactional batch operations when
    /// an error causes all changes to be rolled back.
    Reverted {
        /// Key of the parameter that was reverted.
        key: Key,
        /// Original value before the batch operation.
        old_value: Value,
        /// Failed value that was attempted to be set.
        failed_value: Value,
    },

    /// Emitted when the entire context is reset.
    ContextReset,

    /// Emitted when all parameters are marked clean.
    AllCleaned,
}

impl Event {
    /// Returns the key associated with this event, if any.
    ///
    /// Batch and context-level events return `None`.
    #[must_use]
    pub fn key(&self) -> Option<&Key> {
        match self {
            Self::ValueChanging { key, .. }
            | Self::ValueChanged { key, .. }
            | Self::ValueCleared { key, .. }
            | Self::Validated { key, .. }
            | Self::Touched { key }
            | Self::Dirtied { key }
            | Self::Cleaned { key }
            | Self::Reset { key }
            | Self::SetFailed { key, .. }
            | Self::Reverted { key, .. } => Some(key),
            Self::BatchBegin { .. }
            | Self::BatchEnd { .. }
            | Self::ContextReset
            | Self::AllCleaned => None,
        }
    }

    /// Returns `true` if this is a value-related event.
    #[must_use]
    pub const fn is_value_event(&self) -> bool {
        matches!(
            self,
            Self::ValueChanging { .. } | Self::ValueChanged { .. } | Self::ValueCleared { .. }
        )
    }

    /// Returns `true` if this is a state-related event.
    #[must_use]
    pub const fn is_state_event(&self) -> bool {
        matches!(
            self,
            Self::Touched { .. }
                | Self::Dirtied { .. }
                | Self::Cleaned { .. }
                | Self::Reset { .. }
                | Self::ContextReset
                | Self::AllCleaned
        )
    }

    /// Returns `true` if this is a validation event.
    #[must_use]
    pub const fn is_validation_event(&self) -> bool {
        matches!(self, Self::Validated { .. })
    }

    /// Returns `true` if this is a batch event.
    #[must_use]
    pub const fn is_batch_event(&self) -> bool {
        matches!(
            self,
            Self::BatchBegin { .. }
                | Self::BatchEnd { .. }
                | Self::SetFailed { .. }
                | Self::Reverted { .. }
        )
    }

    // === Constructors ===

    /// Creates a `ValueChanging` event.
    #[must_use]
    pub fn value_changing(key: impl Into<Key>, old_value: Option<Value>, new_value: Value) -> Self {
        Self::ValueChanging {
            key: key.into(),
            old_value,
            new_value,
        }
    }

    /// Creates a `ValueChanged` event.
    #[must_use]
    pub fn value_changed(key: impl Into<Key>, old_value: Option<Value>, new_value: Value) -> Self {
        Self::ValueChanged {
            key: key.into(),
            old_value,
            new_value,
        }
    }

    /// Creates a `ValueCleared` event.
    #[must_use]
    pub fn value_cleared(key: impl Into<Key>, old_value: Value) -> Self {
        Self::ValueCleared {
            key: key.into(),
            old_value,
        }
    }

    /// Creates a `Validated` event.
    #[must_use]
    pub fn validated(
        key: impl Into<Key>,
        is_valid: bool,
        errors: impl Into<Arc<[ValidationError]>>,
    ) -> Self {
        Self::Validated {
            key: key.into(),
            is_valid,
            errors: errors.into(),
        }
    }

    /// Creates a `Validated` event for a successful validation.
    #[must_use]
    pub fn valid(key: impl Into<Key>) -> Self {
        Self::Validated {
            key: key.into(),
            is_valid: true,
            errors: Arc::from([]),
        }
    }

    /// Creates a `Touched` event.
    #[must_use]
    pub fn touched(key: impl Into<Key>) -> Self {
        Self::Touched { key: key.into() }
    }

    /// Creates a `Dirtied` event.
    #[must_use]
    pub fn dirtied(key: impl Into<Key>) -> Self {
        Self::Dirtied { key: key.into() }
    }

    /// Creates a `Cleaned` event.
    #[must_use]
    pub fn cleaned(key: impl Into<Key>) -> Self {
        Self::Cleaned { key: key.into() }
    }

    /// Creates a `Reset` event.
    #[must_use]
    pub fn reset(key: impl Into<Key>) -> Self {
        Self::Reset { key: key.into() }
    }

    /// Creates a `BatchBegin` event.
    #[must_use]
    pub fn batch_begin(id: u64, description: Option<impl Into<SmartStr>>) -> Self {
        Self::BatchBegin {
            id,
            description: description.map(Into::into),
        }
    }

    /// Creates a `BatchEnd` event.
    #[must_use]
    pub const fn batch_end(id: u64, success: bool, partial: bool) -> Self {
        Self::BatchEnd {
            id,
            success,
            partial,
        }
    }

    /// Creates a `SetFailed` event.
    #[must_use]
    pub fn set_failed(key: impl Into<Key>, error: impl Into<SmartStr>) -> Self {
        Self::SetFailed {
            key: key.into(),
            error: error.into(),
        }
    }

    /// Creates a `Reverted` event.
    #[must_use]
    pub fn reverted(key: impl Into<Key>, old_value: Value, failed_value: Value) -> Self {
        Self::Reverted {
            key: key.into(),
            old_value,
            failed_value,
        }
    }
}

/// A validation error with code, message, and field path.
///
/// Designed to be lightweight and cloneable for event broadcasting.
/// Includes the full path to the field for nested object errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    /// Full path to the field (e.g., "user.address.city" for nested objects).
    pub path: SmartStr,
    /// Field name (last segment of path, e.g., "city").
    pub field: SmartStr,
    /// Error code for programmatic handling (e.g., `required`, `min_length`).
    pub code: SmartStr,
    /// Human-readable error message.
    pub message: SmartStr,
}

impl ValidationError {
    /// Creates a new validation error with full path information.
    ///
    /// # Arguments
    ///
    /// * `path` - Full path to the field (e.g., "user.address.city")
    /// * `field` - Field name (e.g., "city")
    /// * `code` - Error code for programmatic handling
    /// * `message` - Human-readable error message
    #[must_use]
    pub fn new(
        path: impl Into<SmartStr>,
        field: impl Into<SmartStr>,
        code: impl Into<SmartStr>,
        message: impl Into<SmartStr>,
    ) -> Self {
        Self {
            path: path.into(),
            field: field.into(),
            code: code.into(),
            message: message.into(),
        }
    }

    /// Creates a validation error for a top-level field (path = field).
    ///
    /// Convenience method when the field is not nested.
    #[must_use]
    pub fn simple(
        field: impl Into<SmartStr>,
        code: impl Into<SmartStr>,
        message: impl Into<SmartStr>,
    ) -> Self {
        let field = field.into();
        Self {
            path: field.clone(),
            field,
            code: code.into(),
            message: message.into(),
        }
    }

    /// Creates a "required" validation error for a field.
    #[must_use]
    pub fn required(field: impl Into<SmartStr>) -> Self {
        Self::simple(field, "required", "This field is required")
    }

    /// Returns the full path to the field.
    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Returns the field name.
    #[must_use]
    pub fn field(&self) -> &str {
        &self.field
    }

    /// Returns the error code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Returns the error message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Creates a `min_length` validation error for a field.
    #[must_use]
    pub fn min_length(field: impl Into<SmartStr>, min: usize) -> Self {
        Self::simple(field, "min_length", format!("Minimum length is {min}"))
    }

    /// Creates a `max_length` validation error for a field.
    #[must_use]
    pub fn max_length(field: impl Into<SmartStr>, max: usize) -> Self {
        Self::simple(field, "max_length", format!("Maximum length is {max}"))
    }

    /// Creates a "min" validation error for numeric values.
    #[must_use]
    pub fn min_value(field: impl Into<SmartStr>, min: f64) -> Self {
        Self::simple(field, "min", format!("Minimum value is {min}"))
    }

    /// Creates a "max" validation error for numeric values.
    #[must_use]
    pub fn max_value(field: impl Into<SmartStr>, max: f64) -> Self {
        Self::simple(field, "max", format!("Maximum value is {max}"))
    }

    /// Creates a "pattern" validation error.
    #[must_use]
    pub fn pattern(field: impl Into<SmartStr>, pattern: &str) -> Self {
        Self::simple(
            field,
            "pattern",
            format!("Value does not match pattern: {pattern}"),
        )
    }

    /// Creates a custom validation error.
    #[must_use]
    pub fn custom(
        field: impl Into<SmartStr>,
        code: impl Into<SmartStr>,
        message: impl Into<SmartStr>,
    ) -> Self {
        Self::simple(field, code, message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_key() {
        let event = Event::touched("username");
        assert_eq!(event.key().map(Key::as_str), Some("username"));

        let batch = Event::batch_begin(1, None::<&str>);
        assert!(batch.key().is_none());
    }

    #[test]
    fn test_event_categories() {
        assert!(Event::value_changed("k", None, Value::Bool(true)).is_value_event());
        assert!(Event::touched("k").is_state_event());
        assert!(Event::valid("k").is_validation_event());
        assert!(Event::batch_begin(1, None::<&str>).is_batch_event());
    }

    #[test]
    fn test_validation_error_constructors() {
        let err = ValidationError::required("test_field");
        assert_eq!(err.code.as_str(), "required");
        assert_eq!(err.field.as_str(), "test_field");

        let err = ValidationError::min_length("username", 5);
        assert_eq!(err.code.as_str(), "min_length");
        assert!(err.message.contains("5"));
    }

    #[test]
    fn test_event_constructors() {
        let e = Event::value_changing("key", None, Value::Int(42));
        assert!(matches!(e, Event::ValueChanging { .. }));

        let e = Event::validated("key", false, vec![ValidationError::required("key")]);
        if let Event::Validated { errors, .. } = e {
            assert_eq!(errors.len(), 1);
        }
    }

    #[test]
    fn test_event_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Event>();
        assert_send_sync::<ValidationError>();
    }

    #[test]
    fn test_event_clone() {
        let event = Event::validated("key", false, vec![ValidationError::required("key")]);
        let cloned = event.clone();

        // Arc should be shared, not deep cloned
        if let (Event::Validated { errors: e1, .. }, Event::Validated { errors: e2, .. }) =
            (&event, &cloned)
        {
            assert!(Arc::ptr_eq(e1, e2));
        }
    }
}
