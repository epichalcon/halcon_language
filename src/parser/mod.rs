use std::collections::HashMap;

use crate::ast::expressions::{
    ArrayLiteral, Boolean, CallExpression, DictLiteral, FunctionLiteral, IfExpression,
    IndexExpression, InfixExpression, IntegerLiteral, PrefixExpression, StringLiteral,
};
use crate::ast::statements::BlockStatement;
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

/// The parser struct is responsable for parsing the tokens obtained from the `Lexer`
pub struct Parser {
    /// The Lexer that will provide the Tokens
    lex: Lexer,

    current_token: Token,
    peek_token: Token,

    /// Stores a list of the errors encountered
    errors: Vec<String>,
}

impl Parser {
    /**
    Returns a Parser with the lexer provided

    # Arguments

    * `lex` - the `Lexer` that will be parsed

    # Examples
    ```
    let lex = Lexer::new(content);
    let mut pars = Parser::new(lex);
    ```
    */
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

    /**
    Returns the list of Errors encountered as a `Vec<String>`

    # Arguments

    no arguments

    # Examples
    ```
    let lex = Lexer::new(content);
    let mut pars = Parser::new(lex);

    if pars.errors().len() != 0 {
        println!("Errors have been found: \n{:?}", pars.errors());
        return;
    }
    ```
    */
    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    /**
    Updates the `self.current_token` and `self.peek_token` moving both to the next token in the list

    # Arguments

    no arguments
    */
    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lex.next_token();
    }

    /**
    The main function of the struct. Returns an AstNode::Program() which will contain the Abstract Syntax Tree of the program

    # Arguments

    no arguments

    # Example
    ```
    let lex = Lexer::new(contents);
    let mut pars = Parser::new(lex);

    let program = pars.parse_program();
    ```
    */
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

    /**
    Parses a statement and returns an `AstNode` containing it
    Valid statements are the Let statement and the Return statement

    # Arguments

    no arguments

    # Example
    ```
    match self.parse_statement() {
        Ok(statement) => program.statements.push(statement),
        Err(_) => (),
    };
    self.next_token();
    ```
    */
    fn parse_statement(&mut self) -> Result<AstNode, MyParseError> {
        match self.current_token {
            Token::Let => Ok(AstNode::LetStatement(self.parse_let_statement()?)),
            Token::Return => Ok(AstNode::ReturnStatement(self.parse_return_statement()?)),
            _ => {
                let res = Ok(self.parse_expression(Precedence::Lowest)?);
                if self.peek_token_is(Token::Semicolon) {
                    self.next_token();
                };
                res
            }
        }
    }

    /**
    Parses the let statement and returns a `LetStatement` struct containing the information
    the let statement is parsed as the following
    'let x = 1;'

    # Arguments

    no arguments

    */
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

    /**
    Parses the return statement and returns a `Return` struct containing the information
    the return statement is parsed as the following
    'return x;'

    # Arguments

    no arguments

    */
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

    /**
    Parses an expression and returns an `AstNode` containing it. Anything that is not a statement is an expression

    # Arguments

    * `precedence` - the active precedence
    */
    fn parse_expression(&mut self, precedence: Precedence) -> Result<AstNode, MyParseError> {
        let mut left_expression = self.execute_prefix_parse_function()?;

        while !self.peek_token_is(Token::Semicolon) && precedence < self.peek_precedence() {
            self.next_token();
            left_expression = self.execute_infix_parse_function(left_expression)?;
        }

        Ok(left_expression)
    }

    /**
    Calls the corresponding parser for each prefix expressions and returns the corresponding`AstNode`. These include:
    * Ids
    * Numbers
    * Booleans
    * If else statement
    * Grouped expressions
    * function literals
    * Strings
    * Array literals
    * Dict literals

    # Arguments

    no arguments
    */
    fn execute_prefix_parse_function(&mut self) -> Result<AstNode, MyParseError> {
        match &self.current_token {
            Token::Id(id) => Ok(self.parse_identifier(id.to_string())?),
            Token::ConstInt(num) => Ok(self.parse_integer_literal(*num)?),
            Token::Not | Token::Minus => Ok(self.parse_prefix_expression()?),
            Token::ConstBool(b) => Ok(self.parse_boolean(*b)?),
            Token::Opar => Ok(self.parse_grouped_expression()?),
            Token::If => Ok(self.parse_if_expression()?),
            Token::Fun => Ok(self.parse_function_literal()?),
            Token::ConstStr(s) => Ok(self.parse_string_literal(s.to_string())?),
            Token::Obrac => Ok(self.parse_array_literal()?),
            Token::Okey => Ok(self.parse_dict_literal()?),
            _ => {
                self.no_prefix_function_error(self.current_token.clone());
                Err(MyParseError)
            }
        }
    }

    /**
    Parses prefix operators and returns the corresponding`AstNode`. These include:
    * not
    * minus

    # Arguments

    no arguments
    */
    fn parse_prefix_expression(&mut self) -> Result<AstNode, MyParseError> {
        let tok = self.current_token.clone();

        self.next_token();

        Ok(AstNode::PrefixExpression(PrefixExpression {
            token: tok.clone(),
            operator: tok.to_string(),
            right: Box::new(self.parse_expression(Precedence::Prefix)?),
        }))
    }

    /**
    Parses an identifier and returns an `AstNode::Identifier`

    # Arguments
    * `id` - The `String` containing the id to parse

    */
    fn parse_identifier(&self, id: String) -> Result<AstNode, MyParseError> {
        Ok(AstNode::Identifier(Identifier {
            token: Token::Id(id.to_string()),
        }))
    }

    /**
    Parses an integer and returns an `AstNode::IntegerLiteral`

    # Arguments
    * `num` - The `i128` containing the integer to parse

    */
    fn parse_integer_literal(&self, num: i128) -> Result<AstNode, MyParseError> {
        Ok(AstNode::IntegerLiteral(IntegerLiteral {
            token: Token::ConstInt(num),
        }))
    }

    /**
    Parses a boolean and returns an `AstNode::Boolean`

    # Arguments
    * `b` - The `bool` containing the boolean to parse

    */
    fn parse_boolean(&self, b: bool) -> Result<AstNode, MyParseError> {
        Ok(AstNode::Boolean(Boolean {
            token: Token::ConstBool(b),
        }))
    }

    /**
    Parses a string literal and returns an `AstNode::StringLiteral`

    # Arguments
    * `s` - The `String` containing the string to parse

    */
    fn parse_string_literal(&self, s: String) -> Result<AstNode, MyParseError> {
        Ok(AstNode::StringLiteral(StringLiteral {
            token: Token::ConstStr(s),
        }))
    }

    /**
    Parses a grouped expression and returns the corresponding `AstNode` containing the information
    A grouped expression is an expression surrounded by ()

    # Arguments
    no arguments

    */
    fn parse_grouped_expression(&mut self) -> Result<AstNode, MyParseError> {
        self.next_token();

        let exp = self.parse_expression(Precedence::Lowest)?;

        self.expect_peek(Token::Cpar);

        Ok(exp)
    }

    /**
    Parses an if else expressin and returns an `AstNode::IfExpression`
    An if expression is parsed as
    if (<exp>) { <exp> } else {<exp>}

    # Arguments
    no arguments
    */
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

    /**
    Parses a function literal expressin and returns an `AstNode::FunctionLiteral`
    A function literal expression is parsed as
    fn (<params>) { <exp> }

    # Arguments
    no arguments
    */
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

    /**
    Parses the parameters of a function and returns a `Vec<Identifier>`
    The params of a function are a list of coma separated identifiers

    # Arguments
    no arguments
    */
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

    /**
    Parses a block of statements and returns a `BlockStatement`
    The block statements will be a series of statements that can be found in functions or if statements

    # Arguments
    no arguments
    */
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

    /**
    Parses an array literal and returns a `AstNode::ArrayLiteral`
    An array expression is parsed as
    [<exp>, <exp>...]

    # Arguments
    no arguments
    */
    fn parse_array_literal(&mut self) -> Result<AstNode, MyParseError> {
        let tok_array = self.current_token.clone();

        let elements = self.parse_expression_list(Token::Cbrac)?;

        Ok(AstNode::ArrayLiteral(ArrayLiteral {
            token: tok_array.clone(),
            elements,
        }))
    }


    /**
    Parses an dict literal and returns a `AstNode::DictLiteral`
    An dict expression is parsed as
    {<exp>: <exp>, ...}

    # Arguments
    no arguments
    */
    fn parse_dict_literal(&mut self) -> Result<AstNode, MyParseError> {
        let dict_tock = self.current_token.clone();

        let mut pairs: HashMap<AstNode, AstNode> = HashMap::new();

        while !self.peek_token_is(Token::Ckey) {
            self.next_token();
            let key = self.parse_expression(Precedence::Lowest)?;

            self.expect_peek(Token::Colon);

            self.next_token();
            let value = self.parse_expression(Precedence::Lowest)?;

            pairs.insert(key, value);

            if !self.peek_token_is(Token::Ckey) && !self.expect_peek(Token::Coma) {
                return Err(MyParseError);
            }
        }

        self.expect_peek(Token::Ckey);

        Ok(AstNode::DictLiteral(DictLiteral {
            token: dict_tock,
            pairs,
        }))
    }


    /**
    Parses infix expressions and returns the corresponding `AstNode`. These include:
    * Boolean operators
    * Math operators
    * call expressios
    * index expressions

    # Arguments

    * `left` - the `AstNode` to the left of the operator
    */
    fn execute_infix_parse_function(&mut self, left: AstNode) -> Result<AstNode, MyParseError> {
        match &self.current_token {
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

    /**
    Parses an infix expression and returns the corresponding `AstNode`

    # Arguments
    * `left` - the expression to be indexed
    *
    */
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


    /**
    Parses an index expression and returns the corresponding `AstNode`
    Index expressions are expressions that access an index of an Array or Dict: array[i]

    # Arguments
    * `left` - the expression to be indexed
    */
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


    /**
    Parses a call expression and returns the corresponding `AstNode`
    Call expressions are expressions that call a function: function(a);

    # Arguments
    * `function` - the function `AstNode` to call
    */
    fn parse_call_expression(&mut self, function: AstNode) -> Result<AstNode, MyParseError> {
        Ok(AstNode::CallExpression(CallExpression {
            token: self.current_token.clone(),
            function: Box::new(function),
            arguments: self.parse_expression_list(Token::Cpar)?,
        }))
    }

    /**
    Parses the list of expressions in between the parenthesis of a call expression or in an Array  and returns a 'Vec<AstNode>'

    # Arguments
    * `end` - the `Token` that will end the list of expressions
    */
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

    /**
    Retruns if the peek_token is equal to the expected token.
    if it is equal, the tokens update to the next ones in the list
    if it is not equal, an error is added to the list of errors

    # Arguments
    * 'tok' - The `Token` expected
    */
    fn expect_peek(&mut self, tok: Token) -> bool {
        if self.peek_token_is(tok.clone()) {
            self.next_token();
            return true;
        }

        self.peek_error(tok);

        return false;
    }

    /**
    Adds an error to the error list specifing the expected token and the actual token

    # Arguments
    * 'tok' - The `Token` expected
    */
    fn peek_error(&mut self, tok: Token) {
        let msg = format!("expected {:?}, actual {:?}", &tok, self.peek_token);
        self.errors.push(msg);
    }


    /**
    Adds an error to the error list when the prefix expression cant be handled

    # Arguments
    * 'tok' - The `Token` that was supposed to be parsed
    */
    fn no_prefix_function_error(&mut self, tok: Token) {
        let msg = format!("No prefix function for {:?} found", &tok);
        self.errors.push(msg);
    }

    /**
    Adds an error to the error list when the infix expression cant be handled

    # Arguments
    * 'tok' - The `Token` that was supposed to be parsed
    */
    fn no_infix_fn_error(&mut self, tok: Token) {
        let msg = format!("No infix function for {:?} found", &tok);
        self.errors.push(msg);
    }


    /**
    Retruns if the current token is equal to the specified token.

    # Arguments
    * 'tok' - The `Token` expected
    */
    fn cur_token_is(&self, tok: Token) -> bool {
        self.current_token.clone() == tok
    }


    /**
    Retruns if the next token is equal to the specified token.

    # Arguments
    * 'tok' - The `Token` expected
    */
    fn peek_token_is(&self, tok: Token) -> bool {
        match (self.peek_token.clone(), tok) {
            (Token::Id(_), Token::Id(_)) => true,
            (t1, t2) => t1 == t2,
        }
    }

    /**
    Retruns the precedence of the next token

    # Arguments
    no arguments
    */
    fn peek_precedence(&self) -> Precedence {
        self.get_precedence_from_token(&self.peek_token)
    }

    /**
    Retruns the precedence of the current token

    # Arguments
    no arguments
    */
    fn current_precedence(&self) -> Precedence {
        self.get_precedence_from_token(&self.current_token)
    }

    /**
    Retruns the precedence of the specified token

    # Arguments
    * 'tok' - The `Token` expected
    */
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
