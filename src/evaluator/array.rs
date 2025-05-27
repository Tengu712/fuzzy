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
    map.insert(
        "@".to_string(),
        Function {
            types: vec!["i32".to_string()],
            code: FunctionCode::Builtin(at),
        },
    );
}

fn length(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let s = unwrap(s, "#");
    Ok(Value::U32(s.len() as u32))
}

fn at(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let s = unwrap(s, "@");
    let Some(Value::I32(o)) = args.pop() else {
        panic!("type missmatched on '[]:@'.");
    };
    let n = if o >= 0 {
        s.get(o as usize)
    } else if o.abs() <= s.len() as i32 {
        s.get((s.len() as i32 + o) as usize)
    } else {
        None
    };
    Ok(n.cloned().unwrap_or(Value::Nil))
}

fn unwrap(s: Value, name: &str) -> Vec<Value> {
    if let Value::Array(s) = s {
        s
    } else {
        panic!("type missmatched on '[]:{name}'.");
    }
}
