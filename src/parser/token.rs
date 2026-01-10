//! Token types for expression parser lexer.

use crate::core::SmartStr;

/// Token produced by the lexer.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    /// String literal: "hello"
    String(SmartStr),
    /// Number literal: 42, 3.14
    Number(f64),
    /// Boolean literal: true, false
    Boolean(bool),

    // Identifiers
    /// Identifier: `field_name`, `email`, etc.
    Ident(SmartStr),

    // Operators
    /// ==
    Eq,
    /// !=
    Ne,
    /// <
    Lt,
    /// <=
    Lte,
    /// >
    Gt,
    /// >=
    Gte,

    // Keywords
    /// `AND`
    And,
    /// `OR`
    Or,
    /// `NOT`
    Not,
    /// `BETWEEN`
    Between,
    /// `IN`
    In,
    /// `CONTAINS`
    Contains,
    /// `STARTS_WITH`
    StartsWith,
    /// `ENDS_WITH`
    EndsWith,
    /// `MATCHES`
    Matches,

    // Delimiters
    /// (
    LParen,
    /// )
    RParen,
    /// [
    LBracket,
    /// ]
    RBracket,
    /// ,
    Comma,
    /// .
    Dot,

    /// End of input
    Eof,
}

impl Token {
    /// Check if token is a binary operator.
    #[must_use]
    pub const fn is_binary_op(&self) -> bool {
        matches!(
            self,
            Self::Eq
                | Self::Ne
                | Self::Lt
                | Self::Lte
                | Self::Gt
                | Self::Gte
                | Self::Contains
                | Self::StartsWith
                | Self::EndsWith
                | Self::Matches
                | Self::Between
                | Self::In
        )
    }

    /// Check if token is a logical operator.
    #[must_use]
    pub const fn is_logical_op(&self) -> bool {
        matches!(self, Self::And | Self::Or)
    }

    /// Check if token is a keyword.
    #[must_use]
    pub const fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::And
                | Self::Or
                | Self::Not
                | Self::Between
                | Self::In
                | Self::Contains
                | Self::StartsWith
                | Self::EndsWith
                | Self::Matches
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_binary_op() {
        assert!(Token::Eq.is_binary_op());
        assert!(Token::Gte.is_binary_op());
        assert!(Token::Contains.is_binary_op());
        assert!(!Token::And.is_binary_op());
        assert!(!Token::Ident("foo".into()).is_binary_op());
    }

    #[test]
    fn test_is_logical_op() {
        assert!(Token::And.is_logical_op());
        assert!(Token::Or.is_logical_op());
        assert!(!Token::Not.is_logical_op());
        assert!(!Token::Eq.is_logical_op());
    }

    #[test]
    fn test_is_keyword() {
        assert!(Token::And.is_keyword());
        assert!(Token::Between.is_keyword());
        assert!(!Token::Eq.is_keyword());
        assert!(!Token::LParen.is_keyword());
    }
}
