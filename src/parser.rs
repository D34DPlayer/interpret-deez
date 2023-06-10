use crate::ast::Program;

use super::ast::{expressions as expr, statements as stmt};
use super::lexer::Lexer;
use super::token::Token;

use anyhow::Result;
use std::iter::Iterator;

mod parse;
use parse::Parse;

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
        match stmt::Statement::parse(self) {
            Ok(stmt::Statement::EOF) => return None,
            x => Some(x),
        }
    }
}

pub fn parse_program<'a>(parser: Parser<'a>) -> Program<'a> {
    Program {
        statements: Vec::from_iter(parser),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_let_stmt(stmt: &stmt::Statement, exp_id: &str) {
        match stmt {
            stmt::Statement::Let(let_stmt) => {
                assert_eq!(let_stmt.token, Token::Let);

                test_ident(&let_stmt.name, exp_id);
            }
            _ => panic!("Not let statement received"),
        }
    }

    fn test_ident(ident: &expr::Identifier, id: &str) {
        assert_eq!(ident.value, id);

        match ident.token {
            Token::Ident(v) => assert_eq!(v, id),
            _ => panic!("Ident token expected"),
        }
    }

    #[test]
    fn test_let_statements() {
        let input = "
        let x = 5;
        let y = 10;
        let urmom = 69;
        ";

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);

        let program = parse_program(parser);

        let expected_identifiers = ["x", "y", "urmom"];

        assert_eq!(program.statements.len(), 3);

        for (stmt, exp_id) in program.statements.iter().zip(expected_identifiers) {
            match stmt {
                Ok(s) => test_let_stmt(s, exp_id),
                Err(err) => panic!("{err}"),
            }
        }
    }
}
