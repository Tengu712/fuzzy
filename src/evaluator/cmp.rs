use super::*;

pub fn insert_compare_functions(maps: &mut FunctionMap, ty: &str) {
    let tid = TypeId::Unit(ty.to_string());
    let map = maps
        .get_mut(&tid)
        .unwrap_or_else(|| panic!("function map for '{ty}' not found."));
    map.insert(
        "==".to_string(),
        Function {
            types: vec![tid.clone()],
            code: FunctionCode::Builtin(equal),
        },
    );
    map.insert(
        "!=".to_string(),
        Function {
            types: vec![tid.clone()],
            code: FunctionCode::Builtin(not_equal),
        },
    );

    if ty == "[]" || ty == "{}" {
        return;
    }

    map.insert(
        "<".to_string(),
        Function {
            types: vec![tid.clone()],
            code: FunctionCode::Builtin(l),
        },
    );
    map.insert(
        ">".to_string(),
        Function {
            types: vec![tid.clone()],
            code: FunctionCode::Builtin(g),
        },
    );
    map.insert(
        "<=".to_string(),
        Function {
            types: vec![tid.clone()],
            code: FunctionCode::Builtin(le),
        },
    );
    map.insert(
        ">=".to_string(),
        Function {
            types: vec![tid.clone()],
            code: FunctionCode::Builtin(ge),
        },
    );
}

fn equal(_: &mut Environment, s: Value, args: Vec<Value>) -> RResult<Value> {
    let o = pop_object(&s, args, "==");
    if s.equal(&o) {
        Ok(Value::Top)
    } else {
        Ok(Value::Nil)
    }
}

fn not_equal(_: &mut Environment, s: Value, args: Vec<Value>) -> RResult<Value> {
    let o = pop_object(&s, args, "!=");
    if !s.equal(&o) {
        Ok(Value::Top)
    } else {
        Ok(Value::Nil)
    }
}

fn l(_: &mut Environment, s: Value, args: Vec<Value>) -> RResult<Value> {
    let o = pop_object(&s, args, "<");
    if s.l(&o) {
        Ok(Value::Top)
    } else {
        Ok(Value::Nil)
    }
}

fn g(_: &mut Environment, s: Value, args: Vec<Value>) -> RResult<Value> {
    let o = pop_object(&s, args, ">");
    if s.g(&o) {
        Ok(Value::Top)
    } else {
        Ok(Value::Nil)
    }
}

fn le(_: &mut Environment, s: Value, args: Vec<Value>) -> RResult<Value> {
    let o = pop_object(&s, args, "<=");
    if s.l(&o) || s.equal(&o) {
        Ok(Value::Top)
    } else {
        Ok(Value::Nil)
    }
}

fn ge(_: &mut Environment, s: Value, args: Vec<Value>) -> RResult<Value> {
    let o = pop_object(&s, args, ">=");
    if s.g(&o) || s.equal(&o) {
        Ok(Value::Top)
    } else {
        Ok(Value::Nil)
    }
}

fn pop_object(s: &Value, mut args: Vec<Value>, name: &str) -> Value {
    if let Some(o) = args.pop() {
        o
    } else {
        panic!("type missmatched on '{}:{name}'.", s.get_typeid().format());
    }
}

macro_rules! define_inequality_compare {
    ($fn:ident, $op:tt) => {
        fn $fn(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Nil, Self::Nil) => false,
                (Self::Top, Self::Top) => false,
                (Self::I8(a), Self::I8(b)) => a $op b,
                (Self::U8(a), Self::U8(b)) => a $op b,
                (Self::I16(a), Self::I16(b)) => a $op b,
                (Self::U16(a), Self::U16(b)) => a $op b,
                (Self::I32(a), Self::I32(b)) => a $op b,
                (Self::U32(a), Self::U32(b)) => a $op b,
                (Self::I64(a), Self::I64(b)) => a $op b,
                (Self::U64(a), Self::U64(b)) => a $op b,
                (Self::I128(a), Self::I128(b)) => a $op b,
                (Self::U128(a), Self::U128(b)) => a $op b,
                (Self::F32(a), Self::F32(b)) => a $op b,
                (Self::F64(a), Self::F64(b)) => a $op b,
                (Self::String(a), Self::String(b)) => a $op b,
                (Self::Symbol(a), Self::Symbol(b)) => a $op b,
                (Self::Array(_), _) | (_, Self::Array(_)) => {
                    panic!("tried to compare array.");
                }
                (Self::Lazy(_), _) | (_, Self::Lazy(_)) => {
                    panic!("tried to compare lazy.");
                }
                (Self::Label(_), _) | (_, Self::Label(_)) => {
                    panic!("tried to compare label.");
                }
                _ => panic!(
                    "tried to compare {} and {}",
                    self.get_typeid().format(),
                    other.get_typeid().format(),
                ),
            }
        }
    };
}

impl Value {
    fn equal(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nil, Self::Nil) => true,
            (Self::Top, Self::Top) => true,
            (Self::I8(a), Self::I8(b)) => a == b,
            (Self::U8(a), Self::U8(b)) => a == b,
            (Self::I16(a), Self::I16(b)) => a == b,
            (Self::U16(a), Self::U16(b)) => a == b,
            (Self::I32(a), Self::I32(b)) => a == b,
            (Self::U32(a), Self::U32(b)) => a == b,
            (Self::I64(a), Self::I64(b)) => a == b,
            (Self::U64(a), Self::U64(b)) => a == b,
            (Self::I128(a), Self::I128(b)) => a == b,
            (Self::U128(a), Self::U128(b)) => a == b,
            (Self::F32(a), Self::F32(b)) => a == b,
            (Self::F64(a), Self::F64(b)) => a == b,
            (Self::String(a), Self::String(b)) => a == b,
            (Self::Symbol(a), Self::Symbol(b)) => a == b,
            (Self::Array(a), Self::Array(b)) => {
                a.len() == b.len()
                    && a.iter()
                        .zip(b.iter())
                        .all(|(x, y)| x.get_typeid() == y.get_typeid() && x.equal(y))
            }
            (Self::Lazy(a), Self::Lazy(b)) => a == b,
            (Self::Label(_), _) | (_, Self::Label(_)) => {
                panic!("tried to compare label.");
            }
            _ => panic!(
                "tried to compare {} and {}",
                self.get_typeid().format(),
                other.get_typeid().format(),
            ),
        }
    }

    define_inequality_compare!(l, <);
    define_inequality_compare!(g, >);
}
