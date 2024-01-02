use self::{expressions::*, statements::*};
pub mod expressions;
pub mod statements;

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstNode {
    Program(Program),
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
    Boolean(Boolean),
    IfExpression(IfExpression),
    FunctionLiteral(FunctionLiteral),
    StringLiteral(StringLiteral),
    ArrayLiteral(ArrayLiteral),
    DictLiteral(DictLiteral),
    IndexExpression(IndexExpression),
    CallExpression(CallExpression),
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    BlockStatement(BlockStatement),
    Assignation(Assignation),
    PostIncrement(PostIncrement),
    PostDecrement(PostDecrement),
}

impl Node for AstNode {
    fn token_literal(&self) -> String {
        match self {
            AstNode::Program(expression) => expression.token_literal(),
            AstNode::Identifier(expression) => expression.token_literal(),
            AstNode::IntegerLiteral(expression) => expression.token_literal(),
            AstNode::PrefixExpression(expression) => expression.token_literal(),
            AstNode::InfixExpression(expression) => expression.token_literal(),
            AstNode::Boolean(expression) => expression.token_literal(),
            AstNode::IfExpression(expression) => expression.token_literal(),
            AstNode::FunctionLiteral(expression) => expression.token_literal(),
            AstNode::CallExpression(expression) => expression.token_literal(),
            AstNode::LetStatement(statement) => statement.token_literal(),
            AstNode::ReturnStatement(statement) => statement.token_literal(),
            AstNode::BlockStatement(statement) => statement.token_literal(),
            AstNode::StringLiteral(statement) => statement.token_literal(),
            AstNode::ArrayLiteral(statement) => statement.token_literal(),
            AstNode::IndexExpression(statement) => statement.token_literal(),
            AstNode::DictLiteral(statement) => statement.token_literal(),
            AstNode::Assignation(statement) => statement.token_literal(),
            AstNode::PostIncrement(statement) => statement.token_literal(),
            AstNode::PostDecrement(statement) => statement.token_literal(),
        }
    }

    fn string(&self) -> String {
        match self {
            AstNode::Program(expression) => expression.string(),
            AstNode::Identifier(expression) => expression.string(),
            AstNode::IntegerLiteral(expression) => expression.string(),
            AstNode::PrefixExpression(expression) => expression.string(),
            AstNode::InfixExpression(expression) => expression.string(),
            AstNode::Boolean(expression) => expression.string(),
            AstNode::IfExpression(expression) => expression.string(),
            AstNode::FunctionLiteral(expression) => expression.string(),
            AstNode::CallExpression(expression) => expression.string(),
            AstNode::LetStatement(statement) => statement.string(),
            AstNode::ReturnStatement(statement) => statement.string(),
            AstNode::BlockStatement(statement) => statement.string(),
            AstNode::StringLiteral(statement) => statement.string(),
            AstNode::ArrayLiteral(statement) => statement.string(),
            AstNode::IndexExpression(statement) => statement.string(),
            AstNode::DictLiteral(statement) => statement.string(),
            AstNode::Assignation(statement) => statement.string(),
            AstNode::PostIncrement(statement) => statement.string(),
            AstNode::PostDecrement(statement) => statement.string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Program {
    pub statements: Vec<AstNode>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }

    fn string(&self) -> String {
        self.statements
            .iter()
            .fold(String::new(), |acc, statement| {
                format!("{acc}{}", statement.string())
            })
    }
}
