use crate::ast::Program;

use super::ast::{expressions as expr, statements as stmt};
use super::lexer::Lexer;
use super::token::Token;

use anyhow::{anyhow, bail, Result};
use std::iter::Iterator;

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

pub trait Parse<'a>
where
    Self: Sized,
{
    fn parse(parser: &mut Parser<'a>) -> Result<Self>;
}

impl<'a> Parse<'a> for stmt::Statement<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        loop {
            if parser.tokens[0].is_none() {
                return Ok(stmt::Statement::EOF);
            }

            match parser.tokens[0].unwrap() {
                Token::Let => return Ok(Self::Let(stmt::Let::parse(parser)?)),
                _ => {}
            };

            parser.read_token();
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
        let token = parser.tokens[0].unwrap();

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

    fn test_let_stmt(stmt: stmt::Statement, exp_id: &str) {
        match stmt {
            stmt::Statement::Let(let_stmt) => {
                assert_eq!(let_stmt.token, Token::Let);

                test_ident(let_stmt.name, exp_id);
            }
            _ => panic!("Not let statement received"),
        }
    }

    fn test_ident(ident: expr::Identifier, id: &str) {
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

        for (exp_id, stmt) in expected_identifiers.iter().zip(program.statements) {
            match stmt {
                Ok(s) => test_let_stmt(s, exp_id),
                Err(err) => panic!("{err}"),
            }
        }
    }
}
