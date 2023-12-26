use super::*;
use crate::lexer::Lexer;
use crate::parser::Parser;

#[test]
fn test_eval_integer_expression() {
    let tests = vec![
        ("5", 5),
        ("10", 10),
        ("-5", -5),
        ("-10", -10),
        ("5 + 5 + 5 + 5 - 10", 10),
        ("2 * 2 * 2 * 2 * 2", 32),
        ("-50 + 100 - 50", 0),
        ("5 * 2 + 10", 20),
        ("5 + 2 * 10", 25),
        ("5 * 2 - 10", 0),
        ("20 - 2 * 10", 0),
        ("50 / 2 * 2 + 10", 60),
        ("2 * (5 + 10)", 30),
        ("3 * 3 * 3 + 10", 37),
        ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);

        test_integer_object(evaluated.into(), expected);
    }
}

#[test]
fn test_eval_boolean_expression() {
    let tests = vec![
        ("true", true),
        ("false", false),
        ("1 < 2", true),
        ("1 > 2", false),
        ("1 < 1", false),
        ("1 > 1", false),
        ("1 == 1", true),
        ("1 != 1", false),
        ("1 == 2", false),
        ("1 != 2", true),
        ("true == true", true),
        ("true != true", false),
        ("false == true", false),
        ("false != true", true),
        ("(1 < 2) == true", true),
        ("(1 < 2) == false", false),
        ("(1 > 2) == true", false),
        ("(1 > 2) == false", true),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);

        test_boolean_object(evaluated.into(), expected);
    }
}

#[test]
fn test_not_operator() {
    let tests = vec![
        ("not true", false),
        ("not false", true),
        ("not 5", false),
        ("not not true", true),
        ("not not false", false),
        ("not not 5", true),
    ];

    for (input, expected) in tests {
        dbg!(&input);
        dbg!(&expected);
        let evaluated = test_eval(input);

        test_boolean_object(evaluated.into(), expected);
    }
}

#[test]
fn test_if_else_expressions() {
    let tests = vec![
        ("if (true) { 10 }", "10"),
        ("if (false) { 10 }", "null"),
        ("if (1) { 10 }", "10"),
        ("if (1 < 2) { 10 }", "10"),
        ("if (1 > 2) { 10 }", "null"),
        ("if (1 < 2) { 10 } else { 20 }", "10"),
        ("if (1 > 2) { 10 } else { 20 }", "20"),
    ];

    for (input, expected) in tests {
        dbg!(&input);
        dbg!(&expected);
        let evaluated = test_eval(input);

        if expected != "null" {
            test_integer_object(evaluated.into(), expected.parse().unwrap())
        } else {
            test_null_object(evaluated.into())
        }
    }
}

#[test]
fn test_return_expression() {
    let tests = vec![
        ("return 10;", 10),
        ("return 10; 9;", 10),
        ("9; return 10; 9;", 10),
        ("return 2 * 5; 9;", 10),
        ("9; return 2 * 5; 9;", 10),
        (
            "
        if (10 > 1) {
            if (10 > 1) {
                return 10;
            }
            return 1;
        }",
            10,
        ),
    ];

    for (input, expected) in tests {
        dbg!(&input);
        dbg!(&expected);
        let evaluated = test_eval(input);

        test_integer_object(evaluated.into(), expected)
    }
}

#[test]
fn test_error_handling() {
    let tests = vec![
        ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
        ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
        ("-true;", "unknown operator: -BOOLEAN"),
        ("false + true;", "unknown operator: BOOLEAN + BOOLEAN"),
        ("5; false + true; 5;", "unknown operator: BOOLEAN + BOOLEAN"),
        (
            "if (true) { false + true}",
            "unknown operator: BOOLEAN + BOOLEAN",
        ),
        (
            "
        if (10 > 1) {
            if (10 > 1) {
                false + true;
            }
            return 1;
        }",
            "unknown operator: BOOLEAN + BOOLEAN",
        ),
        ("foobar;", "identifier not found: foobar"),
    ];

    for (input, expected) in tests {
        dbg!(&input);
        dbg!(&expected);
        let evaluated = test_eval(input);

        test_error_object(evaluated.into(), expected)
    }
}

#[test]
fn test_let_statements() {
    let tests = vec![
        ("let a = 5; a;", 5),
        ("let a = 5 * 5; a;", 25),
        ("let a = 5; let b = a; b;", 5),
        ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
    ];

    for (input, expected) in tests {
        dbg!(&input);
        dbg!(&expected);
        let evaluated = test_eval(input);

        test_integer_object(evaluated.into(), expected)
    }
}

fn test_null_object(_evaluated: Null) {
    assert!(true);
}

fn test_integer_object(evaluated: Integer, expected: i128) {
    assert_eq!(evaluated.value, expected)
}

fn test_boolean_object(evaluated: Boolean, expected: bool) {
    assert_eq!(evaluated.value, expected)
}
fn test_error_object(evaluated: Error, expected: &str) {
    assert_eq!(evaluated.message, expected)
}

fn test_eval(input: &str) -> Box<dyn Object> {
    let mut par = Parser::new(Lexer::new(input.to_string()));
    let pro = par.parse_program();

    let env = Environment::new();

    eval(pro, env).0
}
