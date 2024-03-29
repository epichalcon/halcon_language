use std::collections::HashMap;
use std::hash::Hash;

use crate::ast::statements::BlockStatement;
use crate::ast::Node;
use crate::token::Token;

use super::AstNode;
use super::statements::LetStatement;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<AstNode>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        format!("({}{})", self.operator, self.right.string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Box<AstNode>,
    pub operator: String,
    pub right: Box<AstNode>,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfExpression {
    pub token: Token,
    pub condition: Box<AstNode>,
    pub consequence: BlockStatement,
    pub elifs: Vec<(AstNode, BlockStatement)>,
    pub alternative: Option<BlockStatement>,
}

impl Node for IfExpression {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        let res = format!(
            "if{} {}",
            self.condition.string(),
            self.consequence.string()
        );

        let elifs = format!(
            "{}{}",
            res,
            self.elifs.iter().fold(String::new(), |acc, (cond, cons)| {
                format!("{acc} elif {} {}", cond.string(), cons.string())
            })
        );

        match &self.alternative {
            None => elifs,
            Some(alternative) => {
                format!("{} else {}", elifs, alternative.string())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndexExpression {
    pub token: Token,
    pub left: Box<AstNode>,
    pub index: Box<AstNode>,
}

impl Node for IndexExpression {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        format!("({}[{}])", self.left.string(), self.index.string())
    }
}

//-------------------[literals]-------------------//

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        let parameters =
            self.parameters
                .iter()
                .enumerate()
                .fold(String::new(), |acc, (i, statement)| {
                    if i < self.parameters.len() - 1 {
                        format!("{acc}{}, ", statement.string())
                    } else {
                        format!("{acc}{}", statement.string())
                    }
                });

        format!(
            "{}({}) {{{}}}",
            self.token.to_string(),
            parameters,
            self.body.string()
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringLiteral {
    pub token: Token,
}

impl Node for StringLiteral {
    fn token_literal(&self) -> String {
        match &self.token {
            Token::ConstStr(id) => id.to_string(),
            _ => panic!(),
        }
    }

    fn string(&self) -> String {
        self.token_literal()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayLiteral {
    pub token: Token,
    pub elements: Vec<AstNode>,
}

impl Node for ArrayLiteral {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        let elements =
            self.elements
                .iter()
                .enumerate()
                .fold(String::new(), |acc, (i, statement)| {
                    if i < self.elements.len() - 1 {
                        format!("{acc}{}, ", statement.string())
                    } else {
                        format!("{acc}{}", statement.string())
                    }
                });

        format!("[{}]", elements)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DictLiteral {
    pub token: Token,
    pub pairs: HashMap<AstNode, AstNode>,
}

impl Node for DictLiteral {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        let pairs = self
            .pairs
            .iter()
            .enumerate()
            .fold(String::new(), |acc, (i, (key, val))| {
                if i < self.pairs.len() - 1 {
                    format!("{acc}{}: {}, ", key.string(), val.string())
                } else {
                    format!("{acc}{}: {}", key.string(), val.string())
                }
            });

        format!("{{{}}}", pairs)
    }
}

impl Hash for DictLiteral {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.token.hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallExpression {
    pub token: Token,
    pub function: Box<AstNode>,
    pub arguments: Vec<AstNode>,
}

impl Node for CallExpression {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        let arguments =
            self.arguments
                .iter()
                .enumerate()
                .fold(String::new(), |acc, (i, statement)| {
                    if i < self.arguments.len() - 1 {
                        format!("{acc}{}, ", statement.string())
                    } else {
                        format!("{acc}{}", statement.string())
                    }
                });

        format!("{}({})", self.function.string(), arguments)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PostIncrement {
    pub token: Token,
}

impl Node for PostIncrement {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PostDecrement {
    pub token: Token,
}

impl Node for PostDecrement {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForLoop {
    pub token: Token,
    pub initialization: LetStatement,
    pub condition: Box<AstNode>,
    pub step: Box<AstNode>,
    pub statements: BlockStatement,
}

impl Node for ForLoop {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        format!(
            "for ({}; {}; {}) {{{}}}",
            self.initialization.string(),
            self.condition.string(),
            self.step.string(),
            self.statements.statements.iter().enumerate().fold(
                String::new(),
                |acc, (i, statement)| {
                    if i < self.statements.statements.len() - 1 {
                        format!("{acc}{}, ", statement.string())
                    } else {
                        format!("{acc}{}", statement.string())
                    }
                }
            )
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhileLoop {
    pub token: Token,
    pub condition: Box<AstNode>,
    pub statements: BlockStatement,
}

impl Node for WhileLoop {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        format!(
            "while ({}) {{{}}}",
            self.condition.string(),
            self.statements.statements.iter().enumerate().fold(
                String::new(),
                |acc, (i, statement)| {
                    if i < self.statements.statements.len() - 1 {
                        format!("{acc}{}, ", statement.string())
                    } else {
                        format!("{acc}{}", statement.string())
                    }
                }
            )
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Loop {
    pub token: Token,
    pub statements: BlockStatement,
}

impl Node for Loop {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        format!(
            "loop {{{}}}",
            self.statements.statements.iter().enumerate().fold(
                String::new(),
                |acc, (i, statement)| {
                    if i < self.statements.statements.len() - 1 {
                        format!("{acc}{}, ", statement.string())
                    } else {
                        format!("{acc}{}", statement.string())
                    }
                }
            )
        )
    }
}
