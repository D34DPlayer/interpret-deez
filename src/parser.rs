use crate::ast::{expressions as expr, statements as stmt, Precedence};
use crate::lexer::Lexer;
use crate::token::Token;

use anyhow::Result;
use std::iter::Iterator;

pub mod expressions;
pub mod statements;

pub trait Parse<'a>
where
    Self: Sized,
{
    fn parse(parser: &mut Parser<'a>, precedence: &Precedence) -> Result<Self>;
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    tokens: [Option<Token<'a>>; 2],
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

impl<'a> Iterator for Parser<'a> {
    type Item = Result<stmt::Statement<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        match stmt::Statement::parse(self, &Precedence::Lowest) {
            Ok(stmt::Statement::EOF) => return None,
            x => Some(x),
        }
    }
}

#[cfg(test)]
mod test;
