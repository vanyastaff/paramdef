//! Global validator registry for custom validators.
//!
//! This module provides a global registry that allows users to register
//! custom validators by name. Validators can then be referenced by name
//! in validation rules.
//!
//! # Example
//!
//! ```ignore
//! use paramdef::validation::{ValidatorRegistry, Validator, ValidationContext, ValidationResult};
//! use paramdef::core::Value;
//!
//! // Define custom validator
//! #[derive(Debug)]
//! struct UniqueUsernameValidator;
//!
//! impl Validator for UniqueUsernameValidator {
//!     fn name(&self) -> &str {
//!         "unique_username"
//!     }
//!
//!     fn validate(&self, value: &Value, _ctx: &ValidationContext<'_>) -> ValidationResult {
//!         // Check database for uniqueness
//!         // For demo, just check length
//!         if let Some(s) = value.as_text() {
//!             if s.len() >= 3 {
//!                 return Ok(());
//!             }
//!         }
//!         Err(crate::validation::Error::custom(
//!             "username_taken",
//!             "Username must be at least 3 characters"
//!         ).into())
//!     }
//! }
//!
//! // Register globally
//! ValidatorRegistry::global().write().unwrap()
//!     .register(UniqueUsernameValidator);
//!
//! // Use by name
//! use paramdef::validation::Rule;
//! let rule = Rule::registered("unique_username");
//! ```

use std::sync::{Arc, LazyLock, RwLock};

use crate::core::{FxHashMap, SmartStr};

use super::traits::Validator;

/// Global registry of named validators.
///
/// This registry allows custom validators to be registered globally by name
/// and then referenced in validation rules without coupling to specific types.
///
/// # Thread Safety
///
/// The global registry uses `RwLock` for concurrent access. Multiple readers
/// can access simultaneously, but writes require exclusive access.
///
/// # Example
///
/// ```
/// use paramdef::validation::{ValidatorRegistry, Validator, ValidationContext, ValidationResult};
/// use paramdef::core::Value;
///
/// #[derive(Debug)]
/// struct MyValidator;
///
/// impl Validator for MyValidator {
///     fn name(&self) -> &str { "my_validator" }
///     fn validate(&self, _value: &Value, _ctx: &ValidationContext<'_>) -> ValidationResult {
///         Ok(())
///     }
/// }
///
/// // Register
/// ValidatorRegistry::global().write().unwrap()
///     .register(MyValidator);
///
/// // Check if registered
/// let registry = ValidatorRegistry::global().read().unwrap();
/// assert!(registry.contains("my_validator"));
/// ```
pub struct ValidatorRegistry {
    validators: FxHashMap<SmartStr, Arc<dyn Validator>>,
}

impl Default for ValidatorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidatorRegistry {
    /// Creates a new empty validator registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            validators: FxHashMap::default(),
        }
    }

    /// Returns a reference to the global validator registry.
    ///
    /// The global registry is lazily initialized on first access.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::validation::ValidatorRegistry;
    ///
    /// let registry = ValidatorRegistry::global();
    /// let guard = registry.read().unwrap();
    /// println!("Registered validators: {}", guard.len());
    /// ```
    #[must_use]
    pub fn global() -> &'static RwLock<Self> {
        static REGISTRY: LazyLock<RwLock<ValidatorRegistry>> =
            LazyLock::new(|| RwLock::new(ValidatorRegistry::new()));
        &REGISTRY
    }

    /// Registers a validator by name.
    ///
    /// If a validator with the same name already exists, it will be replaced.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::validation::{ValidatorRegistry, Validator, ValidationContext, ValidationResult};
    /// use paramdef::core::Value;
    ///
    /// #[derive(Debug)]
    /// struct MyValidator;
    ///
    /// impl Validator for MyValidator {
    ///     fn name(&self) -> &str { "my_validator" }
    ///     fn validate(&self, _value: &Value, _ctx: &ValidationContext<'_>) -> ValidationResult {
    ///         Ok(())
    ///     }
    /// }
    ///
    /// let mut registry = ValidatorRegistry::new();
    /// registry.register(MyValidator);
    /// assert!(registry.contains("my_validator"));
    /// ```
    pub fn register(&mut self, validator: impl Validator + 'static) {
        let name = SmartStr::from(validator.name());
        self.validators.insert(name, Arc::new(validator));
    }

    /// Registers a validator that's already wrapped in Arc.
    ///
    /// This is useful when sharing validator instances across multiple registries.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::validation::{ValidatorRegistry, Validator, ValidationContext, ValidationResult};
    /// use paramdef::core::Value;
    /// use std::sync::Arc;
    ///
    /// #[derive(Debug)]
    /// struct MyValidator;
    ///
    /// impl Validator for MyValidator {
    ///     fn name(&self) -> &str { "shared" }
    ///     fn validate(&self, _value: &Value, _ctx: &ValidationContext<'_>) -> ValidationResult {
    ///         Ok(())
    ///     }
    /// }
    ///
    /// let validator = Arc::new(MyValidator);
    /// let mut registry = ValidatorRegistry::new();
    /// registry.register_arc(validator);
    /// ```
    pub fn register_arc(&mut self, validator: Arc<dyn Validator>) {
        let name = SmartStr::from(validator.name());
        self.validators.insert(name, validator);
    }

    /// Retrieves a validator by name.
    ///
    /// Returns `None` if the validator is not registered.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::validation::ValidatorRegistry;
    ///
    /// let registry = ValidatorRegistry::new();
    /// assert!(registry.get("nonexistent").is_none());
    /// ```
    #[must_use]
    pub fn get(&self, name: &str) -> Option<Arc<dyn Validator>> {
        self.validators.get(name).cloned()
    }

    /// Checks if a validator is registered.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::validation::ValidatorRegistry;
    ///
    /// let registry = ValidatorRegistry::new();
    /// assert!(!registry.contains("custom"));
    /// ```
    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.validators.contains_key(name)
    }

    /// Returns the number of registered validators.
    #[must_use]
    pub fn len(&self) -> usize {
        self.validators.len()
    }

    /// Returns `true` if the registry has no validators.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.validators.is_empty()
    }

    /// Returns an iterator over validator names.
    pub fn validator_names(&self) -> impl Iterator<Item = &str> {
        self.validators.values().map(|v| v.name())
    }

    /// Clears all registered validators.
    ///
    /// This is primarily useful for testing.
    pub fn clear(&mut self) {
        self.validators.clear();
    }
}

impl std::fmt::Debug for ValidatorRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValidatorRegistry")
            .field("count", &self.len())
            .field("validators", &self.validator_names().collect::<Vec<_>>())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Value;
    use crate::validation::{ValidationContext, ValidationResult};

    #[derive(Debug)]
    struct TestValidator {
        name: &'static str,
    }

    impl Validator for TestValidator {
        fn name(&self) -> &str {
            self.name
        }

        fn validate(&self, _value: &Value, _ctx: &ValidationContext<'_>) -> ValidationResult {
            Ok(())
        }
    }

    #[test]
    fn test_new_registry() {
        let registry = ValidatorRegistry::new();
        assert_eq!(registry.len(), 0);
        assert!(registry.is_empty());
    }

    #[test]
    fn test_register_and_get() {
        let mut registry = ValidatorRegistry::new();

        registry.register(TestValidator { name: "test1" });

        assert_eq!(registry.len(), 1);
        assert!(registry.contains("test1"));
        assert!(registry.get("test1").is_some());
        assert!(registry.get("nonexistent").is_none());
    }

    #[test]
    fn test_register_multiple() {
        let mut registry = ValidatorRegistry::new();

        registry.register(TestValidator { name: "test1" });
        registry.register(TestValidator { name: "test2" });
        registry.register(TestValidator { name: "test3" });

        assert_eq!(registry.len(), 3);
        assert!(registry.contains("test1"));
        assert!(registry.contains("test2"));
        assert!(registry.contains("test3"));
    }

    #[test]
    fn test_replace_validator() {
        let mut registry = ValidatorRegistry::new();

        registry.register(TestValidator { name: "test" });
        assert_eq!(registry.len(), 1);

        // Register again with same name
        registry.register(TestValidator { name: "test" });
        assert_eq!(registry.len(), 1); // Should still be 1
    }

    #[test]
    fn test_register_arc() {
        let mut registry = ValidatorRegistry::new();
        let validator = Arc::new(TestValidator { name: "arc_test" });

        registry.register_arc(validator);

        assert!(registry.contains("arc_test"));
    }

    #[test]
    fn test_clear() {
        let mut registry = ValidatorRegistry::new();

        registry.register(TestValidator { name: "test1" });
        registry.register(TestValidator { name: "test2" });
        assert_eq!(registry.len(), 2);

        registry.clear();
        assert_eq!(registry.len(), 0);
        assert!(registry.is_empty());
    }

    #[test]
    fn test_validator_names() {
        let mut registry = ValidatorRegistry::new();

        registry.register(TestValidator { name: "alpha" });
        registry.register(TestValidator { name: "beta" });

        let names: Vec<&str> = registry.validator_names().collect();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"alpha"));
        assert!(names.contains(&"beta"));
    }

    #[test]
    fn test_global_registry() {
        // Clear any previous state
        ValidatorRegistry::global().write().unwrap().clear();

        // Register in global
        {
            let mut registry = ValidatorRegistry::global().write().unwrap();
            registry.register(TestValidator {
                name: "global_test",
            });
        }

        // Check from read lock
        {
            let registry = ValidatorRegistry::global().read().unwrap();
            assert!(registry.contains("global_test"));
        }

        // Cleanup
        ValidatorRegistry::global().write().unwrap().clear();
    }
}
