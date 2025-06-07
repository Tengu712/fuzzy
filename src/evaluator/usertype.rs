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

#[derive(Debug, Clone, PartialEq)]
pub struct UserType {
    pub mutable: bool,
    pub fields: Vec<UserTypeField>,
}

#[derive(Default)]
pub struct UserTypeMapStack {
    map: Vec<HashMap<String, UserType>>,
}

impl UserTypeMapStack {
    pub fn push(&mut self) {
        self.map.push(HashMap::new());
    }

    pub fn pop(&mut self) {
        self.map.pop();
    }

    pub fn get(&self, name: &str) -> Option<&UserType> {
        self.map.iter().rev().find_map(|n| n.get(name))
    }

    pub fn insert(&mut self, key: String, ut: UserType) -> RResult<()> {
        if let Some(n) = self.map.iter_mut().rev().find_map(|n| n.get_mut(&key)) {
            if n.mutable {
                *n = ut;
                Ok(())
            } else {
                Err(format!("error: cannot redefine type {key}.").into())
            }
        } else {
            self.map
                .last_mut()
                .expect("user-type map stack is empty.")
                .insert(key, ut);
            Ok(())
        }
    }
}
