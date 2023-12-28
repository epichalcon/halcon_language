use super::*;
use crate::object::{Builtin, ObjectType};

pub fn get_builtin_function(id: &str) -> ObjectType {
    match id {
        "len" => ObjectType::Builtin(Builtin { function: length }),
        "first" => ObjectType::Builtin(Builtin { function: first }),
        "last" => ObjectType::Builtin(Builtin { function: last }),
        "rest" => ObjectType::Builtin(Builtin { function: rest }),
        "push" => ObjectType::Builtin(Builtin { function: push }),
        _ => return new_error(format!("identifier not found: {}", id)),
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
        ObjectType::Array(a) => ObjectType::Integer(Integer {
            value: a.elements.len().try_into().unwrap(),
        }),
        _ => new_error(format!(
            "argument to len not supported, got {}",
            args[0].object_type()
        )),
    }
}

fn first(args: Vec<ObjectType>) -> ObjectType {
    if args.len() != 1 {
        return new_error(format!(
            "wrong number of arguments. got: {}, want: 1",
            args.len()
        ));
    }

    match &args[0] {
        ObjectType::String(s) => {
            if s.value.len() > 0 {
                ObjectType::String(StringObject {
                    value: s.value.chars().next().unwrap().to_string(),
                })
            } else {
                ObjectType::Null
            }
        }
        ObjectType::Array(a) => {
            if a.elements.len() > 0 {
                a.elements[0].clone()
            } else {
                ObjectType::Null
            }
        }
        _ => new_error(format!(
            "argument to first not supported, got {}",
            args[0].object_type()
        )),
    }
}

fn last(args: Vec<ObjectType>) -> ObjectType {
    if args.len() != 1 {
        return new_error(format!(
            "wrong number of arguments. got: {}, want: 1",
            args.len()
        ));
    }

    match &args[0] {
        ObjectType::String(s) => {
            if s.value.len() > 0 {
                ObjectType::String(StringObject {
                    value: s.value.chars().last().unwrap().to_string(),
                })
            } else {
                ObjectType::Null
            }
        }
        ObjectType::Array(a) => {
            if a.elements.len() > 0 {
                a.elements.last().unwrap().clone()
            } else {
                ObjectType::Null
            }
        }
        _ => new_error(format!(
            "argument to first not supported, got {}",
            args[0].object_type()
        )),
    }
}

fn rest(args: Vec<ObjectType>) -> ObjectType {
    if args.len() != 1 {
        return new_error(format!(
            "wrong number of arguments. got: {}, want: 1",
            args.len()
        ));
    }

    match &args[0] {
        ObjectType::String(s) => {
            if s.value.len() > 0 {
                ObjectType::String(StringObject {
                    value: s.value[1..].to_string(),
                })
            } else {
                ObjectType::Null
            }
        }
        ObjectType::Array(a) => {
            if a.elements.len() > 0 {
                ObjectType::Array(Array {
                    elements: a.elements[1..].to_vec(),
                })
            } else {
                ObjectType::Null
            }
        }
        _ => new_error(format!(
            "argument to first not supported, got {}",
            args[0].object_type()
        )),
    }
}

fn push(args: Vec<ObjectType>) -> ObjectType {
    if args.len() != 2 {
        return new_error(format!(
            "wrong number of arguments. got: {}, want: 2",
            args.len()
        ));
    }

    match &args[0] {
        // ObjectType::String(s) => {
        //     if s.value.len() > 0 {
        //         ObjectType::String(StringObject {
        //             value: format!("{}{}", s.value, args[1].),
        //         })
        //     } else {
        //         ObjectType::Null
        //     }
        // }
        ObjectType::Array(a) => {
            let mut new_arr = a.elements.clone();
            new_arr.push(args[1].clone());
            ObjectType::Array(Array { elements: new_arr })
        }
        _ => new_error(format!(
            "argument to first not supported, got {}",
            args[0].object_type()
        )),
    }
}
