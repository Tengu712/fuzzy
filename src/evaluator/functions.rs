macro_rules! builtin_fn {
    ($name: expr, $types: expr, $fn: expr) => {
        ($name.to_string(), ($types, FunctionCode::Builtin($fn)))
    };
}

macro_rules! extract_variant {
    ($value: expr, $variant: ident) => {
        match $value {
            Value::$variant(n) => n,
            _ => panic!("type missmatched."),
        }
    };
}

macro_rules! pop_extract_variant {
    ($value: expr, $variant: ident) => {
        match $value.pop() {
            Some(Value::$variant(n)) => n,
            _ => panic!("type missmatched."),
        }
    };
}

mod array;
mod boolean;
mod cmp;
mod lazy;
mod numeric;
mod print;
mod variable;

use super::{
    types::{ALL_PREMITIVE_TYPES, TypeId},
    value::Value,
    *,
};
use crate::RResult;

type BuiltinFunctionCode = fn(&mut Environment, Value, Vec<Value>) -> RResult<Value>;
type Function = (Vec<TypeId>, FunctionCode);

pub enum TypesCheckResult {
    Undecided,
    Ok,
    Err(String),
}

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

    pub fn check_types(&self, ty: &TypeId, name: &str, values: &[Value]) -> TypesCheckResult {
        let len = values.len();
        let expected = &self.get(ty, name).0;

        if len > expected.len() {
            panic!("too many arguments passed.");
        }

        for (i, (n, m)) in values
            .iter()
            .map(|n| n.typeid())
            .zip(expected.iter())
            .enumerate()
        {
            if &n != m && m != &TypeId::Any {
                return TypesCheckResult::Err(format!(
                    "error: {name} on {ty} expects {m} for #{i} but got {n}."
                ));
            }
        }

        if len < expected.len() {
            TypesCheckResult::Undecided
        } else {
            TypesCheckResult::Ok
        }
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

    fn insert_all(&mut self, ty: &TypeId, funs: Vec<(String, Function)>) {
        if !self.map.contains_key(ty) {
            self.map.insert(ty.clone(), HashMap::new());
        }
        self.map.get_mut(ty).unwrap().reserve(funs.len());
        self.map.get_mut(ty).unwrap().extend(funs);
    }
}
