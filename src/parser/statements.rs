use super::ast::{expressions as expr, statements as stmt, Precedence};
use super::{assert_token, Parse, Parser};
use crate::lexer::token::Token;

use super::error::{Error, Result};

impl Parse for stmt::Statement {
    fn parse(parser: &mut Parser, precedence: &Precedence) -> Result<Self> {
        while parser.tokens[0] == Some(Token::Semicolon) {
            parser.read_token();
        }

        let stmt = match parser.tokens[0] {
            None => Ok(Self::EOF),
            Some(Token::Let) => stmt::Let::parse(parser, precedence).map(Self::Let),
            Some(Token::Return) => stmt::Return::parse(parser, precedence).map(Self::Return),
            Some(_) => stmt::ExpressionStmt::parse(parser, precedence).map(Self::Expression),
        };

        while parser.tokens[0] == Some(Token::Semicolon) {
            parser.read_token();
        }

        stmt
    }
}

impl Parse for stmt::Let {
    fn parse(parser: &mut Parser, precedence: &Precedence) -> Result<Self> {
        assert_token(&parser.tokens[0], Token::Let)?;
        parser.read_token();

        let name = expr::Identifier::parse(parser, precedence)?;

        assert_token(&parser.tokens[1], Token::Assign)?;
        parser.read_token();

        parser.read_token();
        let expression = expr::Expression::parse(parser, precedence)?;

        parser.read_token();
        Ok(Self {
            name,
            value: expression,
        })
    }
}

impl Parse for stmt::Return {
    fn parse(parser: &mut Parser, precedence: &Precedence) -> Result<Self> {
        assert_token(&parser.tokens[0], Token::Return)?;

        parser.read_token();
        let expression = expr::Expression::parse(parser, precedence)?;

        parser.read_token();
        Ok(Self {
            return_value: expression,
        })
    }
}

impl Parse for stmt::ExpressionStmt {
    fn parse(parser: &mut Parser, precedence: &Precedence) -> Result<Self> {
        if parser.tokens[0].is_none() {
            return Err(Error::EOFError);
        }

        let expression = expr::Expression::parse(parser, precedence)?;

        parser.read_token();
        Ok(Self { expression })
    }
}
