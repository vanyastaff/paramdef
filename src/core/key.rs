//! Parameter key type.
//!
//! Keys are used to identify parameters within a schema. They use [`SmartString`]
//! for efficient storage - strings shorter than 23 bytes are stored inline on the
//! stack without heap allocation.
//!
//! # Examples
//!
//! ```
//! use paramdef::core::Key;
//!
//! let key = Key::new("my_parameter");
//! assert_eq!(key.as_str(), "my_parameter");
//! ```

use std::borrow::Borrow;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use smartstring::{LazyCompact, SmartString};

/// A parameter identifier using stack-optimized strings.
///
/// Keys are typically short identifiers like `"username"`, `"port"`, or `"enabled"`.
/// Using [`SmartString`] with [`LazyCompact`] mode means strings up to 23 bytes
/// are stored inline without heap allocation.
///
/// # Examples
///
/// ```
/// use paramdef::core::Key;
///
/// // Create with new()
/// let key = Key::new("config_value");
///
/// // Or use From trait
/// let key2: Key = "config_value".into();
/// assert_eq!(key, key2);
///
/// // Keys deref to str
/// assert_eq!(&*key, "config_value");
///
/// // Display shows the key value
/// assert_eq!(format!("{}", key), "config_value");
/// ```
#[derive(Debug, Clone, Eq)]
pub struct Key(SmartString<LazyCompact>);

impl Key {
    /// Creates a new key from a string-like value.
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::core::Key;
    ///
    /// let key = Key::new("my_param");
    /// assert_eq!(key.as_str(), "my_param");
    ///
    /// // Works with String too
    /// let key2 = Key::new(String::from("other_param"));
    /// assert_eq!(key2.as_str(), "other_param");
    /// ```
    pub fn new(s: impl AsRef<str>) -> Self {
        Self(s.as_ref().into())
    }

    /// Returns the key as a string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use paramdef::core::Key;
    ///
    /// let key = Key::new("test");
    /// assert_eq!(key.as_str(), "test");
    /// ```
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the length of the key in bytes.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the key is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Deref for Key {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Key {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for Key {
    #[inline]
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<str> for Key {
    fn eq(&self, other: &str) -> bool {
        self.0.as_str() == other
    }
}

impl PartialEq<&str> for Key {
    fn eq(&self, other: &&str) -> bool {
        self.0.as_str() == *other
    }
}

impl PartialEq<String> for Key {
    fn eq(&self, other: &String) -> bool {
        self.0.as_str() == other.as_str()
    }
}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Key {
    fn from(s: &str) -> Self {
        Self(s.into())
    }
}

impl From<String> for Key {
    fn from(s: String) -> Self {
        Self(s.into())
    }
}

impl From<SmartString<LazyCompact>> for Key {
    fn from(s: SmartString<LazyCompact>) -> Self {
        Self(s)
    }
}

// =============================================================================
// Serde Support (Feature-Gated)
// =============================================================================

#[cfg(feature = "serde")]
impl Serialize for Key {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Key {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Key::new(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_new() {
        let key = Key::new("my_param");
        assert_eq!(key.as_str(), "my_param");
    }

    #[test]
    fn test_key_from_str() {
        let key: Key = "my_param".into();
        assert_eq!(key.as_str(), "my_param");
    }

    #[test]
    fn test_key_equality() {
        let key1 = Key::new("test_key");
        let key2 = Key::new("test_key");
        let key3 = Key::new("other_key");

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_key_equality_with_str() {
        let key = Key::new("test");
        assert_eq!(key, "test");
        assert_eq!(key, *"test");
        assert_ne!(key, "other");
    }

    #[test]
    fn test_key_equality_with_string() {
        let key = Key::new("test");
        assert_eq!(key, String::from("test"));
    }

    #[test]
    fn test_key_display() {
        let key = Key::new("display_test");
        assert_eq!(format!("{}", key), "display_test");
    }

    #[test]
    fn test_key_from_string() {
        let s = String::from("from_string");
        let key: Key = s.into();
        assert_eq!(key.as_str(), "from_string");
    }

    #[test]
    fn test_key_clone() {
        let key1 = Key::new("clone_test");
        let key2 = key1.clone();
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_key_deref() {
        let key = Key::new("deref_test");
        // Deref allows using str methods directly
        assert!(key.starts_with("deref"));
        assert!(key.ends_with("test"));
        assert_eq!(key.len(), 10);
    }

    #[test]
    fn test_key_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(Key::new("key1"));
        set.insert(Key::new("key2"));
        set.insert(Key::new("key1")); // duplicate

        assert_eq!(set.len(), 2);
        assert!(set.contains(&Key::new("key1")));
        assert!(set.contains(&Key::new("key2")));
    }

    #[test]
    fn test_key_short_string_optimization() {
        // Strings <= 23 bytes should be stored inline
        let short = Key::new("short");
        let exactly_23 = Key::new("12345678901234567890123");

        assert_eq!(short.len(), 5);
        assert_eq!(exactly_23.len(), 23);
    }

    #[test]
    fn test_key_is_empty() {
        let empty = Key::new("");
        let non_empty = Key::new("test");

        assert!(empty.is_empty());
        assert!(!non_empty.is_empty());
    }
}
