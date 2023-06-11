use super::expressions::{Expression, Identifier};
use crate::token::Token;

#[derive(Debug)]
pub enum Statement<'a> {
    Let(Let<'a>),
    Return(Return<'a>),
    Expression(ExpressionStmt<'a>),
    EOF,
}

#[derive(Debug)]
pub struct Let<'a> {
    pub token: Token<'a>,
    pub name: Identifier<'a>,
    pub value: Expression<'a>,
}

#[derive(Debug)]
pub struct Return<'a> {
    pub token: Token<'a>,
    pub return_value: Expression<'a>,
}

#[derive(Debug)]
pub struct ExpressionStmt<'a> {
    pub token: Token<'a>,
    pub expression: Expression<'a>,
}
