use super::Parse;
use super::{expr, stmt, Parser, Precedence, Token};
use anyhow::{anyhow, bail, Result};

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

impl<'a> Parse<'a> for expr::Expression<'a> {
    fn parse(parser: &mut Parser<'a>, precedence: &Precedence) -> Result<Self> {
        let first_expr = match parser.tokens[0] {
            Some(Token::Ident(_)) => {
                expr::Identifier::parse(parser, precedence).map(Self::Identifier)
            }
            Some(Token::Int(_)) => expr::Integer::parse(parser, precedence).map(Self::Integer),
            Some(Token::True) | Some(Token::False) => {
                expr::Boolean::parse(parser, precedence).map(Self::Boolean)
            }
            Some(Token::Bang) | Some(Token::Minus) => {
                expr::Prefix::parse(parser, precedence).map(Self::Prefix)
            }
            Some(Token::LParen) => {
                parser.read_token();
                let expr = expr::Expression::parse(parser, &Precedence::Lowest)?;
                if parser.tokens[1] != Some(Token::RParen) {
                    bail!("')' expected");
                }
                parser.read_token();
                Ok(expr)
            }
            Some(Token::If) => expr::If::parse(parser, precedence).map(Self::If),
            Some(Token::Function) => expr::Function::parse(parser, precedence).map(Self::Function),
            _ => {
                // This is a hack to avoid an infinite loop
                let token = parser.tokens[0].unwrap();
                parser.read_token();
                Err(anyhow!("Expression expected at {:?}", token))
            }
        }?;

        let mut left = first_expr;

        loop {
            match parser.tokens[1] {
                Some(Token::Semicolon) | None => break,
                Some(Token::LParen) => {
                    parser.read_token();
                    let mut call = expr::Call::parse(parser, &Precedence::Lowest)?;
                    *call.function = left;

                    left = Self::Call(call);
                }
                Some(t) => {
                    let new_precedence: Precedence = t.into();
                    if new_precedence <= *precedence {
                        break;
                    }
                    parser.read_token();

                    let mut infix = expr::Infix::parse(parser, &new_precedence)?;
                    *infix.left = left;

                    left = Self::Infix(infix);
                }
            }
        }
        Ok(left)
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
    fn parse(parser: &mut Parser<'a>, precedence: &Precedence) -> Result<Self> {
        let token = parser.tokens[0].ok_or(anyhow!("Token expected"))?;
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

        let right = expr::Expression::parse(parser, precedence)?;
        Ok(Self {
            operator,
            left: Box::new(expr::Expression::Illegal),
            right: Box::new(right),
        })
    }
}

impl<'a> Parse<'a> for expr::Boolean {
    fn parse(parser: &mut Parser<'a>, _: &Precedence) -> Result<Self> {
        match parser.tokens[0] {
            Some(Token::True) => Ok(Self { value: true }),
            Some(Token::False) => Ok(Self { value: false }),
            _ => bail!("Boolean expected"),
        }
    }
}

impl<'a> Parse<'a> for expr::If<'a> {
    fn parse(parser: &mut Parser<'a>, precedence: &Precedence) -> Result<Self> {
        parser.read_token();
        let condition = expr::Expression::parse(parser, precedence)?;
        parser.read_token();

        if parser.tokens[0] != Some(Token::LBrace) {
            bail!("'{{' expected");
        }

        let consequence = stmt::BlockStmt::parse(parser, precedence)?;

        let alternative = if parser.tokens[1] == Some(Token::Else) {
            parser.read_token();
            parser.read_token();
            if parser.tokens[0] != Some(Token::LBrace) {
                bail!("'{{' expected");
            }
            let block = stmt::BlockStmt::parse(parser, precedence)?;
            Some(block)
        } else {
            None
        };

        Ok(Self {
            condition: Box::new(condition),
            consequence,
            alternative,
        })
    }
}

impl<'a> Parse<'a> for Vec<expr::Identifier<'a>> {
    fn parse(parser: &mut Parser<'a>, precedence: &Precedence) -> Result<Self> {
        parser.read_token();
        let mut idents = Vec::new();

        if parser.tokens[0].is_none() || parser.tokens[0] == Some(Token::RParen) {
            return Ok(idents);
        }

        idents.push(expr::Identifier::parse(parser, precedence)?);
        parser.read_token();

        while let Some(Token::Comma) = parser.tokens[0] {
            parser.read_token();
            idents.push(expr::Identifier::parse(parser, precedence)?);
            parser.read_token();
        }
        Ok(idents)
    }
}

impl<'a> Parse<'a> for expr::Function<'a> {
    fn parse(parser: &mut Parser<'a>, precedence: &Precedence) -> Result<Self> {
        parser.read_token();
        if parser.tokens[0] != Some(Token::LParen) {
            bail!("'(' expected");
        };

        let parameters = Vec::parse(parser, precedence)?;

        if parser.tokens[0] != Some(Token::RParen) {
            bail!("')' expected");
        };
        parser.read_token();

        let body = stmt::BlockStmt::parse(parser, precedence)?;

        Ok(Self { parameters, body })
    }
}

impl<'a> Parse<'a> for Vec<expr::Expression<'a>> {
    fn parse(parser: &mut Parser<'a>, precedence: &Precedence) -> Result<Self> {
        parser.read_token();
        let mut exprs = Vec::new();

        if parser.tokens[0].is_none() || parser.tokens[0] == Some(Token::RParen) {
            return Ok(exprs);
        }

        exprs.push(expr::Expression::parse(parser, precedence)?);
        parser.read_token();

        while let Some(Token::Comma) = parser.tokens[0] {
            parser.read_token();
            exprs.push(expr::Expression::parse(parser, precedence)?);
            parser.read_token();
        }
        Ok(exprs)
    }
}

impl<'a> Parse<'a> for expr::Call<'a> {
    fn parse(parser: &mut Parser<'a>, precedence: &Precedence) -> Result<Self> {
        let arguments = Vec::parse(parser, precedence)?;

        Ok(Self {
            function: Box::new(expr::Expression::Illegal),
            arguments,
        })
    }
}
