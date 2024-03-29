use crate::ast::expressions::Identifier;
use crate::ast::Node;
use crate::token::Token;

use super::AstNode;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Box<AstNode>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        format!("let {} = {};", self.name.string(), self.value.string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Box<AstNode>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        format!("return {};", self.return_value.string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<AstNode>,
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        self.statements
            .iter()
            .fold(String::new(), |acc, statement| {
                format!("{acc}{}", statement.string())
            })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operation {
    Sum,
    Minus,
    Mult,
    Divide,
    Assig
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Assignation {
    pub token: Token,
    pub name: Identifier,
    pub value: Box<AstNode>,
    pub operation: Operation
}

impl Node for Assignation {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        format!("{} = {};", self.name.string(), self.value.string())
    }
}
