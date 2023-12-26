use std::collections::HashMap;

use super::ObjectType;

#[derive(Clone, Debug)]
pub struct Environment {
    store: HashMap<String, ObjectType>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed_environment(outer: Self) -> Self {
        let mut env = Self::new();
        env.outer = Some(Box::new(outer));
        env
    }

    pub fn get(&self, name: String) -> Option<&ObjectType> {
        match self.store.get(&name) {
            Some(obj) => Some(obj),
            None => match &self.outer {
                Some(outer) => outer.get(name),
                None => None,
            },
        }
    }

    pub fn set(&mut self, name: &str, obj: ObjectType) {
        self.store.insert(name.to_string(), obj);
    }
}
