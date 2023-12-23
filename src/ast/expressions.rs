use std::fmt::format;

use crate::ast::statements::BlockStatement;
use crate::ast::{Expression, Node};
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum ExpressionNode {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
    Boolean(Boolean),
    IfExpression(IfExpression),
}

impl Node for ExpressionNode {
    fn token_literal(&self) -> String {
        match self {
            ExpressionNode::Identifier(expression) => expression.token_literal(),
            ExpressionNode::IntegerLiteral(expression) => expression.token_literal(),
            ExpressionNode::PrefixExpression(expression) => expression.token_literal(),
            ExpressionNode::InfixExpression(expression) => expression.token_literal(),
            ExpressionNode::Boolean(expression) => expression.token_literal(),
            ExpressionNode::IfExpression(expression) => expression.token_literal(),
        }
    }

    fn string(&self) -> String {
        match self {
            ExpressionNode::Identifier(expression) => expression.string(),
            ExpressionNode::IntegerLiteral(expression) => expression.string(),
            ExpressionNode::PrefixExpression(expression) => expression.string(),
            ExpressionNode::InfixExpression(expression) => expression.string(),
            ExpressionNode::Boolean(expression) => expression.string(),
            ExpressionNode::IfExpression(expression) => expression.string(),
        }
    }
}

impl Expression for ExpressionNode {
    fn expression_node(&self) {
        match self {
            ExpressionNode::Identifier(expression) => expression.expression_node(),
            ExpressionNode::IntegerLiteral(expression) => expression.expression_node(),
            ExpressionNode::PrefixExpression(expression) => expression.expression_node(),
            ExpressionNode::InfixExpression(expression) => expression.expression_node(),
            ExpressionNode::Boolean(expression) => expression.expression_node(),
            ExpressionNode::IfExpression(expression) => expression.expression_node(),
        }
    }
}

//-------------------[Prefix and infix expressions]-------------------//

#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<ExpressionNode>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        format!("({}{})", self.operator, self.right.string())
    }
}

impl Expression for PrefixExpression {
    fn expression_node(&self) {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Box<ExpressionNode>,
    pub operator: String,
    pub right: Box<ExpressionNode>,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        format!(
            "({} {} {})",
            self.left.string(),
            self.operator,
            self.right.string()
        )
    }
}

impl Expression for InfixExpression {
    fn expression_node(&self) {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct IfExpression {
    pub token: Token,
    pub condition: Box<ExpressionNode>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl Node for IfExpression {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        match &self.alternative {
            None => {
                format!(
                    "if{} {}",
                    self.condition.string(),
                    self.consequence.string()
                )
            }
            Some(alternative) => {
                format!(
                    "if{} {}else {}",
                    self.condition.string(),
                    self.consequence.string(),
                    alternative.string()
                )
            }
        }
    }
}
impl Expression for IfExpression {
    fn expression_node(&self) {
        todo!()
    }
}

//-------------------[literals]-------------------//

#[derive(Debug, Clone)]
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

    fn string(&self) -> String {
        self.token_literal()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
}
impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        match &self.token {
            Token::ConstInt(num) => num.to_string(),
            _ => panic!(),
        }
    }

    fn string(&self) -> String {
        self.token_literal()
    }
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Boolean {
    pub token: Token,
}

impl Node for Boolean {
    fn token_literal(&self) -> String {
        match &self.token {
            Token::ConstBool(id) => id.to_string(),
            _ => panic!(),
        }
    }

    fn string(&self) -> String {
        self.token_literal()
    }
}

impl Expression for Boolean {
    fn expression_node(&self) {
        todo!()
    }
}