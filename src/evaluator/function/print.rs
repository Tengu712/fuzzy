use super::*;

pub fn insert_print(maps: &mut FunctionMap, ty: &str) {
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

macro_rules! define_print {
    ($fn: ident) => {
        fn $fn(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
            match &s {
                Value::Nil => $fn!("()"),
                Value::I8(n) => $fn!("{n}"),
                Value::U8(n) => $fn!("{n}"),
                Value::I16(n) => $fn!("{n}"),
                Value::U16(n) => $fn!("{n}"),
                Value::I32(n) => $fn!("{n}"),
                Value::U32(n) => $fn!("{n}"),
                Value::I64(n) => $fn!("{n}"),
                Value::U64(n) => $fn!("{n}"),
                Value::I128(n) => $fn!("{n}"),
                Value::U128(n) => $fn!("{n}"),
                Value::F32(n) => $fn!("{n}"),
                Value::F64(n) => $fn!("{n}"),
                Value::String(n) => $fn!("{n}"),
                Value::Symbol(n) => $fn!("{n}"),
                Value::ExpansionSymbol(_) => panic!("tried to print expansion symbol."),
            }
            Ok(s)
        }
    };
}
define_print!(print);
define_print!(println);
