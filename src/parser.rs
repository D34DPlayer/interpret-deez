use crate::ast::{expressions as expr, statements as stmt};
use crate::ast::{Precedence, Program};
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

pub fn parse_program<'a>(parser: Parser<'a>) -> Program<'a> {
    Program {
        statements: Vec::from_iter(parser),
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    struct PrefixTest {
        pub input: &'static str,
        pub operator: expr::PrefixOp,
        pub value: &'static str,
    }

    #[allow(dead_code)]
    struct InfixTest {
        pub input: &'static str,
        pub left_value: &'static str,
        pub operator: expr::InfixOp,
        pub right_value: &'static str,
    }

    struct OperatorPrecedenceTest {
        pub input: &'static str,
        pub expected: &'static str,
    }

    fn test_let_stmt(stmt: &stmt::Statement, exp_id: &str) {
        match stmt {
            stmt::Statement::Let(let_stmt) => {
                // assert_eq!(let_stmt.token, Token::Let);

                test_ident(&let_stmt.name, exp_id);
            }
            _ => panic!("Not let statement received"),
        }
    }

    fn test_ident(ident: &expr::Identifier, id: &str) {
        assert_eq!(ident.value, id);

        // match ident.token {
        //     Token::Ident(v) => assert_eq!(v, id),
        //     _ => panic!("Ident token expected"),
        // }
    }

    fn test_int(int: &expr::Integer, value: i64) {
        assert_eq!(int.value, value);

        // match int.token {
        //     Token::Int(v) => assert_eq!(v, value.to_string()),
        //     _ => panic!("Int token expected"),
        // }
    }

    fn test_bool(b: &expr::Boolean, value: bool) {
        assert_eq!(b.value, value);
    }

    fn test_literal_expr(ex: &expr::Expression, value: &str) {
        match ex {
            expr::Expression::Identifier(ident) => test_ident(ident, value),
            expr::Expression::Integer(int) => test_int(int, value.parse().unwrap()),
            expr::Expression::Boolean(b) => test_bool(b, value.parse().unwrap()),
            _ => panic!("Not literal expression received"),
        }
    }

    fn test_infix_expr(ex: &expr::Infix, value: &InfixTest) {
        test_literal_expr(&*ex.left, value.left_value);
        assert_eq!(ex.operator, value.operator);
        test_literal_expr(&*ex.right, value.right_value);
    }

    fn test_prefix_expr(ex: &expr::Prefix, value: &PrefixTest) {
        assert_eq!(ex.operator, value.operator);
        test_literal_expr(&*ex.right, value.value);
    }

    fn test_return_stmt(stmt: &stmt::Statement) {
        match stmt {
            stmt::Statement::Return(_) => {
                // assert_eq!(r.token, Token::Return);
            }
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

        assert_eq!(program.statements.len(), 4);

        let mut errors = Vec::new();

        for (stmt, exp_id) in program.statements.iter().zip(expected_identifiers) {
            match stmt {
                Ok(s) => {
                    test_let_stmt(s, exp_id);
                    println!("{}", s);
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

    #[test]
    fn test_prefix_expressions() {
        let tests = vec![
            PrefixTest {
                input: "!5;",
                operator: expr::PrefixOp::Bang,
                value: "5",
            },
            PrefixTest {
                input: "-15;",
                operator: expr::PrefixOp::Minus,
                value: "15",
            },
            PrefixTest {
                input: "!true;",
                operator: expr::PrefixOp::Bang,
                value: "true",
            },
            PrefixTest {
                input: "!false;",
                operator: expr::PrefixOp::Bang,
                value: "false",
            },
            PrefixTest {
                input: "!joe;",
                operator: expr::PrefixOp::Bang,
                value: "joe",
            },
            PrefixTest {
                input: "-mama;",
                operator: expr::PrefixOp::Minus,
                value: "mama",
            },
        ];

        for test in tests {
            let lexer = Lexer::new(test.input);
            let parser = Parser::new(lexer);

            let program = parse_program(parser);

            assert_eq!(program.statements.len(), 1);

            let mut errors = Vec::new();

            for stmt in program.statements {
                match stmt {
                    Ok(s) => match s {
                        stmt::Statement::Expression(expr_stmt) => match expr_stmt.expression {
                            expr::Expression::Prefix(prefix) => {
                                test_prefix_expr(&prefix, &test);
                            }
                            _ => panic!("Not prefix expression received"),
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
    }

    #[test]
    fn test_infix_expressions() {
        let tests = vec![
            InfixTest {
                input: "5 + 5;",
                left_value: "5",
                operator: expr::InfixOp::Plus,
                right_value: "5",
            },
            InfixTest {
                input: "joe - mama;",
                left_value: "joe",
                operator: expr::InfixOp::Minus,
                right_value: "mama",
            },
            InfixTest {
                input: "true * false;",
                left_value: "true",
                operator: expr::InfixOp::Asterisk,
                right_value: "false",
            },
            InfixTest {
                input: "5 / true;",
                left_value: "5",
                operator: expr::InfixOp::ForwardSlash,
                right_value: "true",
            },
            InfixTest {
                input: "5 > True;",
                left_value: "5",
                operator: expr::InfixOp::GreaterThan,
                right_value: "True",
            },
            InfixTest {
                input: "False < false;",
                left_value: "False",
                operator: expr::InfixOp::LessThan,
                right_value: "false",
            },
            InfixTest {
                input: "🍌 == 🍆;",
                left_value: "🍌",
                operator: expr::InfixOp::Equal,
                right_value: "🍆",
            },
            InfixTest {
                input: "5 != 5;",
                left_value: "5",
                operator: expr::InfixOp::NotEqual,
                right_value: "5",
            },
        ];

        for test in tests {
            let lexer = Lexer::new(test.input);
            let parser = Parser::new(lexer);

            let program = parse_program(parser);

            assert_eq!(program.statements.len(), 1);

            let mut errors = Vec::new();

            for stmt in program.statements {
                match stmt {
                    Ok(s) => match s {
                        stmt::Statement::Expression(expr_stmt) => match expr_stmt.expression {
                            expr::Expression::Infix(infix) => {
                                test_infix_expr(&infix, &test);
                            }
                            _ => panic!("Not infix expression received"),
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
    }

    #[test]
    fn test_operator_precedence() {
        let tests = vec![
            OperatorPrecedenceTest {
                input: "-a * b",
                expected: "((-a) * b);",
            },
            OperatorPrecedenceTest {
                input: "!-a",
                expected: "(!(-a));",
            },
            OperatorPrecedenceTest {
                input: "a + b + c",
                expected: "((a + b) + c);",
            },
            OperatorPrecedenceTest {
                input: "a + b - c",
                expected: "((a + b) - c);",
            },
            OperatorPrecedenceTest {
                input: "a * b * c",
                expected: "((a * b) * c);",
            },
            OperatorPrecedenceTest {
                input: "a * b / c",
                expected: "((a * b) / c);",
            },
            OperatorPrecedenceTest {
                input: "-(5 + 5)",
                expected: "(-(5 + 5));",
            },
            OperatorPrecedenceTest {
                input: "a * (b / c)",
                expected: "(a * (b / c));",
            },
            OperatorPrecedenceTest {
                input: "a + b / c",
                expected: "(a + (b / c));",
            },
            OperatorPrecedenceTest {
                input: "a + b * c + d / e - f",
                expected: "(((a + (b * c)) + (d / e)) - f);",
            },
            OperatorPrecedenceTest {
                input: "3 + 4; -5 * 5",
                expected: "(3 + 4);\n((-5) * 5);",
            },
            OperatorPrecedenceTest {
                input: "5 > 4 == 3 < 4",
                expected: "((5 > 4) == (3 < 4));",
            },
            OperatorPrecedenceTest {
                input: "5 < 4 != 3 > 4",
                expected: "((5 < 4) != (3 > 4));",
            },
            OperatorPrecedenceTest {
                input: "3 + 4 * 5 == 3 * 1 + 4 * 5",
                expected: "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)));",
            },
            OperatorPrecedenceTest {
                input: "3 < 5 == true",
                expected: "((3 < 5) == true);",
            },
        ];

        for test in tests {
            let lexer = Lexer::new(test.input);
            let parser = Parser::new(lexer);

            let program = parse_program(parser);

            let mut errors = Vec::new();

            let mut stmts = Vec::new();

            for stmt in program.statements {
                match stmt {
                    Ok(s) => match s {
                        stmt::Statement::Expression(expr_stmt) => {
                            stmts.push(format!("{}", expr_stmt));
                        }
                        _ => panic!("Not expression statement received"),
                    },
                    Err(err) => {
                        println!("Error: {}", err);
                        errors.push(err);
                    }
                }
            }

            assert_eq!(errors.len(), 0, "Errors found: {:?}", errors);
            assert_eq!(stmts.join("\n"), test.expected);
        }
    }
}
