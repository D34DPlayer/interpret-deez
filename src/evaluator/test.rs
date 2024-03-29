use super::object::{environment::Environment, hash::HashableObject, Object, ObjectType};
use super::{error::Error, Evaluate};
use crate::parser::ast::expressions::{InfixOp, PrefixOp};
use crate::parser::error::Error as ParserError;
use crate::parser::Parse;

use std::collections::HashMap;

struct EvalTest {
    pub input: &'static str,
    pub expected: Object,
}

struct EvalErrorTest {
    pub input: &'static str,
    pub expected: Error,
}

fn test_eval_output(test: EvalTest) {
    let parser = test.input.parser();

    let parse_result: Result<Vec<_>, ParserError> = parser.collect();

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
fn test_eval_str() {
    let tests = vec![
        EvalTest {
            input: "\"joe\";",
            expected: Object::Str("joe".into()),
        },
        EvalTest {
            input: "if (\"\") {1} else {2}",
            expected: Object::Integer(2),
        },
        EvalTest {
            input: "if (\"xd\") {1} else {2}",
            expected: Object::Integer(1),
        },
        EvalTest {
            input: "\"joe\" + \" \" + \"mama\";",
            expected: Object::Str("joe mama".into()),
        },
        EvalTest {
            input: "\"joe\" == \"mama\";",
            expected: Object::Boolean(false),
        },
        EvalTest {
            input: "\"joe\" == \"joe\";",
            expected: Object::Boolean(true),
        },
        EvalTest {
            input: "\"a\" < \"b\";",
            expected: Object::Boolean(true),
        },
        EvalTest {
            input: "\"a\" > \"b\";",
            expected: Object::Boolean(false),
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
fn test_eval_builtin_funcs() {
    let tests = vec![
        EvalTest {
            input: "let a = \"\"; len(a)",
            expected: Object::Integer(0),
        },
        EvalTest {
            input: "let a = \"joe\"; len(a)",
            expected: Object::Integer(3),
        },
        EvalTest {
            input: "let a = \"joe mama\"; len(a)",
            expected: Object::Integer(8),
        },
        EvalTest {
            input: "let a = 9; del(\"a\")",
            expected: Object::Integer(9),
        },
    ];

    for test in tests {
        test_eval_output(test)
    }
}

#[test]
fn test_eval_arrays() {
    let tests = vec![
        EvalTest {
            input: "[]",
            expected: Object::Array(vec![]),
        },
        EvalTest {
            input: "[1]",
            expected: Object::Array(vec![Object::Integer(1)]),
        },
        EvalTest {
            input: "[1, true]",
            expected: Object::Array(vec![Object::Integer(1), Object::Boolean(true)]),
        },
    ];

    for test in tests {
        test_eval_output(test)
    }
}

#[test]
fn test_eval_index() {
    let tests = vec![
        EvalTest {
            input: "[1, 2, 3][0]",
            expected: Object::Integer(1),
        },
        EvalTest {
            input: "[1, 2, 3][1]",
            expected: Object::Integer(2),
        },
        EvalTest {
            input: "[1, 2, 3][2]",
            expected: Object::Integer(3),
        },
        EvalTest {
            input: "[1, 2, 3][-1]",
            expected: Object::Integer(3),
        },
        EvalTest {
            input: "let i = 0; [1][i];",
            expected: Object::Integer(1),
        },
        EvalTest {
            input: "[1, 2, 3][1 + 1];",
            expected: Object::Integer(3),
        },
        EvalTest {
            input: "let myArray = [1, 2, 3]; myArray[2];",
            expected: Object::Integer(3),
        },
        EvalTest {
            input: "let myHash = hash!{}; myHash[2];",
            expected: Object::Null,
        },
        EvalTest {
            input: r#"hash!{}["foo"]"#,
            expected: Object::Null,
        },
        EvalTest {
            input: r#"hash!{"foo": 5}["foo"]"#,
            expected: Object::Integer(5),
        },
        EvalTest {
            input: r#"let key = "foo"; hash!{"foo": 5}[key]"#,
            expected: Object::Integer(5),
        },
        EvalTest {
            input: r#"hash!{5: 5}[5]"#,
            expected: Object::Integer(5),
        },
        EvalTest {
            input: r#"hash!{true: 5}[true]"#,
            expected: Object::Integer(5),
        },
        EvalTest {
            input: r#"hash!{false: 5}[false]"#,
            expected: Object::Integer(5),
        },
    ];

    for test in tests {
        test_eval_output(test)
    }
}

#[test]
fn test_eval_hash() {
    let tests = vec![
        EvalTest {
            input: "hash!{}",
            expected: Object::Hash(HashMap::new()),
        },
        EvalTest {
            input: "hash!{1: true}",
            expected: Object::Hash({
                let mut h = HashMap::new();
                h.insert(HashableObject::Integer(1), Object::Boolean(true));
                h
            }),
        },
        EvalTest {
            input: r#"
            let two = "two";
            hash!{
                "one": 10 - 9,
                two: 1 + 1,
                "thr" + "ee": 6 / 2,
                4: 4,
                true: 5,
                false: 6
            }"#,
            expected: Object::Hash({
                let mut h = HashMap::new();
                h.insert(HashableObject::Str("one".into()), Object::Integer(1));
                h.insert(HashableObject::Str("two".into()), Object::Integer(2));
                h.insert(HashableObject::Str("three".into()), Object::Integer(3));
                h.insert(HashableObject::Integer(4), Object::Integer(4));
                h.insert(HashableObject::Boolean(true), Object::Integer(5));
                h.insert(HashableObject::Boolean(false), Object::Integer(6));
                h
            }),
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
        EvalErrorTest {
            input: "len()",
            expected: Error::ArgumentsError {
                expected: 1,
                received: 0,
            },
        },
        EvalErrorTest {
            input: "let x = true; len(x)",
            expected: Error::TypeError {
                expected: ObjectType::Array,
                received: ObjectType::Boolean,
            },
        },
        EvalErrorTest {
            input: "let x = true; del(\"x\"); x",
            expected: Error::IdentifierError("x".into()),
        },
        EvalErrorTest {
            input: "let x = [1]; x[1]",
            expected: Error::IndexError(1),
        },
        EvalErrorTest {
            input: "let x = [1]; x[-2]",
            expected: Error::IndexError(-1),
        },
        EvalErrorTest {
            input: "let x = if (false) {}; hash!{x: 1}",
            expected: Error::HashError(ObjectType::Null),
        },
        EvalErrorTest {
            input: "let x = fn() {1}; hash!{1: 1}[x]",
            expected: Error::HashError(ObjectType::Function),
        },
    ];

    for test in tests {
        let parser = test.input.parser();

        let parse_result: Result<Vec<_>, ParserError> = parser.collect();

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
