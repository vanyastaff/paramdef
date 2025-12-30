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
//! let key: Key = "my_parameter".into();
//! assert_eq!(key.as_str(), "my_parameter");
//! ```

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
/// // Create from string literal
/// let key: Key = "config_value".into();
///
/// // Keys support equality comparison
/// let key2: Key = "config_value".into();
/// assert_eq!(key, key2);
///
/// // Display shows the key value
/// assert_eq!(format!("{}", key), "config_value");
/// ```
pub type Key = SmartString<LazyCompact>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_from_str() {
        let key: Key = "my_param".into();
        assert_eq!(key.as_str(), "my_param");
    }

    #[test]
    fn test_key_equality() {
        let key1: Key = "test_key".into();
        let key2: Key = "test_key".into();
        let key3: Key = "other_key".into();

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_key_display() {
        let key: Key = "display_test".into();
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
        let key1: Key = "clone_test".into();
        let key2 = key1.clone();
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_key_short_string_optimization() {
        // Strings <= 23 bytes should be stored inline
        let short: Key = "short".into();
        let exactly_23: Key = "12345678901234567890123".into();

        assert_eq!(short.len(), 5);
        assert_eq!(exactly_23.len(), 23);
    }
}
