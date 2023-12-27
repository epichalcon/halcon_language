use crate::ast::expressions::{
    ArrayLiteral, Boolean, CallExpression, FunctionLiteral, IfExpression, IndexExpression,
    InfixExpression, IntegerLiteral, PrefixExpression, StringLiteral,
};
use crate::ast::statements::{BlockStatement, ExpressionStatement};
use crate::ast::AstNode;
use crate::ast::{
    expressions::Identifier, statements::LetStatement, statements::ReturnStatement, Program,
};
use crate::lexer::Lexer;
use crate::parser::precedence::Precedence;
use crate::token::Token;

#[cfg(test)]
mod test;

mod precedence;

pub struct Parser {
    lex: Lexer,

    current_token: Token,
    peek_token: Token,

    errors: Vec<String>,
}

impl Parser {
    pub fn new(lex: Lexer) -> Parser {
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

    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lex.next_token();
    }

    pub fn parse_program(&mut self) -> AstNode {
        let mut program = Program { statements: vec![] };

        while self.current_token != Token::Eof {
            match self.parse_statement() {
                Ok(statement) => program.statements.push(statement),
                Err(_) => (),
            };
            self.next_token();
        }

        AstNode::Program(program)
    }

    fn parse_statement(&mut self) -> Result<AstNode, MyParseError> {
        match self.current_token {
            Token::Let => Ok(AstNode::LetStatement(self.parse_let_statement()?)),
            Token::Return => Ok(AstNode::ReturnStatement(self.parse_return_statement()?)),
            _ => Ok(AstNode::ExpressionStatement(
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

        self.expect_peek(Token::Assig);

        self.next_token();

        let expression = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(Token::Semicolon) {
            self.next_token();
        }

        Ok(LetStatement {
            token: tok.clone(),
            name,
            value: Box::new(expression),
        })
    }

    fn parse_return_statement(&mut self) -> Result<ReturnStatement, MyParseError> {
        let tok = &self.current_token.clone();

        self.next_token();

        let expression = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(Token::Semicolon) {
            self.next_token();
        }

        Ok(ReturnStatement {
            token: tok.clone(),
            return_value: Box::new(expression),
        })
    }

    fn parse_expression_statement(&mut self) -> Result<ExpressionStatement, MyParseError> {
        let expression = ExpressionStatement {
            token: self.current_token.clone(),
            expression: Box::new(self.parse_expression(Precedence::Lowest)?),
        };

        if self.peek_token_is(Token::Semicolon) {
            self.next_token();
        }

        Ok(expression)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<AstNode, MyParseError> {
        let mut left_expression = self.execute_prefix_parse_function(self.current_token.clone())?;

        while !self.peek_token_is(Token::Semicolon) && precedence < self.peek_precedence() {
            let peek = self.peek_token.clone();
            self.next_token();
            left_expression = self.execute_infix_parse_function(peek, left_expression)?;
        }

        Ok(left_expression)
    }

    fn execute_prefix_parse_function(&mut self, tok: Token) -> Result<AstNode, MyParseError> {
        match tok {
            Token::Id(id) => Ok(self.parse_identifier(id.to_string())?),
            Token::ConstInt(num) => Ok(self.parse_integer_literal(num)?),
            Token::Not | Token::Minus => Ok(self.parse_prefix_expression()?),
            Token::ConstBool(b) => Ok(self.parse_boolean(b)?),
            Token::Opar => Ok(self.parse_grouped_expression()?),
            Token::If => Ok(self.parse_if_expression()?),
            Token::Fun => Ok(self.parse_function_literal()?),
            Token::ConstStr(s) => Ok(self.parse_string_literal(s)?),
            Token::Obrac => Ok(self.parse_array_literal()?),
            _ => {
                self.no_prefix_fn_error(self.current_token.clone());
                Err(MyParseError)
            }
        }
    }

    fn execute_infix_parse_function(
        &mut self,
        tok: Token,
        left: AstNode,
    ) -> Result<AstNode, MyParseError> {
        match tok {
            Token::Eq
            | Token::Neq
            | Token::Lt
            | Token::Gt
            | Token::Plus
            | Token::Minus
            | Token::Div
            | Token::Mult => Ok(self.parse_infix_expression(left)?),
            Token::Opar => Ok(self.parse_call_expression(left)?),
            Token::Obrac => Ok(self.parse_index_expression(left)?),
            _ => {
                self.no_infix_fn_error(self.current_token.clone());
                Err(MyParseError)
            }
        }
    }

    fn parse_identifier(&self, id: String) -> Result<AstNode, MyParseError> {
        Ok(AstNode::Identifier(Identifier {
            token: Token::Id(id.to_string()),
        }))
    }

    fn parse_integer_literal(&self, num: i128) -> Result<AstNode, MyParseError> {
        Ok(AstNode::IntegerLiteral(IntegerLiteral {
            token: Token::ConstInt(num),
        }))
    }

    fn parse_boolean(&self, b: bool) -> Result<AstNode, MyParseError> {
        Ok(AstNode::Boolean(Boolean {
            token: Token::ConstBool(b),
        }))
    }

    fn parse_string_literal(&self, s: String) -> Result<AstNode, MyParseError> {
        Ok(AstNode::StringLiteral(StringLiteral {
            token: Token::ConstStr(s),
        }))
    }

    fn parse_grouped_expression(&mut self) -> Result<AstNode, MyParseError> {
        self.next_token();

        let exp = self.parse_expression(Precedence::Lowest)?;

        self.expect_peek(Token::Cpar);

        Ok(exp)
    }

    fn parse_if_expression(&mut self) -> Result<AstNode, MyParseError> {
        let if_token = self.current_token.clone();

        self.expect_peek(Token::Opar);
        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;
        self.expect_peek(Token::Cpar);

        self.expect_peek(Token::Okey);

        let consequence = self.parse_block_statement()?;

        if self.peek_token_is(Token::Else) {
            self.next_token();
            self.expect_peek(Token::Okey);

            let alternative = self.parse_block_statement()?;

            Ok(AstNode::IfExpression(IfExpression {
                token: if_token,
                condition: Box::new(condition),
                consequence,
                alternative: Some(alternative),
            }))
        } else {
            Ok(AstNode::IfExpression(IfExpression {
                token: if_token,
                condition: Box::new(condition),
                consequence,
                alternative: None,
            }))
        }
    }

    fn parse_function_literal(&mut self) -> Result<AstNode, MyParseError> {
        let func_tok = self.current_token.clone();

        self.expect_peek(Token::Opar);
        let parameters = self.parse_function_parameters()?;
        self.expect_peek(Token::Okey);

        let block = self.parse_block_statement()?;

        Ok(AstNode::FunctionLiteral(FunctionLiteral {
            token: func_tok,
            parameters,
            body: block,
        }))
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<Identifier>, MyParseError> {
        let mut identifiers: Vec<Identifier> = vec![];

        if self.peek_token_is(Token::Cpar) {
            self.next_token();
            return Ok(identifiers);
        }

        self.next_token();

        let ident = Identifier {
            token: self.current_token.clone(),
        };

        identifiers.push(ident);

        while self.peek_token_is(Token::Coma) {
            self.next_token();
            self.next_token();
            let ident = Identifier {
                token: self.current_token.clone(),
            };
            identifiers.push(ident);
        }

        self.expect_peek(Token::Cpar);

        Ok(identifiers)
    }

    fn parse_block_statement(&mut self) -> Result<BlockStatement, MyParseError> {
        let block_token = self.current_token.clone();

        let mut block_statements = vec![];

        self.next_token();

        while !self.cur_token_is(Token::Ckey) && !self.cur_token_is(Token::Eof) {
            let statement = self.parse_statement()?;
            block_statements.push(statement.clone());
            self.next_token();
        }

        Ok(BlockStatement {
            token: block_token,
            statements: block_statements,
        })
    }

    fn parse_array_literal(&mut self) -> Result<AstNode, MyParseError> {
        let tok_array = self.current_token.clone();

        let elements = self.parse_expression_list(Token::Cbrac)?;

        Ok(AstNode::ArrayLiteral(ArrayLiteral {
            token: tok_array.clone(),
            elements,
        }))
    }

    fn parse_prefix_expression(&mut self) -> Result<AstNode, MyParseError> {
        let tok = self.current_token.clone();

        self.next_token();

        Ok(AstNode::PrefixExpression(PrefixExpression {
            token: tok.clone(),
            operator: tok.to_string(),
            right: Box::new(self.parse_expression(Precedence::Prefix)?),
        }))
    }

    fn parse_index_expression(&mut self, left: AstNode) -> Result<AstNode, MyParseError> {
        let in_token = self.current_token.clone();

        self.next_token();

        let index = self.parse_expression(Precedence::Lowest)?;

        self.expect_peek(Token::Cbrac);

        Ok(AstNode::IndexExpression(IndexExpression {
            token: in_token.clone(),
            left: Box::new(left),
            index: Box::new(index),
        }))
    }

    fn parse_infix_expression(&mut self, left: AstNode) -> Result<AstNode, MyParseError> {
        let tok = self.current_token.clone();

        let precedence = self.current_precedence();
        self.next_token();

        Ok(AstNode::InfixExpression(InfixExpression {
            token: tok.clone(),
            left: Box::new(left),
            operator: tok.to_string(),
            right: Box::new(self.parse_expression(precedence)?),
        }))
    }

    fn parse_call_expression(&mut self, function: AstNode) -> Result<AstNode, MyParseError> {
        Ok(AstNode::CallExpression(CallExpression {
            token: self.current_token.clone(),
            function: Box::new(function),
            arguments: self.parse_expression_list(Token::Cpar)?,
        }))
    }

    fn parse_expression_list(&mut self, end: Token) -> Result<Vec<AstNode>, MyParseError> {
        let mut args = vec![];

        if self.peek_token_is(end.clone()) {
            self.next_token();
            return Ok(args);
        }

        self.next_token();

        args.push(self.parse_expression(Precedence::Lowest)?);

        while self.peek_token_is(Token::Coma) {
            self.next_token();
            self.next_token();
            args.push(self.parse_expression(Precedence::Lowest)?);
        }

        self.expect_peek(end);

        Ok(args)
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
            Token::Opar => Precedence::Call,
            Token::Obrac => Precedence::Index,
            _ => Precedence::Lowest,
        }
    }
}

#[derive(Debug)]
struct MyParseError;
