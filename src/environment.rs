use crate::enums::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct Environment {
    pub values: HashMap<String, Value>,
    pub parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.values.get(name) {
            Some(value.clone())
        } else if let Some(parent) = &self.parent {
            parent.borrow().get(name)
        } else {
            None
        }
    }

    pub fn set(&mut self, name: String, value: Value) {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            return;
        }

        if let Some(parent) = &self.parent {
            if parent.borrow().has(&name) {
                parent.borrow_mut().set(name, value);
                return;
            }
        }

        self.values.insert(name, value);
    }

    pub fn has(&self, name: &str) -> bool {
        if self.values.contains_key(name) {
            return true;
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().has(name);
        }

        false
    }
}
