use core::panic;

use crate::{
    ast::{
        statements::{ExpressionStatement, LetStatement},
        Node,
    },
    lexer,
};

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

fn test_let_statement(test: &str, statement: &&StatementNode) {
    match statement {
        StatementNode::LetStatement(st) => assert_eq!(test, st.name.token_literal()),
        _ => panic!(),
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

    assert_eq!("let myVar = otherVar;\nreturn myVar;\n", p.string());
}

#[test]
fn test_identifier_expression() {
    let input = "foobar;";

    let lex = Lexer::new(input.to_string());

    let mut par = Parser::new(lex);
    let program = par.parse_program();
    check_parse_errors(par);

    assert_eq!(1, program.statements.len());

    let StatementNode::ExpressionStatement(exp) = &program.statements[0] else {
        panic!()
    };

    let ExpressionNode::Identifier(id) = &exp.expression else {
        panic!()
    };

    assert_eq!("foobar", id.token_literal())
}

#[test]
fn test_integer_literal_expression() {
    let input = "5;";

    let lex = Lexer::new(input.to_string());

    let mut par = Parser::new(lex);
    let program = par.parse_program();
    check_parse_errors(par);

    assert_eq!(1, program.statements.len());

    let StatementNode::ExpressionStatement(exp) = &program.statements[0] else {
        panic!()
    };

    let ExpressionNode::IntegerLiteral(id) = &exp.expression else {
        panic!()
    };

    assert_eq!("5", id.token_literal())
}

#[test]
fn test_parse_prefix_expressions() {
    let tests = vec![("not 5;", "Not", "5"), ("-15;", "Minus", "15")];

    for input in tests {
        let lex = Lexer::new(input.0.to_string());

        let mut par = Parser::new(lex);
        let program = par.parse_program();
        check_parse_errors(par);

        assert_eq!(1, program.statements.len());

        let StatementNode::ExpressionStatement(exp) = &program.statements[0] else {
            panic!(
                "Expected expression statement, got {:?}",
                program.statements[0]
            )
        };

        let ExpressionNode::PrefixExpression(pref) = &exp.expression else {
            panic!("Expected prefix expression, got {:?}", exp.expression)
        };

        assert_eq!(input.1, pref.operator);
        let ExpressionNode::IntegerLiteral(ref num) = *pref.right else {
            panic!()
        };

        assert_eq!(input.2, num.token_literal())
    }
}

#[test]
fn test_parse_infix_expressions() {
    let tests = vec![
        ("5 + 5;", "5", "Plus", "5"),
        ("5-5;", "5", "Minus", "5"),
        ("5*5;", "5", "Mult", "5"),
        ("5/5;", "5", "Div", "5"),
        ("5<5;", "5", "Lt", "5"),
        ("5>5;", "5", "Gt", "5"),
        ("5==5;", "5", "Eq", "5"),
        ("5!=5;", "5", "Neq", "5"),
    ];

    for input in tests {
        let lex = Lexer::new(input.0.to_string());

        let mut par = Parser::new(lex);
        let program = par.parse_program();
        check_parse_errors(par);

        assert_eq!(1, program.statements.len());

        let StatementNode::ExpressionStatement(exp) = &program.statements[0] else {
            panic!(
                "Expected expression statement, got {:?}",
                program.statements[0]
            )
        };

        let ExpressionNode::InfixExpression(pref) = &exp.expression else {
            panic!("Expected prefix expression, got {:?}", exp.expression)
        };

        let ExpressionNode::IntegerLiteral(ref num) = *pref.left else {
            panic!()
        };

        assert_eq!(input.1, num.token_literal());

        assert_eq!(input.2, pref.operator);

        let ExpressionNode::IntegerLiteral(ref num) = *pref.right else {
            panic!()
        };

        assert_eq!(input.3, num.token_literal())
    }
}

#[test]
fn test_operator_precedence_parsing() {
    let tests = vec![
        ("-a * b", "((Minus a) Mult b)\n"),
        ("not -a", "(Not (Minus a))\n"),
        ("a + b + c", "((a Plus b) Plus c)\n"),
        ("a * b * c", "((a Mult b) Mult c)\n"),
        ("a + b - c", "((a Plus b) Minus c)\n"),
        ("a * b / c", "((a Mult b) Div c)\n"),
        ("a + b / c", "(a Plus (b Div c))\n"),
        (
            "a + b * c + d / e - f",
            "(((a Plus (b Mult c)) Plus (d Div e)) Minus f)\n",
        ),
        ("3 + 4; -5 * 5", "(3 Plus 4)\n((Minus 5) Mult 5)\n"),
        ("5 < 4 != 3 > 4", "((5 Lt 4) Neq (3 Gt 4))\n"),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 Plus (4 Mult 5)) Eq ((3 Mult 1) Plus (4 Mult 5)))\n",
        ),
    ];

    for (input, expected) in tests {
        let lex = Lexer::new(input.to_string());
        let mut parser = Parser::new(lex);
        let program = parser.parse_program();
        check_parse_errors(parser);

        let actual = program.string();
        assert_eq!(expected, actual);
    }
}
