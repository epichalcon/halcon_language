use self::environment::Environment;
use crate::{
    ast::{expressions::Identifier, statements::BlockStatement, Node},
    Args,
};
use std::{fmt::Debug, i128};

pub mod environment;

pub const INTEGER: &str = "INTEGER";
pub const BOOLEAN: &str = "BOOLEAN";
pub const NULL: &str = "NULL";
pub const RETURN: &str = "RETURN";
pub const ERROR: &str = "ERROR";
pub const FUNCTION: &str = "FUNCTION";
pub const STRING: &str = "STRING";
pub const BUILTIN: &str = "BUILTIN";

pub trait Object: Debug {
    fn object_type(&self) -> String;
    fn inspect(&self) -> String;
}

#[derive(Debug, Clone)]
pub enum ObjectType {
    Integer(Integer),
    Boolean(Boolean),
    Null,
    Return(ReturnValue),
    Error(Error),
    Function(Function),
    String(StringObject),
    Builtin(Builtin),
}

impl Object for ObjectType {
    fn object_type(&self) -> String {
        match self {
            ObjectType::Integer(ty) => ty.object_type(),
            ObjectType::Boolean(ty) => ty.object_type(),
            ObjectType::Null => NULL.to_string(),
            ObjectType::Return(ty) => ty.object_type(),
            ObjectType::Error(ty) => ty.object_type(),
            ObjectType::Function(ty) => ty.object_type(),
            ObjectType::String(ty) => ty.object_type(),
            ObjectType::Builtin(ty) => ty.object_type(),
        }
    }

    fn inspect(&self) -> String {
        match self {
            ObjectType::Integer(ty) => ty.inspect(),
            ObjectType::Boolean(ty) => ty.inspect(),
            ObjectType::Null => "null".to_string(),
            ObjectType::Return(ty) => ty.inspect(),
            ObjectType::Error(ty) => ty.inspect(),
            ObjectType::Function(ty) => ty.inspect(),
            ObjectType::String(ty) => ty.inspect(),
            ObjectType::Builtin(ty) => ty.inspect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Integer {
    pub value: i128,
}

impl Object for Integer {
    fn object_type(&self) -> String {
        INTEGER.to_string()
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Boolean {
    pub value: bool,
}

impl Object for Boolean {
    fn object_type(&self) -> String {
        BOOLEAN.to_string()
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Null {}

impl Object for Null {
    fn object_type(&self) -> String {
        NULL.to_string()
    }

    fn inspect(&self) -> String {
        "null".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct StringObject {
    pub value: String,
}

impl Object for StringObject {
    fn object_type(&self) -> String {
        STRING.to_string()
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct ReturnValue {
    pub value: Box<ObjectType>,
}

impl Object for ReturnValue {
    fn object_type(&self) -> String {
        RETURN.to_string()
    }

    fn inspect(&self) -> String {
        self.value.inspect()
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
}

impl Object for Error {
    fn object_type(&self) -> String {
        ERROR.to_string()
    }

    fn inspect(&self) -> String {
        format!("ERROR: {}", self.message)
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
    pub env: Environment,
}

impl Object for Function {
    fn object_type(&self) -> String {
        FUNCTION.to_string()
    }

    fn inspect(&self) -> String {
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

        format!("fn({}) {{{}}}", parameters, self.body.string())
    }
}

pub type BuiltinFunction = fn(args: Vec<ObjectType>) -> ObjectType;

#[derive(Debug, Clone)]
pub struct Builtin {
    pub function: BuiltinFunction,
}

impl Object for Builtin {
    fn object_type(&self) -> String {
        BUILTIN.to_string()
    }

    fn inspect(&self) -> String {
        BUILTIN.to_string()
    }
}
