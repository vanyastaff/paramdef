use thiserror::Error;
use crate::parser::token::Token;

#[derive(Debug, Error, PartialEq)]
pub enum ParseError {
    #[error("Unexpected token: expected {expected:?}, found {found:?}")]
    UnexpectedToken {
        expected: String,
        found: String,
    },

    #[error("Invalid character: {0}")]
    InvalidCharacter(char),

    #[error("Invalid number format: {0}")]
    InvalidNumber(String),

    #[error("Unterminated string")]
    UnterminatedString,

    #[error("Unknown function: {0}")]
    UnknownFunction(String),

    #[error("Invalid arguments for function {0}: {1}")]
    InvalidArguments(String, String),

    #[error("Expression error: {0}")]
    Custom(String),
}

pub type ParseResult<T> = Result<T, ParseError>;
