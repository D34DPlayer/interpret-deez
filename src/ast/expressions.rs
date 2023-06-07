use super::Node;
use crate::token::Token;

pub enum Expression<'a> {
    Identifier(Identifier<'a>),
}

impl Node for Expression<'_> {
    fn token_literal(&self) -> &Token {
        match self {
            Self::Identifier(x) => x.token_literal(),
        }
    }
}

pub struct Identifier<'a> {
    token: Token<'a>,
    value: String,
}

impl Node for Identifier<'_> {
    fn token_literal(&self) -> &Token {
        &self.token
    }
}
