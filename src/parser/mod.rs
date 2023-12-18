use std::marker::PhantomData;
use std::ops::Mul;

use crate::ast::{
    Expression, ExpressionNode, Identifier, LetStatement, Program, ReturnStatement, Statement,
    StatementNode,
};
use crate::lexer::Lexer;
use crate::token::Token;

struct Parser {
    lex: Lexer,

    current_token: Token,
    peek_token: Token,

    errors: Vec<String>,
}

impl Parser {
    fn new(lex: Lexer) -> Parser {
        let mut p = Parser {
            lex,
            current_token: Token::Invalid(b'0'.to_string()),
            peek_token: Token::Invalid(b'0'.to_string()),
            errors: vec![],
        };

        p.next_token();
        p.next_token();

        p
    }

    fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lex.next_token();
    }

    fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: vec![] };

        while self.current_token != Token::Eof {
            match self.parse_statement() {
                Ok(statement) => program.statements.push(statement),
                Err(_) => (),
            };
            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> Result<StatementNode, MyParseError> {
        match self.current_token {
            Token::Let => Ok(StatementNode::LetStatement(self.parse_let_statement()?)),
            Token::Return => Ok(StatementNode::ReturnStatement(
                self.parse_return_statement()?,
            )),
            _ => Err(MyParseError),
        }
    }

    fn parse_let_statement(&mut self) -> Result<LetStatement, MyParseError> {
        let tok = &self.current_token.clone();

        if !self.expect_peek(Token::Id("".to_string())) {
            return Err(MyParseError);
        }

        let name = Identifier {
            token: self.current_token.clone(),
        };

        if !self.expect_peek(Token::Assig) {
            return Err(MyParseError);
        }

        while !self.cur_token_is(Token::Semicolon) {
            self.next_token();
        }

        Ok(LetStatement {
            token: tok.clone(),
            name,
            value: ExpressionNode::Identifier(Identifier {
                token: Token::Invalid("".to_string()),
            }),
        })
    }

    fn parse_return_statement(&mut self) -> Result<ReturnStatement, MyParseError> {
        let tok = &self.current_token.clone();

        while !self.cur_token_is(Token::Semicolon) {
            self.next_token();
        }

        Ok(ReturnStatement {
            token: tok.clone(),
            return_value: ExpressionNode::Identifier(Identifier {
                token: Token::Invalid("".to_string()),
            }),
        })
    }

    fn expect_peek(&mut self, tok: Token) -> bool {
        if self.peek_token_is(tok.clone()) {
            self.next_token();
            return true;
        }

        self.peek_error(tok);

        return false;
    }

    fn peek_error(&mut self, tok: Token) {
        let msg = format!("expected {:?}, actual {:?}", &tok, self.peek_token);
        self.errors.push(msg);
    }

    fn cur_token_is(&self, tok: Token) -> bool {
        match (self.current_token.clone(), tok) {
            (Token::Id(_), Token::Id(_)) => true,
            (t1, t2) => t1 == t2,
        }
    }
    fn peek_token_is(&self, tok: Token) -> bool {
        match (self.peek_token.clone(), tok) {
            (Token::Id(_), Token::Id(_)) => true,
            (t1, t2) => t1 == t2,
        }
    }
}

#[derive(Debug)]
struct MyParseError;

#[cfg(test)]
mod test {
    use core::panic;

    use crate::ast::{Identifier, LetStatement, Node};

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
}
