pub mod ast;
pub mod error;
pub mod expressions;
pub mod statements;
#[cfg(test)]
mod test;

use std::iter::Iterator;

use crate::lexer::token::Token;
use crate::lexer::Lexer;
use ast::statements as stmt;
use error::Result;

pub trait Parse
where
    Self: Sized,
{
    fn parse(parser: &mut Parser, precedence: &ast::Precedence) -> Result<Self>;
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    tokens: [Option<Token>; 2],
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
        let mut p = Self {
            lexer,
            tokens: [None, None],
        };

        // Fill up the token buffer
        p.read_token();
        p.read_token();

        p
    }

    fn read_token(&mut self) {
        self.tokens.swap(0, 1);
        self.tokens[1] = self.lexer.next();
    }
}

impl Iterator for Parser<'_> {
    type Item = Result<stmt::Statement>;
    fn next(&mut self) -> Option<Self::Item> {
        match stmt::Statement::parse(self, &ast::Precedence::Lowest) {
            Ok(stmt::Statement::EOF) => None,
            x => Some(x),
        }
    }
}

pub fn assert_token(received: &Option<Token>, expected: Token) -> Result<()> {
    match received {
        Some(x) => {
            if *x == expected {
                Ok(())
            } else {
                Err(error::Error::UnexpectedTokenError {
                    expected: expected,
                    received: x.clone(),
                })
            }
        }
        None => Err(error::Error::EOFError),
    }
}
