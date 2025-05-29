mod array;
mod boolean;
mod cmp;
mod lazy;
mod numeric;
mod print;
mod symbol;
mod variable;

use super::{
    types::{ALL_PREMITIVE_TYPES, TypeId},
    value::Value,
    *,
};
use crate::RResult;

type BuiltinFunctionCode = fn(&mut Environment, Value, Vec<Value>) -> RResult<Value>;
type Function = (Vec<TypeId>, FunctionCode);

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionCode {
    Builtin(BuiltinFunctionCode),
}

pub struct FunctionMap {
    map: HashMap<TypeId, HashMap<String, Function>>,
}

impl Default for FunctionMap {
    fn default() -> Self {
        let mut map = FunctionMap {
            map: HashMap::new(),
        };

        for n in ALL_PREMITIVE_TYPES {
            map.map.insert(n.clone(), HashMap::new());
            cmp::insert(&mut map, n);
            print::insert(&mut map, n);
            variable::insert(&mut map, n);
        }
        array::insert(&mut map);
        boolean::insert(&mut map);
        lazy::insert(&mut map);
        numeric::insert(&mut map);
        symbol::insert(&mut map);

        map
    }
}

impl FunctionMap {
    pub fn is_defined(&self, ty: &TypeId, name: &str) -> bool {
        self.map
            .get(ty)
            .map(|n| n.contains_key(name))
            .unwrap_or(false)
    }

    pub fn get_types(&self, ty: &TypeId, name: &str) -> &Vec<TypeId> {
        &self.get(ty, name).0
    }

    pub fn get_code(&self, ty: &TypeId, name: &str) -> FunctionCode {
        self.get(ty, name).1.clone()
    }

    fn get(&self, ty: &TypeId, name: &str) -> &Function {
        self.map
            .get(ty)
            .unwrap_or_else(|| panic!("no function defined on {ty}."))
            .get(name)
            .unwrap_or_else(|| panic!("function {name} not defined on {ty}."))
    }

    fn insert(&mut self, ty: &TypeId, name: String, fun: Function) {
        if !self.map.contains_key(ty) {
            self.map.insert(ty.clone(), HashMap::new());
        }
        self.map.get_mut(ty).unwrap().insert(name, fun);
    }

    fn insert_all(&mut self, ty: &TypeId, funs: Vec<(String, Function)>) {
        if !self.map.contains_key(ty) {
            self.map.insert(ty.clone(), HashMap::new());
        }
        self.map.get_mut(ty).unwrap().reserve(funs.len());
        self.map.get_mut(ty).unwrap().extend(funs);
    }
}
