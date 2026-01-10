//! Recursive descent parser for expressions.

use crate::core::Value;
use crate::expr::{Expr, Rule};
use crate::parser::token::Token;
use std::sync::Arc;

/// Parser for expression strings.
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    /// Create a new parser from tokens.
    #[must_use]
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Parse a Rule from tokens.
    ///
    /// # Errors
    /// Returns error if invalid syntax encountered.
    pub fn parse_rule(&mut self) -> Result<Rule, String> {
        // Default to Local target for now
        // Future: support explicit target syntax like "field:mode.eq('advanced')"
        let expr = self.parse_expr()?;
        Ok(Rule::local(expr))
    }

    /// Parse an expression (top level).
    ///
    /// # Errors
    /// Returns error if invalid syntax encountered.
    pub fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_or_expr()
    }

    // Grammar: or_expr := and_expr ( OR and_expr )*
    fn parse_or_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and_expr()?;

        while self.current() == &Token::Or {
            self.advance(); // consume OR
            let right = self.parse_and_expr()?;
            left = Expr::or(vec![left, right]);
        }

        Ok(left)
    }

    // Grammar: and_expr := not_expr ( AND not_expr )*
    fn parse_and_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_not_expr()?;

        while self.current() == &Token::And {
            self.advance(); // consume AND
            let right = self.parse_not_expr()?;
            left = Expr::and(vec![left, right]);
        }

        Ok(left)
    }

    // Grammar: not_expr := NOT primary_expr | primary_expr
    fn parse_not_expr(&mut self) -> Result<Expr, String> {
        if self.current() == &Token::Not {
            self.advance(); // consume NOT
            let expr = self.parse_primary_expr()?;
            Ok(Expr::not(expr))
        } else {
            self.parse_primary_expr()
        }
    }

    // Grammar: primary_expr := comparison | function_call | ( expr )
    fn parse_primary_expr(&mut self) -> Result<Expr, String> {
        match self.current() {
            Token::LParen => {
                self.advance(); // consume (
                let expr = self.parse_expr()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            Token::Ident(_) => {
                // Could be function call or field comparison
                if self.peek() == Some(&Token::LParen) {
                    self.parse_function_call()
                } else {
                    self.parse_comparison()
                }
            }
            _ => Err(format!("Unexpected token: {:?}", self.current())),
        }
    }

    // Grammar: function_call := identifier ( args? )
    fn parse_function_call(&mut self) -> Result<Expr, String> {
        let func_name = if let Token::Ident(name) = self.current() {
            name.clone()
        } else {
            return Err(format!("Expected identifier, got {:?}", self.current()));
        };
        self.advance();

        self.expect(Token::LParen)?;

        // Parse arguments (if any)
        let args = if self.current() != &Token::RParen {
            self.parse_args()?
        } else {
            Vec::new()
        };

        self.expect(Token::RParen)?;

        // Map function names to Expr variants
        self.map_function_to_expr(&func_name, args)
    }

    fn parse_args(&mut self) -> Result<Vec<Value>, String> {
        let mut args = Vec::new();

        loop {
            args.push(self.parse_value()?);

            if self.current() == &Token::Comma {
                self.advance(); // consume ,
            } else {
                break;
            }
        }

        Ok(args)
    }

    // Grammar: comparison := field op value
    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let _field = if let Token::Ident(name) = self.current() {
            name.clone()
        } else {
            return Err(format!("Expected field name, got {:?}", self.current()));
        };
        self.advance();

        let op = self.current().clone();

        if !op.is_binary_op() {
            return Err(format!("Expected binary operator, got {:?}", op));
        }
        self.advance();

        let value = self.parse_value()?;

        // Map operator to Expr variant
        self.map_comparison_to_expr(&op, value)
    }

    fn parse_value(&mut self) -> Result<Value, String> {
        match self.current() {
            Token::String(s) => {
                let value = Value::text(s.as_str());
                self.advance();
                Ok(value)
            }
            Token::Number(n) => {
                let value = Value::Float(*n);
                self.advance();
                Ok(value)
            }
            Token::Boolean(b) => {
                let value = Value::Bool(*b);
                self.advance();
                Ok(value)
            }
            Token::LBracket => self.parse_array(),
            _ => Err(format!("Expected value, got {:?}", self.current())),
        }
    }

    fn parse_array(&mut self) -> Result<Value, String> {
        self.expect(Token::LBracket)?;

        let mut values = Vec::new();

        if self.current() != &Token::RBracket {
            loop {
                values.push(self.parse_value()?);

                if self.current() == &Token::Comma {
                    self.advance(); // consume ,
                } else {
                    break;
                }
            }
        }

        self.expect(Token::RBracket)?;

        Ok(Value::Array(Arc::from(values)))
    }

    #[allow(clippy::too_many_lines)]
    fn map_function_to_expr(&self, func_name: &str, args: Vec<Value>) -> Result<Expr, String> {
        let func_upper = func_name.to_uppercase();

        match func_upper.as_str() {
            // Validation functions (no args)
            "EMAIL" => {
                if !args.is_empty() {
                    return Err(format!("email() takes no arguments, got {}", args.len()));
                }
                Ok(Expr::email())
            }
            "URL" => {
                if !args.is_empty() {
                    return Err(format!("url() takes no arguments, got {}", args.len()));
                }
                Ok(Expr::url())
            }
            "UUID" => {
                if !args.is_empty() {
                    return Err(format!("uuid() takes no arguments, got {}", args.len()));
                }
                Ok(Expr::uuid())
            }
            "REQUIRED" => {
                if !args.is_empty() {
                    return Err(format!("required() takes no arguments, got {}", args.len()));
                }
                Ok(Expr::required())
            }
            "EMPTY" | "IS_EMPTY" => {
                if !args.is_empty() {
                    return Err(format!("empty() takes no arguments, got {}", args.len()));
                }
                Ok(Expr::is_empty())
            }
            "NOT_EMPTY" | "IS_NOT_EMPTY" => {
                if !args.is_empty() {
                    return Err(format!(
                        "not_empty() takes no arguments, got {}",
                        args.len()
                    ));
                }
                Ok(Expr::is_not_empty())
            }
            "UNIQUE_ITEMS" => {
                if !args.is_empty() {
                    return Err(format!(
                        "unique_items() takes no arguments, got {}",
                        args.len()
                    ));
                }
                Ok(Expr::unique_items())
            }

            // String functions (1 arg)
            "STARTS_WITH" | "STARTSWITH" => {
                if args.len() != 1 {
                    return Err(format!(
                        "starts_with() takes 1 argument, got {}",
                        args.len()
                    ));
                }
                if let Some(s) = args[0].as_text() {
                    Ok(Expr::starts_with(s))
                } else {
                    Err("starts_with() requires string argument".to_string())
                }
            }
            "ENDS_WITH" | "ENDSWITH" => {
                if args.len() != 1 {
                    return Err(format!("ends_with() takes 1 argument, got {}", args.len()));
                }
                if let Some(s) = args[0].as_text() {
                    Ok(Expr::ends_with(s))
                } else {
                    Err("ends_with() requires string argument".to_string())
                }
            }

            // Length functions (1 arg)
            "MIN_LENGTH" => {
                if args.len() != 1 {
                    return Err(format!("min_length() takes 1 argument, got {}", args.len()));
                }
                if let Some(n) = args[0].as_float() {
                    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
                    Ok(Expr::min_length(n as usize))
                } else {
                    Err("min_length() requires number argument".to_string())
                }
            }
            "MAX_LENGTH" => {
                if args.len() != 1 {
                    return Err(format!("max_length() takes 1 argument, got {}", args.len()));
                }
                if let Some(n) = args[0].as_float() {
                    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
                    Ok(Expr::max_length(n as usize))
                } else {
                    Err("max_length() requires number argument".to_string())
                }
            }
            "LENGTH" => {
                if args.len() != 1 {
                    return Err(format!("length() takes 1 argument, got {}", args.len()));
                }
                if let Some(n) = args[0].as_float() {
                    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
                    Ok(Expr::length(n as usize))
                } else {
                    Err("length() requires number argument".to_string())
                }
            }

            // Numeric functions (1 arg)
            "MIN" => {
                if args.len() != 1 {
                    return Err(format!("min() takes 1 argument, got {}", args.len()));
                }
                if let Some(n) = args[0].as_float() {
                    Ok(Expr::min(n))
                } else {
                    Err("min() requires number argument".to_string())
                }
            }
            "MAX" => {
                if args.len() != 1 {
                    return Err(format!("max() takes 1 argument, got {}", args.len()));
                }
                if let Some(n) = args[0].as_float() {
                    Ok(Expr::max(n))
                } else {
                    Err("max() requires number argument".to_string())
                }
            }

            // Numeric functions (no args)
            "POSITIVE" => {
                if !args.is_empty() {
                    return Err(format!("positive() takes no arguments, got {}", args.len()));
                }
                Ok(Expr::positive())
            }
            "NEGATIVE" => {
                if !args.is_empty() {
                    return Err(format!("negative() takes no arguments, got {}", args.len()));
                }
                Ok(Expr::negative())
            }
            "INTEGER" => {
                if !args.is_empty() {
                    return Err(format!("integer() takes no arguments, got {}", args.len()));
                }
                Ok(Expr::integer())
            }

            _ => Err(format!("Unknown function: {}", func_name)),
        }
    }

    fn map_comparison_to_expr(&self, op: &Token, value: Value) -> Result<Expr, String> {
        match op {
            Token::Eq => Ok(Expr::eq(value)),
            Token::Ne => Ok(Expr::ne(value)),
            Token::Lt => {
                if let Some(n) = value.as_float() {
                    Ok(Expr::lt(n))
                } else {
                    Err("Lt operator requires numeric value".to_string())
                }
            }
            Token::Lte => {
                if let Some(n) = value.as_float() {
                    Ok(Expr::lte(n))
                } else {
                    Err("Lte operator requires numeric value".to_string())
                }
            }
            Token::Gt => {
                if let Some(n) = value.as_float() {
                    Ok(Expr::gt(n))
                } else {
                    Err("Gt operator requires numeric value".to_string())
                }
            }
            Token::Gte => {
                if let Some(n) = value.as_float() {
                    Ok(Expr::gte(n))
                } else {
                    Err("Gte operator requires numeric value".to_string())
                }
            }
            Token::Contains => Ok(Expr::contains(value)),
            Token::StartsWith => {
                if let Some(s) = value.as_text() {
                    Ok(Expr::starts_with(s))
                } else {
                    Err("STARTS_WITH requires string value".to_string())
                }
            }
            Token::EndsWith => {
                if let Some(s) = value.as_text() {
                    Ok(Expr::ends_with(s))
                } else {
                    Err("ENDS_WITH requires string value".to_string())
                }
            }
            #[cfg(feature = "validation")]
            Token::Matches => {
                if let Some(s) = value.as_text() {
                    Ok(Expr::matches(s))
                } else {
                    Err("MATCHES requires string value".to_string())
                }
            }
            #[cfg(not(feature = "validation"))]
            Token::Matches => Err("MATCHES requires 'validation' feature".to_string()),
            _ => Err(format!("Unsupported operator: {:?}", op)),
        }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos + 1)
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if self.current() == &expected {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", expected, self.current()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::lexer::Lexer;

    fn parse(input: &str) -> Result<Expr, String> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        parser.parse_expr()
    }

    #[test]
    fn test_parse_simple_comparison() {
        let expr = parse("age >= 18").unwrap();
        assert!(matches!(expr, Expr::Gte(_)));
    }

    #[test]
    fn test_parse_logical_and() {
        let expr = parse("age >= 18 AND premium == true").unwrap();
        assert!(matches!(expr, Expr::And(_)));
    }

    #[test]
    fn test_parse_logical_or() {
        let expr = parse("admin == true OR moderator == true").unwrap();
        assert!(matches!(expr, Expr::Or(_)));
    }

    #[test]
    fn test_parse_not() {
        let expr = parse("NOT empty()").unwrap();
        assert!(matches!(expr, Expr::Not(_)));
    }

    #[test]
    fn test_parse_parentheses() {
        let expr = parse("(age >= 18 OR premium == true) AND active == true").unwrap();
        assert!(matches!(expr, Expr::And(_)));
    }

    #[test]
    fn test_parse_function_email() {
        let expr = parse("email()").unwrap();
        assert!(matches!(expr, Expr::Email));
    }

    #[test]
    fn test_parse_function_min_length() {
        let expr = parse("min_length(5)").unwrap();
        assert!(matches!(expr, Expr::MinLength(5)));
    }

    #[test]
    fn test_parse_function_starts_with() {
        let expr = parse(r#"starts_with("admin")"#).unwrap();
        assert!(matches!(expr, Expr::StartsWith(_)));
    }

    #[test]
    fn test_parse_complex() {
        let expr = parse("email() AND min_length(5) AND max_length(100)").unwrap();

        // Parser creates nested binary And expressions
        assert!(matches!(expr, Expr::And(_)));
    }

    #[test]
    fn test_parse_string_value() {
        let expr = parse(r#"name == "admin""#).unwrap();

        if let Expr::Eq(value) = expr {
            assert_eq!(value.as_text(), Some("admin"));
        } else {
            panic!("Expected Eq expression");
        }
    }

    #[test]
    fn test_parse_boolean_value() {
        let expr = parse("active == true").unwrap();

        if let Expr::Eq(value) = expr {
            assert_eq!(value.as_bool(), Some(true));
        } else {
            panic!("Expected Eq expression");
        }
    }

    #[test]
    fn test_error_missing_paren() {
        let result = parse("(age >= 18");
        assert!(result.is_err());
    }

    #[test]
    fn test_error_invalid_operator() {
        let result = parse("age 18");
        assert!(result.is_err());
    }
}
