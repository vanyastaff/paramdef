//! Built-in function parsers.
//!
//! This module contains parser implementations for all built-in validation
//! and transformation functions. Each parser converts function arguments
//! into the corresponding [`Expr`] variant.

use crate::core::Value;
use crate::expr::Expr;

use super::function_parser::{Arity, FunctionParser};
use super::registry::FunctionRegistry;

// =============================================================================
// Validation Functions (No Arguments)
// =============================================================================

macro_rules! define_no_arg_parser {
    ($name:ident, $func_name:literal, $expr:expr) => {
        struct $name;

        impl FunctionParser for $name {
            fn name(&self) -> &'static str {
                $func_name
            }

            fn parse(&self, _args: &[Value]) -> Result<Expr, String> {
                Ok($expr)
            }

            fn arity(&self) -> Arity {
                Arity::Fixed(0)
            }
        }
    };
}

define_no_arg_parser!(EmailParser, "email", Expr::email());
define_no_arg_parser!(UrlParser, "url", Expr::url());
define_no_arg_parser!(UuidParser, "uuid", Expr::uuid());
define_no_arg_parser!(RequiredParser, "required", Expr::required());
define_no_arg_parser!(EmptyParser, "empty", Expr::is_empty());
define_no_arg_parser!(IsEmptyParser, "is_empty", Expr::is_empty());
define_no_arg_parser!(NotEmptyParser, "not_empty", Expr::is_not_empty());
define_no_arg_parser!(IsNotEmptyParser, "is_not_empty", Expr::is_not_empty());
define_no_arg_parser!(UniqueItemsParser, "unique_items", Expr::unique_items());
define_no_arg_parser!(PositiveParser, "positive", Expr::positive());
define_no_arg_parser!(NegativeParser, "negative", Expr::negative());
define_no_arg_parser!(IntegerParser, "integer", Expr::integer());

// =============================================================================
// String Functions (1 Argument - String)
// =============================================================================

macro_rules! define_string_arg_parser {
    ($name:ident, $func_name:literal, $expr_fn:expr) => {
        struct $name;

        impl FunctionParser for $name {
            fn name(&self) -> &'static str {
                $func_name
            }

            fn parse(&self, args: &[Value]) -> Result<Expr, String> {
                if let Some(s) = args[0].as_text() {
                    Ok($expr_fn(s))
                } else {
                    Err(format!("{}() requires string argument", $func_name))
                }
            }

            fn arity(&self) -> Arity {
                Arity::Fixed(1)
            }
        }
    };
}

define_string_arg_parser!(StartsWithParser, "starts_with", Expr::starts_with);
define_string_arg_parser!(EndsWithParser, "ends_with", Expr::ends_with);
define_string_arg_parser!(ContainsParser, "contains", |s: &str| Expr::contains(
    Value::text(s)
));

#[cfg(feature = "validation")]
define_string_arg_parser!(MatchesParser, "matches", Expr::matches);

// Alias for starts_with (common alternative name)
struct StartsWithAliasParser;
impl FunctionParser for StartsWithAliasParser {
    fn name(&self) -> &'static str {
        "startswith"
    }
    fn parse(&self, args: &[Value]) -> Result<Expr, String> {
        StartsWithParser.parse(args)
    }
    fn arity(&self) -> Arity {
        Arity::Fixed(1)
    }
}

// Alias for ends_with (common alternative name)
struct EndsWithAliasParser;
impl FunctionParser for EndsWithAliasParser {
    fn name(&self) -> &'static str {
        "endswith"
    }
    fn parse(&self, args: &[Value]) -> Result<Expr, String> {
        EndsWithParser.parse(args)
    }
    fn arity(&self) -> Arity {
        Arity::Fixed(1)
    }
}

// =============================================================================
// Length Functions (1 Argument - Number → usize)
// =============================================================================

macro_rules! define_length_parser {
    ($name:ident, $func_name:literal, $expr_fn:expr) => {
        struct $name;

        impl FunctionParser for $name {
            fn name(&self) -> &'static str {
                $func_name
            }

            fn parse(&self, args: &[Value]) -> Result<Expr, String> {
                let n = match &args[0] {
                    Value::Int(i) => *i as f64,
                    Value::Float(f) => *f,
                    _ => return Err(format!("{}() requires number argument", $func_name)),
                };
                #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
                Ok($expr_fn(n as usize))
            }

            fn arity(&self) -> Arity {
                Arity::Fixed(1)
            }
        }
    };
}

define_length_parser!(MinLengthParser, "min_length", Expr::min_length);
define_length_parser!(MaxLengthParser, "max_length", Expr::max_length);
define_length_parser!(LengthParser, "length", Expr::length);

// =============================================================================
// Numeric Functions (1 Argument - Number → f64)
// =============================================================================

macro_rules! define_numeric_parser {
    ($name:ident, $func_name:literal, $expr_fn:expr) => {
        struct $name;

        impl FunctionParser for $name {
            fn name(&self) -> &'static str {
                $func_name
            }

            fn parse(&self, args: &[Value]) -> Result<Expr, String> {
                let n = match &args[0] {
                    Value::Int(i) => *i as f64,
                    Value::Float(f) => *f,
                    _ => return Err(format!("{}() requires number argument", $func_name)),
                };
                Ok($expr_fn(n))
            }

            fn arity(&self) -> Arity {
                Arity::Fixed(1)
            }
        }
    };
}

define_numeric_parser!(MinParser, "min", Expr::min);
define_numeric_parser!(MaxParser, "max", Expr::max);

// =============================================================================
// Registration Function
// =============================================================================

/// Registers all built-in function parsers.
///
/// This function is called by [`FunctionRegistry::with_builtins()`].
pub(super) fn register_all(registry: &mut FunctionRegistry) {
    // Validation functions (no args)
    registry.register(EmailParser);
    registry.register(UrlParser);
    registry.register(UuidParser);
    registry.register(RequiredParser);
    registry.register(EmptyParser);
    registry.register(IsEmptyParser);
    registry.register(NotEmptyParser);
    registry.register(IsNotEmptyParser);
    registry.register(UniqueItemsParser);
    registry.register(PositiveParser);
    registry.register(NegativeParser);
    registry.register(IntegerParser);

    // String functions (1 string arg)
    registry.register(StartsWithParser);
    registry.register(StartsWithAliasParser);
    registry.register(EndsWithParser);
    registry.register(EndsWithAliasParser);
    registry.register(ContainsParser);

    #[cfg(feature = "validation")]
    registry.register(MatchesParser);

    // Length functions (1 numeric arg)
    registry.register(MinLengthParser);
    registry.register(MaxLengthParser);
    registry.register(LengthParser);

    // Numeric functions (1 numeric arg)
    registry.register(MinParser);
    registry.register(MaxParser);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_parser() {
        let parser = EmailParser;
        assert_eq!(parser.name(), "email");
        assert_eq!(parser.arity(), Arity::Fixed(0));

        let result = parser.parse(&[]);
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), Expr::Email));
    }

    #[test]
    fn test_min_length_parser() {
        let parser = MinLengthParser;
        assert_eq!(parser.name(), "min_length");
        assert_eq!(parser.arity(), Arity::Fixed(1));

        let result = parser.parse(&[Value::Float(5.0)]);
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), Expr::MinLength(5)));
    }

    #[test]
    fn test_min_length_parser_int() {
        let parser = MinLengthParser;

        let result = parser.parse(&[Value::Int(10)]);
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), Expr::MinLength(10)));
    }

    #[test]
    fn test_min_length_parser_invalid_arg() {
        let parser = MinLengthParser;

        let result = parser.parse(&[Value::text("not a number")]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("requires number"));
    }

    #[test]
    fn test_starts_with_parser() {
        let parser = StartsWithParser;
        assert_eq!(parser.name(), "starts_with");

        let result = parser.parse(&[Value::text("admin")]);
        assert!(result.is_ok());
        if let Expr::StartsWith(ref s) = result.unwrap() {
            assert_eq!(s.as_ref() as &str, "admin");
        } else {
            panic!("Expected StartsWith variant");
        }
    }

    #[test]
    fn test_starts_with_alias() {
        let parser = StartsWithAliasParser;
        assert_eq!(parser.name(), "startswith");

        let result = parser.parse(&[Value::text("test")]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_register_all() {
        let mut registry = FunctionRegistry::new();
        register_all(&mut registry);

        // Should have all built-in functions
        assert!(registry.contains("email"));
        assert!(registry.contains("url"));
        assert!(registry.contains("min_length"));
        assert!(registry.contains("starts_with"));
        assert!(registry.contains("startswith")); // alias
        assert!(registry.contains("positive"));
    }
}
