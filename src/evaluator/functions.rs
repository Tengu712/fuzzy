macro_rules! builtin_fn {
    ($name: expr, $types: expr, $fn: expr) => {
        (
            $name.to_string(),
            Function {
                private: false,
                types: $types,
                code: FunctionCode::Builtin($fn),
            },
        )
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
mod symbol;
mod variable;

use super::{
    types::{ALL_PREMITIVE_TYPES, TypeId},
    value::Value,
    *,
};
use crate::RResult;

type BuiltinFunctionCode = fn(&mut Environment, Value, Vec<Value>) -> RResult<Value>;

pub enum TypesCheckResult {
    Undecided,
    Ok,
    Err(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    private: bool,
    types: Vec<TypeId>,
    code: FunctionCode,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionCode {
    Builtin(BuiltinFunctionCode),
    UserDefined(Vec<Token>),
}

#[derive(Default)]
pub struct FunctionMapStack {
    map: Vec<HashMap<TypeId, HashMap<String, Function>>>,
}

impl FunctionMapStack {
    pub fn push(&mut self) {
        if !self.map.is_empty() {
            self.map.push(HashMap::new());
            return;
        }

        self.map.push(HashMap::new());
        for n in ALL_PREMITIVE_TYPES {
            self.map
                .last_mut()
                .unwrap()
                .insert(n.clone(), HashMap::new());
            cmp::insert(self, n);
            print::insert(self, n);
            variable::insert(self, n);
        }
        array::insert(self);
        boolean::insert(self);
        lazy::insert(self);
        numeric::insert(self);
        symbol::insert(self);
    }

    pub fn pop(&mut self) {
        self.map.pop();
    }

    fn get(&self, ty: &TypeId, vn: &str) -> Option<&Function> {
        self.map.iter().rev().find_map(|n| n.get(ty)?.get(vn))
    }

    pub fn is_defined(&self, ty: &TypeId, vn: &str) -> bool {
        self.get(ty, vn).is_some()
    }

    pub fn check_types(&self, ty: &TypeId, vn: &str, values: &[Value]) -> TypesCheckResult {
        let len = values.len();
        let expected = &self
            .get(ty, vn)
            .unwrap_or_else(|| panic!("{vn} on {ty} not defined."))
            .types;

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
                    "error: {vn} on {ty} expects {m} for #{i} but got {n}."
                ));
            }
        }

        if len < expected.len() {
            TypesCheckResult::Undecided
        } else {
            TypesCheckResult::Ok
        }
    }

    pub fn get_code(&self, ty: &TypeId, vn: &str) -> FunctionCode {
        self.get(ty, vn)
            .unwrap_or_else(|| panic!("{vn} on {ty} not defined."))
            .code
            .clone()
    }

    fn insert_new_type(&mut self, ty: TypeId) {
        if !self.map.iter().any(|n| n.contains_key(&ty)) {
            self.map
                .last_mut()
                .expect("funciton map stack is empty.")
                .insert(ty, HashMap::new());
        }
    }

    fn insert_user_defined(&mut self, ty: &TypeId, vn: String, fun: Function) -> RResult<()> {
        if !self.map.iter().any(|n| n.contains_key(ty)) {
            return Err(format!("error: the type {ty} is not defined.").into());
        }
        if let Some(n) = self
            .map
            .iter_mut()
            .rev()
            .find_map(|n| n.get_mut(ty)?.get_mut(&vn))
        {
            *n = fun;
        } else {
            let n = self.map.last_mut().expect("function map stack is empty.");
            if !n.contains_key(ty) {
                n.insert(ty.clone(), HashMap::new());
            }
            n.get_mut(ty).unwrap().insert(vn, fun);
        }
        Ok(())
    }

    fn insert_all(&mut self, ty: &TypeId, funs: Vec<(String, Function)>) {
        let n = self
            .map
            .last_mut()
            .expect("function map stack is empty.")
            .get_mut(ty)
            .unwrap_or_else(|| panic!("function map for {ty} not inserted."));
        n.reserve(funs.len());
        n.extend(funs);
    }
}
