use super::{types::TypeId, value::Value};
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

    pub fn get_unwrap(&self, sty: Option<TypeId>, name: &str) -> RResult<Value> {
        if let Some((pn, cn, private)) = split_member_access(name) {
            let Some(v) = self.get(pn) else {
                return Err(format!("error: undefined variable {pn} found.").into());
            };
            let Value::UserType((ty, n)) = v else {
                return Err(format!("error: {pn} is builtin-type but it has no field.").into());
            };
            let Some(n) = n.get(cn) else {
                return Err(format!("error: {pn} doesn't have the member {cn}.").into());
            };
            if private != n.private {
                let e = if n.private { "private" } else { "public" };
                let r = if private { "private" } else { "public" };
                return Err(
                    format!("error: {cn} of {pn} defined as {e} but specified {r}.").into(),
                );
            }
            if private && sty.map(|n| &n == ty).unwrap_or(false) {
                return Err(format!("error: {cn} of {pn} is private.").into());
            }
            return Ok(v.clone());
        }
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

fn split_member_access(name: &str) -> Option<(&str, &str, bool)> {
    if let Some((pn, cn)) = name.split_once("::") {
        Some((pn, cn, true))
    } else if let Some((pn, cn)) = name.split_once(":") {
        Some((pn, cn, false))
    } else {
        None
    }
}
