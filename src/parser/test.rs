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

    let exp = get_expression_statement(&program.statements[0]);

    test_identifier(&exp.expression, "foobar");
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

    let exp = get_expression_statement(&program.statements[0]);

    test_int_literal(&exp.expression, "5");
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

        let exp = get_expression_statement(&program.statements[0]);

        test_boolean(&exp.expression, expected);
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

        let exp = get_expression_statement(&program.statements[0]);

        test_prefix_expression(&exp.expression, operator, right_expected);
    }
}

#[test]
fn test_parse_infix_expressions() {
    let tests = vec![
        ("5 + 5;", "5", "+", "5"),
        ("5-5;", "5", "-", "5"),
        ("5*5;", "5", "*", "5"),
        ("5/5;", "5", "/", "5"),
        ("5<5;", "5", "<", "5"),
        ("5>5;", "5", ">", "5"),
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

        let exp = get_expression_statement(&program.statements[0]);

        test_infix_expression(&exp.expression, left_expected, operator, right_expected);
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
fn test_if_expression() {
    let input = "if (x < y) { x }";

    let lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(lex);
    let parse_program = parser.parse_program();
    let program = get_program(&parse_program);

    check_parse_errors(parser);
    assert_eq!(1, program.statements.len());

    let exp = get_expression_statement(&program.statements[0]);

    let if_expression = match *exp.expression.clone() {
        AstNode::IfExpression(if_expression) => if_expression,
        actual => panic!("Expected an if expression, got {:?}", actual),
    };

    test_infix_expression(&if_expression.condition, "x", "<", "y");

    assert_eq!(1, if_expression.consequence.statements.len());

    let consequence = get_expression_statement(&if_expression.consequence.statements[0]);

    test_identifier(&consequence.expression, "x")
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

    let exp = get_expression_statement(&program.statements[0]);

    let if_expression = match *exp.expression.clone() {
        AstNode::IfExpression(if_expression) => if_expression,
        actual => panic!("Expected an if expression, got {:?}", actual),
    };

    test_infix_expression(&if_expression.condition, "x", "<", "y");

    assert_eq!(1, if_expression.consequence.statements.len());

    let consequence = get_expression_statement(&if_expression.consequence.statements[0]);

    test_identifier(&consequence.expression, "x");

    let alternative_block = match &if_expression.alternative {
        Some(alt) => alt,
        None => panic!("expected an alternative"),
    };

    assert_eq!(1, alternative_block.statements.len());

    let alternative = get_expression_statement(&alternative_block.statements[0]);

    test_identifier(&alternative.expression, "y")
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

    let exp = get_expression_statement(&program.statements[0]);

    let function = match *exp.expression.clone() {
        AstNode::FunctionLiteral(if_expression) => if_expression,
        actual => panic!("Expected a function literal, got {:?}", actual),
    };

    assert_eq!(2, function.parameters.len());

    assert_eq!("x", function.parameters[0].token_literal());
    assert_eq!("y", function.parameters[1].token_literal());

    assert_eq!(1, function.body.statements.len());

    let exp = get_expression_statement(&function.body.statements[0]);

    test_infix_expression(&exp.expression, "x", "+", "y");
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

        let exp = get_expression_statement(&program.statements[0]);

        let function = match *exp.expression.clone() {
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

    let exp = get_expression_statement(&program.statements[0]);

    let exp = match *exp.expression.clone() {
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

    let exp = get_expression_statement(&program.statements[0]);

    test_string_literal(&exp.expression, "hello world")
}

//-------------------[Test helpers]-------------------//

fn get_program(program: &AstNode) -> &Program {
    match program {
        AstNode::Program(exp) => exp,
        actual => panic!("Expected an expression statement, got {:?}", actual),
    }
}

fn get_expression_statement(statement: &AstNode) -> &ExpressionStatement {
    match statement {
        AstNode::ExpressionStatement(exp) => exp,
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
