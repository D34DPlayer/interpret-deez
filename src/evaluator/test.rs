use super::{error::Error, Evaluate};
use crate::ast::expressions::{InfixOp, PrefixOp};
use crate::ast::statements::Statement;
use crate::lexer::Lexer;
use crate::object::{Environment, Object, ObjectType};
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

fn test_eval_output(test: EvalTest) {
    let lexer = Lexer::new(test.input);
    let parser = Parser::new(lexer);

    let parse_result: Result<Vec<Statement>> = parser.collect();

    let env = Environment::new_heap(None);

    match parse_result {
        Ok(stmts) => {
            match stmts.eval_return(env) {
                Ok(x) => assert_eq!(x, test.expected, "Failed input: {}", test.input),
                Err(e) => panic!("Error evaluating: {e}"),
            };
        }
        Err(e) => panic!("Error parsing: {e}"),
    }
}

#[test]
fn test_eval_literals() {
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
    ];

    for test in tests {
        test_eval_output(test)
    }
}

#[test]
fn test_eval_prefix() {
    let tests = vec![
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
    ];

    for test in tests {
        test_eval_output(test)
    }
}

#[test]
fn test_eval_infix() {
    let tests = vec![
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
    ];

    for test in tests {
        test_eval_output(test)
    }
}

#[test]
fn test_eval_if() {
    let tests = vec![
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
    ];

    for test in tests {
        test_eval_output(test)
    }
}

#[test]
fn test_eval_return() {
    let tests = vec![
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
        test_eval_output(test)
    }
}

#[test]
fn test_eval_let() {
    let tests = vec![
        EvalTest {
            input: "let a = 5; a;",
            expected: Object::Integer(5),
        },
        EvalTest {
            input: "let a = 5 * 5; a;",
            expected: Object::Integer(25),
        },
        EvalTest {
            input: "let a = 5; let b = a; b;",
            expected: Object::Integer(5),
        },
        EvalTest {
            input: "let a = 5; let b = a; let c = a + b + 5; c;",
            expected: Object::Integer(15),
        },
    ];

    for test in tests {
        test_eval_output(test)
    }
}

#[test]
fn test_eval_funcs() {
    let tests = vec![
        EvalTest {
            input: "let three = fn() { 69; return 3; 21 }; three()",
            expected: Object::Integer(3),
        },
        EvalTest {
            input: "let identity = fn(x) { x; }; identity(5)",
            expected: Object::Integer(5),
        },
        EvalTest {
            input: "let identity = fn(x) { return x; }; identity(5);",
            expected: Object::Integer(5),
        },
        EvalTest {
            input: "let double = fn(x) { x * 2; }; double(5);",
            expected: Object::Integer(10),
        },
        EvalTest {
            input: "let add = fn(x, y) { x + y; }; add(6, 5);",
            expected: Object::Integer(11),
        },
        EvalTest {
            input: "let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5))",
            expected: Object::Integer(20),
        },
        EvalTest {
            input: "fn(x) { x; }(11)",
            expected: Object::Integer(11),
        },
        EvalTest {
            input: r#"
              let newAdder = fn(x) {fn(y) { x + y };};
              
              let addTwo = newAdder(2);
              addTwo(2);"#,
            expected: Object::Integer(4),
        },
    ];

    for test in tests {
        test_eval_output(test)
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
            input: "-true",
            expected: Error::PrefixError {
                operator: PrefixOp::Minus,
                type_value: ObjectType::Boolean,
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
        EvalErrorTest {
            input: "ur_mom",
            expected: Error::IdentifierError("ur_mom".to_string()),
        },
        EvalErrorTest {
            input: "let x = 69; x()",
            expected: Error::CallableError(ObjectType::Integer),
        },
        EvalErrorTest {
            input: "let x = fn(a) {a}; x()",
            expected: Error::ArgumentsError {
                expected: 1,
                received: 0,
            },
        },
    ];

    for test in tests {
        let lexer = Lexer::new(test.input);
        let parser = Parser::new(lexer);

        let parse_result: Result<Vec<Statement>> = parser.collect();

        let env = Environment::new_heap(None);

        match parse_result {
            Ok(stmts) => {
                match stmts.eval_return(env) {
                    Ok(_) => panic!("Input '{}' was expected to error", test.input),
                    Err(e) => assert_eq!(e, test.expected),
                };
            }
            Err(e) => panic!("Error parsing: {e}"),
        }
    }
}
