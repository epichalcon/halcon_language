use std::collections::HashMap;

use crate::ast::{statements::LetStatement, Node};

use super::*;

#[test]
fn test_parse_let_statement() {
    let tests = vec![
        ("let x = 5;", "x", "5"),
        ("let y = true;", "y", "true"),
        ("let foobar = y;", "foobar", "y"),
    ];

    for (input, ident, equals) in tests {
        let mut par = Parser::new(Lexer::new(input.to_string()));
        let parse_program = par.parse_program();
        let pro = get_program(&parse_program);
        check_parse_errors(par);
        assert_eq!(1, pro.statements.len());

        test_let_statement(ident, &pro.statements[0]);

        let statement = match &pro.statements[0] {
            AstNode::LetStatement(exp) => exp,
            actual => panic!("Expected an expression statement, got {:?}", actual),
        };

        test_literal_expression(&statement.value, equals)
    }
}

#[test]
fn test_parse_return_statement() {
    let input = "return 3;
                    return 5;
                    return 2323124;";

    let mut par = Parser::new(Lexer::new(input.to_string()));
    let binding = par.parse_program();
    let pro = get_program(&binding);
    check_parse_errors(par);
    assert_eq!(3, pro.statements.len());

    for statement in &pro.statements {
        match statement {
            AstNode::ReturnStatement(_) => (),
            _ => panic!(),
        }
    }
}

#[test]
fn test_string() {
    let p = Program {
        statements: vec![
            AstNode::LetStatement(LetStatement {
                token: Token::Let,
                name: Identifier {
                    token: Token::Id("myVar".to_string()),
                },
                value: Box::new(AstNode::Identifier(Identifier {
                    token: Token::Id("otherVar".to_string()),
                })),
            }),
            AstNode::ReturnStatement(ReturnStatement {
                token: Token::Return,
                return_value: Box::new(AstNode::Identifier(Identifier {
                    token: Token::Id("myVar".to_string()),
                })),
            }),
        ],
    };

    assert_eq!("let myVar = otherVar;return myVar;", p.string());
}

#[test]
fn test_identifier_expression() {
    let input = "foobar;";

    let lex = Lexer::new(input.to_string());

    let mut par = Parser::new(lex);
    let program = &par.parse_program();
    let program = get_program(program);
    check_parse_errors(par);

    assert_eq!(1, program.statements.len());

    test_identifier(&program.statements[0], "foobar");
}

#[test]
fn test_integer_literal_expression() {
    let input = "5;";

    let lex = Lexer::new(input.to_string());

    let mut par = Parser::new(lex);
    let parse_program = par.parse_program();
    let program = get_program(&parse_program);
    check_parse_errors(par);

    assert_eq!(1, program.statements.len());

    let exp = &program.statements[0];

    test_int_literal(&exp, "5");
}

#[test]
fn test_boolean_literal_expression() {
    let tests = vec![("true;", "true"), ("false;", "false")];

    for (input, expected) in tests {
        let lex = Lexer::new(input.to_string());

        let mut par = Parser::new(lex);
        let parse_program = par.parse_program();
        let program = get_program(&parse_program);
        check_parse_errors(par);

        assert_eq!(1, program.statements.len());

        let exp = &program.statements[0];

        test_boolean(&exp, expected);
    }
}

#[test]
fn test_parse_prefix_expressions() {
    let tests = vec![
        ("not 5;", "not", "5"),
        ("-15;", "-", "15"),
        ("not true", "not", "true"),
    ];

    for (input, operator, right_expected) in tests {
        let lex = Lexer::new(input.to_string());

        let mut par = Parser::new(lex);
        let parse_program = par.parse_program();
        let program = get_program(&parse_program);

        assert_eq!(1, program.statements.len());

        let exp = &program.statements[0];

        test_prefix_expression(&exp, operator, right_expected);
    }
}

#[test]
fn test_parse_infix_expressions() {
    let tests = vec![
        ("5 + 5;", "5", "+", "5"),
        ("5-5;", "5", "-", "5"),
        ("5*5;", "5", "*", "5"),
        ("5/5;", "5", "/", "5"),
        ("5%5;", "5", "%", "5"),
        ("5<5;", "5", "<", "5"),
        ("5>5;", "5", ">", "5"),
        ("5>=5;", "5", ">=", "5"),
        ("5<=5;", "5", "<=", "5"),
        ("5==5;", "5", "==", "5"),
        ("5!=5;", "5", "!=", "5"),
        ("true == true;", "true", "==", "true"),
        ("true != false;", "true", "!=", "false"),
        ("false == false;", "false", "==", "false"),
    ];

    for (input, left_expected, operator, right_expected) in tests {
        let lex = Lexer::new(input.to_string());

        let mut par = Parser::new(lex);
        let parse_program = par.parse_program();
        let program = get_program(&parse_program);
        check_parse_errors(par);

        assert_eq!(1, program.statements.len());

        let exp = &program.statements[0];

        test_infix_expression(&exp, left_expected, operator, right_expected);
    }
}

#[test]
fn test_operator_precedence_parsing() {
    let tests = vec![
        ("-a * b", "((-a) * b)"),
        ("not -a", "(not(-a))"),
        ("a + b + c", "((a + b) + c)"),
        ("a * b * c", "((a * b) * c)"),
        ("a + b - c", "((a + b) - c)"),
        ("a * b / c", "((a * b) / c)"),
        ("a + b / c", "(a + (b / c))"),
        ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
        ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
        ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
        ("true", "true"),
        ("false", "false"),
        ("5 < 4 != true", "((5 < 4) != true)"),
        ("5 < 4 == false", "((5 < 4) == false)"),
        ("a + (b + c) + d", "((a + (b + c)) + d)"),
        ("(4 + 4) * 3", "((4 + 4) * 3)"),
        ("2 / (3 + 4)", "(2 / (3 + 4))"),
        ("-(5 + 5)", "(-(5 + 5))"),
        ("not(true == true)", "(not(true == true))"),
        ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
        (
            "add(a, b, 1, 2 * 3, 4 + 5, add(6, 5 * 3))",
            "add(a, b, 1, (2 * 3), (4 + 5), add(6, (5 * 3)))",
        ),
        (
            "add(a + b + c * d / f + g)",
            "add((((a + b) + ((c * d) / f)) + g))",
        ),
        (
            "a * [1, 2, 3, 4][b * c] * d",
            "((a * ([1, 2, 3, 4][(b * c)])) * d)",
        ),
        (
            "add(a * b[2], b[1], 2 * [1, 2][1])",
            "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))",
        ),
    ];

    for (input, expected) in tests {
        let lex = Lexer::new(input.to_string());
        let mut parser = Parser::new(lex);
        let program = parser.parse_program();

        check_parse_errors(parser);

        assert_eq!(expected, program.string());
    }
}

#[test]
fn test_assignation() {
    let tests = vec![
        ("a = 1;", "a", "1", Operation::Assig),
        ("a += 1;", "a", "1", Operation::Sum),
        ("a -= 1;", "a", "1", Operation::Minus),
        ("a *= 1;", "a", "1", Operation::Mult),
        ("a /= 1;", "a", "1", Operation::Divide),
    ];

    for (input, name, value, operation) in tests {
        let lex = Lexer::new(input.to_string());
        let mut parser = Parser::new(lex);
        let parse_program = parser.parse_program();
        let program = get_program(&parse_program);

        check_parse_errors(parser);
        assert_eq!(1, program.statements.len());

        let exp = &program.statements[0];

        let assignation = match exp.clone() {
            AstNode::Assignation(assig) => assig,
            actual => panic!("Expected an assignation statement, got {:?}", actual),
        };

        assert_eq!(&assignation.name.string(), name);
        assert_eq!(assignation.operation, operation);
        test_literal_expression(&assignation.value, value);
    }
}


#[test]
fn test_post_increment() {
    let input = "a++;";

    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let parse_program = parser.parse_program();
    let program = get_program(&parse_program);

    check_parse_errors(parser);
    assert_eq!(1, program.statements.len());

    let exp = &program.statements[0];

    let post_inc = match exp.clone() {
        AstNode::PostIncrement(assig) => assig,
        actual => panic!("Expected an post increment expression, got {:?}", actual),
    };

    assert_eq!(&post_inc.string(), "a");
}


#[test]
fn test_post_decrement() {
    let input = "a--;";

    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let parse_program = parser.parse_program();
    let program = get_program(&parse_program);

    check_parse_errors(parser);
    assert_eq!(1, program.statements.len());

    let exp = &program.statements[0];

    let post_dec = match exp.clone() {
        AstNode::PostDecrement(assig) => assig,
        actual => panic!("Expected an post decrement expression, got {:?}", actual),
    };

    assert_eq!(&post_dec.string(), "a");
}

#[test]
fn test_if_expression() {
    let input = "if (x < y) { x }";

    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let parse_program = parser.parse_program();
    let program = get_program(&parse_program);

    check_parse_errors(parser);
    assert_eq!(1, program.statements.len());

    let exp = &program.statements[0];

    let if_expression = match exp.clone() {
        AstNode::IfExpression(if_expression) => if_expression,
        actual => panic!("Expected an if expression, got {:?}", actual),
    };

    test_infix_expression(&if_expression.condition, "x", "<", "y");

    assert_eq!(1, if_expression.consequence.statements.len());

    let consequence = &if_expression.consequence.statements[0];

    test_identifier(&consequence, "x")
}

#[test]
fn test_if_else_expression() {
    let input = "if (x < y) { x } else { y }";

    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let parse_program = parser.parse_program();
    let program = get_program(&parse_program);

    check_parse_errors(parser);
    assert_eq!(1, program.statements.len());

    let exp = &program.statements[0];

    let if_expression = match exp.clone() {
        AstNode::IfExpression(if_expression) => if_expression,
        actual => panic!("Expected an if expression, got {:?}", actual),
    };

    test_infix_expression(&if_expression.condition, "x", "<", "y");

    assert_eq!(1, if_expression.consequence.statements.len());

    let consequence = &if_expression.consequence.statements[0];

    test_identifier(&consequence, "x");

    let alternative_block = match &if_expression.alternative {
        Some(alt) => alt,
        None => panic!("expected an alternative"),
    };

    assert_eq!(1, alternative_block.statements.len());

    let alternative = &alternative_block.statements[0];

    test_identifier(&alternative, "y")
}


#[test]
fn test_if_elif_else_expression() {
    let input = "if (x < y) { x } elif (x == y) { x } else { y }";

    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let parse_program = parser.parse_program();
    let program = get_program(&parse_program);

    check_parse_errors(parser);
    assert_eq!(1, program.statements.len());

    let exp = &program.statements[0];

    let if_expression = match exp.clone() {
        AstNode::IfExpression(if_expression) => if_expression,
        actual => panic!("Expected an if expression, got {:?}", actual),
    };

    test_infix_expression(&if_expression.condition, "x", "<", "y");

    assert_eq!(1, if_expression.consequence.statements.len());

    let consequence = &if_expression.consequence.statements[0];

    test_identifier(&consequence, "x");


    let elifs =  &if_expression.elifs;

    assert_eq!(1, elifs.len());

    let (elif_cond, elif_cons) = elifs[0].clone();

    test_infix_expression(&elif_cond, "x", "==", "y");

    assert_eq!(1, elif_cons.statements.len());

    let consequence = &elif_cons.statements[0];

    test_identifier(&consequence, "x");

    let alternative_block = match &if_expression.alternative {
        Some(alt) => alt,
        None => panic!("expected an alternative"),
    };

    assert_eq!(1, alternative_block.statements.len());

    let alternative = &alternative_block.statements[0];

    test_identifier(&alternative, "y")
}

#[test]
fn test_function_literal_parsing() {
    let input = "fun (x, y) {x + y}";

    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let binding = parser.parse_program();
    let program = get_program(&binding);

    check_parse_errors(parser);
    assert_eq!(1, program.statements.len());

    let exp = &program.statements[0];

    let function = match exp.clone() {
        AstNode::FunctionLiteral(if_expression) => if_expression,
        actual => panic!("Expected a function literal, got {:?}", actual),
    };

    assert_eq!(2, function.parameters.len());

    assert_eq!("x", function.parameters[0].token_literal());
    assert_eq!("y", function.parameters[1].token_literal());

    assert_eq!(1, function.body.statements.len());

    let exp = &function.body.statements[0];

    test_infix_expression(&exp, "x", "+", "y");
}

#[test]
fn test_function_parameter_parsing() {
    let tests = vec![
        ("fun () {}", vec![]),
        ("fun (x) {}", vec!["x"]),
        ("fun (x,y) {}", vec!["x", "y"]),
    ];

    for (input, expected) in tests {
        let lex = Lexer::new(input.to_string());
        let mut parser = Parser::new(lex);
        let parse_program = parser.parse_program();
        let program = get_program(&parse_program);

        check_parse_errors(parser);

        let exp = &program.statements[0];

        let function = match exp {
            AstNode::FunctionLiteral(if_expression) => if_expression,
            actual => panic!("Expected a function literal, got {:?}", actual),
        };

        assert_eq!(expected.len(), function.parameters.len());

        for (i, ident) in expected.iter().enumerate() {
            assert_eq!(ident.to_string(), function.parameters[i].token_literal());
        }
    }
}

#[test]
fn test_call_expresion_parsing() {
    let input = "add(1, 2 + 3, 4 * 5)";

    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let binding = parser.parse_program();
    let program = get_program(&binding);

    check_parse_errors(parser);
    assert_eq!(1, program.statements.len());

    let exp = &program.statements[0];

    let exp = match exp.clone() {
        AstNode::CallExpression(if_expression) => if_expression,
        actual => panic!("Expected a call expression, got {:?}", actual),
    };

    assert_eq!("add", exp.function.string());
    assert_eq!(3, exp.arguments.len());

    test_literal_expression(&exp.arguments[0], "1");
    test_infix_expression(&exp.arguments[1], "2", "+", "3");
    test_infix_expression(&exp.arguments[2], "4", "*", "5");
}

#[test]
fn test_string_literal_expression() {
    let input = r#""hello world""#;
    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let binding = parser.parse_program();
    let program = get_program(&binding);

    check_parse_errors(parser);
    assert_eq!(1, program.statements.len());

    let exp = &program.statements[0];

    test_string_literal(&exp, "hello world")
}

#[test]
fn test_parse_array() {
    let input = "[1, 2 * 2, 3 + 3]";

    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let binding = parser.parse_program();
    let program = get_program(&binding);

    check_parse_errors(parser);
    assert_eq!(1, program.statements.len());

    let exp = &program.statements[0];

    let elem = test_array(&exp, 3);

    test_int_literal(&elem[0], "1");
    test_infix_expression(&elem[1], "2", "*", "2");
    test_infix_expression(&elem[2], "3", "+", "3");
}

#[test]
fn test_parsing_index_expreson() {
    let input = "myArray[1 + 1]";

    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let binding = parser.parse_program();
    let program = get_program(&binding);

    check_parse_errors(parser);
    assert_eq!(1, program.statements.len());
    let exp = &program.statements[0];

    let index_expression = match exp.clone() {
        AstNode::IndexExpression(if_expression) => if_expression,
        actual => panic!("Expected an index expression, got {:?}", actual),
    };

    test_identifier(&index_expression.left, "myArray");
    test_infix_expression(&index_expression.index, "1", "+", "1");
}

#[test]
fn test_parsing_hash_literals_string_keys() {
    let input = r#"{"one": 1, "two": 2, "three": 3}"#;

    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let binding = parser.parse_program();
    let program = get_program(&binding);

    check_parse_errors(parser);
    assert_eq!(1, program.statements.len());
    let exp = &program.statements[0];

    let dictionary_expression = match exp.clone() {
        AstNode::DictLiteral(if_expression) => if_expression,
        actual => panic!("Expected an index expression, got {:?}", actual),
    };

    assert_eq!(3, dictionary_expression.pairs.len());

    let mut expected: HashMap<String, i128> = HashMap::new();
    expected.insert("one".to_string(), 1);
    expected.insert("two".to_string(), 2);
    expected.insert("three".to_string(), 3);

    for (key, val) in dictionary_expression.pairs.iter() {
        let literal = match key {
            AstNode::StringLiteral(id) => id.string(),
            actual => panic!("Expected a string literal expression, got {:?}", actual),
        };

        let expected_value = expected.get(&literal).unwrap();

        test_int_literal(val, &expected_value.to_string());
    }
}

#[test]
fn test_parsing_empty_dict() {
    let input = r#"{}"#;

    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let binding = parser.parse_program();
    let program = get_program(&binding);

    check_parse_errors(parser);
    assert_eq!(1, program.statements.len());
    let exp = &program.statements[0];

    let dictionary_expression = match exp.clone() {
        AstNode::DictLiteral(if_expression) => if_expression,
        actual => panic!("Expected an index expression, got {:?}", actual),
    };

    assert_eq!(0, dictionary_expression.pairs.len());
}

#[test]
fn test_parsing_expression_dict() {
    let input = r#"{"one": 0 + 1, "two": 10 - 8, "three": 15 / 5}"#;

    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let binding = parser.parse_program();
    let program = get_program(&binding);

    check_parse_errors(parser);
    assert_eq!(1, program.statements.len());
    let exp = &program.statements[0];

    let dictionary_expression = match exp.clone() {
        AstNode::DictLiteral(if_expression) => if_expression,
        actual => panic!("Expected an index expression, got {:?}", actual),
    };

    assert_eq!(3, dictionary_expression.pairs.len());

    let mut expected: HashMap<String, fn(AstNode)> = HashMap::new();
    expected.insert("one".to_string(), |e| {
        test_infix_expression(&e, "0", "+", "1")
    });
    expected.insert("two".to_string(), |e| {
        test_infix_expression(&e, "10", "-", "8")
    });
    expected.insert("three".to_string(), |e| {
        test_infix_expression(&e, "15", "/", "5")
    });

    for (key, val) in dictionary_expression.pairs.iter() {
        let literal = match key {
            AstNode::StringLiteral(id) => id.string(),
            actual => panic!("Expected a string literal expression, got {:?}", actual),
        };

        let test_funct = expected.get(&literal).unwrap();

        test_funct(val.clone())
    }
}

//-------------------[Test helpers]-------------------//

fn test_array(array: &AstNode, expected_len: usize) -> Vec<AstNode> {
    match array {
        AstNode::ArrayLiteral(arr) => {
            assert_eq!(expected_len, arr.elements.len());
            arr.elements.clone()
        }
        actual => panic!("Expected an identifier expression, got {:?}", actual),
    }
}

fn get_program(program: &AstNode) -> &Program {
    match program {
        AstNode::Program(exp) => exp,
        actual => panic!("Expected an expression statement, got {:?}", actual),
    }
}

fn test_identifier(exp: &AstNode, expected: &str) {
    match exp {
        AstNode::Identifier(id) => assert_eq!(expected, id.token_literal()),
        actual => panic!("Expected an identifier expression, got {:?}", actual),
    }
}

fn test_int_literal(exp: &AstNode, expected: &str) {
    match exp {
        AstNode::IntegerLiteral(id) => assert_eq!(expected, id.token_literal()),
        actual => panic!("Expected an integer literal, got {:?}", actual),
    }
}

fn test_boolean(exp: &AstNode, expected: &str) {
    match exp {
        AstNode::Boolean(id) => assert_eq!(expected, id.token_literal()),
        actual => panic!("Expected a boolean expression, got {:?}", actual),
    }
}

fn test_string_literal(exp: &AstNode, expected: &str) {
    match exp {
        AstNode::StringLiteral(id) => assert_eq!(expected, id.token_literal()),
        actual => panic!("Expected a string literal expression, got {:?}", actual),
    }
}

fn test_let_statement(test: &str, statement: &AstNode) {
    match &statement {
        AstNode::LetStatement(st) => assert_eq!(test, st.name.token_literal()),
        actual => panic!("Expected a let statement, got {:?}", actual),
    }
}

fn test_literal_expression(exp: &AstNode, expected: &str) {
    if expected.chars().next().unwrap().is_digit(10) {
        test_int_literal(exp, expected);
    } else if expected == "true" || expected == "false" {
        test_boolean(exp, expected);
    } else {
        test_identifier(exp, expected);
    }
}

fn test_prefix_expression(expression: &AstNode, operator: &str, right_expected: &str) {
    let op_expr = match expression {
        AstNode::PrefixExpression(exp) => exp,
        actual => panic!("Expected a prefix expression, got {:?}", actual),
    };

    assert_eq!(op_expr.operator, operator);

    test_literal_expression(&op_expr.right, right_expected);
}

fn test_infix_expression(
    expression: &AstNode,
    left_expected: &str,
    operator: &str,
    right_expected: &str,
) {
    let op_expr = match expression {
        AstNode::InfixExpression(exp) => exp,
        actual => panic!("Expected an infix expression, got {:?}", actual),
    };

    test_literal_expression(&op_expr.left, left_expected);

    assert_eq!(op_expr.operator, operator);

    test_literal_expression(&op_expr.right, right_expected);
}

fn check_parse_errors(par: Parser) {
    let errors = par.errors();
    if errors.len() == 0 {
        return;
    }

    dbg!(errors.len());
    dbg!("---------------------");
    for error in errors {
        dbg!(error);
    }
    panic!()
}
