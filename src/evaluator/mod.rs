use std::env;

use crate::{
    ast::{AstNode, Node},
    object::environment::Environment,
    object::{
        Boolean, Error, Integer, Null, Object, ReturnValue, BOOLEAN, ERROR, INTEGER, NULL, RETURN,
    },
    token::Token,
};

#[cfg(test)]
mod test;

pub fn eval(node: AstNode, env: Environment) -> (Box<dyn Object>, Environment) {
    match node {
        AstNode::Program(program) => eval_statements(program.statements, env),
        AstNode::ExpressionStatement(expression) => eval(*expression.expression, env),
        AstNode::BlockStatement(expressions) => eval_statements(expressions.statements, env),

        AstNode::PrefixExpression(prefix_expression) => {
            let (right, env) = eval(*prefix_expression.right, env);
            if is_error(&right) {
                return (right, env);
            }
            (
                eval_prefix_expression(prefix_expression.operator, right),
                env,
            )
        }
        AstNode::InfixExpression(infix_expression) => {
            let (left, env) = eval(*infix_expression.left, env);
            if is_error(&left) {
                return (left, env);
            }
            let (right, env) = eval(*infix_expression.right, env);
            if is_error(&right) {
                return (right, env);
            }
            (
                eval_infix_expression(left, infix_expression.operator, right),
                env,
            )
        }
        AstNode::IfExpression(if_expression) => eval_if_expression(if_expression, env),
        AstNode::FunctionLiteral(_) => todo!(),
        AstNode::CallExpression(_) => todo!(),
        AstNode::LetStatement(let_statement) => {
            let (val, mut env) = eval(*let_statement.value, env);
            if is_error(&val) {
                return (val, env);
            }

            env.set(
                let_statement.name.token_literal().as_str(),
                val.object_type(),
                val.inspect(),
            );
            (val, env)
        }
        AstNode::ReturnStatement(return_statement) => {
            let (val, env) = eval(*return_statement.return_value, env);
            if is_error(&val) {
                return (val, env);
            }

            return (Box::new(ReturnValue { value: val }), env);
        }

        AstNode::Identifier(id) => eval_identifier(id, env),
        AstNode::IntegerLiteral(integer_literal) => (
            Box::new(Integer {
                value: match integer_literal.token {
                    Token::ConstInt(val) => val,
                    _ => panic!("Not a valid number"),
                },
            }),
            env,
        ),
        AstNode::Boolean(boolean_literal) => (
            Box::new(Boolean {
                value: match boolean_literal.token {
                    Token::ConstBool(val) => val,
                    _ => panic!("Not a valid boolean"),
                },
            }),
            env,
        ),
    }
}

fn eval_identifier(
    id: crate::ast::expressions::Identifier,
    env: Environment,
) -> (Box<dyn Object>, Environment) {
    match env.get(id.token_literal()) {
        Some(obj) => (obj, env),
        None => {
            return (
                Box::new(new_error(format!(
                    "identifier not found: {}",
                    id.token_literal()
                ))),
                env,
            )
        }
    }
}

fn eval_if_expression(
    if_expression: crate::ast::expressions::IfExpression,
    mut env: Environment,
) -> (Box<dyn Object>, Environment) {
    let (condition, env) = eval(*if_expression.condition, env);
    if is_error(&condition) {
        return (condition, env);
    }
    if is_truthy(&condition) {
        eval(AstNode::BlockStatement(if_expression.consequence), env)
    } else {
        match if_expression.alternative {
            Some(alternative) => eval(AstNode::BlockStatement(alternative), env),
            None => (Box::new(Null {}), env),
        }
    }
}

fn is_truthy(condition: &Box<dyn Object>) -> bool {
    match condition.object_type().as_str() {
        BOOLEAN => {
            if condition.inspect() == "true" {
                true
            } else {
                false
            }
        }
        NULL => false,
        _ => true,
    }
}

fn eval_prefix_expression(operator: String, right: Box<dyn Object>) -> Box<dyn Object> {
    match operator.as_str() {
        "not" => eval_not_operator(right),
        "-" => eval_minus_prefix_operator(right),
        _ => Box::new(new_error(format!(
            "unknown operator: {} {}",
            operator,
            right.object_type()
        ))),
    }
}

fn eval_not_operator(right: Box<dyn Object>) -> Box<dyn Object> {
    match right.object_type().as_str() {
        BOOLEAN => match right.inspect().as_str() {
            "true" => Box::new(Boolean { value: false }),
            "false" => Box::new(Boolean { value: true }),
            _ => panic!("Boolean incorrectly formed"),
        },
        NULL => Box::new(Boolean { value: true }),
        _ => Box::new(Boolean { value: false }),
    }
}

fn eval_minus_prefix_operator(right: Box<dyn Object>) -> Box<dyn Object> {
    if right.object_type() != INTEGER {
        return Box::new(new_error(format!(
            "unknown operator: -{}",
            right.object_type()
        )));
    }

    let value: i128 = right.inspect().parse().unwrap();
    Box::new(Integer { value: -value })
}

fn eval_infix_expression(
    left: Box<dyn Object>,
    operator: String,
    right: Box<dyn Object>,
) -> Box<dyn Object> {
    if left.object_type() == INTEGER && right.object_type() == INTEGER {
        eval_infix_integer_expression(left, operator, right)
    } else if left.object_type() != right.object_type() {
        Box::new(new_error(format!(
            "type mismatch: {} {} {}",
            left.object_type(),
            operator,
            right.object_type()
        )))
    } else if operator == "==" {
        Box::new(Boolean {
            value: left.inspect() == right.inspect(),
        })
    } else if operator == "!=" {
        Box::new(Boolean {
            value: left.inspect() != right.inspect(),
        })
    } else {
        Box::new(new_error(format!(
            "unknown operator: {} {} {}",
            left.object_type(),
            operator,
            right.object_type()
        )))
    }
}

fn eval_infix_integer_expression(
    left: Box<dyn Object>,
    operator: String,
    right: Box<dyn Object>,
) -> Box<dyn Object> {
    let left_val: i128 = left.inspect().parse().unwrap();
    let right_val: i128 = right.inspect().parse().unwrap();
    match operator.as_str() {
        "+" => Box::new(Integer {
            value: left_val + right_val,
        }),
        "-" => Box::new(Integer {
            value: left_val - right_val,
        }),
        "*" => Box::new(Integer {
            value: left_val * right_val,
        }),
        "/" => Box::new(Integer {
            value: left_val / right_val,
        }),
        "<" => Box::new(Boolean {
            value: left_val < right_val,
        }),
        ">" => Box::new(Boolean {
            value: left_val > right_val,
        }),
        "==" => Box::new(Boolean {
            value: left_val == right_val,
        }),
        "!=" => Box::new(Boolean {
            value: left_val != right_val,
        }),
        _ => Box::new(new_error(format!(
            "unknown operator: {} {} {}",
            left.object_type(),
            operator,
            right.object_type()
        ))),
    }
}

fn eval_statements(statements: Vec<AstNode>, env: Environment) -> (Box<dyn Object>, Environment) {
    let mut result = None;
    let mut new_env = env.clone();

    for statement in statements {
        let (partial_result, an_env) = eval(statement, new_env.clone());

        new_env = an_env;

        if partial_result.object_type() == RETURN {
            let ret: ReturnValue = partial_result.into();
            return (ret.value, env);
        }
        if partial_result.object_type() == ERROR {
            return (partial_result, env);
        }

        result = Some(partial_result)
    }

    match result {
        Some(res) => (res, new_env),
        None => panic!("no statements in program"),
    }
}

fn new_error(message: String) -> Error {
    Error { message }
}

fn is_error(obj: &Box<dyn Object>) -> bool {
    obj.object_type() == ERROR
}
