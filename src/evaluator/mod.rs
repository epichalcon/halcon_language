use std::collections::HashMap;

use crate::ast::expressions::{DictLiteral, Identifier, IfExpression};
use crate::object::{
    Array, Dict, Function, Object, StringObject, ARRAY, BUILTIN, DICT, FUNCTION, STRING,
};

use crate::{
    ast::{AstNode, Node},
    object::environment::Environment,
    object::{
        Boolean, Error, Integer, ObjectType, ReturnValue, BOOLEAN, ERROR, INTEGER, NULL, RETURN,
    },
    token::Token,
};

use self::builtin::get_builtin_function;

mod builtin;
#[cfg(test)]
mod test;

pub struct Evaluator {
    pub env: Environment,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }

    pub fn new_env(env: Environment) -> Self {
        Self { env }
    }

    pub fn eval(&mut self, node: AstNode) -> ObjectType {
        match node {
            AstNode::Program(program) => self.eval_program(program.statements),
            AstNode::ExpressionStatement(expression) => self.eval(*expression.expression),
            AstNode::BlockStatement(expressions) => self.eval_statements(expressions.statements),

            AstNode::PrefixExpression(prefix_expression) => {
                let right = self.eval(*prefix_expression.right);
                if is_error(&right) {
                    return right;
                }
                self.eval_prefix_expression(prefix_expression.operator, right)
            }
            AstNode::InfixExpression(infix_expression) => {
                let left = self.eval(*infix_expression.left);
                if is_error(&left) {
                    return left;
                }
                let right = self.eval(*infix_expression.right);
                if is_error(&right) {
                    return right;
                }
                self.eval_infix_expression(left, infix_expression.operator, right)
            }
            AstNode::IfExpression(if_expression) => self.eval_if_expression(if_expression),
            AstNode::FunctionLiteral(function_literal) => {
                let parameters = function_literal.parameters;
                let body = function_literal.body;
                ObjectType::Function(Function {
                    parameters,
                    body,
                    env: self.env.clone(),
                })
            }
            AstNode::CallExpression(call) => {
                let function = self.eval(*call.function);
                if is_error(&function) {
                    return function;
                }
                let args = self.eval_expressions(call.arguments.clone());
                if args.len() == 1 && is_error(&args[0]) {
                    return args[0].clone();
                }

                self.apply_function(function, args)
            }
            AstNode::LetStatement(let_statement) => {
                let val = self.eval(*let_statement.value);
                if is_error(&val) {
                    return val;
                }

                self.env
                    .set(let_statement.name.token_literal().as_str(), val.clone());
                val
            }
            AstNode::ReturnStatement(return_statement) => {
                let val = self.eval(*return_statement.return_value);
                if is_error(&val) {
                    return val;
                }

                return ObjectType::Return(ReturnValue {
                    value: Box::new(val),
                });
            }

            AstNode::Identifier(id) => self.eval_identifier(id),
            AstNode::IntegerLiteral(integer_literal) => ObjectType::Integer(Integer {
                value: match integer_literal.token {
                    Token::ConstInt(val) => val,
                    _ => panic!("Not a valid number"),
                },
            }),
            AstNode::Boolean(boolean_literal) => ObjectType::Boolean(Boolean {
                value: match boolean_literal.token {
                    Token::ConstBool(val) => val,
                    _ => panic!("Not a valid boolean"),
                },
            }),
            AstNode::StringLiteral(string_literal) => ObjectType::String(StringObject {
                value: match string_literal.token {
                    Token::ConstStr(val) => val,
                    _ => panic!("Not a valid boolean"),
                },
            }),
            AstNode::ArrayLiteral(array_literal) => {
                let elements = self.eval_expressions(array_literal.elements);
                if elements.len() == 1 && is_error(&elements[0]) {
                    elements[0].clone()
                } else {
                    ObjectType::Array(Array { elements })
                }
            }
            AstNode::IndexExpression(index_expression) => {
                let left = self.eval(*index_expression.left);
                if is_error(&left) {
                    return left;
                }
                let index = self.eval(*index_expression.index);
                if is_error(&index) {
                    return left;
                }

                self.eval_index_expression(left, index)
            }
            AstNode::DictLiteral(dict) => self.eval_hash_literal(dict),
            _ => panic!("Ast node not treated"),
        }
    }

    fn eval_program(&mut self, statements: Vec<AstNode>) -> ObjectType {
        let mut result = None;

        for statement in statements {
            let partial_result = self.eval(statement);

            if let ObjectType::Return(ret) = partial_result {
                return *ret.value;
            }
            if let ObjectType::Error(_) = partial_result {
                return partial_result;
            }

            result = Some(partial_result)
        }

        match result {
            Some(res) => res,
            None => panic!("no statements in program"),
        }
    }

    fn eval_statements(&mut self, statements: Vec<AstNode>) -> ObjectType {
        let mut result = None;

        for statement in statements {
            let partial_result = self.eval(statement);

            if partial_result.object_type() == RETURN || partial_result.object_type() == ERROR {
                return partial_result;
            }
            result = Some(partial_result)
        }

        match result {
            Some(res) => res,
            None => panic!("no statements in program"),
        }
    }

    fn eval_prefix_expression(&mut self, operator: String, right: ObjectType) -> ObjectType {
        match operator.as_str() {
            "not" => self.eval_not_operator(right),
            "-" => self.eval_minus_prefix_operator(right),
            _ => new_error(format!(
                "unknown operator: {} {}",
                operator,
                right.object_type()
            )),
        }
    }

    fn eval_infix_expression(
        &mut self,
        left: ObjectType,
        operator: String,
        right: ObjectType,
    ) -> ObjectType {
        if left.object_type() == INTEGER && right.object_type() == INTEGER {
            self.eval_infix_integer_expression(left, operator, right)
        } else if left.object_type() == STRING && right.object_type() == STRING {
            self.eval_infix_string_expression(left, operator, right)
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

    fn eval_if_expression(&mut self, if_expression: IfExpression) -> ObjectType {
        let condition = self.eval(*if_expression.condition);
        if is_error(&condition) {
            return condition;
        }
        if is_truthy(condition) {
            self.eval(AstNode::BlockStatement(if_expression.consequence))
        } else {
            match if_expression.alternative {
                Some(alternative) => self.eval(AstNode::BlockStatement(alternative)),
                None => ObjectType::Null,
            }
        }
    }

    fn eval_expressions(&mut self, arguments: Vec<AstNode>) -> Vec<ObjectType> {
        let mut result = vec![];

        for argument in arguments {
            let evaluated = self.eval(argument);
            if is_error(&evaluated) {
                return vec![evaluated];
            }

            result.push(evaluated)
        }

        result
    }

    fn eval_index_expression(&mut self, left: ObjectType, index: ObjectType) -> ObjectType {
        if left.object_type() == ARRAY && index.object_type() == INTEGER {
            self.eval_array_index_expression(left, index)
        } else if left.object_type() == DICT {
            self.eval_dictionary_index_expression(left, index)
        } else {
            new_error(format!(
                "index operator not supported: {}",
                left.object_type()
            ))
        }
    }

    fn eval_array_index_expression(&self, arr: ObjectType, index: ObjectType) -> ObjectType {
        let array = match arr {
            ObjectType::Array(array) => array,
            _ => panic!("Should be an array"),
        };

        let idx = match index {
            ObjectType::Integer(array) => array.value,
            _ => panic!("Should be an array"),
        };

        if idx < 0 || idx as usize >= array.elements.len() {
            return new_error(format!(
                "index: {} out of bounds: {}",
                idx,
                array.elements.len()
            ));
        }

        array.elements[idx as usize].clone()
    }

    fn apply_function(&mut self, fun: ObjectType, args: Vec<ObjectType>) -> ObjectType {
        match fun {
            ObjectType::Function(mut function) => {
                let previous_env = self.env.clone();
                function.env.update(&previous_env);
                let extended_env = self.extended_function_env(function.clone(), args);

                self.env = extended_env;
                let evaluated = self.eval(AstNode::BlockStatement(function.body));
                self.env = previous_env;

                unwrap_return_value(evaluated)
            }
            ObjectType::Builtin(function) => (function.function)(args),
            actual => {
                return new_error(format!("not a function {}", actual.object_type().as_str()))
            }
        }
    }

    fn extended_function_env(&self, function: Function, args: Vec<ObjectType>) -> Environment {
        let mut env = Environment::new_enclosed_environment(&function.env);

        for (i, param) in function.parameters.iter().enumerate() {
            env.set(param.token_literal().as_str(), args[i].clone());
        }

        return env;
    }

    fn eval_identifier(&mut self, id: Identifier) -> ObjectType {
        match self.env.get(id.token_literal()) {
            Some(obj) => return obj.clone(),
            None => {}
        };

        get_builtin_function(id.token_literal().as_str())
    }

    fn eval_not_operator(&self, right: ObjectType) -> ObjectType {
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

    fn eval_minus_prefix_operator(&self, right: ObjectType) -> ObjectType {
        if right.object_type() != INTEGER {
            return new_error(format!("unknown operator: -{}", right.object_type()));
        }

        let value: i128 = right.inspect().parse().unwrap();
        ObjectType::Integer(Integer { value: -value })
    }

    fn eval_infix_integer_expression(
        &self,
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
        &self,
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

    fn eval_hash_literal(&mut self, dict: DictLiteral) -> ObjectType {
        let mut pairs = HashMap::new();

        for (key_node, val_node) in dict.pairs.iter() {
            let key = self.eval(key_node.clone());

            if is_error(&key) {
                return new_error(format!("unusable as hash key: {}", key.object_type()));
            }

            match key.object_type().as_str() {
                FUNCTION | ERROR | ARRAY | BUILTIN | RETURN | DICT => {
                    return new_error(format!("unusable as hash key: {}", key.object_type()))
                }
                _ => (),
            }

            let value = self.eval(val_node.clone());
            if is_error(&value) {
                return new_error(format!("unusable as hash key: {}", value.object_type()));
            }

            pairs.insert(key, value);
        }

        ObjectType::Dict(Dict { pairs })
    }

    fn eval_dictionary_index_expression(&self, left: ObjectType, index: ObjectType) -> ObjectType {
        let dict = match left {
            ObjectType::Dict(dic) => dic,
            _ => panic!(),
        };

        match index.object_type().as_str() {
            FUNCTION | ERROR | ARRAY | BUILTIN | RETURN | DICT => {
                return new_error(format!("unusable as hash key: {}", index.object_type()))
            }
            _ => (),
        }

        dict.pairs.get(&index).unwrap_or(&ObjectType::Null).clone()
    }
}
fn new_error(message: String) -> ObjectType {
    ObjectType::Error(Error { message })
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

fn is_error(obj: &ObjectType) -> bool {
    obj.object_type() == ERROR
}

fn unwrap_return_value(evaluated: ObjectType) -> ObjectType {
    match evaluated {
        ObjectType::Return(return_value) => *return_value.value,
        _ => evaluated,
    }
}
