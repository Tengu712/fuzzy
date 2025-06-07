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
        } else if let Some((var_name, member_name)) = split_member_access(name) {
            if let Some(var_value) = self.get(var_name) {
                match var_value {
                    Value::UserType((_, fields)) => {
                        let is_private = member_name.starts_with(':');
                        let actual_name = if is_private {
                            &member_name[1..]
                        } else {
                            member_name
                        };
                        
                        if let Some(member) = fields.get(actual_name) {
                            if member.private == is_private {
                                Ok(member.value.clone())
                            } else {
                                let access_type = if is_private { "private" } else { "public" };
                                let member_type = if member.private { "private" } else { "public" };
                                Err(format!("error: cannot access {} member {} with {} accessor.", member_type, actual_name, access_type).into())
                            }
                        } else {
                            Err(format!("error: member {} not found.", actual_name).into())
                        }
                    }
                    _ => Err(format!("error: variable {} is not a user-defined type.", var_name).into())
                }
            } else {
                Err(format!("error: undefined variable {} found.", var_name).into())
            }
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

    pub fn insert_self(&mut self, value: Value) {
        let n = Variable {
            mutable: true,
            value,
        };
        self.map
            .last_mut()
            .expect("variable map stack is empty.")
            .insert("##".to_string(), n);
    }
}

fn split_member_access(name: &str) -> Option<(&str, &str)> {
    if let Some((var_name, member_name)) = name.split_once("::") {
        Some((var_name, member_name))
    } else if let Some((var_name, member_name)) = name.split_once(":") {
        Some((var_name, member_name))
    } else {
        None
    }
}
