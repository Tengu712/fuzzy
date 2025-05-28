use super::*;

pub fn insert_compare_functions(maps: &mut FunctionMap, ty: &str) {
    let map = maps
        .get_mut(ty)
        .unwrap_or_else(|| panic!("function map for '{ty}' not found."));
    map.insert(
        "==".to_string(),
        Function {
            types: vec![ty.to_string()],
            code: FunctionCode::Builtin(equal),
        },
    );
}

fn equal(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let Some(o) = args.pop() else {
        panic!("type missmatched on '{}:=='.", s.get_typeid());
    };
    if s.equal(&o) {
        Ok(Value::Top)
    } else {
        Ok(Value::Nil)
    }
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
                a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x.equal(y))
            }
            (Self::Lazy(a), Self::Lazy(b)) => a == b,
            (Self::Label(_), _) | (_, Self::Label(_)) => {
                panic!("tried to compare label.");
            }
            _ => false,
        }
    }
}
