macro_rules! builtin_fn {
    ($name: expr, $types: expr, $fn: expr) => {
        (
            $name.to_string(),
            Function {
                mutable: false,
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
mod usertype;
mod variable;

use super::{types::*, value::Value, *};
use crate::RResult;

type BuiltinFunctionCode = fn(&mut Environment, Value, Vec<Value>) -> RResult<Value>;

pub enum TypesCheckResult {
    Undecided,
    Ok,
    Err(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    mutable: bool,
    private: bool,
    types: Vec<TypeId>,
    code: FunctionCode,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionCode {
    Builtin(BuiltinFunctionCode),
    UserDefined(Vec<Token>),
}

type FunctionMap = HashMap<TypeId, HashMap<String, Function>>;

#[derive(Default)]
pub struct FunctionMapStack {
    builtins: FunctionMap,
    users: Vec<FunctionMap>,
}

impl FunctionMapStack {
    pub fn push(&mut self) {
        if !self.users.is_empty() {
            self.users.push(HashMap::new());
            return;
        }

        self.users.push(HashMap::new());

        for n in ALL_PREMITIVE_TYPES {
            self.users
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
        usertype::insert(self);
    }

    pub fn pop(&mut self) {
        self.users.pop();
    }

    fn get(&self, ty: &TypeId, vn: &str) -> Option<&Function> {
        if let Some(n) = self.builtins.get(ty).and_then(|n| n.get(vn)) {
            Some(n)
        } else {
            self.users.iter().rev().find_map(|n| n.get(ty)?.get(vn))
        }
    }

    pub fn is_defined(&self, sty: Option<TypeId>, ty: &TypeId, vn: &str) -> bool {
        self.get(ty, vn)
            .map(|n| !n.private || sty.map(|n| &n == ty).unwrap_or(false))
            .unwrap_or(false)
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
        if !self.users.iter().any(|n| n.contains_key(&ty)) {
            self.users
                .last_mut()
                .expect("funciton map stack is empty.")
                .insert(ty, HashMap::new());
        }
    }

    fn insert_user_defined(&mut self, ty: &TypeId, vn: String, fun: Function) -> RResult<()> {
        if !self.users.iter().any(|n| n.contains_key(ty)) {
            return Err(format!("error: the type {ty} is not defined.").into());
        }
        if let Some(n) = self
            .users
            .iter_mut()
            .rev()
            .find_map(|n| n.get_mut(ty)?.get_mut(&vn))
        {
            *n = fun;
        } else {
            let n = self.users.last_mut().expect("function map stack is empty.");
            if !n.contains_key(ty) {
                n.insert(ty.clone(), HashMap::new());
            }
            n.get_mut(ty).unwrap().insert(vn, fun);
        }
        Ok(())
    }

    fn insert_builtins(&mut self, ty: &TypeId, funs: Vec<(String, Function)>) {
        if !self.builtins.contains_key(ty) {
            self.builtins.insert(ty.clone(), HashMap::new());
        }
        let n = self
            .builtins
            .get_mut(ty)
            .unwrap_or_else(|| panic!("function map for {ty} not inserted."));
        n.reserve(funs.len());
        n.extend(funs);
    }
}

fn convert_symbols_to_typeids(n: &[Value]) -> RResult<Vec<TypeId>> {
    let mut v = Vec::new();
    for n in n {
        match n {
            Value::Symbol(n) => v.push(TypeId::from(n)),
            Value::Array(n) => v.push(TypeId::Function(convert_symbols_to_typeids(n)?)),
            _ => return Err(format!("error: the element of type list must be symbol or array of symbols but passed '{}'.", n.typeid()).into()),
        }
    }
    Ok(v)
}
