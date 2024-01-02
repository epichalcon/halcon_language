use core::panic;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

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
        ("10 % 2 + 10", 10),
        ("1 + 3 % 2", 2),
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

        test_integer_object(evaluated, expected);
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
        ("1 <= 2", true),
        ("1 >= 2", false),
        ("1 <= 1", true),
        ("1 >= 1", true),
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

        test_boolean_object(evaluated, expected);
    }
}

#[test]
fn test_string_literal() {
    let input = r#""Hello world""#;
    let evaluated = test_eval(input);

    test_string_object(evaluated, "Hello world".to_string());
}

#[test]
fn test_string_literal_concatenation() {
    let input = r#""Hello" + " " + "world""#;
    let evaluated = test_eval(input);

    test_string_object(evaluated, "Hello world".to_string());
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

        test_boolean_object(evaluated, expected);
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
            test_integer_object(evaluated, expected.parse().unwrap())
        } else {
            test_null_object(evaluated)
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

        test_integer_object(evaluated, expected)
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
        (r#""hello" - "world";"#, "unknown operator: STRING - STRING"),
        (r#"len(1)"#, "argument to len not supported, got INTEGER"),
        (
            r#"len("one", "two")"#,
            "wrong number of arguments. got: 2, want: 1",
        ),
        ("[1, 2, 3][3]", "index: 3 out of bounds: 3"),
        ("[1, 2, 3][-1]", "index: -1 out of bounds: 3"),
        ("{[1]: 1}", "unusable as hash key: ARRAY"),
        ("{fun(x) {x}: 1}", "unusable as hash key: FUNCTION"),
        (
            r#"{"name": "asdf"}[fun(x) {x}]"#,
            "unusable as hash key: FUNCTION",
        ),
    ];

    for (input, expected) in tests {
        dbg!(&input);
        dbg!(&expected);
        let evaluated = test_eval(input);

        test_error_object(evaluated, expected)
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

        test_integer_object(evaluated, expected)
    }
}

#[test]
fn test_function_object() {
    let input = "fun(x) {x + 2}";

    dbg!(&input);
    let evaluated = test_eval(input);

    let eval = match evaluated {
        ObjectType::Function(exp) => exp,
        actual => panic!("Expected a function, got {:?}", actual),
    };

    assert_eq!(1, eval.parameters.len());
    assert_eq!("x", eval.parameters[0].token_literal());
    assert_eq!("(x + 2)", eval.body.string());
}

#[test]
fn test_function_aplication() {
    let tests = vec![
        ("let identity = fun(x) {x;}; identity(5);", 5),
        ("let identity = fun(x) {return x;}; identity(5);", 5),
        ("let double = fun(x) {x * 2;}; double(5);", 10),
        ("let add = fun(x, y) {x + y;}; add(5, 2);", 7),
        ("let add = fun(x, y) {x + y;}; add(5 + 5, add(5, 5));", 20),
        ("fun(x) {x;}(5)", 5),
    ];

    for (input, expected) in tests {
        dbg!(&input);
        dbg!(&expected);
        let evaluated = test_eval(input);

        test_integer_object(evaluated, expected)
    }
}

#[test]
fn test_closures() {
    let input = "let new_adder = fun(x) {
                    fun(y) {x + y};
                };
                let add_two = new_adder(2);
                add_two(2)";

    dbg!(&input);
    let evaluated = test_eval(input);
    test_integer_object(evaluated, 4)
}

#[test]
fn test_recursive() {
    let input =
        "let factorial = fun(n) { if (n == 0) { 1 } else { n * factorial(n - 1) } }; factorial(5);";

    dbg!(&input);
    let evaluated = test_eval(input);
    test_integer_object(evaluated, 120)
}

#[test]
fn test_builtin_functions() {
    let tests = vec![
        (r#"len("")"#, 0),
        (r#"len("four")"#, 4),
        (r#"len("hello world")"#, 11),
    ];

    for (input, expected) in tests {
        dbg!(&input);
        dbg!(&expected);
        let evaluated = test_eval(input);

        test_integer_object(evaluated, expected)
    }
}

#[test]
fn test_array_literals() {
    let input = "[1, 2 * 2, 3 + 3]";

    dbg!(&input);
    let evaluated = test_eval(input);
    let result = match evaluated {
        ObjectType::Array(exp) => exp,
        actual => panic!("Expected an array, got {:?}", actual),
    };

    assert_eq!(3, result.elements.len());

    test_integer_object(result.elements[0].clone(), 1);
    test_integer_object(result.elements[1].clone(), 4);
    test_integer_object(result.elements[2].clone(), 6);
}

#[test]
fn test_array_index_expression() {
    let tests = vec![
        ("[1, 2, 3][0]", 1),
        ("[1, 2, 3][1]", 2),
        ("[1, 2, 3][2]", 3),
        ("let i = 0; [1][i]", 1),
        ("[1, 2, 3][1 + 1]", 3),
        ("let myArray = [1, 2, 3]; myArray[2]", 3),
        (
            "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2]",
            6,
        ),
        ("let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i]", 2),
    ];

    for (input, expected) in tests {
        dbg!(&input);
        dbg!(&expected);
        let evaluated = test_eval(input);

        test_integer_object(evaluated, expected)
    }
}

#[test]
fn test_hash_key() {
    let hello1 = StringObject {
        value: "Hello".to_string(),
    };
    let hello2 = StringObject {
        value: "Hello".to_string(),
    };

    let diff = StringObject {
        value: "Hello 2".to_string(),
    };

    let mut hasher1 = DefaultHasher::new();
    let mut hasher2 = DefaultHasher::new();
    let mut hasher3 = DefaultHasher::new();
    hello1.hash(&mut hasher1);
    hello2.hash(&mut hasher2);
    diff.hash(&mut hasher3);

    assert_eq!(hasher1.finish(), hasher2.finish());
    assert_ne!(hasher1.finish(), hasher3.finish());
}

#[test]
fn test_hash_literals() {
    let input = r#"let two = "two";
    {
    "one": 10 - 9,
    two: 1 + 1,
    "thr" + "ee": 6 / 2,
    4: 4,
    true: 5,
    false: 6,
    }"#;

    dbg!(&input);

    let mut dictionary = HashMap::new();

    dictionary.insert(
        ObjectType::String(StringObject {
            value: "one".to_string(),
        }),
        1,
    );
    dictionary.insert(
        ObjectType::String(StringObject {
            value: "two".to_string(),
        }),
        2,
    );
    dictionary.insert(
        ObjectType::String(StringObject {
            value: "three".to_string(),
        }),
        3,
    );
    dictionary.insert(ObjectType::Integer(Integer { value: 4 }), 4);
    dictionary.insert(ObjectType::Boolean(Boolean { value: true }), 5);
    dictionary.insert(ObjectType::Boolean(Boolean { value: false }), 6);

    let evaluated = test_eval(input);

    let result = match evaluated {
        ObjectType::Dict(dic) => dic,
        other => panic!("expected dict, found {:?}", other),
    };

    assert_eq!(result.pairs.len(), dictionary.len());

    for (expkey, expval) in dictionary.iter() {
        let val = result.pairs.get(expkey).unwrap();

        test_integer_object(val.clone(), *expval);
    }
}

#[test]
fn test_hash_index_expression() {
    let tests = vec![
        (r#"{"foo": 5}["foo"]"#, 5),
        (r#"{"foo": 5}["bar"]"#, -1),
        (r#"{}["bar"]"#, -1),
        (r#"let key = "foo"; {"foo": 5}[key]"#, 5),
        (r#"{5: 5}[5]"#, 5),
        (r#"{true: 5}[true]"#, 5),
        (r#"{false: 5}[false]"#, 5),
    ];

    for (input, expected) in tests {
        dbg!(&input);
        dbg!(&expected);
        let evaluated = test_eval(input);

        if expected == -1 {
            test_null_object(evaluated)
        } else {
            test_integer_object(evaluated, expected)
        }
    }
}

//-------------------[Test helpers]-------------------//

fn test_null_object(evaluated: ObjectType) {
    match evaluated {
        ObjectType::Null => (),
        actual => panic!("Expected an null, got {:?}", actual),
    };
}

fn test_integer_object(evaluated: ObjectType, expected: i128) {
    let eval = match evaluated {
        ObjectType::Integer(exp) => exp,
        actual => panic!("Expected an integer, got {:?}", actual),
    };
    assert_eq!(eval.value, expected)
}

fn test_boolean_object(evaluated: ObjectType, expected: bool) {
    let eval = match evaluated {
        ObjectType::Boolean(exp) => exp,
        actual => panic!("Expected an boolean, got {:?}", actual),
    };
    assert_eq!(eval.value, expected)
}

fn test_string_object(evaluated: ObjectType, expected: String) {
    let eval = match evaluated {
        ObjectType::String(exp) => exp,
        actual => panic!("Expected a string, got {:?}", actual),
    };
    assert_eq!(eval.value, expected)
}

fn test_error_object(evaluated: ObjectType, expected: &str) {
    let eval = match evaluated {
        ObjectType::Error(exp) => exp,
        actual => panic!("Expected an error, got {:?}", actual),
    };
    assert_eq!(eval.message, expected)
}

fn test_eval(input: &str) -> ObjectType {
    let mut par = Parser::new(Lexer::new(input.to_string()));
    let pro = par.parse_program();

    let mut evaluator = Evaluator::new_env(Environment::new());

    evaluator.eval(pro)
}
