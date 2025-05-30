use super::value::Value;
use crate::RResult;
use std::collections::HashMap;

pub struct Variable {
    pub value: Value,
    pub mutable: bool,
}

#[derive(Default)]
pub struct VariableMapStack {
    map: Vec<HashMap<String, Variable>>,
}

impl VariableMapStack {
    pub fn push(&mut self) {
        self.map.push(HashMap::new());
    }

    pub fn pop(&mut self) {
        self.map.pop();
    }

    pub fn get(&self, name: &str) -> Option<&Variable> {
        self.map.iter().rev().find_map(|n| n.get(name))
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Variable> {
        self.map.iter_mut().rev().find_map(|n| n.get_mut(name))
    }

    pub fn get_unwrap(&self, name: &str) -> RResult<Value> {
        if let Some(n) = self.get(name) {
            // OPTIMIZE: remove clone.
            Ok(n.value.clone())
        } else {
            Err(format!("error: undefined variable {name} found.").into())
        }
    }

    pub fn insert(&mut self, key: String, value: Variable) {
        self.map
            .last_mut()
            .expect("variable map stack is empty.")
            .insert(key, value);
    }
}
