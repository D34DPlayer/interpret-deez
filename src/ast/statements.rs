use super::expressions::{Expression, Identifier};
use super::Node;
use crate::token::Token;

#[derive(Debug)]
pub enum Statement<'a> {
    Let(Let<'a>),
    Return(Return<'a>),
    Expression(ExpressionStmt<'a>),
    EOF,
}

impl Node for Statement<'_> {
    fn token_literal(&self) -> &Token {
        match self {
            Self::Let(x) => x.token_literal(),
            Self::Return(x) => x.token_literal(),
            Self::Expression(x) => x.token_literal(),
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

#[derive(Debug)]
pub struct Return<'a> {
    pub token: Token<'a>,
    pub return_value: Expression<'a>,
}

impl Node for Return<'_> {
    fn token_literal(&self) -> &Token {
        &self.token
    }
}

#[derive(Debug)]
pub struct ExpressionStmt<'a> {
    pub token: Token<'a>,
    pub expression: Expression<'a>,
}

impl Node for ExpressionStmt<'_> {
    fn token_literal(&self) -> &Token {
        &self.token
    }
}
