use std::fmt::Debug;

pub mod environment;

pub const INTEGER: &str = "INTEGER";
pub const BOOLEAN: &str = "BOOLEAN";
pub const NULL: &str = "NULL";
pub const RETURN: &str = "RETURN";
pub const ERROR: &str = "ERROR";

pub trait Object: Debug {
    fn object_type(&self) -> String;
    fn inspect(&self) -> String;
}

#[derive(Debug)]
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

impl From<Box<dyn Object>> for Integer {
    fn from(value: Box<dyn Object>) -> Self {
        if let Ok(integer) = value.inspect().parse::<i128>() {
            Integer { value: integer }
        } else {
            panic!("could not transform Object into Integer")
        }
    }
}

#[derive(Debug)]
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

impl From<Box<dyn Object>> for Boolean {
    fn from(value: Box<dyn Object>) -> Self {
        let boolean_string = value.inspect();
        Boolean {
            value: match boolean_string.as_str() {
                "true" => true,
                "false" => false,
                _ => panic!("Could not transform Object into Boolean"),
            },
        }
    }
}

#[derive(Debug)]
pub struct Null {}

impl Object for Null {
    fn object_type(&self) -> String {
        NULL.to_string()
    }

    fn inspect(&self) -> String {
        "null".to_string()
    }
}

impl From<Box<dyn Object>> for Null {
    fn from(value: Box<dyn Object>) -> Self {
        let value = value.inspect();
        if value == "null" {
            Null {}
        } else {
            panic!("Could not transform Object into Null");
        }
    }
}

#[derive(Debug)]
pub struct ReturnValue {
    pub value: Box<dyn Object>,
}

impl Object for ReturnValue {
    fn object_type(&self) -> String {
        RETURN.to_string()
    }

    fn inspect(&self) -> String {
        self.value.inspect()
    }
}

impl From<Box<dyn Object>> for ReturnValue {
    fn from(value: Box<dyn Object>) -> Self {
        let ret_value: Box<dyn Object> = match value.object_type().as_str() {
            BOOLEAN => {
                if value.inspect() == "true" {
                    Box::new(Boolean { value: true })
                } else {
                    Box::new(Boolean { value: false })
                }
            }
            INTEGER => Box::new(Integer {
                value: value.inspect().parse().unwrap(),
            }),
            NULL => Box::new(Null {}),
            RETURN => value,
            _ => panic!("not a valid type"),
        };

        Self { value: ret_value }
    }
}

#[derive(Debug)]
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

impl From<Box<dyn Object>> for Error {
    fn from(value: Box<dyn Object>) -> Self {
        if value.object_type() != ERROR {
            panic!("Could not tranform into ERROR")
        }

        Self {
            message: value.inspect()[7..].to_string(),
        }
    }
}
