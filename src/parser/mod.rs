//! Expression parser for string-based rule definitions.
//!
//! Provides a lexer and recursive descent parser for parsing expression strings
//! into `Expr` and `Rule` types.
//!
//! # Examples
//!
//! ```no_run
//! use paramdef::parser::parse;
//!
//! // Simple comparison
//! let expr = parse("age >= 18").unwrap();
//!
//! // Logical operators
//! let expr = parse("age >= 18 AND premium == true").unwrap();
//!
//! // Function calls
//! let expr = parse("email() AND min_length(5)").unwrap();
//!
//! // Parentheses
//! let expr = parse("(age >= 18 OR guardian) AND active").unwrap();
//! ```

mod builtins;
mod function_parser;
mod lexer;
#[allow(clippy::module_inception)]
mod parser;
mod registry;
mod token;

pub use function_parser::{Arity, FunctionParser};
pub use lexer::Lexer;
pub use parser::Parser;
pub use registry::FunctionRegistry;
pub use token::Token;

use crate::expr::{Expr, Rule};

/// Parse an expression from a string.
///
/// # Examples
///
/// ```
/// use paramdef::parser::parse;
///
/// let expr = parse("age >= 18").unwrap();
/// let expr = parse("email() AND min_length(5)").unwrap();
/// ```
///
/// # Errors
/// Returns error if the string contains invalid syntax.
pub fn parse(input: &str) -> Result<Expr, String> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    parser.parse_expr()
}

/// Parse a rule from a string.
///
/// # Examples
///
/// ```
/// use paramdef::parser::parse_rule;
///
/// let rule = parse_rule("email() AND min_length(5)").unwrap();
/// ```
///
/// # Errors
/// Returns error if the string contains invalid syntax.
pub fn parse_rule(input: &str) -> Result<Rule, String> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    parser.parse_rule()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let expr = parse("age >= 18").unwrap();
        assert!(matches!(expr, Expr::Gte(_)));
    }

    #[test]
    fn test_parse_rule() {
        let rule = parse_rule("email()").unwrap();
        assert!(matches!(rule.expr, Expr::Email));
    }
}
