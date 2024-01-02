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

/// The evaluator struct is the responsable of evaluating the parsed program
pub struct Evaluator {
    /// the `env` variable holds the active environment of the program
    pub env: Environment,
}

#[allow(unreachable_patterns)]
impl Evaluator {
    /**
    Returns an Evaluator

    # Arguments

    # Examples
    ```
    let mut evaluator = Evaluator::new();
    ```
    */
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }

    /**
    Returns an Evaluator  with the specified `Environment`

    # Arguments
    * `env` - the `Environment` to be used

    # Examples
    ```
    let mut env = Environment::new();
    let mut evaluator = Evaluator::new_env(env);
    ```
    */
    pub fn new_env(env: Environment) -> Self {
        Self { env }
    }

    /**
    Returns the `ObjectType` of the processed `AstNode`. The function will call the coresponding functions for each type of `AstNode`

    # Arguments
    * `node` - the `AstNode` to parse

    # Examples
    ```
    let lex = Lexer::new(scanned);
    let mut pars = Parser::new(lex);

    let program = pars.parse_program();

    let mut evaluator = Evaluator::new();
    let evaluated = evaluator.eval(program);
    ```
    */
    pub fn eval(&mut self, node: AstNode) -> ObjectType {
        match node {
            AstNode::BlockStatement(expressions) => self.eval_statements(expressions.statements),
            AstNode::Program(program) => self.eval_program(program.statements),

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
                let args = self.eval_list_expressions(call.arguments.clone());
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
                let elements = self.eval_list_expressions(array_literal.elements);
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
            AstNode::DictLiteral(dict) => self.eval_dict_literal(dict),
            _ => panic!("Ast node not treated"),
        }
    }

    /**
    Evaluates every statement in the program and returns the `ObjectType` of the last statement,
    of the return statement (unpacking the contents) or an ObjectType::Error if an error has occured.

    # Arguments
    * `statements` - The list of statements to evaluate
    */
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

    /**
    Evaluates every statement in a list of statements and returns the `ObjectType` of the last statement,
    of the return statement or an ObjectType::Error if an error has occured.

    # Arguments
    * `statements` - The list of statements to evaluate
    */
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

    /**
    Evaluates a prefix operator expression and returns the result. This includes:
    * not
    * minus

    # Arguments
    * `operator` - the prefix operator to parse
    * `right` - the Object to apply the operator
    */
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

    /**
    Evaluates an infix operator expression and returns the result. This includes:
    * boolean operatorions -> true == true
    * numerical operations -> 1 + 3

    # Arguments
    * `left` - the left Object to apply the operator
    * `operator` - the prefix operator to parse
    * `right` - the right Object to apply the operator
    */
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

    /**
    Evaluates an if else expression returns the result.

    # Arguments
    * `if_expression` - the if expression to parse
    */
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

    /**
    Evaluates a list of expressions (as the ones found in Arrays) and returns the results in a `Vec<ObjectType>`

    # Arguments
    * `expressions` - the expressions to evaluate
    */
    fn eval_list_expressions(&mut self, expressions: Vec<AstNode>) -> Vec<ObjectType> {
        let mut result = vec![];

        for argument in expressions {
            let evaluated = self.eval(argument);
            if is_error(&evaluated) {
                return vec![evaluated];
            }

            result.push(evaluated)
        }
        result
    }

    /**
    Evaluates an index expressionand returns the result

    # Arguments
    * `left` - the Object to index
    * `index` - the index to apply
    */
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

    /**
    Evaluates the dict literal. If an unhashable key is used an `ObjectType::Error` is returned

    # Arguments
    * `dict` - the dictionary to evaluate
    */
    fn eval_dict_literal(&mut self, dict: DictLiteral) -> ObjectType {
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

    /**
    Evaluates an array index expressionand returns the result

    # Arguments
    * `left` - the Array to index
    * `index` - the index to apply
    */
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

    /**
    Evaluates an dictionary index expressionand returns the result

    # Arguments
    * `left` - the dictionary to index
    * `index` - the index to apply
    */
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

    /**
    Executes the call to a function, either user defined or built in, and returns the result.
    the environment of the function will be updated with the current active environment

    # Arguments
    * `fun` - the function to call
    * `args` - the arguments to pass to the function
    */
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

    /**
    Returns a new `Environment` with the functions environment as the outer environment and the paramenters
    as the inner environment

    # Arguments
    * `function` - the functions with the `Environment` to extend
    * `args` - the arguments to include in the new `Environment`
    */
    fn extended_function_env(&self, function: Function, args: Vec<ObjectType>) -> Environment {
        let mut env = Environment::new_enclosed_environment(&function.env);

        for (i, param) in function.parameters.iter().enumerate() {
            env.set(param.token_literal().as_str(), args[i].clone());
        }

        return env;
    }

    /**
    Searches an identifier in the active `Environment` and returns the result
    or returns an `ObjectType::Builtin` if the identifier is a builtin function

    # Arguments
    * `id` - the id to evaluate
    */
    fn eval_identifier(&mut self, id: Identifier) -> ObjectType {
        match self.env.get(id.token_literal()) {
            Some(obj) => return obj.clone(),
            None => {}
        };

        get_builtin_function(id.token_literal().as_str())
    }

    /**
    Evaluates a not operator, negating the truthiness of the value

    # Arguments
    * `right` - the object to negate
    */
    fn eval_not_operator(&self, right: ObjectType) -> ObjectType {
        ObjectType::Boolean(Boolean {
            value: !is_truthy(right),
        })
    }

    /**
    Evaluates a minus prefix operator. If the `ObjectType` is not an Integer, an `ObjectType:Error` will be returned

    # Arguments
    * `right` - the object to apply
    */
    fn eval_minus_prefix_operator(&self, right: ObjectType) -> ObjectType {
        match right {
            ObjectType::Integer(int) => ObjectType::Integer(Integer { value: -int.value }),

            other => new_error(format!("unknown operator: -{}", other.object_type())),
        }
    }

    /**
    Evaluates the infix integers operators. If the operator is not supported an `ObjectType::Error` is returned

    # Arguments
    * `right` - the right `ObjectType` to evaluate
    * `operator` - the operator to evaluate
    * `left` - the left `ObjectType` to evaluate
    */
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
            ">=" => ObjectType::Boolean(Boolean {
                value: left_val >= right_val,
            }),
            "<=" => ObjectType::Boolean(Boolean {
                value: left_val <= right_val,
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

    /**
    Evaluates the infix string expressions. If the operator is not supported an `ObjectType::Error` is returned

    # Arguments
    * `right` - the right `ObjectType` to evaluate
    * `operator` - the operator to evaluate
    * `left` - the left `ObjectType` to evaluate
    */
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
}

/**
Returns an `ObjectType::Error` with the specified message

# Arguments
* `message` - the error message
*/
fn new_error(message: String) -> ObjectType {
    ObjectType::Error(Error { message })
}


/**
Returns if an object is truthy. The results are the following
- true -> truthy
- false -> not truthy
- Null -> not truthy
- other objects -> truthy

# Arguments
* `condition` - the `ObjectType` to evaluate
*/
fn is_truthy(condition: ObjectType) -> bool {
    match condition {
        ObjectType::Boolean(boolean) => boolean.value,
        ObjectType::Null => false,
        _ => true,
    }
}

/**
Returns if an `ObjectType` is an error
# Arguments
* `obj` - the `ObjectType` to evaluate
*/
fn is_error(obj: &ObjectType) -> bool {
    obj.object_type() == ERROR
}

/**
Returns the inner object of an `ObjectType::Return`
# Arguments
* `obj` - the `ObjectType` to evaluate
*/
fn unwrap_return_value(evaluated: ObjectType) -> ObjectType {
    match evaluated {
        ObjectType::Return(return_value) => *return_value.value,
        _ => evaluated,
    }
}
