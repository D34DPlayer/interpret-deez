use super::Parse;
use super::{expr, stmt, Parser, Precedence, Token};
use anyhow::{anyhow, Result};

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
        parser.read_token();

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
