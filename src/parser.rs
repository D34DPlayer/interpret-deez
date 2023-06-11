use crate::ast::Program;
use crate::ast::{expressions as expr, statements as stmt};
use crate::lexer::Lexer;
use crate::token::Token;

use anyhow::Result;
use std::iter::Iterator;

pub mod parse;
pub mod parseExpr;
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

    fn skip_semicolon(&mut self) {
        while let Some(Token::Semicolon) = self.tokens[1] {
            self.read_token();
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<stmt::Statement<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        match stmt::Statement::parse(self, &parseExpr::Precedence::Lowest) {
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

    fn test_int(int: &expr::Integer, value: i64) {
        assert_eq!(int.value, value);

        match int.token {
            Token::Int(v) => assert_eq!(v, value.to_string()),
            _ => panic!("Int token expected"),
        }
    }

    fn test_return_stmt(stmt: &stmt::Statement) {
        match stmt {
            stmt::Statement::Return(r) => assert_eq!(r.token, Token::Return),
            _ => panic!("Not return statement received"),
        }
    }

    #[test]
    fn test_let_statements() {
        let input = "
        let x = 5;
        let y = 10;
        let urmom = 69;
        let joe = 12;
        ";

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);

        let program = parse_program(parser);

        let expected_identifiers = ["x", "y", "urmom", "joe"];

        println!("{:?}", program.statements);

        assert_eq!(program.statements.len(), 4);

        let mut errors = Vec::new();

        for (stmt, exp_id) in program.statements.iter().zip(expected_identifiers) {
            match stmt {
                Ok(s) => {
                    test_let_stmt(s, exp_id);
                    println!("{:?}", s);
                }
                Err(err) => {
                    println!("Error: {}", err);
                    errors.push(err);
                }
            }
        }

        assert_eq!(errors.len(), 0, "Errors found: {:?}", errors);
    }

    #[test]
    fn test_return_statements() {
        let input = "
        return 5;
        return 10;
        return 69;
        ";

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);

        let program = parse_program(parser);

        assert_eq!(program.statements.len(), 3);

        let mut errors = Vec::new();

        for stmt in program.statements {
            match stmt {
                Ok(s) => test_return_stmt(&s),
                Err(err) => {
                    println!("Error: {}", err);
                    errors.push(err);
                }
            }
        }

        assert_eq!(errors.len(), 0, "Errors found: {:?}", errors);
    }

    #[test]
    fn test_ident_expressions() {
        let input = "foobar;";

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);

        let program = parse_program(parser);

        assert_eq!(program.statements.len(), 1);

        let mut errors = Vec::new();

        for stmt in program.statements {
            match stmt {
                Ok(s) => match s {
                    stmt::Statement::Expression(expr_stmt) => match expr_stmt.expression {
                        expr::Expression::Identifier(ident) => test_ident(&ident, "foobar"),
                        _ => panic!("Not identifier expression received"),
                    },
                    _ => panic!("Not expression statement received"),
                },
                Err(err) => {
                    println!("Error: {}", err);
                    errors.push(err);
                }
            }
        }

        assert_eq!(errors.len(), 0, "Errors found: {:?}", errors);
    }

    #[test]
    fn test_int_expressions() {
        let input = "5;";
        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);

        let program = parse_program(parser);

        assert_eq!(program.statements.len(), 1);

        let mut errors = Vec::new();

        for stmt in program.statements {
            match stmt {
                Ok(s) => match s {
                    stmt::Statement::Expression(expr_stmt) => match expr_stmt.expression {
                        expr::Expression::Integer(int) => test_int(&int, 5),
                        _ => panic!("Not integer expression received"),
                    },
                    _ => panic!("Not expression statement received"),
                },
                Err(err) => {
                    println!("Error: {}", err);
                    errors.push(err);
                }
            }
        }
    }
}
