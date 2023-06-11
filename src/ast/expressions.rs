use crate::token::Token;

#[derive(Debug)]
pub enum Expression<'a> {
    Identifier(Identifier<'a>),
    Integer(Integer<'a>),
    Prefix(Prefix<'a>),
    Illegal,
}

#[derive(Debug)]
pub struct Identifier<'a> {
    pub token: Token<'a>,
    pub value: &'a str,
}

#[derive(Debug)]
pub struct Integer<'a> {
    pub token: Token<'a>,
    pub value: i64,
}

#[derive(Debug, PartialEq)]
pub enum PrefixOp {
    Bang,
    Minus,
}

#[derive(Debug)]
pub struct Prefix<'a> {
    pub token: Token<'a>,
    pub operator: PrefixOp,
    pub right: Box<Expression<'a>>,
}
