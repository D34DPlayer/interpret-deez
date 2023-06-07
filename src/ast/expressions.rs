use super::Node;
use crate::token::Token;

#[derive(Debug)]
pub enum Expression<'a> {
    Identifier(Identifier<'a>),
    Illegal,
}

impl Node for Expression<'_> {
    fn token_literal(&self) -> &Token {
        match self {
            Self::Identifier(x) => x.token_literal(),
            _ => &Token::Illegal('F'),
        }
    }
}

#[derive(Debug)]
pub struct Identifier<'a> {
    pub token: Token<'a>,
    pub value: &'a str,
}

impl Node for Identifier<'_> {
    fn token_literal(&self) -> &Token {
        &self.token
    }
}
