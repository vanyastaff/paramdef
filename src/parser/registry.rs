//! Function registry for extensible expression parsing.
//!
//! The registry pattern allows custom function parsers to be registered
//! at runtime, making the expression parser fully extensible.
//!
//! # Example
//!
//! ```
//! use paramdef::parser::{FunctionRegistry, FunctionParser, Arity};
//! use paramdef::expr::Expr;
//! use paramdef::core::Value;
//!
//! // Custom function parser
//! struct CustomValidator;
//!
//! impl FunctionParser for CustomValidator {
//!     fn name(&self) -> &'static str { "custom" }
//!     fn arity(&self) -> Arity { Arity::Fixed(1) }
//!     fn parse(&self, args: &[Value]) -> Result<Expr, String> {
//!         // Custom parsing logic
//!         Ok(Expr::required())
//!     }
//! }
//!
//! // Create registry and register custom parser
//! let mut registry = FunctionRegistry::new();
//! registry.register(CustomValidator);
//!
//! // Parse expression using custom function
//! let expr = registry.parse("custom", &[Value::Int(42)])?;
//! # Ok::<(), String>(())
//! ```

use std::sync::Arc;

use crate::core::{FxHashMap, SmartStr, Value};
use crate::expr::Expr;

use super::function_parser::FunctionParser;

/// Registry of function parsers for expression parsing.
///
/// The registry maps function names (case-insensitive) to their parsers.
/// It supports both built-in functions and custom user-defined functions.
///
/// # Thread Safety
///
/// The registry can be wrapped in `Arc` for shared access across threads.
/// Function parsers are stored as `Arc<dyn FunctionParser>` for cheap cloning.
#[derive(Clone)]
pub struct FunctionRegistry {
    functions: FxHashMap<SmartStr, Arc<dyn FunctionParser>>,
}

impl Default for FunctionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl FunctionRegistry {
    /// Creates an empty registry.
    ///
    /// Use [`with_builtins()`](Self::with_builtins) to create a registry
    /// pre-populated with built-in function parsers.
    #[must_use]
    pub fn new() -> Self {
        Self {
            functions: FxHashMap::default(),
        }
    }

    /// Creates a registry with all built-in function parsers registered.
    ///
    /// This is the recommended way to create a registry for most use cases.
    /// Built-in functions include: email, url, uuid, `min_length`, `max_length`,
    /// `starts_with`, `ends_with`, min, max, positive, negative, integer, etc.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::parser::FunctionRegistry;
    /// use paramdef::core::Value;
    ///
    /// let registry = FunctionRegistry::with_builtins();
    /// let expr = registry.parse("email", &[])?;
    /// # Ok::<(), String>(())
    /// ```
    #[must_use]
    pub fn with_builtins() -> Self {
        let mut registry = Self::new();
        super::builtins::register_all(&mut registry);
        registry
    }

    /// Registers a function parser.
    ///
    /// Function names are case-insensitive. If a parser with the same name
    /// already exists, it will be replaced.
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::parser::{FunctionRegistry, FunctionParser, Arity};
    /// use paramdef::expr::Expr;
    /// use paramdef::core::Value;
    ///
    /// struct MyParser;
    /// impl FunctionParser for MyParser {
    ///     fn name(&self) -> &'static str { "my_func" }
    ///     fn arity(&self) -> Arity { Arity::Fixed(0) }
    ///     fn parse(&self, _args: &[Value]) -> Result<Expr, String> {
    ///         Ok(Expr::required())
    ///     }
    /// }
    ///
    /// let mut registry = FunctionRegistry::new();
    /// registry.register(MyParser);
    /// ```
    pub fn register(&mut self, parser: impl FunctionParser + 'static) {
        let name = parser.name().to_uppercase();
        self.functions
            .insert(SmartStr::from(name), Arc::new(parser));
    }

    /// Registers a parser that's already wrapped in Arc.
    ///
    /// This is useful when sharing parser instances across multiple registries.
    pub fn register_arc(&mut self, parser: Arc<dyn FunctionParser>) {
        let name = parser.name().to_uppercase();
        self.functions.insert(SmartStr::from(name), parser);
    }

    /// Parses a function call into an expression.
    ///
    /// # Arguments
    ///
    /// * `func_name` - Function name (case-insensitive)
    /// * `args` - Function arguments
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Function is not registered
    /// - Wrong number of arguments
    /// - Invalid argument types
    ///
    /// # Example
    ///
    /// ```
    /// use paramdef::parser::FunctionRegistry;
    /// use paramdef::core::Value;
    ///
    /// let registry = FunctionRegistry::with_builtins();
    ///
    /// // Parse with correct arguments
    /// let expr = registry.parse("min_length", &[Value::Int(5)])?;
    ///
    /// // Error: unknown function
    /// let err = registry.parse("unknown", &[]);
    /// assert!(err.is_err());
    ///
    /// // Error: wrong argument count
    /// let err = registry.parse("email", &[Value::Int(42)]);
    /// assert!(err.is_err());
    /// # Ok::<(), String>(())
    /// ```
    pub fn parse(&self, func_name: &str, args: &[Value]) -> Result<Expr, String> {
        let func_upper = func_name.to_uppercase();

        let parser = self
            .functions
            .get(func_upper.as_str())
            .ok_or_else(|| format!("Unknown function: {func_name}"))?;

        // Validate arity before parsing
        let arity = parser.arity();
        if !arity.accepts(args.len()) {
            return Err(format!(
                "{}() requires {}, got {}",
                func_name,
                arity.describe(),
                args.len()
            ));
        }

        parser.parse(args)
    }

    /// Returns true if the function is registered.
    #[must_use]
    pub fn contains(&self, func_name: &str) -> bool {
        let func_upper = func_name.to_uppercase();
        self.functions.contains_key(func_upper.as_str())
    }

    /// Returns the number of registered functions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.functions.len()
    }

    /// Returns true if the registry has no functions.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.functions.is_empty()
    }

    /// Returns an iterator over registered function names.
    pub fn function_names(&self) -> impl Iterator<Item = &str> {
        self.functions.values().map(|p| p.name())
    }
}

impl std::fmt::Debug for FunctionRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionRegistry")
            .field("count", &self.len())
            .field("functions", &self.function_names().collect::<Vec<_>>())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestParser {
        name: &'static str,
        arity: Arity,
    }

    impl FunctionParser for TestParser {
        fn name(&self) -> &'static str {
            self.name
        }

        fn parse(&self, _args: &[Value]) -> Result<Expr, String> {
            Ok(Expr::required())
        }

        fn arity(&self) -> Arity {
            self.arity
        }
    }

    #[test]
    fn test_register_and_parse() {
        let mut registry = FunctionRegistry::new();
        registry.register(TestParser {
            name: "test",
            arity: Arity::Fixed(0),
        });

        assert!(registry.contains("test"));
        assert!(registry.contains("TEST")); // case-insensitive

        let result = registry.parse("test", &[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unknown_function() {
        let registry = FunctionRegistry::new();
        let result = registry.parse("unknown", &[]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown function"));
    }

    #[test]
    fn test_arity_validation() {
        let mut registry = FunctionRegistry::new();
        registry.register(TestParser {
            name: "test",
            arity: Arity::Fixed(2),
        });

        // Wrong arg count
        let result = registry.parse("test", &[]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("requires 2 arguments"));

        // Correct arg count
        let result = registry.parse("test", &[Value::Int(1), Value::Int(2)]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_case_insensitive() {
        let mut registry = FunctionRegistry::new();
        registry.register(TestParser {
            name: "test",
            arity: Arity::Fixed(0),
        });

        assert!(registry.parse("test", &[]).is_ok());
        assert!(registry.parse("TEST", &[]).is_ok());
        assert!(registry.parse("TeSt", &[]).is_ok());
    }

    #[test]
    fn test_replace_parser() {
        let mut registry = FunctionRegistry::new();

        registry.register(TestParser {
            name: "test",
            arity: Arity::Fixed(0),
        });

        // Replace with different arity
        registry.register(TestParser {
            name: "test",
            arity: Arity::Fixed(1),
        });

        // Old arity should fail
        assert!(registry.parse("test", &[]).is_err());

        // New arity should work
        assert!(registry.parse("test", &[Value::Int(1)]).is_ok());
    }

    #[test]
    fn test_with_builtins() {
        let registry = FunctionRegistry::with_builtins();

        // Should have built-in functions
        assert!(registry.len() > 0);
        assert!(registry.contains("email"));
        assert!(registry.contains("url"));
        assert!(registry.contains("min_length"));
    }
}
