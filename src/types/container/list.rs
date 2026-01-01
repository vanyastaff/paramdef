//! List container for dynamic arrays.
//!
//! List represents a dynamic collection of items based on a template,
//! similar to a JSON array. All items share the same structure defined
//! by the item template.
//!
//! # Ranking Lists
//!
//! Lists can be marked as rankable when item order represents priority:
//!
//! ```ignore
//! // Simple ranking - order = priority
//! let priorities = List::builder("task_priorities")
//!     .item_template(Text::builder("task").build())
//!     .rankable()
//!     .build()?;
//!
//! // Ranking with configuration
//! let features = List::builder("feature_ranking")
//!     .item_template(Text::builder("feature").build())
//!     .ranking_config(RankingConfig::default()
//!         .show_numbers(true)
//!         .direction(RankDirection::HighestFirst))
//!     .unique(true)
//!     .build()?;
//! ```

use std::any::Any;
use std::fmt;
use std::sync::Arc;

use crate::core::{Flags, Key, Metadata, SmartStr};
use crate::types::kind::NodeKind;
use crate::types::traits::{Container, Node};

// =============================================================================
// RankingConfig
// =============================================================================

/// Direction of ranking (which end is highest priority).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RankDirection {
    /// First item = highest priority (rank 1).
    #[default]
    HighestFirst,
    /// First item = lowest priority.
    LowestFirst,
}

impl RankDirection {
    /// Returns the name of this direction.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::HighestFirst => "highest_first",
            Self::LowestFirst => "lowest_first",
        }
    }
}

/// Configuration for rankable lists.
///
/// When a list is rankable, the order of items represents their priority
/// or ranking. This is useful for:
/// - Priority ordering
/// - Preference rankings
/// - Feature prioritization
/// - Candidate ranking
///
/// # Example
///
/// ```ignore
/// use paramdef::types::container::{List, RankingConfig, RankDirection};
/// use paramdef::types::leaf::Text;
///
/// let ranking = List::builder("priorities")
///     .item_template(Text::builder("item").build())
///     .ranking_config(RankingConfig::default()
///         .show_numbers(true)
///         .direction(RankDirection::HighestFirst))
///     .build()?;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RankingConfig {
    /// Whether to display rank numbers in UI (1, 2, 3...).
    show_numbers: bool,
    /// Direction of ranking.
    direction: RankDirection,
}

impl RankingConfig {
    /// Creates a new ranking configuration with defaults.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether to show rank numbers in UI.
    #[must_use]
    pub fn show_numbers(mut self, show: bool) -> Self {
        self.show_numbers = show;
        self
    }

    /// Sets the ranking direction.
    #[must_use]
    pub fn direction(mut self, direction: RankDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Returns whether rank numbers should be shown.
    #[must_use]
    pub fn shows_numbers(&self) -> bool {
        self.show_numbers
    }

    /// Returns the ranking direction.
    #[must_use]
    pub fn get_direction(&self) -> RankDirection {
        self.direction
    }
}

/// A container for dynamic arrays of items.
///
/// List is one of the six container types. It holds a collection of items
/// based on a template and produces a `Value::Array` when serialized.
///
/// # Example
///
/// ```ignore
/// use paramdef::container::{List, Object};
/// use paramdef::types::leaf::Text;
///
/// let headers = List::builder("headers")
///     .label("HTTP Headers")
///     .item_template(Object::builder("header")
///         .field("name", Text::builder("name").required().build())
///         .field("value", Text::builder("value").build())
///         .build())
///     .min_items(0)
///     .max_items(20)
///     .sortable(true)
///     .build();
/// ```
#[derive(Clone)]
pub struct List {
    metadata: Metadata,
    flags: Flags,
    item_template: Arc<dyn Node>,
    min_items: Option<usize>,
    max_items: Option<usize>,
    unique: bool,
    sortable: bool,
    /// Ranking configuration (if item order represents priority).
    ranking: Option<RankingConfig>,
    /// Cached children for Container trait
    children_cache: Arc<[Arc<dyn Node>]>,
}

impl fmt::Debug for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("List")
            .field("metadata", &self.metadata)
            .field("flags", &self.flags)
            .field("min_items", &self.min_items)
            .field("max_items", &self.max_items)
            .field("unique", &self.unique)
            .field("sortable", &self.sortable)
            .field("ranking", &self.ranking)
            .finish_non_exhaustive()
    }
}

impl List {
    /// Creates a new builder for a List.
    #[must_use]
    pub fn builder(key: impl Into<Key>) -> ListBuilder {
        ListBuilder::new(key)
    }

    /// Returns the flags for this list.
    #[inline]
    #[must_use]
    pub fn flags(&self) -> Flags {
        self.flags
    }

    /// Returns the item template.
    #[inline]
    #[must_use]
    pub fn item_template(&self) -> &Arc<dyn Node> {
        &self.item_template
    }

    /// Returns the minimum number of items, if set.
    #[inline]
    #[must_use]
    pub fn min_items(&self) -> Option<usize> {
        self.min_items
    }

    /// Returns the maximum number of items, if set.
    #[inline]
    #[must_use]
    pub fn max_items(&self) -> Option<usize> {
        self.max_items
    }

    /// Returns whether items must be unique.
    #[inline]
    #[must_use]
    pub fn is_unique(&self) -> bool {
        self.unique
    }

    /// Returns whether the list is sortable.
    #[inline]
    #[must_use]
    pub fn is_sortable(&self) -> bool {
        self.sortable
    }

    /// Returns whether the list is rankable (order represents priority).
    #[inline]
    #[must_use]
    pub fn is_rankable(&self) -> bool {
        self.ranking.is_some()
    }

    /// Returns the ranking configuration, if set.
    #[must_use]
    pub fn ranking_config(&self) -> Option<&RankingConfig> {
        self.ranking.as_ref()
    }
}

impl Node for List {
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

impl Container for List {
    fn children(&self) -> &[Arc<dyn Node>] {
        &self.children_cache
    }
}

// =============================================================================
// Builder
// =============================================================================

/// Builder for [`List`].
pub struct ListBuilder {
    key: Key,
    label: Option<SmartStr>,
    description: Option<SmartStr>,
    flags: Flags,
    item_template: Option<Arc<dyn Node>>,
    min_items: Option<usize>,
    max_items: Option<usize>,
    unique: bool,
    sortable: bool,
    ranking: Option<RankingConfig>,
}

impl fmt::Debug for ListBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ListBuilder")
            .field("key", &self.key)
            .field("label", &self.label)
            .field("description", &self.description)
            .field("flags", &self.flags)
            .field("has_template", &self.item_template.is_some())
            .field("min_items", &self.min_items)
            .field("max_items", &self.max_items)
            .field("unique", &self.unique)
            .field("sortable", &self.sortable)
            .field("ranking", &self.ranking)
            .finish()
    }
}

impl ListBuilder {
    /// Creates a new builder with the given key.
    #[must_use]
    pub fn new(key: impl Into<Key>) -> Self {
        Self {
            key: key.into(),
            label: None,
            description: None,
            flags: Flags::empty(),
            item_template: None,
            min_items: None,
            max_items: None,
            unique: false,
            sortable: false,
            ranking: None,
        }
    }

    /// Sets the label for this list.
    #[must_use]
    pub fn label(mut self, label: impl Into<SmartStr>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the description for this list.
    #[must_use]
    pub fn description(mut self, description: impl Into<SmartStr>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the flags for this list.
    #[must_use]
    pub fn flags(mut self, flags: Flags) -> Self {
        self.flags = flags;
        self
    }

    /// Marks this list as required (must have at least one item).
    #[must_use]
    pub fn required(mut self) -> Self {
        self.flags |= Flags::REQUIRED;
        self
    }

    /// Sets the item template.
    #[must_use]
    pub fn item_template(mut self, template: impl Node + 'static) -> Self {
        self.item_template = Some(Arc::new(template));
        self
    }

    /// Sets the item template with an already-wrapped Arc.
    #[must_use]
    pub fn item_template_arc(mut self, template: Arc<dyn Node>) -> Self {
        self.item_template = Some(template);
        self
    }

    /// Sets the minimum number of items.
    #[must_use]
    pub fn min_items(mut self, min: usize) -> Self {
        self.min_items = Some(min);
        self
    }

    /// Sets the maximum number of items.
    #[must_use]
    pub fn max_items(mut self, max: usize) -> Self {
        self.max_items = Some(max);
        self
    }

    /// Sets whether items must be unique.
    #[must_use]
    pub fn unique(mut self, unique: bool) -> Self {
        self.unique = unique;
        self
    }

    /// Sets whether the list is sortable by the user.
    #[must_use]
    pub fn sortable(mut self, sortable: bool) -> Self {
        self.sortable = sortable;
        self
    }

    /// Makes this list rankable with default configuration.
    ///
    /// When a list is rankable, the order of items represents their priority.
    /// First item = highest priority by default.
    ///
    /// This also enables sortable automatically.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let priorities = List::builder("priorities")
    ///     .item_template(Text::builder("task").build())
    ///     .rankable()
    ///     .build()?;
    /// ```
    #[must_use]
    pub fn rankable(mut self) -> Self {
        self.ranking = Some(RankingConfig::default());
        self.sortable = true; // Ranking implies sortable
        self
    }

    /// Makes this list rankable with custom configuration.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let ranking = List::builder("candidates")
    ///     .item_template(Text::builder("name").build())
    ///     .ranking_config(RankingConfig::new()
    ///         .show_numbers(true)
    ///         .direction(RankDirection::HighestFirst))
    ///     .build()?;
    /// ```
    #[must_use]
    pub fn ranking_config(mut self, config: RankingConfig) -> Self {
        self.ranking = Some(config);
        self.sortable = true; // Ranking implies sortable
        self
    }

    /// Builds the List.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No item template was provided
    /// - `min_items` is greater than `max_items`
    pub fn build(self) -> crate::core::Result<List> {
        let mut metadata = Metadata::new(self.key);
        if let Some(label) = self.label {
            metadata = metadata.with_label(label);
        }
        if let Some(description) = self.description {
            metadata = metadata.with_description(description);
        }

        let item_template = self
            .item_template
            .ok_or_else(|| crate::core::Error::missing_required("item_template"))?;

        // Validate min_items <= max_items
        if let (Some(min), Some(max)) = (self.min_items, self.max_items) {
            if min > max {
                return Err(crate::core::Error::validation(
                    "invalid_bounds",
                    format!("min_items ({min}) cannot be greater than max_items ({max})"),
                ));
            }
        }

        // Build children cache (contains item_template)
        let children_cache: Arc<[Arc<dyn Node>]> = Arc::from([Arc::clone(&item_template)]);

        Ok(List {
            metadata,
            flags: self.flags,
            item_template,
            min_items: self.min_items,
            max_items: self.max_items,
            unique: self.unique,
            sortable: self.sortable,
            ranking: self.ranking,
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
    fn test_list_basic() {
        let list = List::builder("tags")
            .label("Tags")
            .item_template(Text::builder("tag").build())
            .build()
            .unwrap();

        assert_eq!(list.key().as_str(), "tags");
        assert_eq!(list.metadata().label(), Some("Tags"));
        assert_eq!(list.kind(), NodeKind::Container);
    }

    #[test]
    fn test_list_constraints() {
        let list = List::builder("items")
            .item_template(Text::builder("item").build())
            .min_items(1)
            .max_items(10)
            .unique(true)
            .sortable(true)
            .build()
            .unwrap();

        assert_eq!(list.min_items(), Some(1));
        assert_eq!(list.max_items(), Some(10));
        assert!(list.is_unique());
        assert!(list.is_sortable());
    }

    #[test]
    fn test_list_no_constraints() {
        let list = List::builder("items")
            .item_template(Text::builder("item").build())
            .build()
            .unwrap();

        assert_eq!(list.min_items(), None);
        assert_eq!(list.max_items(), None);
        assert!(!list.is_unique());
        assert!(!list.is_sortable());
    }

    #[test]
    fn test_list_flags() {
        let list = List::builder("required_list")
            .item_template(Text::builder("item").build())
            .required()
            .build()
            .unwrap();

        assert!(list.flags().contains(Flags::REQUIRED));
    }

    #[test]
    fn test_list_item_template() {
        let template = Text::builder("tag").build();
        let list = List::builder("tags")
            .item_template(template)
            .build()
            .unwrap();

        assert_eq!(list.item_template().key().as_str(), "tag");
    }

    #[test]
    fn test_list_requires_template() {
        let result = List::builder("no_template").build();
        assert!(result.is_err());
    }

    #[test]
    fn test_list_min_max_validation() {
        // Valid: min < max
        let result = List::builder("valid")
            .item_template(Text::builder("item").build())
            .min_items(1)
            .max_items(10)
            .build();
        assert!(result.is_ok());

        // Valid: min == max
        let result = List::builder("equal")
            .item_template(Text::builder("item").build())
            .min_items(5)
            .max_items(5)
            .build();
        assert!(result.is_ok());

        // Invalid: min > max
        let result = List::builder("invalid")
            .item_template(Text::builder("item").build())
            .min_items(10)
            .max_items(5)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_list_rankable_simple() {
        let list = List::builder("priorities")
            .item_template(Text::builder("task").build())
            .rankable()
            .build()
            .unwrap();

        assert!(list.is_rankable());
        assert!(list.is_sortable()); // Ranking implies sortable
        let config = list.ranking_config().unwrap();
        assert!(!config.shows_numbers());
        assert_eq!(config.get_direction(), RankDirection::HighestFirst);
    }

    #[test]
    fn test_list_rankable_with_config() {
        let list = List::builder("candidates")
            .item_template(Text::builder("name").build())
            .ranking_config(
                RankingConfig::new()
                    .show_numbers(true)
                    .direction(RankDirection::LowestFirst),
            )
            .build()
            .unwrap();

        assert!(list.is_rankable());
        assert!(list.is_sortable());
        let config = list.ranking_config().unwrap();
        assert!(config.shows_numbers());
        assert_eq!(config.get_direction(), RankDirection::LowestFirst);
    }

    #[test]
    fn test_list_not_rankable_by_default() {
        let list = List::builder("items")
            .item_template(Text::builder("item").build())
            .build()
            .unwrap();

        assert!(!list.is_rankable());
        assert!(list.ranking_config().is_none());
    }

    #[test]
    fn test_list_rankable_with_unique() {
        let list = List::builder("feature_ranking")
            .item_template(Text::builder("feature").build())
            .rankable()
            .unique(true)
            .min_items(3)
            .build()
            .unwrap();

        assert!(list.is_rankable());
        assert!(list.is_unique());
        assert_eq!(list.min_items(), Some(3));
    }

    #[test]
    fn test_ranking_config_builder() {
        let config = RankingConfig::new()
            .show_numbers(true)
            .direction(RankDirection::LowestFirst);

        assert!(config.shows_numbers());
        assert_eq!(config.get_direction(), RankDirection::LowestFirst);
    }

    #[test]
    fn test_rank_direction_name() {
        assert_eq!(RankDirection::HighestFirst.name(), "highest_first");
        assert_eq!(RankDirection::LowestFirst.name(), "lowest_first");
    }
}
