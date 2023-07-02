use super::Parse;
use super::{expr, stmt, Parser, Precedence, Token};
use anyhow::{anyhow, bail, Result};

impl<'a> Parse<'a> for stmt::Statement<'a> {
    fn parse(parser: &mut Parser<'a>, precedence: &Precedence) -> Result<Self> {
        while parser.tokens[0] == Some(Token::Semicolon) {
            parser.read_token();
        }

        let stmt = match parser.tokens[0] {
            None => Ok(Self::EOF),
            Some(Token::LBrace) => {
                let s = stmt::BlockStmt::parse(parser, precedence)?;
                parser.read_token();
                Ok(Self::Block(s))
            }
            Some(Token::Let) => stmt::Let::parse(parser, precedence).map(|x| Self::Let(x)),
            Some(Token::Return) => stmt::Return::parse(parser, precedence).map(|x| Self::Return(x)),
            Some(_) => stmt::ExpressionStmt::parse(parser, precedence).map(|x| Self::Expression(x)),
        };

        while parser.tokens[0] == Some(Token::Semicolon) {
            parser.read_token();
        }

        stmt
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

impl<'a> Parse<'a> for stmt::BlockStmt<'a> {
    fn parse(parser: &mut Parser<'a>, precedence: &Precedence) -> Result<Self> {
        let mut statements = Vec::new();

        if parser.tokens[0] != Some(Token::LBrace) {
            bail!("Expected opening brace")
        }
        parser.read_token();

        while parser.tokens[0] != Some(Token::RBrace) && !parser.tokens[0].is_none() {
            let s: stmt::Statement<'_> = stmt::Statement::parse(parser, precedence)?;
            statements.push(s);
        }

        if parser.tokens[0] != Some(Token::RBrace) {
            bail!("Expected closing brace")
        }
        // we stop at the "{" since this can be the end of an expression
        //parser.read_token();

        Ok(Self { statements })
    }
}
