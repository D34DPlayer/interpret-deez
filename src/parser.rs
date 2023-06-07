use crate::ast::expressions::Expression;
use crate::ast::statements::Statement;
use crate::ast::Program;

use super::ast::{expressions as expr, statements as stmt};
use super::lexer::Lexer;
use super::token::Token;

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

        p.read_token();
        p.read_token();

        p
    }

    fn read_token(&mut self) {
        self.tokens.swap(0, 1);
        self.tokens[1] = self.lexer.next();
    }

    fn parse_stmt(&mut self) -> Option<Statement<'a>> {
        match self.tokens[0] {
            Some(Token::Let) => self.parse_let_stmt(),
            _ => None,
        }
    }

    fn parse_let_stmt(&mut self) -> Option<Statement<'a>> {
        let token = self.tokens[0].unwrap();

        let id = match self.tokens[1] {
            Some(Token::Ident(i)) => i,
            _ => return None,
        };

        let name = expr::Identifier {
            token: self.tokens[1].unwrap(),
            value: id,
        };

        self.read_token();

        if let Some(Token::Assign) = self.tokens[1] {
            // TODO expression parsing
            while self.tokens[0] != Some(Token::Semicolon) {
                self.read_token();
            }

            Some(Statement::Let(stmt::Let {
                token,
                name,
                value: Expression::Illegal,
            }))
        } else {
            None
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Statement<'a>;
    fn next(&mut self) -> Option<Statement<'a>> {
        loop {
            if self.tokens[0].is_none() {
                return None;
            }

            let stmt_opt = self.parse_stmt();
            self.read_token();

            if let Some(stmt) = stmt_opt {
                return Some(stmt);
            }
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
            test_let_stmt(stmt, exp_id);
        }
    }
}
