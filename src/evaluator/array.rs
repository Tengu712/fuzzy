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
    map.insert(
        "@@".to_string(),
        Function {
            types: vec!["i32".to_string(), "_".to_string()],
            code: FunctionCode::Builtin(replace),
        },
    );
}

fn length(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let s = unwrap_subject(s, "#");
    Ok(Value::U32(s.len() as u32))
}

fn at(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let s = unwrap_subject(s, "@");
    let o = unwrap_index(args.pop(), "@");
    let n = if o >= 0 {
        s.get(o as usize)
    } else {
        let i = o + s.len() as i32;
        if i >= 0 { s.get(i as usize) } else { None }
    };
    Ok(n.cloned().unwrap_or(Value::Nil))
}

fn replace(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let mut s = unwrap_subject(s, "@@");
    let o = unwrap_index(args.pop(), "@@");
    let Some(t) = args.pop() else {
        panic!("type missmatched on '[]:@@'.");
    };
    let n = if o >= 0 {
        s.get_mut(o as usize)
    } else {
        let i = o + s.len() as i32;
        if i >= 0 { s.get_mut(i as usize) } else { None }
    };
    if let Some(n) = n {
        *n = t;
    }
    Ok(Value::Array(s))
}

fn unwrap_subject(s: Value, name: &str) -> Vec<Value> {
    if let Value::Array(s) = s {
        s
    } else {
        panic!("type missmatched on '[]:{name}'.");
    }
}

fn unwrap_index(o: Option<Value>, name: &str) -> i32 {
    if let Some(Value::I32(o)) = o {
        o
    } else {
        panic!("type missmatched on '[]:{name}'.");
    }
}
