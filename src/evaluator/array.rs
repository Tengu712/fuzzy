use super::*;

pub fn insert_array_functions(maps: &mut FunctionMap) {
    let map = maps
        .get_mut("[]")
        .unwrap_or_else(|| panic!("function map for '[]' not found."));
    map.insert(
        "#".to_string(),
        Function {
            types: Vec::new(),
            code: FunctionCode::Builtin(length),
        },
    );
}

fn length(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let s = unwrap(s, "#");
    Ok(Value::U32(s.len() as u32))
}

fn unwrap(s: Value, name: &str) -> Vec<Value> {
    if let Value::Array(s) = s {
        s
    } else {
        panic!("type missmatched on '[]:{name}'.");
    }
}
