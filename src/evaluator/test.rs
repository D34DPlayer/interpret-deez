use super::Evaluate;
use crate::ast::expressions::{InfixOp, PrefixOp};
use crate::ast::statements::Statement;
use crate::evaluator::error::Error;
use crate::lexer::Lexer;
use crate::object::{Object, ObjectType};
use crate::parser::Parser;
use anyhow::Result;

struct EvalTest {
    pub input: &'static str,
    pub expected: Object,
}

struct EvalErrorTest {
    pub input: &'static str,
    pub expected: Error,
}

#[test]
fn test_eval() {
    let tests = vec![
        EvalTest {
            input: "5",
            expected: Object::Integer(5),
        },
        EvalTest {
            input: "10",
            expected: Object::Integer(10),
        },
        EvalTest {
            input: "10;69",
            expected: Object::Integer(69),
        },
        EvalTest {
            input: "12;true",
            expected: Object::Boolean(true),
        },
        EvalTest {
            input: "false",
            expected: Object::Boolean(false),
        },
        EvalTest {
            input: "!false",
            expected: Object::Boolean(true),
        },
        EvalTest {
            input: "!true",
            expected: Object::Boolean(false),
        },
        EvalTest {
            input: "!5",
            expected: Object::Boolean(false),
        },
        EvalTest {
            input: "!0",
            expected: Object::Boolean(true),
        },
        EvalTest {
            input: "!!true",
            expected: Object::Boolean(true),
        },
        EvalTest {
            input: "-21",
            expected: Object::Integer(-21),
        },
        EvalTest {
            input: "-69",
            expected: Object::Integer(-69),
        },
        EvalTest {
            input: "--12",
            expected: Object::Integer(12),
        },
        EvalTest {
            input: "60 + 9",
            expected: Object::Integer(69),
        },
        EvalTest {
            input: "60 / 3",
            expected: Object::Integer(20),
        },
        EvalTest {
            input: "60 / 7",
            expected: Object::Integer(8),
        },
        EvalTest {
            input: "60 < 7",
            expected: Object::Boolean(false),
        },
        EvalTest {
            input: "60 > 7",
            expected: Object::Boolean(true),
        },
        EvalTest {
            input: "60 == 7",
            expected: Object::Boolean(false),
        },
        EvalTest {
            input: "60 == 60",
            expected: Object::Boolean(true),
        },
        EvalTest {
            input: "60 != 7",
            expected: Object::Boolean(true),
        },
        EvalTest {
            input: "60 != 60",
            expected: Object::Boolean(false),
        },
        EvalTest {
            input: "(5 + 10 * 2 + 15 / 3) * 2 + -10",
            expected: Object::Integer(50),
        },
        EvalTest {
            input: "-50 + 100 + -50",
            expected: Object::Integer(0),
        },
        EvalTest {
            input: "(1 > 2) == false",
            expected: Object::Boolean(true),
        },
        EvalTest {
            input: "if (true) { 10 }",
            expected: Object::Integer(10),
        },
        EvalTest {
            input: "if (false) { 10 }",
            expected: Object::Null,
        },
        EvalTest {
            input: "if (1) { 10 }",
            expected: Object::Integer(10),
        },
        EvalTest {
            input: "if (0) { 10 }",
            expected: Object::Null,
        },
        EvalTest {
            input: "if (1 > 2) { 10 } else { 20 }",
            expected: Object::Integer(20),
        },
        EvalTest {
            input: "if (1 < 2) { 10 } else { 20 }",
            expected: Object::Integer(10),
        },
        EvalTest {
            input: "return 10",
            expected: Object::Integer(10),
        },
        EvalTest {
            input: "return 10; return 2;",
            expected: Object::Integer(10),
        },
        EvalTest {
            input: "12; return 10; return 2;",
            expected: Object::Integer(10),
        },
        EvalTest {
            input: "if (10 > 1) {if (10 > 1) {return 10;} return 1;}",
            expected: Object::Integer(10),
        },
    ];

    for test in tests {
        let lexer = Lexer::new(test.input);
        let parser = Parser::new(lexer);

        let parse_result: Result<Vec<Statement>> = parser.collect();

        match parse_result {
            Ok(stmts) => {
                match stmts.eval_return() {
                    Ok(x) => assert_eq!(x, test.expected, "Failed input: {}", test.input),
                    Err(e) => panic!("Error evaluating: {e}"),
                };
            }
            Err(e) => panic!("Error parsing: {e}"),
        }
    }
}

#[test]
fn test_eval_errors() {
    let tests = vec![
        EvalErrorTest {
            input: "5 + true;",
            expected: Error::InfixError {
                operator: InfixOp::Plus,
                type_left: ObjectType::Integer,
                type_right: ObjectType::Boolean,
            },
        },
        EvalErrorTest {
            input: "2; 5 + true; 5;",
            expected: Error::InfixError {
                operator: InfixOp::Plus,
                type_left: ObjectType::Integer,
                type_right: ObjectType::Boolean,
            },
        },
        EvalErrorTest {
            input: "null + 5",
            expected: Error::InfixError {
                operator: InfixOp::Plus,
                type_left: ObjectType::Null,
                type_right: ObjectType::Integer,
            },
        },
        EvalErrorTest {
            input: "5 * null",
            expected: Error::InfixError {
                operator: InfixOp::Asterisk,
                type_left: ObjectType::Integer,
                type_right: ObjectType::Null,
            },
        },
        EvalErrorTest {
            input: "-null",
            expected: Error::PrefixError {
                operator: PrefixOp::Minus,
                type_value: ObjectType::Null,
            },
        },
        EvalErrorTest {
            input: "if (10 > 1) { true + false; }",
            expected: Error::InfixError {
                operator: InfixOp::Plus,
                type_left: ObjectType::Boolean,
                type_right: ObjectType::Boolean,
            },
        },
        EvalErrorTest {
            input: r#"
            if (10 > 1) {
                if (10 > 1) {
                  return false + false;
                }
              
                return 1;
            }"#,
            expected: Error::InfixError {
                operator: InfixOp::Plus,
                type_left: ObjectType::Boolean,
                type_right: ObjectType::Boolean,
            },
        },
    ];

    for test in tests {
        let lexer = Lexer::new(test.input);
        let parser = Parser::new(lexer);

        let parse_result: Result<Vec<Statement>> = parser.collect();

        match parse_result {
            Ok(stmts) => {
                match stmts.eval_return() {
                    Ok(_) => panic!("Input was expected to error"),
                    Err(e) => match e.downcast::<Error>() {
                        Ok(e) => assert_eq!(e, test.expected),
                        Err(e) => panic!("Unexpected error received: {e}"),
                    },
                };
            }
            Err(e) => panic!("Error parsing: {e}"),
        }
    }
}
