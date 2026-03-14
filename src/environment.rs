use crate::enums::Expression;
use crate::enums::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub enum EnvironmentRecord {
    Value(Value),
    Expression(Box<Expression>),
}

#[derive(Clone)]
pub struct Environment {
    pub records: HashMap<String, EnvironmentRecord>,
    pub parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn get(&self, name: &str) -> Option<EnvironmentRecord> {
        if let Some(value) = self.records.get(name) {
            Some(value.clone())
        } else if let Some(parent) = &self.parent {
            parent.borrow().get(name)
        } else {
            None
        }
    }

    pub fn set(&mut self, name: String, record: EnvironmentRecord) {
        if self.records.contains_key(&name) {
            self.records.insert(name, record);
            return;
        }

        if let Some(parent) = &self.parent {
            if parent.borrow().has(&name) {
                parent.borrow_mut().set(name, record);
                return;
            }
        }

        self.records.insert(name, record);
    }

    pub fn has(&self, name: &str) -> bool {
        if self.records.contains_key(name) {
            return true;
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().has(name);
        }

        false
    }
}
