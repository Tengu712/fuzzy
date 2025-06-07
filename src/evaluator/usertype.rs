use super::types::TypeId;
use crate::RResult;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct UserTypeField {
    pub mutable: bool,
    pub private: bool,
    pub name: String,
    pub ty: TypeId,
}

#[derive(Default)]
pub struct UserTypeMapStack {
    map: Vec<HashMap<String, Vec<UserTypeField>>>,
}

impl UserTypeMapStack {
    pub fn push(&mut self) {
        self.map.push(HashMap::new());
    }

    pub fn pop(&mut self) {
        self.map.pop();
    }

    pub fn get(&self, name: &str) -> Option<&Vec<UserTypeField>> {
        self.map.iter().rev().find_map(|n| n.get(name))
    }

    pub fn insert(&mut self, key: String, ut: Vec<UserTypeField>) -> RResult<()> {
        if self.map.iter_mut().rev().any(|n| n.contains_key(&key)) {
            return Err(format!("error: cannot redefine type.").into());
        }
        self.map
            .last_mut()
            .expect("user-type map stack is empty.")
            .insert(key, ut);
        Ok(())
    }
}
