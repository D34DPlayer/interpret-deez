use anyhow::Result;

use crate::token::Token;

pub mod expressions;
pub mod statements;

pub struct Program<'a> {
    pub statements: Vec<Result<statements::Statement<'a>>>,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

impl<'a> From<Token<'a>> for Precedence {
    fn from(token: Token) -> Self {
        match token {
            Token::Equal | Token::NotEqual => Precedence::Equals,
            Token::LessThan | Token::GreaterThan => Precedence::LessGreater,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Asterisk | Token::ForwardSlash => Precedence::Product,
            Token::LParen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }
}
