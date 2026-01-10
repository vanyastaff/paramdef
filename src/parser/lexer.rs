//! Lexer for tokenizing expression strings.

use crate::core::SmartStr;
use crate::parser::token::Token;
use std::iter::Peekable;
use std::str::Chars;

/// Lexer for tokenizing expression strings.
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    current_pos: usize,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer from input string.
    #[must_use]
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            current_pos: 0,
        }
    }

    /// Get the next token.
    ///
    /// # Errors
    /// Returns error if invalid syntax encountered.
    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();

        match self.peek() {
            None => Ok(Token::Eof),
            Some(&ch) => match ch {
                '(' => {
                    self.advance();
                    Ok(Token::LParen)
                }
                ')' => {
                    self.advance();
                    Ok(Token::RParen)
                }
                '[' => {
                    self.advance();
                    Ok(Token::LBracket)
                }
                ']' => {
                    self.advance();
                    Ok(Token::RBracket)
                }
                ',' => {
                    self.advance();
                    Ok(Token::Comma)
                }
                '.' => {
                    self.advance();
                    Ok(Token::Dot)
                }
                '"' => self.read_string(),
                '=' => {
                    self.advance();
                    if self.peek() == Some(&'=') {
                        self.advance();
                        Ok(Token::Eq)
                    } else {
                        Err(format!(
                            "Unexpected character '=' at position {}",
                            self.current_pos
                        ))
                    }
                }
                '!' => {
                    self.advance();
                    if self.peek() == Some(&'=') {
                        self.advance();
                        Ok(Token::Ne)
                    } else {
                        Err(format!(
                            "Unexpected character '!' at position {}",
                            self.current_pos
                        ))
                    }
                }
                '<' => {
                    self.advance();
                    if self.peek() == Some(&'=') {
                        self.advance();
                        Ok(Token::Lte)
                    } else {
                        Ok(Token::Lt)
                    }
                }
                '>' => {
                    self.advance();
                    if self.peek() == Some(&'=') {
                        self.advance();
                        Ok(Token::Gte)
                    } else {
                        Ok(Token::Gt)
                    }
                }
                _ if ch.is_ascii_digit() || ch == '-' => self.read_number(),
                _ if ch.is_alphabetic() || ch == '_' => Ok(self.read_identifier()),
                _ => Err(format!(
                    "Unexpected character '{}' at position {}",
                    ch, self.current_pos
                )),
            },
        }
    }

    /// Tokenize entire input into a vector of tokens.
    ///
    /// # Errors
    /// Returns error if invalid syntax encountered.
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token()?;
            if token == Token::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }

        Ok(tokens)
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn advance(&mut self) -> Option<char> {
        self.current_pos += 1;
        self.input.next()
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_string(&mut self) -> Result<Token, String> {
        self.advance(); // consume opening "
        let mut string = String::new();

        loop {
            match self.peek() {
                None => {
                    return Err(format!(
                        "Unterminated string at position {}",
                        self.current_pos
                    ));
                }
                Some(&'"') => {
                    self.advance(); // consume closing "
                    break;
                }
                Some(&'\\') => {
                    self.advance(); // consume \
                    let Some(&ch) = self.peek() else {
                        return Err(format!(
                            "Unterminated string at position {}",
                            self.current_pos
                        ));
                    };

                    self.advance();
                    // Simple escape sequences
                    match ch {
                        'n' => string.push('\n'),
                        't' => string.push('\t'),
                        'r' => string.push('\r'),
                        '"' => string.push('"'),
                        '\\' => string.push('\\'),
                        _ => {
                            string.push('\\');
                            string.push(ch);
                        }
                    }
                }
                Some(&ch) => {
                    string.push(ch);
                    self.advance();
                }
            }
        }

        Ok(Token::String(SmartStr::from(string)))
    }

    fn read_number(&mut self) -> Result<Token, String> {
        let mut number = String::new();

        // Handle negative sign
        if self.peek() == Some(&'-') {
            number.push('-');
            self.advance();
        }

        // Read digits before decimal point
        while let Some(&ch) = self.peek() {
            if ch.is_ascii_digit() {
                number.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // Read decimal part
        if self.peek() == Some(&'.') {
            number.push('.');
            self.advance();

            while let Some(&ch) = self.peek() {
                if ch.is_ascii_digit() {
                    number.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        number.parse::<f64>().map(Token::Number).map_err(|_| {
            format!(
                "Invalid number '{}' at position {}",
                number, self.current_pos
            )
        })
    }

    fn read_identifier(&mut self) -> Token {
        let mut ident = String::new();

        while let Some(&ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // Check for keywords (case-insensitive)
        // Only core logical operators and booleans are keywords
        // Function-like keywords (STARTS_WITH, etc.) are kept as identifiers
        let ident_upper = ident.to_uppercase();
        match ident_upper.as_str() {
            "AND" => Token::And,
            "OR" => Token::Or,
            "NOT" => Token::Not,
            "TRUE" => Token::Boolean(true),
            "FALSE" => Token::Boolean(false),
            _ => Token::Ident(SmartStr::from(ident)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let mut lexer = Lexer::new("age >= 18");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Ident("age".into()),
                Token::Gte,
                Token::Number(18.0),
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_tokenize_logical() {
        let mut lexer = Lexer::new("age >= 18 AND premium == true");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Ident("age".into()),
                Token::Gte,
                Token::Number(18.0),
                Token::And,
                Token::Ident("premium".into()),
                Token::Eq,
                Token::Boolean(true),
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_tokenize_string() {
        let mut lexer = Lexer::new(r#"name == "admin""#);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Ident("name".into()),
                Token::Eq,
                Token::String("admin".into()),
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_tokenize_function_call() {
        let mut lexer = Lexer::new("email()");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Ident("email".into()),
                Token::LParen,
                Token::RParen,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_tokenize_parentheses() {
        let mut lexer = Lexer::new("(age >= 18 OR premium) AND active");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Ident("age".into()),
                Token::Gte,
                Token::Number(18.0),
                Token::Or,
                Token::Ident("premium".into()),
                Token::RParen,
                Token::And,
                Token::Ident("active".into()),
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_tokenize_decimal() {
        let mut lexer = Lexer::new("price >= 19.99");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Ident("price".into()),
                Token::Gte,
                Token::Number(19.99),
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_tokenize_negative() {
        let mut lexer = Lexer::new("temp < -5.5");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Ident("temp".into()),
                Token::Lt,
                Token::Number(-5.5),
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_tokenize_string_escapes() {
        let mut lexer = Lexer::new(r#"msg == "hello\"world\n""#);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Ident("msg".into()),
                Token::Eq,
                Token::String("hello\"world\n".into()),
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_tokenize_case_insensitive_keywords() {
        let mut lexer = Lexer::new("age >= 18 and premium or trial");
        let tokens = lexer.tokenize().unwrap();

        assert!(matches!(tokens[3], Token::And));
        assert!(matches!(tokens[5], Token::Or));
    }

    #[test]
    fn test_error_unterminated_string() {
        let mut lexer = Lexer::new(r#"name == "admin"#);
        let result = lexer.tokenize();

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unterminated string"));
    }

    #[test]
    fn test_error_invalid_operator() {
        let mut lexer = Lexer::new("age = 18");
        let result = lexer.tokenize();

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unexpected character '='"));
    }
}
