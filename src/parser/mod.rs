use crate::ast::expressions::{Boolean, InfixExpression, IntegerLiteral, PrefixExpression};
use crate::ast::statements::ExpressionStatement;
use crate::ast::{
    expressions::ExpressionNode, expressions::Identifier, statements::LetStatement,
    statements::ReturnStatement, statements::StatementNode, Program,
};
use crate::lexer::Lexer;
use crate::parser::precedence::Precedence;
use crate::token::Token;

#[cfg(test)]
mod test;

mod precedence;

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
            _ => Ok(StatementNode::ExpressionStatement(
                self.parse_expression_statement()?,
            )),
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

    fn parse_expression_statement(&mut self) -> Result<ExpressionStatement, MyParseError> {
        let expression = ExpressionStatement {
            token: self.current_token.clone(),
            expression: self.parse_expression(Precedence::Lowest)?,
        };

        if self.peek_token_is(Token::Semicolon) {
            self.next_token();
        }

        Ok(expression)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<ExpressionNode, MyParseError> {
        let mut left_expression = self.execute_prefix_parse_function(self.current_token.clone())?;

        while !self.peek_token_is(Token::Semicolon) && precedence < self.peek_precedence() {
            let peek = self.peek_token.clone();
            self.next_token();
            left_expression = self.execute_infix_parse_function(peek, left_expression)?;
        }

        Ok(left_expression)
    }

    fn execute_prefix_parse_function(
        &mut self,
        tok: Token,
    ) -> Result<ExpressionNode, MyParseError> {
        match tok {
            Token::Id(id) => Ok(self.parse_identifier(id.to_string())?),
            Token::ConstInt(num) => Ok(self.parse_integer_literal(num.to_string())?),
            Token::Not | Token::Minus => Ok(self.parse_prefix_expression()?),
            Token::ConstBool(b) => Ok(self.parse_boolean(b)?),
            Token::Opar => Ok(self.parse_grouped_expression()?),
            _ => {
                self.no_prefix_fn_error(self.current_token.clone());
                Err(MyParseError)
            }
        }
    }

    fn execute_infix_parse_function(
        &mut self,
        tok: Token,
        left: ExpressionNode,
    ) -> Result<ExpressionNode, MyParseError> {
        match tok {
            Token::Eq
            | Token::Neq
            | Token::Lt
            | Token::Gt
            | Token::Plus
            | Token::Minus
            | Token::Div
            | Token::Mult => Ok(self.parse_infix_expression(left)?),
            _ => {
                self.no_infix_fn_error(self.current_token.clone());
                Err(MyParseError)
            }
        }
    }

    fn parse_identifier(&self, id: String) -> Result<ExpressionNode, MyParseError> {
        Ok(ExpressionNode::Identifier(Identifier {
            token: Token::Id(id.to_string()),
        }))
    }

    fn parse_integer_literal(&self, num: String) -> Result<ExpressionNode, MyParseError> {
        Ok(ExpressionNode::IntegerLiteral(IntegerLiteral {
            token: Token::ConstInt(num.to_string()),
        }))
    }

    fn parse_boolean(&self, b: String) -> Result<ExpressionNode, MyParseError> {
        Ok(ExpressionNode::Boolean(Boolean {
            token: Token::ConstBool(b.to_string()),
        }))
    }

    fn parse_grouped_expression(&mut self) -> Result<ExpressionNode, MyParseError> {
        self.next_token();

        let exp = self.parse_expression(Precedence::Lowest)?;

        self.expect_peek(Token::Cpar);

        Ok(exp)
    }

    fn parse_prefix_expression(&mut self) -> Result<ExpressionNode, MyParseError> {
        let tok = self.current_token.clone();

        self.next_token();

        Ok(ExpressionNode::PrefixExpression(PrefixExpression {
            token: tok.clone(),
            operator: tok.to_string(),
            right: Box::new(self.parse_expression(Precedence::Prefix)?),
        }))
    }

    fn parse_infix_expression(
        &mut self,
        left: ExpressionNode,
    ) -> Result<ExpressionNode, MyParseError> {
        let tok = self.current_token.clone();

        let precedence = self.current_precedence();
        self.next_token();

        Ok(ExpressionNode::InfixExpression(InfixExpression {
            token: tok.clone(),
            left: Box::new(left),
            operator: tok.to_string(),
            right: Box::new(self.parse_expression(precedence)?),
        }))
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

    fn no_prefix_fn_error(&mut self, tok: Token) {
        let msg = format!("No prefix function for {:?} found", &tok);
        self.errors.push(msg);
    }

    fn no_infix_fn_error(&mut self, tok: Token) {
        let msg = format!("No infix function for {:?} found", &tok);
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

    fn peek_precedence(&self) -> Precedence {
        self.get_precedence_from_token(&self.peek_token)
    }

    fn current_precedence(&self) -> Precedence {
        self.get_precedence_from_token(&self.current_token)
    }

    fn get_precedence_from_token(&self, tok: &Token) -> Precedence {
        match tok {
            Token::Eq => Precedence::Equals,
            Token::Neq => Precedence::Equals,
            Token::Lt => Precedence::LessGreater,
            Token::Gt => Precedence::LessGreater,
            Token::Plus => Precedence::Sum,
            Token::Minus => Precedence::Sum,
            Token::Div => Precedence::Product,
            Token::Mult => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }
}

#[derive(Debug)]
struct MyParseError;
