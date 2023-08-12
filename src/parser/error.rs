use crate::lexer::token::Token;

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("Input stopped unexpectedly")]
    EOFError,
    #[error("Expected {expected}, received {received}")]
    UnexpectedTokenError { expected: Token, received: Token },
    #[error("Expression expected")]
    PrefixTokenError(Token),
    #[error("Error parsing integer")]
    ParseIntError,
    #[error("unknown parser error")]
    Unknown,
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
