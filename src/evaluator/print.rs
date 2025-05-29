use super::{types::TypeId, *};
use std::fmt::{Display, Result};

pub fn insert_print(maps: &mut FunctionMap, ty: &TypeId) {
    let map = maps
        .get_mut(ty)
        .unwrap_or_else(|| panic!("function map for '{ty}' not found."));

    map.insert(
        "!".to_string(),
        Function {
            types: Vec::new(),
            code: FunctionCode::Builtin(print),
        },
    );
    map.insert(
        "!!".to_string(),
        Function {
            types: Vec::new(),
            code: FunctionCode::Builtin(println),
        },
    );
}

fn print(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    print!("{s}");
    Ok(s)
}

fn println(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    println!("{s}");
    Ok(s)
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        match self {
            Self::Nil => write!(f, "()"),
            Self::Top => write!(f, "T"),
            Self::I8(n) => write!(f, "{n}"),
            Self::U8(n) => write!(f, "{n}"),
            Self::I16(n) => write!(f, "{n}"),
            Self::U16(n) => write!(f, "{n}"),
            Self::I32(n) => write!(f, "{n}"),
            Self::U32(n) => write!(f, "{n}"),
            Self::I64(n) => write!(f, "{n}"),
            Self::U64(n) => write!(f, "{n}"),
            Self::I128(n) => write!(f, "{n}"),
            Self::U128(n) => write!(f, "{n}"),
            Self::F32(n) => write!(f, "{n}"),
            Self::F64(n) => write!(f, "{n}"),
            Self::String(n) => write!(f, "{n}"),
            Self::Symbol(n) => write!(f, "{n}"),
            Self::Array(n) => {
                let mut s = "[".to_string();
                for (i, m) in n.iter().enumerate() {
                    s.push_str(&m.to_string());
                    if i < n.len() - 1 {
                        s.push(' ');
                    }
                }
                s.push(']');
                write!(f, "{s}")
            }
            Self::Lazy(_) => write!(f, "{{}}"),
            Self::Function(_) => write!(f, "{{}}"),
            Self::Label(_) => panic!("tried to format label."),
        }
    }
}
