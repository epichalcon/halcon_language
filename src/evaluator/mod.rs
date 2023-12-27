use std::env::args;

use crate::object::{Builtin, BuiltinFunction, Object, StringObject, STRING};

use crate::{
    ast::{AstNode, Node},
    object::environment::Environment,
    object::{
        Boolean, Error, Integer, ObjectType, ReturnValue, BOOLEAN, ERROR, INTEGER, NULL, RETURN,
    },
    token::Token,
};

#[cfg(test)]
mod test;

pub fn eval(node: AstNode, env: Environment) -> (ObjectType, Environment) {
    match node {
        AstNode::Program(program) => eval_program(program.statements, env),
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
        AstNode::FunctionLiteral(function_literal) => {
            let parameters = function_literal.parameters;
            let body = function_literal.body;
            (
                ObjectType::Function(crate::object::Function {
                    parameters,
                    body,
                    env: env.clone(),
                }),
                env,
            )
        }
        AstNode::CallExpression(call) => {
            let (function, env) = eval(*call.function, env.clone());
            if is_error(&function) {
                return (function, env);
            }
            let args = eval_expressions(call.arguments.clone(), env.clone());
            if args.len() == 1 && is_error(&args[0]) {
                return (args[0].clone(), env.clone());
            }

            (apply_function(function, args), env)
        }
        AstNode::LetStatement(let_statement) => {
            let (val, mut env) = eval(*let_statement.value, env);
            if is_error(&val) {
                return (val, env);
            }

            env.set(let_statement.name.token_literal().as_str(), val.clone());
            (val, env)
        }
        AstNode::ReturnStatement(return_statement) => {
            let (val, env) = eval(*return_statement.return_value, env);
            if is_error(&val) {
                return (val, env);
            }

            return (
                ObjectType::Return(ReturnValue {
                    value: Box::new(val),
                }),
                env,
            );
        }

        AstNode::Identifier(id) => eval_identifier(id, env),
        AstNode::IntegerLiteral(integer_literal) => (
            ObjectType::Integer(Integer {
                value: match integer_literal.token {
                    Token::ConstInt(val) => val,
                    _ => panic!("Not a valid number"),
                },
            }),
            env,
        ),
        AstNode::Boolean(boolean_literal) => (
            ObjectType::Boolean(Boolean {
                value: match boolean_literal.token {
                    Token::ConstBool(val) => val,
                    _ => panic!("Not a valid boolean"),
                },
            }),
            env,
        ),
        AstNode::StringLiteral(string_literal) => (
            ObjectType::String(StringObject {
                value: match string_literal.token {
                    Token::ConstStr(val) => val,
                    _ => panic!("Not a valid boolean"),
                },
            }),
            env,
        ),
        _ => panic!("Ast node not treated"),
    }
}

fn eval_program(statements: Vec<AstNode>, env: Environment) -> (ObjectType, Environment) {
    let mut result = None;
    let mut new_env = env.clone();

    for statement in statements {
        let (partial_result, an_env) = eval(statement, new_env.clone());

        new_env = an_env;

        if let ObjectType::Return(ret) = partial_result {
            return (*ret.value, env);
        }
        if let ObjectType::Error(_) = partial_result {
            return (partial_result, env);
        }

        result = Some(partial_result)
    }

    match result {
        Some(res) => (res, new_env),
        None => panic!("no statements in program"),
    }
}

fn eval_statements(statements: Vec<AstNode>, env: Environment) -> (ObjectType, Environment) {
    let mut result = None;
    let mut new_env = env.clone();

    for statement in statements {
        let (partial_result, an_env) = eval(statement, new_env.clone());

        new_env = an_env;

        if partial_result.object_type() == RETURN || partial_result.object_type() == ERROR {
            return (partial_result, new_env);
        }
        result = Some(partial_result)
    }

    match result {
        Some(res) => (res, new_env),
        None => panic!("no statements in program"),
    }
}

fn eval_prefix_expression(operator: String, right: ObjectType) -> ObjectType {
    match operator.as_str() {
        "not" => eval_not_operator(right),
        "-" => eval_minus_prefix_operator(right),
        _ => new_error(format!(
            "unknown operator: {} {}",
            operator,
            right.object_type()
        )),
    }
}

fn eval_infix_expression(left: ObjectType, operator: String, right: ObjectType) -> ObjectType {
    if left.object_type() == INTEGER && right.object_type() == INTEGER {
        eval_infix_integer_expression(left, operator, right)
    } else if left.object_type() == STRING && right.object_type() == STRING {
        eval_infix_string_expression(left, operator, right)
    } else if left.object_type() != right.object_type() {
        new_error(format!(
            "type mismatch: {} {} {}",
            left.object_type(),
            operator,
            right.object_type()
        ))
    } else if operator == "==" {
        ObjectType::Boolean(Boolean {
            value: left.inspect() == right.inspect(),
        })
    } else if operator == "!=" {
        ObjectType::Boolean(Boolean {
            value: left.inspect() != right.inspect(),
        })
    } else {
        new_error(format!(
            "unknown operator: {} {} {}",
            left.object_type(),
            operator,
            right.object_type()
        ))
    }
}

fn eval_if_expression(
    if_expression: crate::ast::expressions::IfExpression,
    env: Environment,
) -> (ObjectType, Environment) {
    let (condition, env) = eval(*if_expression.condition, env);
    if is_error(&condition) {
        return (condition, env);
    }
    if is_truthy(condition) {
        eval(AstNode::BlockStatement(if_expression.consequence), env)
    } else {
        match if_expression.alternative {
            Some(alternative) => eval(AstNode::BlockStatement(alternative), env),
            None => (ObjectType::Null, env),
        }
    }
}

fn eval_expressions(arguments: Vec<AstNode>, env: Environment) -> Vec<ObjectType> {
    let mut result = vec![];

    for argument in arguments {
        let (evaluated, _) = eval(argument, env.clone());
        if is_error(&evaluated) {
            return vec![evaluated];
        }

        result.push(evaluated)
    }

    result
}

fn apply_function(fun: ObjectType, args: Vec<ObjectType>) -> ObjectType {
    match fun {
        ObjectType::Function(function) => {
            let extended_env = extended_function_env(function.clone(), args);
            let (evaluated, _) = eval(AstNode::BlockStatement(function.body), extended_env);
            unwrap_return_value(evaluated)
        }
        ObjectType::Builtin(function) => (function.function)(args),
        actual => return new_error(format!("not a function {}", actual.object_type().as_str())),
    }
}

fn unwrap_return_value(evaluated: ObjectType) -> ObjectType {
    match evaluated {
        ObjectType::Return(return_value) => *return_value.value,
        _ => evaluated,
    }
}

fn extended_function_env(function: crate::object::Function, args: Vec<ObjectType>) -> Environment {
    let mut env = Environment::new_enclosed_environment(function.env);

    for (i, param) in function.parameters.iter().enumerate() {
        env.set(param.token_literal().as_str(), args[i].clone());
    }

    return env;
}

fn eval_identifier(
    id: crate::ast::expressions::Identifier,
    env: Environment,
) -> (ObjectType, Environment) {
    match env.get(id.token_literal()) {
        Some(obj) => return (obj.clone(), env),
        None => {}
    };

    match id.token_literal().as_str() {
        "len" => (ObjectType::Builtin(Builtin { function: length }), env),
        _ => {
            return (
                new_error(format!("identifier not found: {}", id.token_literal())),
                env,
            )
        }
    }
}

fn length(args: Vec<ObjectType>) -> ObjectType {
    if args.len() != 1 {
        return new_error(format!(
            "wrong number of arguments. got: {}, want: 1",
            args.len()
        ));
    }

    match &args[0] {
        ObjectType::String(s) => ObjectType::Integer(Integer {
            value: s.value.len().try_into().unwrap(),
        }),
        _ => new_error(format!(
            "argument to len not supported, got {}",
            args[0].object_type()
        )),
    }
}

fn is_truthy(condition: ObjectType) -> bool {
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

fn eval_not_operator(right: ObjectType) -> ObjectType {
    match right.object_type().as_str() {
        BOOLEAN => match right.inspect().as_str() {
            "true" => ObjectType::Boolean(Boolean { value: false }),
            "false" => ObjectType::Boolean(Boolean { value: true }),
            _ => panic!("Boolean incorrectly formed"),
        },
        NULL => ObjectType::Boolean(Boolean { value: true }),
        _ => ObjectType::Boolean(Boolean { value: false }),
    }
}

fn eval_minus_prefix_operator(right: ObjectType) -> ObjectType {
    if right.object_type() != INTEGER {
        return new_error(format!("unknown operator: -{}", right.object_type()));
    }

    let value: i128 = right.inspect().parse().unwrap();
    ObjectType::Integer(Integer { value: -value })
}

fn eval_infix_integer_expression(
    left: ObjectType,
    operator: String,
    right: ObjectType,
) -> ObjectType {
    let left_val: i128 = left.inspect().parse().unwrap();
    let right_val: i128 = right.inspect().parse().unwrap();
    match operator.as_str() {
        "+" => ObjectType::Integer(Integer {
            value: left_val + right_val,
        }),
        "-" => ObjectType::Integer(Integer {
            value: left_val - right_val,
        }),
        "*" => ObjectType::Integer(Integer {
            value: left_val * right_val,
        }),
        "/" => ObjectType::Integer(Integer {
            value: left_val / right_val,
        }),
        "<" => ObjectType::Boolean(Boolean {
            value: left_val < right_val,
        }),
        ">" => ObjectType::Boolean(Boolean {
            value: left_val > right_val,
        }),
        "==" => ObjectType::Boolean(Boolean {
            value: left_val == right_val,
        }),
        "!=" => ObjectType::Boolean(Boolean {
            value: left_val != right_val,
        }),
        _ => new_error(format!(
            "unknown operator: {} {} {}",
            left.object_type(),
            operator,
            right.object_type()
        )),
    }
}

fn eval_infix_string_expression(
    left: ObjectType,
    operator: String,
    right: ObjectType,
) -> ObjectType {
    let left_val: String = left.inspect().parse().unwrap();
    let right_val: String = right.inspect().parse().unwrap();
    match operator.as_str() {
        "+" => ObjectType::String(StringObject {
            value: left_val + &right_val,
        }),
        _ => new_error(format!(
            "unknown operator: {} {} {}",
            left.object_type(),
            operator,
            right.object_type()
        )),
    }
}

fn new_error(message: String) -> ObjectType {
    ObjectType::Error(Error { message })
}

fn is_error(obj: &ObjectType) -> bool {
    obj.object_type() == ERROR
}
