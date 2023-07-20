use super::Evaluate;
use crate::ast::statements::Statement;
use crate::lexer::Lexer;
use crate::object::Object;
use crate::parser::Parser;
use anyhow::Result;

struct EvalTest {
    pub input: &'static str,
    pub expected: Object,
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
    ];

    for test in tests {
        let lexer = Lexer::new(test.input);
        let parser = Parser::new(lexer);

        let parse_result: Result<Vec<Statement>> = parser.collect();

        match parse_result {
            Ok(stmts) => {
                assert_eq!(stmts.eval(), test.expected, "Failed input: {}", test.input);
            }
            Err(e) => panic!("Error parsing: {}", e),
        }
    }
}
