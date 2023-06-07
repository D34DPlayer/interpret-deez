use super::expressions::{Expression, Identifier};
use super::Node;
use crate::token::Token;

pub enum Statement<'a> {
    Let(Let<'a>),
}

impl Node for Statement<'_> {
    fn token_literal(&self) -> &Token {
        match self {
            Self::Let(x) => x.token_literal(),
        }
    }
}

pub struct Let<'a> {
    token: Token<'a>,
    name: Identifier<'a>,
    value: Expression<'a>,
}

impl Node for Let<'_> {
    fn token_literal(&self) -> &Token {
        &self.token
    }
}
