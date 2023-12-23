use core::panic;

use crate::ast::{statements::LetStatement, Node};

use super::*;

#[test]
fn test_parse_let_statement() {
    let input = "let x = 5;
                    let y = 10;
                    let foo = 2323124;";

    let mut par = Parser::new(Lexer::new(input.to_string()));
    let pro = par.parse_program();
    check_parse_errors(par);
    assert_eq!(3, pro.statements.len());

    let tests = vec!["x", "y", "foo"];

    for (i, test) in tests.iter().enumerate() {
        test_let_statement(test, &&pro.statements[i]);
    }
}

#[test]
fn test_parse_return_statement() {
    let input = "return 3;
                    return 5;
                    return 2323124;";

    let mut par = Parser::new(Lexer::new(input.to_string()));
    let pro = par.parse_program();
    check_parse_errors(par);
    assert_eq!(3, pro.statements.len());

    for statement in pro.statements {
        match statement {
            StatementNode::ReturnStatement(_) => (),
            _ => panic!(),
        }
    }
}

#[test]
fn test_string() {
    let p = Program {
        statements: vec![
            StatementNode::LetStatement(LetStatement {
                token: Token::Let,
                name: Identifier {
                    token: Token::Id("myVar".to_string()),
                },
                value: ExpressionNode::Identifier(Identifier {
                    token: Token::Id("otherVar".to_string()),
                }),
            }),
            StatementNode::ReturnStatement(ReturnStatement {
                token: Token::Return,
                return_value: ExpressionNode::Identifier(Identifier {
                    token: Token::Id("myVar".to_string()),
                }),
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
    let program = par.parse_program();
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
    let program = par.parse_program();
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
        let program = par.parse_program();
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
        let program = par.parse_program();
        check_parse_errors(par);

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
        let program = par.parse_program();
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
    ];

    for (input, expected) in tests {
        let lex = Lexer::new(input.to_string());
        let mut parser = Parser::new(lex);
        let program = parser.parse_program();

        check_parse_errors(parser);

        assert_eq!(expected, program.string());
    }
}

//-------------------[Test helpers]-------------------//

fn get_expression_statement(statement: &StatementNode) -> &ExpressionStatement {
    match statement {
        StatementNode::ExpressionStatement(exp) => exp,
        actual => panic!("Expected an expression statement, got {:?}", actual),
    }
}

fn test_identifier(exp: &ExpressionNode, expected: &str) {
    match exp {
        ExpressionNode::Identifier(id) => assert_eq!(expected, id.token_literal()),
        actual => panic!("Expected an identifier expression, got {:?}", actual),
    }
}

fn test_int_literal(exp: &ExpressionNode, expected: &str) {
    match exp {
        ExpressionNode::IntegerLiteral(id) => assert_eq!(expected, id.token_literal()),
        actual => panic!("Expected an identifier expression, got {:?}", actual),
    }
}

fn test_boolean(exp: &ExpressionNode, expected: &str) {
    match exp {
        ExpressionNode::Boolean(id) => assert_eq!(expected, id.token_literal()),
        actual => panic!("Expected a boolean expression, got {:?}", actual),
    }
}

fn test_let_statement(test: &str, statement: &&StatementNode) {
    match statement {
        StatementNode::LetStatement(st) => assert_eq!(test, st.name.token_literal()),
        actual => panic!("Expected a let statement, got {:?}", actual),
    }
}

fn test_literal_expression(exp: &ExpressionNode, expected: &str) {
    if expected.chars().next().unwrap().is_digit(10) {
        test_int_literal(exp, expected);
    } else if expected == "true" || expected == "false" {
        test_boolean(exp, expected);
    } else {
        test_identifier(exp, expected);
    }
}

fn test_prefix_expression(expression: &ExpressionNode, operator: &str, right_expected: &str) {
    let op_expr = match expression {
        ExpressionNode::PrefixExpression(exp) => exp,
        actual => panic!("Expected a prefix expression, got {:?}", actual),
    };

    assert_eq!(op_expr.operator, operator);

    test_literal_expression(&op_expr.right, right_expected);
}

fn test_infix_expression(
    expression: &ExpressionNode,
    left_expected: &str,
    operator: &str,
    right_expected: &str,
) {
    let op_expr = match expression {
        ExpressionNode::InfixExpression(exp) => exp,
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
