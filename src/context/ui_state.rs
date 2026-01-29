//! UI state management for parameter nodes.
//!
//! This module provides [`UiStateManager`] for managing presentation-only state
//! (collapsed panels, scroll positions, etc.) separately from parameter data.
//!

#![allow(clippy::doc_markdown)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::default_trait_access)]
#![allow(clippy::unwrap_or_default)]
#![allow(clippy::unnecessary_map_or)]
//! # Architecture
//!
//! UI state is mutable, per-context state that does NOT belong in the immutable
//! schema layer. This maintains the three-layer architecture:
//! - **Schema Layer**: Immutable definitions (Arc-shared)
//! - **Runtime Layer**: Mutable values and UI state (Context-owned)
//! - **Value Layer**: Runtime data representation
//!
//! # Example
//!
//! ```
//! use paramdef::context::UiStateManager;
//! use paramdef::core::Key;
//!
//! let mut ui_state = UiStateManager::new();
//! ui_state.set_panel_collapsed("settings", true);
//! assert!(ui_state.is_panel_collapsed(&Key::from("settings")));
//! ```

use crate::core::Key;
use rustc_hash::FxHashMap;
use std::time::Instant;

/// Panel UI state (collapsed/expanded, interaction timestamp).
///
/// This stores presentation state for a single panel that should not be
/// part of the immutable schema.
#[derive(Debug, Clone)]
pub struct PanelState {
    /// Whether the panel is collapsed (true) or expanded (false).
    pub collapsed: bool,

    /// Last time the panel was interacted with.
    ///
    /// Used for UI features like "recently used" or "auto-collapse after N minutes".
    /// Not serialized to avoid timestamp issues across sessions.
    pub last_interaction: Option<Instant>,
}

impl Default for PanelState {
    fn default() -> Self {
        Self {
            collapsed: false, // Default to expanded
            last_interaction: None,
        }
    }
}

impl PanelState {
    /// Creates a new panel state with the given collapsed setting.
    pub fn new(collapsed: bool) -> Self {
        Self {
            collapsed,
            last_interaction: Some(Instant::now()),
        }
    }
}

/// Manages UI presentation state for parameter nodes.
///
/// This stores state like collapsed panels, selected tabs, scroll positions
/// that are specific to UI presentation and should not be part of the
/// immutable schema.
///
/// # Thread Safety
///
/// UiStateManager is NOT `Sync` because it contains `Instant` fields which
/// are not `Sync`. For multi-threaded contexts, each thread should have its
/// own Context (and thus its own UiStateManager).
///
/// # Serialization
///
/// With the `serde` feature, UiStateManager can be serialized. Only the
/// collapsed state is serialized; timestamps are skipped to avoid issues
/// across sessions.
#[derive(Debug, Clone, Default)]
pub struct UiStateManager {
    /// Panel states indexed by key.
    panel_states: FxHashMap<Key, PanelState>,
}

impl UiStateManager {
    /// Creates a new empty UI state manager.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::context::UiStateManager;
    ///
    /// let ui_state = UiStateManager::new();
    /// assert_eq!(ui_state.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            panel_states: FxHashMap::default(),
        }
    }

    /// Creates a UI state manager with pre-allocated capacity.
    ///
    /// Use this if you know approximately how many panels you'll have
    /// to avoid reallocations.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::context::UiStateManager;
    ///
    /// let ui_state = UiStateManager::with_capacity(10);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            panel_states: FxHashMap::with_capacity_and_hasher(capacity, Default::default()),
        }
    }

    /// Sets whether a panel is collapsed.
    ///
    /// This also updates the panel's last interaction timestamp.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::context::UiStateManager;
    /// use paramdef::core::Key;
    ///
    /// let mut ui_state = UiStateManager::new();
    /// ui_state.set_panel_collapsed("settings", true);
    /// assert!(ui_state.is_panel_collapsed(&Key::from("settings")));
    /// ```
    pub fn set_panel_collapsed(&mut self, key: impl Into<Key>, collapsed: bool) {
        let key = key.into();
        let state = self
            .panel_states
            .entry(key)
            .or_insert_with(PanelState::default);

        state.collapsed = collapsed;
        state.last_interaction = Some(Instant::now());
    }

    /// Returns whether a panel is collapsed.
    ///
    /// Returns `false` if the panel has no state (defaults to expanded).
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::context::UiStateManager;
    /// use paramdef::core::Key;
    ///
    /// let mut ui_state = UiStateManager::new();
    /// assert!(!ui_state.is_panel_collapsed(&Key::from("settings"))); // Default: expanded
    ///
    /// ui_state.set_panel_collapsed("settings", true);
    /// assert!(ui_state.is_panel_collapsed(&Key::from("settings")));
    /// ```
    pub fn is_panel_collapsed(&self, key: &Key) -> bool {
        self.panel_states
            .get(key)
            .map_or(false, |state| state.collapsed)
    }

    /// Gets the panel state for a key, if it exists.
    ///
    /// Returns `None` if no state has been set for this panel.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::context::UiStateManager;
    ///
    /// let mut ui_state = UiStateManager::new();
    /// assert!(ui_state.get_panel_state(&"settings".into()).is_none());
    ///
    /// ui_state.set_panel_collapsed("settings", true);
    /// assert!(ui_state.get_panel_state(&"settings".into()).is_some());
    /// ```
    pub fn get_panel_state(&self, key: &Key) -> Option<&PanelState> {
        self.panel_states.get(key)
    }

    /// Gets or creates a panel state for a key.
    ///
    /// If the state doesn't exist, creates it with default values (expanded).
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::context::UiStateManager;
    ///
    /// let mut ui_state = UiStateManager::new();
    /// let state = ui_state.get_or_create_panel_state("settings".into());
    /// assert!(!state.collapsed); // Default: expanded
    /// ```
    pub fn get_or_create_panel_state(&mut self, key: Key) -> &mut PanelState {
        self.panel_states
            .entry(key)
            .or_insert_with(PanelState::default)
    }

    /// Clears all UI state.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::context::UiStateManager;
    ///
    /// let mut ui_state = UiStateManager::new();
    /// ui_state.set_panel_collapsed("settings", true);
    /// assert_eq!(ui_state.len(), 1);
    ///
    /// ui_state.clear();
    /// assert_eq!(ui_state.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.panel_states.clear();
    }

    /// Returns the number of panels with stored state.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::context::UiStateManager;
    ///
    /// let mut ui_state = UiStateManager::new();
    /// assert_eq!(ui_state.len(), 0);
    ///
    /// ui_state.set_panel_collapsed("panel1", true);
    /// ui_state.set_panel_collapsed("panel2", false);
    /// assert_eq!(ui_state.len(), 2);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.panel_states.len()
    }

    /// Returns whether there is any UI state stored.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::context::UiStateManager;
    ///
    /// let mut ui_state = UiStateManager::new();
    /// assert!(ui_state.is_empty());
    ///
    /// ui_state.set_panel_collapsed("settings", true);
    /// assert!(!ui_state.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.panel_states.is_empty()
    }

    /// Returns an iterator over all panel states.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::context::UiStateManager;
    ///
    /// let mut ui_state = UiStateManager::new();
    /// ui_state.set_panel_collapsed("panel1", true);
    /// ui_state.set_panel_collapsed("panel2", false);
    ///
    /// let collapsed_panels: Vec<_> = ui_state.iter()
    ///     .filter(|(_, state)| state.collapsed)
    ///     .map(|(key, _)| key.clone())
    ///     .collect();
    ///
    /// assert_eq!(collapsed_panels.len(), 1);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = (&Key, &PanelState)> {
        self.panel_states.iter()
    }
}

// Serialization support (feature-gated)
#[cfg(feature = "serde")]
mod serde_support {
    use super::{FxHashMap, Key, PanelState, UiStateManager};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::collections::HashMap;

    impl Serialize for UiStateManager {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            // Serialize as a map of Key -> collapsed (bool)
            // Skip last_interaction timestamps (not useful across sessions)
            let collapsed_map: HashMap<&Key, bool> = self
                .panel_states
                .iter()
                .map(|(key, state)| (key, state.collapsed))
                .collect();

            collapsed_map.serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for UiStateManager {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let collapsed_map: HashMap<Key, bool> = HashMap::deserialize(deserializer)?;

            let panel_states: FxHashMap<Key, PanelState> = collapsed_map
                .into_iter()
                .map(|(key, collapsed)| {
                    (
                        key,
                        PanelState {
                            collapsed,
                            last_interaction: None, // Don't deserialize timestamps
                        },
                    )
                })
                .collect();

            Ok(UiStateManager { panel_states })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UiStateManager;

    #[test]
    fn test_ui_state_creation_and_defaults() {
        let ui_state = UiStateManager::new();
        assert_eq!(ui_state.len(), 0);
        assert!(ui_state.is_empty());

        // Default is expanded (not collapsed)
        assert!(!ui_state.is_panel_collapsed(&"nonexistent".into()));
    }

    #[test]
    fn test_panel_collapsed_set_get() {
        let mut ui_state = UiStateManager::new();

        // Set collapsed
        ui_state.set_panel_collapsed("settings", true);
        assert!(ui_state.is_panel_collapsed(&"settings".into()));
        assert_eq!(ui_state.len(), 1);

        // Set expanded
        ui_state.set_panel_collapsed("settings", false);
        assert!(!ui_state.is_panel_collapsed(&"settings".into()));
        assert_eq!(ui_state.len(), 1); // Still tracked
    }

    #[test]
    fn test_panel_state_interaction_timestamp() {
        let mut ui_state = UiStateManager::new();

        ui_state.set_panel_collapsed("settings", true);

        let state = ui_state.get_panel_state(&"settings".into()).unwrap();
        assert!(state.last_interaction.is_some());
    }

    #[test]
    fn test_multiple_panels_independent_state() {
        let mut ui_state = UiStateManager::new();

        ui_state.set_panel_collapsed("panel1", true);
        ui_state.set_panel_collapsed("panel2", false);
        ui_state.set_panel_collapsed("panel3", true);

        assert!(ui_state.is_panel_collapsed(&"panel1".into()));
        assert!(!ui_state.is_panel_collapsed(&"panel2".into()));
        assert!(ui_state.is_panel_collapsed(&"panel3".into()));

        assert_eq!(ui_state.len(), 3);
    }

    #[test]
    fn test_ui_state_clear_and_len() {
        let mut ui_state = UiStateManager::new();

        ui_state.set_panel_collapsed("panel1", true);
        ui_state.set_panel_collapsed("panel2", true);
        assert_eq!(ui_state.len(), 2);

        ui_state.clear();
        assert_eq!(ui_state.len(), 0);
        assert!(ui_state.is_empty());
    }

    #[test]
    fn test_get_or_create_panel_state() {
        let mut ui_state = UiStateManager::new();

        let state = ui_state.get_or_create_panel_state("new_panel".into());
        assert!(!state.collapsed); // Default

        state.collapsed = true;
        assert!(ui_state.is_panel_collapsed(&"new_panel".into()));
    }

    #[test]
    fn test_ui_state_iter() {
        let mut ui_state = UiStateManager::new();

        ui_state.set_panel_collapsed("panel1", true);
        ui_state.set_panel_collapsed("panel2", false);
        ui_state.set_panel_collapsed("panel3", true);

        let collapsed_count = ui_state.iter().filter(|(_, state)| state.collapsed).count();

        assert_eq!(collapsed_count, 2);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_ui_state_serialize_deserialize() {
        let mut ui_state = UiStateManager::new();
        ui_state.set_panel_collapsed("panel1", true);
        ui_state.set_panel_collapsed("panel2", false);

        let json = serde_json::to_string(&ui_state).unwrap();
        let deserialized: UiStateManager = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.len(), 2);
        assert!(deserialized.is_panel_collapsed(&"panel1".into()));
        assert!(!deserialized.is_panel_collapsed(&"panel2".into()));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_ui_state_last_interaction_not_serialized() {
        let mut ui_state = UiStateManager::new();
        ui_state.set_panel_collapsed("panel1", true);

        let json = serde_json::to_string(&ui_state).unwrap();
        let deserialized: UiStateManager = serde_json::from_str(&json).unwrap();

        // Timestamp should not be serialized/deserialized
        let state = deserialized.get_panel_state(&"panel1".into()).unwrap();
        assert!(state.last_interaction.is_none());
    }
}
