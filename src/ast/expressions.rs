use crate::ast::Expression;
use crate::ast::Node;
use crate::token::Token;

#[derive(Debug)]
pub enum ExpressionNode {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
    Boolean(Boolean),
}

impl Node for ExpressionNode {
    fn token_literal(&self) -> String {
        match self {
            ExpressionNode::Identifier(expression) => expression.token_literal(),
            ExpressionNode::IntegerLiteral(expression) => expression.token_literal(),
            ExpressionNode::PrefixExpression(expression) => expression.token_literal(),
            ExpressionNode::InfixExpression(expression) => expression.token_literal(),
            ExpressionNode::Boolean(expression) => expression.token_literal(),
        }
    }

    fn string(&self) -> String {
        match self {
            ExpressionNode::Identifier(expression) => expression.string(),
            ExpressionNode::IntegerLiteral(expression) => expression.string(),
            ExpressionNode::PrefixExpression(expression) => expression.string(),
            ExpressionNode::InfixExpression(expression) => expression.string(),
            ExpressionNode::Boolean(expression) => expression.string(),
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
        }
    }
}

//-------------------[Prefix and infix expressions]-------------------//

#[derive(Debug)]
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

#[derive(Debug)]
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

//-------------------[literals]-------------------//

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

    fn string(&self) -> String {
        self.token_literal()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {
        todo!()
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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
