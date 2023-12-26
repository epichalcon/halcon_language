use std::collections::HashMap;

use super::{Boolean, Integer, Null, Object, BOOLEAN, INTEGER, NULL};

#[derive(Clone, Debug)]
pub struct Environment {
    store: HashMap<String, (String, String)>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, name: String) -> Option<Box<dyn Object>> {
        let Some((object_type, value)) = self.store.get(&name) else {
            return None;
        };

        Some(create_object(object_type.to_string(), value.to_string()))
    }

    pub fn set(&mut self, name: &str, ty: String, val: String) {
        self.store.insert(name.to_string(), (ty, val));
    }
}

fn create_object(ty: String, val: String) -> Box<dyn Object> {
    match ty.as_str() {
        INTEGER => {
            if let Ok(integer) = val.parse::<i128>() {
                Box::new(Integer { value: integer })
            } else {
                panic!("could not transform Object into Integer")
            }
        }
        BOOLEAN => Box::new(Boolean {
            value: match val.as_str() {
                "true" => true,
                "false" => false,
                _ => panic!("Could not transform Object into Boolean"),
            },
        }),
        NULL => Box::new(Null {}),
        _ => panic!("not a valid type"),
    }
}
