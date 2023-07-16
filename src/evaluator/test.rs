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
fn test_int_eval() {
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
    ];

    for test in tests {
        let lexer = Lexer::new(test.input);
        let parser = Parser::new(lexer);

        let parse_result: Result<Vec<Statement>> = parser.collect();

        match parse_result {
            Ok(stmts) => {
                assert_eq!(stmts.eval(), test.expected);
            }
            Err(e) => panic!("Error parsing: {}", e),
        }
    }
}
