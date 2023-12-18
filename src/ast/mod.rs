use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<StatementNode>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }
}

#[derive(Debug)]
pub enum StatementNode {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
}

impl Node for StatementNode {
    fn token_literal(&self) -> String {
        match self {
            StatementNode::LetStatement(statement) => statement.token_literal(),
            StatementNode::ReturnStatement(statement) => statement.token_literal(),
            _ => panic!(),
        }
    }
}

impl Statement for StatementNode {
    fn statement_node(&self) {
        match self {
            StatementNode::LetStatement(statement) => statement.statement_node(),
            StatementNode::ReturnStatement(statement) => statement.statement_node(),
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub enum ExpressionNode {
    Identifier(Identifier),
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: ExpressionNode,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {
        todo!()
    }
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: ExpressionNode,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {
        todo!()
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        match &self.token {
            Token::Id(id) => id.to_string(),
            _ => panic!(),
        }
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {
        todo!()
    }
}
