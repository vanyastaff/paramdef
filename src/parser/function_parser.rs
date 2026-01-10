//! Function parser trait for extensible expression parsing.
//!
//! This module provides the core abstraction for parsing function calls
//! in expressions. Function parsers convert function names and arguments
//! into [`Expr`] variants.
//!
//! # Example
//!
//! ```
//! use paramdef::parser::{FunctionParser, Arity};
//! use paramdef::expr::Expr;
//! use paramdef::core::Value;
//!
//! struct EmailParser;
//!
//! impl FunctionParser for EmailParser {
//!     fn name(&self) -> &'static str {
//!         "email"
//!     }
//!
//!     fn parse(&self, args: &[Value]) -> Result<Expr, String> {
//!         if !args.is_empty() {
//!             return Err(format!("email() takes no arguments, got {}", args.len()));
//!         }
//!         Ok(Expr::email())
//!     }
//!
//!     fn arity(&self) -> Arity {
//!         Arity::Fixed(0)
//!     }
//! }
//! ```

use crate::core::Value;
use crate::expr::Expr;

/// Defines how many arguments a function accepts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arity {
    /// Exact number of arguments required.
    ///
    /// Example: `email()` requires exactly 0 arguments.
    Fixed(usize),

    /// Range of arguments accepted (inclusive).
    ///
    /// Example: `between(min, max)` requires exactly 2 arguments: Range(2, 2).
    /// Example: `clamp(value, min, max)` could accept 1-3 args: Range(1, 3).
    Range(usize, usize),

    /// Any number of arguments accepted.
    ///
    /// Example: `and(...)` accepts 0 or more arguments.
    Variadic,
}

impl Arity {
    /// Checks if the given argument count is valid for this arity.
    #[must_use]
    pub fn accepts(&self, count: usize) -> bool {
        match self {
            Self::Fixed(n) => count == *n,
            Self::Range(min, max) => count >= *min && count <= *max,
            Self::Variadic => true,
        }
    }

    /// Returns a human-readable description of the expected argument count.
    #[must_use]
    pub fn describe(&self) -> String {
        match self {
            Self::Fixed(0) => "no arguments".to_string(),
            Self::Fixed(1) => "1 argument".to_string(),
            Self::Fixed(n) => format!("{n} arguments"),
            Self::Range(min, max) if min == max => format!("{min} arguments"),
            Self::Range(min, max) => format!("{min}-{max} arguments"),
            Self::Variadic => "any number of arguments".to_string(),
        }
    }
}

/// Trait for parsing function calls into expressions.
///
/// Function parsers are registered in a [`FunctionRegistry`](super::FunctionRegistry)
/// and invoked when the parser encounters a function call in an expression.
///
/// # Thread Safety
///
/// Function parsers must be `Send + Sync` as they may be shared across threads
/// when the registry is wrapped in `Arc`.
///
/// # Example
///
/// ```
/// use paramdef::parser::{FunctionParser, Arity};
/// use paramdef::expr::Expr;
/// use paramdef::core::Value;
///
/// struct MinLengthParser;
///
/// impl FunctionParser for MinLengthParser {
///     fn name(&self) -> &'static str {
///         "min_length"
///     }
///
///     fn parse(&self, args: &[Value]) -> Result<Expr, String> {
///         if args.len() != 1 {
///             return Err(format!("min_length() takes 1 argument, got {}", args.len()));
///         }
///
///         if let Some(n) = args[0].as_float() {
///             #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
///             Ok(Expr::min_length(n as usize))
///         } else {
///             Err("min_length() requires numeric argument".to_string())
///         }
///     }
///
///     fn arity(&self) -> Arity {
///         Arity::Fixed(1)
///     }
/// }
/// ```
pub trait FunctionParser: Send + Sync {
    /// Returns the function name (case-insensitive).
    ///
    /// The registry will normalize names to uppercase for matching,
    /// but this should return the canonical lowercase name for documentation.
    fn name(&self) -> &'static str;

    /// Parses the function arguments into an expression.
    ///
    /// # Arguments
    ///
    /// * `args` - The arguments passed to the function call
    ///
    /// # Errors
    ///
    /// Returns an error string if:
    /// - Wrong number of arguments
    /// - Invalid argument types
    /// - Other parsing errors
    fn parse(&self, args: &[Value]) -> Result<Expr, String>;

    /// Returns the arity (argument count requirements) of this function.
    ///
    /// The parser uses this for early validation before calling `parse()`.
    fn arity(&self) -> Arity;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arity_fixed() {
        let arity = Arity::Fixed(2);
        assert!(!arity.accepts(0));
        assert!(!arity.accepts(1));
        assert!(arity.accepts(2));
        assert!(!arity.accepts(3));
        assert_eq!(arity.describe(), "2 arguments");
    }

    #[test]
    fn test_arity_range() {
        let arity = Arity::Range(1, 3);
        assert!(!arity.accepts(0));
        assert!(arity.accepts(1));
        assert!(arity.accepts(2));
        assert!(arity.accepts(3));
        assert!(!arity.accepts(4));
        assert_eq!(arity.describe(), "1-3 arguments");
    }

    #[test]
    fn test_arity_variadic() {
        let arity = Arity::Variadic;
        assert!(arity.accepts(0));
        assert!(arity.accepts(1));
        assert!(arity.accepts(100));
        assert_eq!(arity.describe(), "any number of arguments");
    }

    #[test]
    fn test_arity_describe_special_cases() {
        assert_eq!(Arity::Fixed(0).describe(), "no arguments");
        assert_eq!(Arity::Fixed(1).describe(), "1 argument");
        assert_eq!(Arity::Range(2, 2).describe(), "2 arguments");
    }
}
