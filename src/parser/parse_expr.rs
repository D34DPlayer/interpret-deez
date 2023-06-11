use super::{expr, Token};

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

pub trait ParseExpr<'a>
where
    Self: Sized,
{
    fn prefix_parse_expr(&self) -> expr::Expression;
    fn infix_parse_expr(&self, ex: expr::Expression) -> expr::Expression;
}

impl<'a> ParseExpr<'a> for Precedence {
    fn prefix_parse_expr(&self) -> expr::Expression {
        match self {
            _ => expr::Expression::Illegal,
        }
    }
    fn infix_parse_expr(&self, _: expr::Expression) -> expr::Expression {
        match self {
            _ => expr::Expression::Illegal,
        }
    }
}
