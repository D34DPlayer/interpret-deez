use super::expressions::{Expression, Identifier};
use super::Node;
use crate::token::Token;

#[derive(Debug)]
pub enum Statement<'a> {
    Let(Let<'a>),
    EOF,
}

impl Node for Statement<'_> {
    fn token_literal(&self) -> &Token {
        match self {
            Self::Let(x) => x.token_literal(),
            _ => &Token::Illegal('F'),
        }
    }
}

#[derive(Debug)]
pub struct Let<'a> {
    pub token: Token<'a>,
    pub name: Identifier<'a>,
    pub value: Expression<'a>,
}

impl Node for Let<'_> {
    fn token_literal(&self) -> &Token {
        &self.token
    }
}
