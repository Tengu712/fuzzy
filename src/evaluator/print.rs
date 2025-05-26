use super::*;

pub fn insert_print(maps: &mut FunctionMap, ty: &str) {
    if ty == "{}" {
        return;
    }
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
    print!("{}", s.format());
    Ok(s)
}

fn println(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    println!("{}", s.format());
    Ok(s)
}

impl Value {
    fn format(&self) -> String {
        match self {
            Self::Nil => "()".to_string(),
            Self::I8(n) => n.to_string(),
            Self::U8(n) => n.to_string(),
            Self::I16(n) => n.to_string(),
            Self::U16(n) => n.to_string(),
            Self::I32(n) => n.to_string(),
            Self::U32(n) => n.to_string(),
            Self::I64(n) => n.to_string(),
            Self::U64(n) => n.to_string(),
            Self::I128(n) => n.to_string(),
            Self::U128(n) => n.to_string(),
            Self::F32(n) => n.to_string(),
            Self::F64(n) => n.to_string(),
            Self::String(n) => n.clone(),
            Self::Symbol(n) => n.clone(),
            Self::Array(n) => {
                let mut s = "[".to_string();
                for (i, m) in n.iter().enumerate() {
                    s.push_str(&m.format());
                    if i < n.len() - 1 {
                        s.push(' ');
                    }
                }
                s.push(']');
                s
            }
            Self::Lazy(_) => "{}".to_string(),
            Self::Label(_) => panic!("tried to format label."),
        }
    }
}
