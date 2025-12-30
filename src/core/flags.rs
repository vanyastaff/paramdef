//! Schema and runtime flags for parameters.
//!
//! This module provides two distinct flag types:
//! - [`Flags`] - Schema-level, immutable attributes defined at parameter creation
//! - [`StateFlags`] - Runtime-level, mutable state tracked during parameter usage

use bitflags::bitflags;

bitflags! {
    /// Schema-level parameter attributes.
    ///
    /// These flags are set when defining a parameter and remain immutable.
    /// They describe the parameter's behavior and constraints.
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::core::Flags;
    ///
    /// // Single flag
    /// let required = Flags::REQUIRED;
    /// assert!(required.is_required());
    ///
    /// // Combine multiple flags
    /// let sensitive = Flags::REQUIRED | Flags::SENSITIVE | Flags::WRITE_ONLY;
    /// assert!(sensitive.is_required());
    /// assert!(sensitive.is_sensitive());
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct Flags: u64 {
        /// Parameter must have a value (cannot be null/empty).
        const REQUIRED = 1 << 0;

        /// Parameter cannot be modified by users.
        const READONLY = 1 << 1;

        /// Parameter should not be displayed in UI.
        const HIDDEN = 1 << 2;

        /// Parameter is for advanced users only.
        const ADVANCED = 1 << 3;

        /// Parameter contains sensitive data (passwords, tokens).
        const SENSITIVE = 1 << 4;

        /// Parameter value should not be included in serialization output.
        const WRITE_ONLY = 1 << 5;

        /// Parameter should not be persisted/saved.
        const SKIP_SAVE = 1 << 6;

        /// Parameter is computed at runtime (not user-editable).
        const RUNTIME = 1 << 7;

        /// Parameter can be animated/keyframed.
        const ANIMATABLE = 1 << 8;

        /// Parameter updates should be applied in realtime.
        const REALTIME = 1 << 9;

        /// Parameter is deprecated and should show warning.
        const DEPRECATED = 1 << 10;

        /// Parameter is experimental/unstable.
        const EXPERIMENTAL = 1 << 11;

        /// Parameter supports expression/formula input.
        const EXPRESSION = 1 << 12;

        /// Parameter should be replicated across network.
        const REPLICATED = 1 << 13;

        /// Parameter is disabled (grayed out in UI).
        const DISABLED = 1 << 14;
    }
}

impl Flags {
    /// Returns `true` if the REQUIRED flag is set.
    #[inline]
    #[must_use]
    pub const fn is_required(self) -> bool {
        self.contains(Self::REQUIRED)
    }

    /// Returns `true` if the READONLY flag is set.
    #[inline]
    #[must_use]
    pub const fn is_readonly(self) -> bool {
        self.contains(Self::READONLY)
    }

    /// Returns `true` if the HIDDEN flag is set.
    #[inline]
    #[must_use]
    pub const fn is_hidden(self) -> bool {
        self.contains(Self::HIDDEN)
    }

    /// Returns `true` if the ADVANCED flag is set.
    #[inline]
    #[must_use]
    pub const fn is_advanced(self) -> bool {
        self.contains(Self::ADVANCED)
    }

    /// Returns `true` if the SENSITIVE flag is set.
    #[inline]
    #[must_use]
    pub const fn is_sensitive(self) -> bool {
        self.contains(Self::SENSITIVE)
    }

    /// Returns `true` if the `WRITE_ONLY` flag is set.
    #[inline]
    #[must_use]
    pub const fn is_write_only(self) -> bool {
        self.contains(Self::WRITE_ONLY)
    }

    /// Returns `true` if the `SKIP_SAVE` flag is set.
    #[inline]
    #[must_use]
    pub const fn is_skip_save(self) -> bool {
        self.contains(Self::SKIP_SAVE)
    }

    /// Returns `true` if the RUNTIME flag is set.
    #[inline]
    #[must_use]
    pub const fn is_runtime(self) -> bool {
        self.contains(Self::RUNTIME)
    }

    /// Returns `true` if the ANIMATABLE flag is set.
    #[inline]
    #[must_use]
    pub const fn is_animatable(self) -> bool {
        self.contains(Self::ANIMATABLE)
    }

    /// Returns `true` if the REALTIME flag is set.
    #[inline]
    #[must_use]
    pub const fn is_realtime(self) -> bool {
        self.contains(Self::REALTIME)
    }

    /// Returns `true` if the DEPRECATED flag is set.
    #[inline]
    #[must_use]
    pub const fn is_deprecated(self) -> bool {
        self.contains(Self::DEPRECATED)
    }

    /// Returns `true` if the EXPERIMENTAL flag is set.
    #[inline]
    #[must_use]
    pub const fn is_experimental(self) -> bool {
        self.contains(Self::EXPERIMENTAL)
    }

    /// Returns `true` if the EXPRESSION flag is set.
    #[inline]
    #[must_use]
    pub const fn is_expression(self) -> bool {
        self.contains(Self::EXPRESSION)
    }

    /// Returns `true` if the REPLICATED flag is set.
    #[inline]
    #[must_use]
    pub const fn is_replicated(self) -> bool {
        self.contains(Self::REPLICATED)
    }

    /// Returns `true` if the DISABLED flag is set.
    #[inline]
    #[must_use]
    pub const fn is_disabled(self) -> bool {
        self.contains(Self::DISABLED)
    }

    /// Returns flags for a computed/runtime value.
    ///
    /// Combines `RUNTIME | READONLY | SKIP_SAVE`.
    #[inline]
    #[must_use]
    pub const fn computed() -> Self {
        Self::RUNTIME.union(Self::READONLY).union(Self::SKIP_SAVE)
    }

    /// Returns flags for an animatable property.
    ///
    /// Combines ANIMATABLE | REALTIME.
    #[inline]
    #[must_use]
    pub const fn animatable() -> Self {
        Self::ANIMATABLE.union(Self::REALTIME)
    }

    /// Returns flags for sensitive data.
    ///
    /// Combines `SENSITIVE | WRITE_ONLY`.
    #[inline]
    #[must_use]
    pub const fn sensitive() -> Self {
        Self::SENSITIVE.union(Self::WRITE_ONLY)
    }
}

bitflags! {
    /// Runtime parameter state flags.
    ///
    /// These flags track the current state of a parameter during usage.
    /// They are mutable and change as the user interacts with parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::core::StateFlags;
    ///
    /// let mut state = StateFlags::empty();
    ///
    /// // Mark as touched when user interacts
    /// state |= StateFlags::TOUCHED;
    /// assert!(state.is_touched());
    ///
    /// // Mark as dirty when value changes
    /// state |= StateFlags::DIRTY;
    /// assert!(state.is_dirty());
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct StateFlags: u8 {
        /// Value has changed since last save/sync.
        const DIRTY = 1 << 0;

        /// User has interacted with the parameter.
        const TOUCHED = 1 << 1;

        /// Parameter has passed validation.
        const VALID = 1 << 2;

        /// Parameter is currently visible.
        const VISIBLE = 1 << 3;

        /// Parameter is currently enabled (not grayed out).
        const ENABLED = 1 << 4;

        /// Parameter is currently readonly (runtime override).
        const READONLY = 1 << 5;
    }
}

impl StateFlags {
    /// Returns `true` if the DIRTY flag is set.
    #[inline]
    #[must_use]
    pub const fn is_dirty(self) -> bool {
        self.contains(Self::DIRTY)
    }

    /// Returns `true` if the TOUCHED flag is set.
    #[inline]
    #[must_use]
    pub const fn is_touched(self) -> bool {
        self.contains(Self::TOUCHED)
    }

    /// Returns `true` if the VALID flag is set.
    #[inline]
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.contains(Self::VALID)
    }

    /// Returns `true` if the VISIBLE flag is set.
    #[inline]
    #[must_use]
    pub const fn is_visible(self) -> bool {
        self.contains(Self::VISIBLE)
    }

    /// Returns `true` if the ENABLED flag is set.
    #[inline]
    #[must_use]
    pub const fn is_enabled(self) -> bool {
        self.contains(Self::ENABLED)
    }

    /// Returns `true` if the READONLY flag is set.
    #[inline]
    #[must_use]
    pub const fn is_readonly(self) -> bool {
        self.contains(Self::READONLY)
    }

    /// Returns default initial state (VISIBLE | ENABLED).
    #[inline]
    #[must_use]
    pub const fn initial() -> Self {
        Self::VISIBLE.union(Self::ENABLED)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Flags tests
    #[test]
    fn test_flags_required() {
        let flags = Flags::REQUIRED;
        assert!(flags.is_required());
        assert!(!flags.is_readonly());
    }

    #[test]
    fn test_flags_readonly() {
        let flags = Flags::READONLY;
        assert!(flags.is_readonly());
        assert!(!flags.is_required());
    }

    #[test]
    fn test_flags_hidden() {
        let flags = Flags::HIDDEN;
        assert!(flags.is_hidden());
    }

    #[test]
    fn test_flags_sensitive() {
        let flags = Flags::SENSITIVE;
        assert!(flags.is_sensitive());
    }

    #[test]
    fn test_flags_combination() {
        let flags = Flags::REQUIRED | Flags::READONLY | Flags::HIDDEN;
        assert!(flags.is_required());
        assert!(flags.is_readonly());
        assert!(flags.is_hidden());
        assert!(!flags.is_sensitive());
    }

    #[test]
    fn test_flags_computed_convenience() {
        let flags = Flags::computed();
        assert!(flags.is_runtime());
        assert!(flags.is_readonly());
        assert!(flags.is_skip_save());
    }

    #[test]
    fn test_flags_animatable_convenience() {
        let flags = Flags::animatable();
        assert!(flags.is_animatable());
        assert!(flags.is_realtime());
    }

    #[test]
    fn test_flags_sensitive_convenience() {
        let flags = Flags::sensitive();
        assert!(flags.is_sensitive());
        assert!(flags.is_write_only());
    }

    #[test]
    fn test_flags_default() {
        let flags = Flags::default();
        assert!(flags.is_empty());
    }

    // StateFlags tests
    #[test]
    fn test_state_flags_dirty() {
        let flags = StateFlags::DIRTY;
        assert!(flags.is_dirty());
        assert!(!flags.is_touched());
    }

    #[test]
    fn test_state_flags_touched() {
        let flags = StateFlags::TOUCHED;
        assert!(flags.is_touched());
        assert!(!flags.is_dirty());
    }

    #[test]
    fn test_state_flags_valid() {
        let flags = StateFlags::VALID;
        assert!(flags.is_valid());
    }

    #[test]
    fn test_state_flags_combination() {
        let mut flags = StateFlags::empty();
        flags |= StateFlags::DIRTY;
        flags |= StateFlags::TOUCHED;

        assert!(flags.is_dirty());
        assert!(flags.is_touched());
        assert!(!flags.is_valid());
    }

    #[test]
    fn test_state_flags_initial() {
        let flags = StateFlags::initial();
        assert!(flags.is_visible());
        assert!(flags.is_enabled());
        assert!(!flags.is_dirty());
        assert!(!flags.is_touched());
    }

    #[test]
    fn test_state_flags_default() {
        let flags = StateFlags::default();
        assert!(flags.is_empty());
    }

    #[test]
    fn test_flags_independence() {
        // Verify Flags and StateFlags are completely independent types
        let schema_flags = Flags::REQUIRED;
        let runtime_flags = StateFlags::DIRTY;

        // They should not be comparable (different types)
        assert!(schema_flags.is_required());
        assert!(runtime_flags.is_dirty());
    }
}
