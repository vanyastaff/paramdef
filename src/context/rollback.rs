//! Optimized rollback storage for transactional updates.
//!
//! Uses stack-allocated buffer for small transactions (≤8 fields) and
//! heap-allocated HashMap for larger transactions. This eliminates heap
//! allocations for the common case of small transactional updates.

use crate::core::{Key, Value};
use rustc_hash::FxHashMap;

/// Rollback storage for transactional updates.
///
/// Optimizes for the common case of small transactions by using a stack-allocated
/// buffer for up to 8 fields. Automatically upgrades to heap storage when needed.
///
/// # Performance
///
/// - **Small** variant (≤8 fields): Zero heap allocations, stack-only storage
/// - **Large** variant (>8 fields): Single heap allocation for HashMap
/// - Automatic upgrade: Seamless transition from Small to Large
///
/// # Example
///
/// ```
/// use paramdef::context::rollback::RollbackStorage;
/// use paramdef::core::Value;
///
/// let mut storage = RollbackStorage::new();
///
/// // Store up to 8 values without heap allocation
/// storage.store("field1".into(), Some(Value::Int(1)));
/// storage.store("field2".into(), Some(Value::Int(2)));
///
/// assert!(storage.is_small());
/// assert_eq!(storage.len(), 2);
/// ```
pub enum RollbackStorage {
    /// Stack-allocated storage for ≤8 fields.
    ///
    /// Uses a fixed-size array on the stack to avoid heap allocations
    /// for the common case of small transactional updates.
    Small {
        /// Stack-allocated buffer of (Key, Option<Value>) pairs.
        /// Option<Value> allows storing None to indicate field was cleared.
        buffer: [(Key, Option<Value>); 8],
        /// Number of items currently stored.
        count: usize,
    },

    /// Heap-allocated storage for >8 fields.
    ///
    /// Uses FxHashMap for efficient storage of larger transactions.
    Large(FxHashMap<Key, Option<Value>>),
}

impl RollbackStorage {
    /// Creates a new empty rollback storage.
    ///
    /// Starts as `Small` variant for zero-allocation initialization.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::context::rollback::RollbackStorage;
    ///
    /// let storage = RollbackStorage::new();
    /// assert!(storage.is_small());
    /// assert_eq!(storage.len(), 0);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::Small {
            // Initialize buffer with empty keys and None values
            // We use const block to create the array without requiring Default
            buffer: [
                (Key::from(""), None),
                (Key::from(""), None),
                (Key::from(""), None),
                (Key::from(""), None),
                (Key::from(""), None),
                (Key::from(""), None),
                (Key::from(""), None),
                (Key::from(""), None),
            ],
            count: 0,
        }
    }

    /// Creates rollback storage with a known capacity.
    ///
    /// If capacity > 8, starts as `Large` variant to avoid upgrade overhead.
    /// Otherwise starts as `Small` variant.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::context::rollback::RollbackStorage;
    ///
    /// let small = RollbackStorage::with_capacity(5);
    /// assert!(small.is_small());
    ///
    /// let large = RollbackStorage::with_capacity(10);
    /// assert!(large.is_large());
    /// ```
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        if capacity > 8 {
            Self::Large(FxHashMap::with_capacity_and_hasher(
                capacity,
                Default::default(),
            ))
        } else {
            Self::new()
        }
    }

    /// Stores a key-value pair for rollback.
    ///
    /// Automatically upgrades from `Small` to `Large` when storing the 9th item.
    ///
    /// # Arguments
    ///
    /// * `key` - The parameter key
    /// * `value` - The original value (Some) or None if parameter was unset
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::context::rollback::RollbackStorage;
    /// use paramdef::core::Value;
    ///
    /// let mut storage = RollbackStorage::new();
    /// storage.store("field1".into(), Some(Value::Int(42)));
    /// storage.store("field2".into(), None); // Was unset
    /// assert_eq!(storage.len(), 2);
    /// ```
    pub fn store(&mut self, key: Key, value: Option<Value>) {
        match self {
            Self::Small { buffer, count } => {
                if *count < 8 {
                    // Store in buffer
                    buffer[*count] = (key, value);
                    *count += 1;
                } else {
                    // Upgrade to Large
                    self.upgrade_to_large();
                    // Store in the new Large variant
                    if let Self::Large(map) = self {
                        map.insert(key, value);
                    }
                }
            }
            Self::Large(map) => {
                map.insert(key, value);
            }
        }
    }

    /// Upgrades from Small to Large variant.
    ///
    /// Copies all items from the stack buffer to a new HashMap.
    fn upgrade_to_large(&mut self) {
        if let Self::Small { buffer, count } = self {
            let mut map = FxHashMap::with_capacity_and_hasher(16, Default::default());

            // Copy all items from buffer to map
            // We clone because Key doesn't implement Default
            for i in 0..*count {
                let (key, value) = buffer[i].clone();
                map.insert(key, value);
            }

            *self = Self::Large(map);
        }
    }

    /// Returns the number of stored items.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::context::rollback::RollbackStorage;
    /// use paramdef::core::Value;
    ///
    /// let mut storage = RollbackStorage::new();
    /// assert_eq!(storage.len(), 0);
    ///
    /// storage.store("key".into(), Some(Value::Int(1)));
    /// assert_eq!(storage.len(), 1);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Small { count, .. } => *count,
            Self::Large(map) => map.len(),
        }
    }

    /// Returns `true` if the storage is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::context::rollback::RollbackStorage;
    ///
    /// let storage = RollbackStorage::new();
    /// assert!(storage.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns `true` if using `Small` (stack) variant.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::context::rollback::RollbackStorage;
    ///
    /// let storage = RollbackStorage::new();
    /// assert!(storage.is_small());
    /// ```
    #[must_use]
    pub const fn is_small(&self) -> bool {
        matches!(self, Self::Small { .. })
    }

    /// Returns `true` if using `Large` (heap) variant.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::context::rollback::RollbackStorage;
    ///
    /// let storage = RollbackStorage::with_capacity(10);
    /// assert!(storage.is_large());
    /// ```
    #[must_use]
    pub const fn is_large(&self) -> bool {
        matches!(self, Self::Large(..))
    }

    /// Clears all stored items.
    ///
    /// For `Small` variant, just resets count. For `Large` variant, clears the HashMap.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::context::rollback::RollbackStorage;
    /// use paramdef::core::Value;
    ///
    /// let mut storage = RollbackStorage::new();
    /// storage.store("key".into(), Some(Value::Int(1)));
    /// assert_eq!(storage.len(), 1);
    ///
    /// storage.clear();
    /// assert_eq!(storage.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        match self {
            Self::Small { count, .. } => {
                // Just reset count - buffer items will be overwritten
                *count = 0;
            }
            Self::Large(map) => map.clear(),
        }
    }

    /// Returns an iterator over the stored items.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::context::rollback::RollbackStorage;
    /// use paramdef::core::Value;
    ///
    /// let mut storage = RollbackStorage::new();
    /// storage.store("a".into(), Some(Value::Int(1)));
    /// storage.store("b".into(), Some(Value::Int(2)));
    ///
    /// let items: Vec<_> = storage.iter().collect();
    /// assert_eq!(items.len(), 2);
    /// ```
    pub fn iter(&self) -> RollbackStorageIter<'_> {
        match self {
            Self::Small { buffer, count } => RollbackStorageIter::Small {
                buffer,
                count: *count,
                index: 0,
            },
            Self::Large(map) => RollbackStorageIter::Large(map.iter()),
        }
    }
}

impl Default for RollbackStorage {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator over RollbackStorage items.
pub enum RollbackStorageIter<'a> {
    /// Iterator over Small variant buffer.
    Small {
        buffer: &'a [(Key, Option<Value>); 8],
        count: usize,
        index: usize,
    },
    /// Iterator over Large variant HashMap.
    Large(std::collections::hash_map::Iter<'a, Key, Option<Value>>),
}

impl<'a> Iterator for RollbackStorageIter<'a> {
    type Item = (&'a Key, &'a Option<Value>);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Small {
                buffer,
                count,
                index,
            } => {
                if *index < *count {
                    let item = &buffer[*index];
                    *index += 1;
                    Some((&item.0, &item.1))
                } else {
                    None
                }
            }
            Self::Large(iter) => iter.next(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_small() {
        let storage = RollbackStorage::new();
        assert!(storage.is_small());
        assert_eq!(storage.len(), 0);
    }

    #[test]
    fn test_with_capacity_chooses_variant() {
        let small = RollbackStorage::with_capacity(5);
        assert!(small.is_small());

        let large = RollbackStorage::with_capacity(10);
        assert!(large.is_large());
    }

    #[test]
    fn test_store_in_small() {
        let mut storage = RollbackStorage::new();
        storage.store("key".into(), Some(Value::Int(42)));
        assert_eq!(storage.len(), 1);
        assert!(storage.is_small());
    }

    #[test]
    fn test_upgrade_on_9th_item() {
        let mut storage = RollbackStorage::new();

        for i in 0..8 {
            storage.store(format!("key{}", i).into(), Some(Value::Int(i)));
        }
        assert!(storage.is_small());

        storage.store("key8".into(), Some(Value::Int(8)));
        assert!(storage.is_large());
        assert_eq!(storage.len(), 9);
    }

    #[test]
    fn test_clear() {
        let mut storage = RollbackStorage::new();
        storage.store("key".into(), Some(Value::Int(42)));
        assert_eq!(storage.len(), 1);

        storage.clear();
        assert_eq!(storage.len(), 0);
        assert!(storage.is_empty());
    }

    #[test]
    fn test_iter_small() {
        let mut storage = RollbackStorage::new();
        storage.store("a".into(), Some(Value::Int(1)));
        storage.store("b".into(), Some(Value::Int(2)));

        let items: Vec<_> = storage.iter().collect();
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn test_iter_large() {
        let mut storage = RollbackStorage::with_capacity(10);
        storage.store("a".into(), Some(Value::Int(1)));
        storage.store("b".into(), Some(Value::Int(2)));

        let items: Vec<_> = storage.iter().collect();
        assert_eq!(items.len(), 2);
    }
}
