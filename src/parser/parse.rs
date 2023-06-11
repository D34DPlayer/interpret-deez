use super::expr::Precedence;
use super::{expr, stmt, Parser, Token};
use anyhow::{anyhow, bail, Result};

pub trait Parse<'a>
where
    Self: Sized,
{
    fn parse(parser: &mut Parser<'a>, precedence: &Precedence) -> Result<Self>;
}

impl<'a> Parse<'a> for stmt::Statement<'a> {
    fn parse(parser: &mut Parser<'a>, precedence: &Precedence) -> Result<Self> {
        loop {
            return match parser.tokens[0] {
                None => Ok(Self::EOF),
                Some(Token::Semicolon) => {
                    parser.read_token();
                    continue;
                }
                Some(Token::Let) => stmt::Let::parse(parser, precedence).map(|x| Self::Let(x)),
                Some(Token::Return) => {
                    stmt::Return::parse(parser, precedence).map(|x| Self::Return(x))
                }
                Some(_) => {
                    stmt::ExpressionStmt::parse(parser, precedence).map(|x| Self::Expression(x))
                }
            };
        }
    }
}

impl<'a> Parse<'a> for expr::Identifier<'a> {
    fn parse(parser: &mut Parser<'a>, _: &Precedence) -> Result<Self> {
        match parser.tokens[0] {
            Some(Token::Ident(value)) => Ok(expr::Identifier { value }),
            _ => {
                bail!("Identifier expected")
            }
        }
    }
}

impl<'a> Parse<'a> for stmt::Let<'a> {
    fn parse(parser: &mut Parser<'a>, precedence: &Precedence) -> Result<Self> {
        parser.tokens[0].ok_or(anyhow!("Token expected"))?;
        parser.read_token();

        let name = expr::Identifier::parse(parser, precedence)?;
        parser.read_token();

        if let Some(Token::Assign) = parser.tokens[0] {
            parser.read_token();
            let expression = expr::Expression::parse(parser, precedence)?;
            parser.read_token();

            Ok(Self {
                name,
                value: expression,
            })
        } else {
            Err(anyhow!("Expected assignment in let statement"))
        }
    }
}

impl<'a> Parse<'a> for stmt::Return<'a> {
    fn parse(parser: &mut Parser<'a>, precedence: &Precedence) -> Result<Self> {
        parser.tokens[0].ok_or(anyhow!("Token expected"))?;

        let expression = expr::Expression::parse(parser, precedence)?;
        parser.read_token();

        Ok(Self {
            return_value: expression,
        })
    }
}

impl<'a> Parse<'a> for stmt::ExpressionStmt<'a> {
    fn parse(parser: &mut Parser<'a>, precedence: &Precedence) -> Result<Self> {
        parser.tokens[0].ok_or(anyhow!("Token expected"))?;

        let expression = expr::Expression::parse(parser, precedence)?;
        parser.read_token();

        Ok(Self { expression })
    }
}

impl<'a> Parse<'a> for expr::Expression<'a> {
    fn parse(parser: &mut Parser<'a>, precedence: &Precedence) -> Result<Self> {
        let first_expr = match parser.tokens[0] {
            Some(Token::Ident(_)) => {
                expr::Identifier::parse(parser, precedence).map(|i| Self::Identifier(i))
            }
            Some(Token::Int(_)) => {
                expr::Integer::parse(parser, precedence).map(|i| Self::Integer(i))
            }
            Some(Token::Bang) | Some(Token::Minus) => {
                expr::Prefix::parse(parser, precedence).map(|p| Self::Prefix(p))
            }
            _ => Err(anyhow!("Expression expected")),
        }?;

        let mut left = Box::new(first_expr);

        loop {
            match parser.tokens[1] {
                Some(Token::Semicolon) => break,
                Some(t) => {
                    let new_precedence: Precedence = t.into();
                    if new_precedence <= *precedence {
                        break;
                    }
                    parser.read_token();

                    let mut infix = expr::Infix::parse(parser, precedence)?;
                    infix.left = left;

                    left = Box::new(Self::Infix(infix));
                }
                None => break,
            }
        }
        Ok(*left)
    }
}

impl<'a> Parse<'a> for expr::Integer {
    fn parse(parser: &mut Parser<'a>, _: &Precedence) -> Result<Self> {
        match parser.tokens[0] {
            Some(Token::Int(value)) => Ok(Self {
                value: value.parse::<i64>()?,
            }),
            _ => bail!("Integer expected"),
        }
    }
}

impl<'a> Parse<'a> for expr::Prefix<'a> {
    fn parse(parser: &mut Parser<'a>, _: &Precedence) -> Result<Self> {
        match parser.tokens[0] {
            Some(Token::Bang) | Some(Token::Minus) => {
                let token = parser.tokens[0].unwrap();
                parser.read_token();

                let operator = match token {
                    Token::Bang => expr::PrefixOp::Bang,
                    Token::Minus => expr::PrefixOp::Minus,
                    _ => bail!("Prefix operator expected"),
                };

                let right = expr::Expression::parse(parser, &Precedence::Prefix)?;

                Ok(Self {
                    operator,
                    right: Box::new(right),
                })
            }
            _ => bail!("Prefix operator expected"),
        }
    }
}

impl<'a> Parse<'a> for expr::Infix<'a> {
    fn parse(parser: &mut Parser<'a>, _: &Precedence) -> Result<Self> {
        let token = parser.tokens[0].ok_or(anyhow!("Token expected"))?;
        let precedence: Precedence = token.into();
        let operator = match token {
            Token::Plus => expr::InfixOp::Plus,
            Token::Minus => expr::InfixOp::Minus,
            Token::Asterisk => expr::InfixOp::Asterisk,
            Token::ForwardSlash => expr::InfixOp::ForwardSlash,
            Token::Equal => expr::InfixOp::Equal,
            Token::NotEqual => expr::InfixOp::NotEqual,
            Token::LessThan => expr::InfixOp::LessThan,
            Token::GreaterThan => expr::InfixOp::GreaterThan,
            _ => bail!("Infix operator expected"),
        };
        parser.read_token();

        let right = expr::Expression::parse(parser, &precedence)?;
        return Ok(Self {
            operator,
            left: Box::new(expr::Expression::Illegal),
            right: Box::new(right),
        });
    }
}
