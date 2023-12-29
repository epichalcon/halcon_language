use self::environment::Environment;
use crate::ast::{expressions::Identifier, statements::BlockStatement, Node};
use std::hash::Hash;
use std::{collections::HashMap, fmt::Debug, i128};

pub mod environment;

pub const INTEGER: &str = "INTEGER";
pub const BOOLEAN: &str = "BOOLEAN";
pub const NULL: &str = "NULL";
pub const RETURN: &str = "RETURN";
pub const ERROR: &str = "ERROR";
pub const FUNCTION: &str = "FUNCTION";
pub const STRING: &str = "STRING";
pub const BUILTIN: &str = "BUILTIN";
pub const ARRAY: &str = "ARRAY";
pub const DICT: &str = "DICT";

pub trait Object: Debug {
    fn object_type(&self) -> String;
    fn inspect(&self) -> String;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ObjectType {
    Integer(Integer),
    Boolean(Boolean),
    Null,
    Return(ReturnValue),
    Error(Error),
    Function(Function),
    String(StringObject),
    Builtin(Builtin),
    Array(Array),
    Dict(Dict),
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
            ObjectType::Array(ty) => ty.object_type(),
            ObjectType::Dict(ty) => ty.object_type(),
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
            ObjectType::Array(ty) => ty.inspect(),
            ObjectType::Dict(ty) => ty.inspect(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
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

impl Hash for Integer {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.object_type().hash(state);
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
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

impl Hash for Boolean {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.object_type().hash(state);
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Null {}

impl Object for Null {
    fn object_type(&self) -> String {
        NULL.to_string()
    }

    fn inspect(&self) -> String {
        "null".to_string()
    }
}

impl Hash for Null {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.object_type().hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl Hash for StringObject {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.object_type().hash(state);
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
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

impl Hash for ReturnValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
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

impl Hash for Error {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        panic!("not able to hash errors")
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

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.parameters == other.parameters && self.body == other.body
    }
}

impl Eq for Function {}

impl Hash for Function {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        panic!("not able to hash functions")
    }
}

pub type BuiltinFunction = fn(args: Vec<ObjectType>) -> ObjectType;

#[derive(Debug, Eq, PartialEq, Clone)]
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

impl Hash for Builtin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        panic!("not able to hash builtins")
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Array {
    pub elements: Vec<ObjectType>,
}

impl Object for Array {
    fn object_type(&self) -> String {
        return ARRAY.to_string();
    }

    fn inspect(&self) -> String {
        let elements =
            self.elements
                .iter()
                .enumerate()
                .fold(String::new(), |acc, (i, statement)| {
                    if i < self.elements.len() - 1 {
                        format!("{acc}{}, ", statement.inspect())
                    } else {
                        format!("{acc}{}", statement.inspect())
                    }
                });

        format!("[{}]", elements)
    }
}

impl Hash for Array {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        panic!("not able to hash Arrays")
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Dict {
    pub pairs: HashMap<ObjectType, ObjectType>,
}

impl Object for Dict {
    fn object_type(&self) -> String {
        return DICT.to_string();
    }

    fn inspect(&self) -> String {
        let pairs = self
            .pairs
            .iter()
            .enumerate()
            .fold(String::new(), |acc, (i, (key, val))| {
                if i < self.pairs.len() - 1 {
                    format!("{acc}{}: {}, ", key.inspect(), val.inspect())
                } else {
                    format!("{acc}{}: {}", key.inspect(), val.inspect())
                }
            });

        format!("{{{}}}", pairs)
    }
}

impl Hash for Dict {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        panic!("not able to hash dictionarys")
    }
}
