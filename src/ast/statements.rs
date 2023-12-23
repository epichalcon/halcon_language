use crate::ast::expressions::ExpressionNode;
use crate::ast::expressions::Identifier;
use crate::ast::Node;
use crate::ast::Statement;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum StatementNode {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    ExpressionStatement(ExpressionStatement),
    BlockStatement(BlockStatement),
}

impl Node for StatementNode {
    fn token_literal(&self) -> String {
        match self {
            StatementNode::LetStatement(statement) => statement.token_literal(),
            StatementNode::ReturnStatement(statement) => statement.token_literal(),
            StatementNode::ExpressionStatement(statement) => statement.token_literal(),
            StatementNode::BlockStatement(statement) => statement.token_literal(),
        }
    }

    fn string(&self) -> String {
        match self {
            StatementNode::LetStatement(statement) => statement.string(),
            StatementNode::ReturnStatement(statement) => statement.string(),
            StatementNode::ExpressionStatement(statement) => statement.string(),
            StatementNode::BlockStatement(statement) => statement.string(),
        }
    }
}

impl Statement for StatementNode {
    fn statement_node(&self) {
        match self {
            StatementNode::LetStatement(statement) => statement.statement_node(),
            StatementNode::ReturnStatement(statement) => statement.statement_node(),
            StatementNode::ExpressionStatement(statement) => statement.statement_node(),
            StatementNode::BlockStatement(statement) => statement.statement_node(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: ExpressionNode,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        format!("let {} = {};", self.name.string(), self.value.string())
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: ExpressionNode,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        format!("return {};", self.return_value.string())
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: ExpressionNode,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        self.expression.string()
    }
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<StatementNode>,
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

impl Statement for BlockStatement {
    fn statement_node(&self) {
        todo!()
    }
}
