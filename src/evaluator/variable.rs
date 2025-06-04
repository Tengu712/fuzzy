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

    pub fn is_mutable(&self, name: &str) -> Option<bool> {
        self.map
            .iter()
            .rev()
            .find_map(|n| n.get(name))
            .map(|n| n.mutable)
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.map
            .iter()
            .rev()
            .find_map(|n| n.get(name))
            .map(|n| &n.value)
    }

    pub fn get_unwrap(&self, name: &str) -> RResult<Value> {
        if let Some(n) = self.get(name) {
            // OPTIMIZE: remove clone.
            Ok(n.clone())
        } else {
            Err(format!("error: undefined variable {name} found.").into())
        }
    }

    pub fn insert(&mut self, key: String, value: Variable) -> RResult<()> {
        if let Some(n) = self.map.iter_mut().rev().find_map(|n| n.get_mut(&key)) {
            if n.mutable {
                *n = value;
                Ok(())
            } else {
                Err(format!("error: cannot redefine variable {key}.").into())
            }
        } else {
            self.map
                .last_mut()
                .expect("variable map stack is empty.")
                .insert(key, value);
            Ok(())
        }
    }
}
