use super::{expr, stmt, Parser, Token};
use anyhow::{anyhow, bail, Result};

pub trait Parse<'a>
where
    Self: Sized,
{
    fn parse(parser: &mut Parser<'a>) -> Result<Self>;
}

impl<'a> Parse<'a> for stmt::Statement<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        loop {
            match parser.tokens[0] {
                None => return Ok(stmt::Statement::EOF),
                Some(Token::Let) => return Ok(Self::Let(stmt::Let::parse(parser)?)),
                Some(_) => parser.read_token()
            }
        }
    }
}

impl<'a> Parse<'a> for expr::Identifier<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        match parser.tokens[1] {
            Some(Token::Ident(value)) => {
                parser.read_token();
                Ok(expr::Identifier {
                    token: parser.tokens[0].unwrap(),
                    value,
                })
            }
            _ => {
                parser.read_token();
                bail!("Identifier expected")
            }
        }
    }
}

impl<'a> Parse<'a> for stmt::Let<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let token = parser.tokens[0].ok_or(anyhow!("Token expected"))?;

        let name = expr::Identifier::parse(parser)?;

        if let Some(Token::Assign) = parser.tokens[1] {
            // TODO expression parsing
            while parser.tokens[0] != Some(Token::Semicolon) {
                // Temporary, I think it explodes when semicolon missing
                parser.read_token();
            }

            Ok(Self {
                token,
                name,
                value: expr::Expression::Illegal,
            })
        } else {
            Err(anyhow!("Expected assignment in let statement"))
        }
    }
}
