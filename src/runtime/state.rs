//! Runtime state for parameters.

use std::time::Instant;

use crate::core::{Error, StateFlags};

/// Runtime state for a single parameter.
///
/// Tracks the current state flags, validation errors, and when the value
/// was last modified.
#[derive(Debug, Clone)]
pub struct State {
    /// State flags (dirty, touched, valid, etc.).
    flags: StateFlags,
    /// Validation errors from the last validation run.
    errors: Vec<Error>,
    /// Timestamp of last modification.
    modified_at: Option<Instant>,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    /// Creates a new parameter state with default values.
    #[must_use]
    pub fn new() -> Self {
        Self {
            flags: StateFlags::VALID, // Initially valid (no value set yet)
            errors: Vec::new(),
            modified_at: None,
        }
    }

    /// Returns the current state flags.
    #[must_use]
    pub fn flags(&self) -> StateFlags {
        self.flags
    }

    /// Returns `true` if the parameter value has been modified.
    #[must_use]
    pub fn is_dirty(&self) -> bool {
        self.flags.contains(StateFlags::DIRTY)
    }

    /// Returns `true` if the parameter has been interacted with.
    #[must_use]
    pub fn is_touched(&self) -> bool {
        self.flags.contains(StateFlags::TOUCHED)
    }

    /// Returns `true` if the parameter is currently valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.flags.contains(StateFlags::VALID)
    }

    /// Returns the validation errors.
    #[must_use]
    pub fn errors(&self) -> &[Error] {
        &self.errors
    }

    /// Returns when the value was last modified.
    #[must_use]
    pub fn modified_at(&self) -> Option<Instant> {
        self.modified_at
    }

    /// Marks the parameter as dirty and updates the modified timestamp.
    pub fn mark_dirty(&mut self) {
        self.flags.insert(StateFlags::DIRTY);
        self.modified_at = Some(Instant::now());
    }

    /// Marks the parameter as touched.
    pub fn mark_touched(&mut self) {
        self.flags.insert(StateFlags::TOUCHED);
    }

    /// Marks the parameter as clean (not dirty).
    pub fn mark_clean(&mut self) {
        self.flags.remove(StateFlags::DIRTY);
    }

    /// Sets the validation result.
    pub fn set_validation_result(&mut self, errors: Vec<Error>) {
        self.errors = errors;
        if self.errors.is_empty() {
            self.flags.insert(StateFlags::VALID);
        } else {
            self.flags.remove(StateFlags::VALID);
        }
    }

    /// Resets the state to initial values.
    pub fn reset(&mut self) {
        self.flags = StateFlags::VALID;
        self.errors.clear();
        self.modified_at = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_init() {
        let state = State::new();

        assert!(!state.is_dirty());
        assert!(!state.is_touched());
        assert!(state.is_valid());
        assert!(state.errors().is_empty());
        assert!(state.modified_at().is_none());
    }

    #[test]
    fn test_state_mark_dirty() {
        let mut state = State::new();

        state.mark_dirty();

        assert!(state.is_dirty());
        assert!(state.modified_at().is_some());
    }

    #[test]
    fn test_state_mark_touched() {
        let mut state = State::new();

        state.mark_touched();

        assert!(state.is_touched());
    }

    #[test]
    fn test_state_mark_clean() {
        let mut state = State::new();
        state.mark_dirty();

        state.mark_clean();

        assert!(!state.is_dirty());
    }

    #[test]
    fn test_state_validation_success() {
        let mut state = State::new();

        state.set_validation_result(vec![]);

        assert!(state.is_valid());
        assert!(state.errors().is_empty());
    }

    #[test]
    fn test_state_validation_failure() {
        let mut state = State::new();

        state.set_validation_result(vec![Error::missing_required("test")]);

        assert!(!state.is_valid());
        assert_eq!(state.errors().len(), 1);
    }

    #[test]
    fn test_state_reset() {
        let mut state = State::new();
        state.mark_dirty();
        state.mark_touched();
        state.set_validation_result(vec![Error::missing_required("test")]);

        state.reset();

        assert!(!state.is_dirty());
        assert!(!state.is_touched());
        assert!(state.is_valid());
        assert!(state.errors().is_empty());
        assert!(state.modified_at().is_none());
    }
}
