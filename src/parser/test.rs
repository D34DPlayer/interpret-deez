use std::fmt::Debug;

use super::ast::{expressions as expr, statements as stmt};
use crate::parser::Parse;

struct PrefixTest {
    pub input: &'static str,
    pub operator: expr::PrefixOp,
    pub value: &'static str,
}

struct InfixTest {
    pub input: &'static str,
    pub left_value: &'static str,
    pub operator: expr::InfixOp,
    pub right_value: &'static str,
}

struct ParserOutputTest {
    pub input: &'static str,
    pub expected: &'static str,
}

struct FnParamsTest {
    pub input: &'static str,
    pub parameters: Vec<&'static str>,
}

struct HashTest {
    pub input: &'static str,
    pub entries: Vec<(expr::Expression, expr::Expression)>,
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
    assert_eq!(*ident.value, *id);

    // match ident.token {
    //     Token::Ident(v) => assert_eq!(v, id),
    //     _ => panic!("Ident token expected"),
    // }
}

fn test_str(ident: &expr::Str, id: &str) {
    assert_eq!(*ident.value, *id);

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
        expr::Expression::Str(s) => test_str(s, value),
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

    let parser = input.parser();

    let statements: Vec<_> = parser.collect();

    let expected_identifiers = ["x", "y", "urmom", "joe"];

    assert_eq!(statements.len(), 4);

    let mut errors = Vec::new();

    for (stmt, exp_id) in statements.iter().zip(expected_identifiers) {
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

    let parser = input.parser();

    let statements: Vec<_> = parser.collect();

    assert_eq!(statements.len(), 3);

    let mut errors = Vec::new();

    for stmt in statements {
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

    let parser = input.parser();

    let statements: Vec<_> = parser.collect();

    assert_eq!(statements.len(), 1);

    let mut errors = Vec::new();
    for stmt in statements {
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
    let parser = input.parser();

    let statements: Vec<_> = parser.collect();

    assert_eq!(statements.len(), 1);

    let mut errors = Vec::new();

    for stmt in statements {
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
fn test_str_expressions() {
    let input = "\"joe mama\";";
    let parser = input.parser();

    let statements: Vec<_> = parser.collect();

    assert_eq!(statements.len(), 1);

    let mut errors = Vec::new();

    for stmt in statements {
        match stmt {
            Ok(s) => match s {
                stmt::Statement::Expression(expr_stmt) => match expr_stmt.expression {
                    expr::Expression::Str(s) => test_str(&s, "joe mama"),
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
        let parser = test.input.parser();

        let statements: Vec<_> = parser.collect();

        assert_eq!(statements.len(), 1);

        let mut errors = Vec::new();

        for stmt in statements {
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
        let parser = test.input.parser();

        let statements: Vec<_> = parser.collect();

        assert_eq!(statements.len(), 1);

        let mut errors = Vec::new();

        for stmt in statements {
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
        ParserOutputTest {
            input: "-a * b",
            expected: "((-a) * b);",
        },
        ParserOutputTest {
            input: "!-a",
            expected: "(!(-a));",
        },
        ParserOutputTest {
            input: "a + b + c",
            expected: "((a + b) + c);",
        },
        ParserOutputTest {
            input: "a + b - c",
            expected: "((a + b) - c);",
        },
        ParserOutputTest {
            input: "a * b * c",
            expected: "((a * b) * c);",
        },
        ParserOutputTest {
            input: "a * b / c",
            expected: "((a * b) / c);",
        },
        ParserOutputTest {
            input: "-(5 + 5)",
            expected: "(-(5 + 5));",
        },
        ParserOutputTest {
            input: "a * (b / c)",
            expected: "(a * (b / c));",
        },
        ParserOutputTest {
            input: "a + b / c",
            expected: "(a + (b / c));",
        },
        ParserOutputTest {
            input: "a + b * c + d / e - f",
            expected: "(((a + (b * c)) + (d / e)) - f);",
        },
        ParserOutputTest {
            input: "3 + 4; -5 * 5",
            expected: "(3 + 4);\n((-5) * 5);",
        },
        ParserOutputTest {
            input: "5 > 4 == 3 < 4",
            expected: "((5 > 4) == (3 < 4));",
        },
        ParserOutputTest {
            input: "5 < 4 != 3 > 4",
            expected: "((5 < 4) != (3 > 4));",
        },
        ParserOutputTest {
            input: "3 + 4 * 5 == 3 * 1 + 4 * 5",
            expected: "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)));",
        },
        ParserOutputTest {
            input: "3 < 5 == true",
            expected: "((3 < 5) == true);",
        },
        ParserOutputTest {
            input: "a + add(b * c) + d",
            expected: "((a + add((b * c))) + d);",
        },
        ParserOutputTest {
            input: "(a + add)(b * c) + d",
            expected: "((a + add)((b * c)) + d);",
        },
        ParserOutputTest {
            input: "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
            expected: "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)));",
        },
        ParserOutputTest {
            input: "add(a + b + c * d / f + g)",
            expected: "add((((a + b) + ((c * d) / f)) + g));",
        },
    ];

    for test in tests {
        let parser = test.input.parser();

        let statements: Vec<_> = parser.collect();

        let mut errors = Vec::new();

        let mut stmts = Vec::new();

        for stmt in statements {
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

#[test]
fn test_if() {
    let input = "
        if (x < y) { x };
        ";

    let parser = input.parser();

    let statements: Vec<_> = parser.collect();

    let length = statements.len();
    if length != 1 {
        for stmt in statements {
            match stmt {
                Ok(s) => println!("{}", s),
                Err(err) => println!("Error: {}", err),
            }
        }
        panic!("Expected 1 statement, got {}", length);
    }

    let mut errors = Vec::new();

    for stmt in statements {
        match stmt {
            Ok(s) => match s {
                stmt::Statement::Expression(expr_stmt) => match expr_stmt.expression {
                    expr::Expression::If(if_expr) => {
                        match *if_expr.condition {
                            expr::Expression::Infix(infix) => {
                                test_infix_expr(
                                    &infix,
                                    &InfixTest {
                                        input: "x < y",
                                        left_value: "x",
                                        operator: expr::InfixOp::LessThan,
                                        right_value: "y",
                                    },
                                );
                            }
                            _ => panic!("Not infix expression received"),
                        }

                        match &if_expr.consequence.statements[0] {
                            stmt::Statement::Expression(expr_stmt) => {
                                test_literal_expr(&expr_stmt.expression, "x");
                            }
                            _ => panic!("Not expression statement received"),
                        }
                    }
                    _ => panic!("Not if expression received"),
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
fn test_if_else() {
    let input = "
        if (x < y) { x } else { y };
        ";

    let parser = input.parser();

    let statements: Vec<_> = parser.collect();

    let length = statements.len();
    if length != 1 {
        for stmt in statements {
            match stmt {
                Ok(s) => println!("{}", s),
                Err(err) => println!("Error: {}", err),
            }
        }
        panic!("Expected 1 statement, got {}", length);
    }

    let mut errors = Vec::new();
    for stmt in statements {
        match stmt {
            Ok(s) => match s {
                stmt::Statement::Expression(expr_stmt) => match expr_stmt.expression {
                    expr::Expression::If(if_expr) => {
                        match *if_expr.condition {
                            expr::Expression::Infix(infix) => {
                                test_infix_expr(
                                    &infix,
                                    &InfixTest {
                                        input: "x < y",
                                        left_value: "x",
                                        operator: expr::InfixOp::LessThan,
                                        right_value: "y",
                                    },
                                );
                            }
                            _ => panic!("Not infix expression received"),
                        }

                        match &if_expr.consequence.statements[0] {
                            stmt::Statement::Expression(expr_stmt) => {
                                test_literal_expr(&expr_stmt.expression, "x");
                            }
                            _ => panic!("Not expression statement received"),
                        }

                        match &if_expr.alternative {
                            Some(alt) => match &alt.statements[0] {
                                stmt::Statement::Expression(expr_stmt) => {
                                    test_literal_expr(&expr_stmt.expression, "y");
                                }
                                _ => panic!("Not expression statement received"),
                            },
                            None => panic!("No alternative found"),
                        }
                    }
                    _ => panic!("Not if expression received"),
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
fn test_fn() {
    let input = "fn(x, y) { x + y; }";
    let parser = input.parser();

    let statements: Vec<_> = parser.collect();

    let length = statements.len();
    if length != 1 {
        for stmt in statements {
            match stmt {
                Ok(s) => println!("{}", s),
                Err(err) => println!("Error: {}", err),
            }
        }
        panic!("Expected 1 statement, got {}", length);
    }

    let mut errors = Vec::new();
    for stmt in statements {
        match stmt {
            Ok(s) => match s {
                stmt::Statement::Expression(expr_stmt) => match expr_stmt.expression {
                    expr::Expression::Function(fn_expr) => {
                        let expected_params = vec!["x", "y"];
                        for (x, y) in fn_expr.parameters.iter().zip(expected_params) {
                            assert_eq!(*x.value, *y);
                        }
                        assert_eq!(fn_expr.parameters.len(), 2);
                        assert_eq!(fn_expr.body.statements.len(), 1);

                        match &fn_expr.body.statements[0] {
                            stmt::Statement::Expression(e) => match &e.expression {
                                expr::Expression::Infix(i) => {
                                    let value = &InfixTest {
                                        input: "x+y",
                                        left_value: "x",
                                        operator: expr::InfixOp::Plus,
                                        right_value: "y",
                                    };
                                    test_infix_expr(i, value);
                                }
                                _ => panic!("Not expression statement received"),
                            },
                            _ => panic!("Not expression statement received"),
                        }
                    }
                    _ => panic!("Not if expression received"),
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
fn test_fn_params() {
    let tests = vec![
        FnParamsTest {
            input: "fn() {};",
            parameters: vec![],
        },
        FnParamsTest {
            input: "fn(x) {};",
            parameters: vec!["x"],
        },
        FnParamsTest {
            input: "fn(y,z) {};",
            parameters: vec!["y", "z"],
        },
    ];

    for test in tests {
        let parser = test.input.parser();

        let statements: Vec<_> = parser.collect();

        assert_eq!(statements.len(), 1);

        let mut errors = Vec::new();

        for stmt in statements {
            match stmt {
                Ok(s) => match s {
                    stmt::Statement::Expression(expr_stmt) => match expr_stmt.expression {
                        expr::Expression::Function(f) => {
                            for (a, b) in f.parameters.iter().zip(&test.parameters) {
                                assert_eq!(*a.value, **b);
                            }
                        }
                        _ => panic!("Not function expression received"),
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
fn test_call_expr() {
    let input = "add(1, 2 * 3, 4 + 5);";
    let parser = input.parser();

    let statements: Vec<_> = parser.collect();

    let length = statements.len();
    if length != 1 {
        for stmt in statements {
            match stmt {
                Ok(s) => println!("{}", s),
                Err(err) => println!("Error: {}", err),
            }
        }
        panic!("Expected 1 statement, got {}", length);
    }

    let mut errors = Vec::new();
    for stmt in statements {
        match stmt {
            Ok(s) => match s {
                stmt::Statement::Expression(expr_stmt) => match expr_stmt.expression {
                    expr::Expression::Call(c) => {
                        match *c.function {
                            expr::Expression::Identifier(i) => test_ident(&i, "add"),
                            _ => panic!("Wrong type of function received"),
                        }

                        match &c.arguments[0] {
                            expr::Expression::Integer(i) => test_int(&i, 1),
                            _ => panic!("Wrong first arg"),
                        }

                        let second = InfixTest {
                            input: "2*3",
                            left_value: "2",
                            operator: expr::InfixOp::Asterisk,
                            right_value: "3",
                        };
                        match &c.arguments[1] {
                            expr::Expression::Infix(i) => test_infix_expr(&i, &second),
                            _ => panic!("Wrong first arg"),
                        }

                        let third = InfixTest {
                            input: "4+5",
                            left_value: "4",
                            operator: expr::InfixOp::Plus,
                            right_value: "5",
                        };
                        match &c.arguments[2] {
                            expr::Expression::Infix(i) => test_infix_expr(&i, &third),
                            _ => panic!("Wrong first arg"),
                        }
                    }
                    _ => panic!("Not if expression received"),
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
fn test_array_expr() {
    let tests = vec![
        ParserOutputTest {
            input: "[1, 2, 3+2]",
            expected: "[1, 2, (3 + 2)];",
        },
        ParserOutputTest {
            input: "[]",
            expected: "[];",
        },
        ParserOutputTest {
            input: "a[12]",
            expected: "a[12];",
        },
        ParserOutputTest {
            input: "a[12 + 1]",
            expected: "a[(12 + 1)];",
        },
        ParserOutputTest {
            input: "a[joe]",
            expected: "a[joe];",
        },
        ParserOutputTest {
            input: "len(a[joe])",
            expected: "len(a[joe]);",
        },
    ];

    for test in tests {
        let parser = test.input.parser();

        let statements: Vec<_> = parser.collect();

        let length = statements.len();
        if length != 1 {
            for stmt in statements {
                match stmt {
                    Ok(s) => println!("{}", s),
                    Err(err) => println!("Error: {}", err),
                }
            }
            panic!("Expected 1 statement, got {}", length);
        }

        assert_eq!(test.expected, statements[0].as_ref().unwrap().to_string());
    }
}

#[test]
fn test_hash_expr() {
    let tests = vec![
        HashTest {
            input: "hash!{\"a\": 1}",
            entries: vec![(
                expr::Expression::Str(expr::Str { value: "a".into() }),
                expr::Expression::Integer(expr::Integer { value: 1 }),
            )],
        },
        HashTest {
            input: "hash!{\"a\": 1, joe: true, true: joe}",
            entries: vec![
                (
                    expr::Expression::Str(expr::Str { value: "a".into() }),
                    expr::Expression::Integer(expr::Integer { value: 1 }),
                ),
                (
                    expr::Expression::Identifier(expr::Identifier {
                        value: "joe".into(),
                    }),
                    expr::Expression::Boolean(expr::Boolean { value: true }),
                ),
                (
                    expr::Expression::Boolean(expr::Boolean { value: true }),
                    expr::Expression::Identifier(expr::Identifier {
                        value: "joe".into(),
                    }),
                ),
            ],
        },
        HashTest {
            input: "hash!{\"a\": 1}",
            entries: vec![(
                expr::Expression::Str(expr::Str { value: "a".into() }),
                expr::Expression::Integer(expr::Integer { value: 1 }),
            )],
        },
        HashTest {
            input: "hash!{}",
            entries: vec![],
        },
    ];

    for test in tests {
        let parser = test.input.parser();

        let statements: Vec<_> = parser.collect();

        let length = statements.len();
        if length != 1 {
            for stmt in statements {
                match stmt {
                    Ok(s) => println!("{}", s),
                    Err(err) => println!("Error: {}", err),
                }
            }
            panic!("Expected 1 statement, got {}", length);
        }

        match &statements[0] {
            Ok(s) => match s {
                stmt::Statement::Expression(e) => match &e.expression {
                    expr::Expression::Hash(h) => {
                        vec_eq(&test.entries, &h.entries);
                    }
                    _ => panic!("Hash expected"),
                },
                _ => panic!("Expression expected"),
            },
            Err(err) => panic!("Error: {}", err),
        }
    }
}

fn vec_eq<T: PartialEq + Debug>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let a_len = a.len();
    let b_len = b.len();
    if a_len != b_len {
        panic!("Len mismatch {a_len} != {b_len}")
    }

    for (i, (a_el, b_el)) in a.iter().zip(b).enumerate() {
        assert_eq!(a_el, b_el, "Mismatch at index {i}")
    }

    true
}
