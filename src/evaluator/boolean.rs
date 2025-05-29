use super::{types::TypeId, *};

pub fn insert_bool_functions(maps: &mut FunctionMap) {
    let map = maps
        .get_mut(&TypeId::Bool)
        .unwrap_or_else(|| panic!("function map for '{}' not found.", TypeId::Bool));

    map.insert(
        "~".to_string(),
        Function {
            types: Vec::new(),
            code: FunctionCode::Builtin(not),
        },
    );
    map.insert(
        ">>".to_string(),
        Function {
            types: vec![TypeId::Lazy],
            code: FunctionCode::Builtin(on_then),
        },
    );
    map.insert(
        "!>".to_string(),
        Function {
            types: vec![TypeId::Lazy],
            code: FunctionCode::Builtin(on_else),
        },
    );
    map.insert(
        "&&".to_string(),
        Function {
            types: vec![TypeId::Bool],
            code: FunctionCode::Builtin(and),
        },
    );
    map.insert(
        "||".to_string(),
        Function {
            types: vec![TypeId::Bool],
            code: FunctionCode::Builtin(or),
        },
    );
}

fn not(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    if unwrap_subject(s, "~") {
        Ok(Value::Nil)
    } else {
        Ok(Value::Top)
    }
}

fn on_then(env: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    if unwrap_subject(s.clone(), ">>") {
        let mut o = unwrap_lazy_block(args.pop(), ">>");
        let _ = eval_block(env, &mut o)?;
    }
    Ok(s)
}

fn on_else(env: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    if !unwrap_subject(s.clone(), "!>") {
        let mut o = unwrap_lazy_block(args.pop(), "!>");
        let _ = eval_block(env, &mut o)?;
    }
    Ok(s)
}

fn and(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let s = unwrap_subject(s.clone(), "&&");
    let o = unwrap_object(args.pop(), "&&");
    if s && o {
        Ok(Value::Top)
    } else {
        Ok(Value::Nil)
    }
}

fn or(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let s = unwrap_subject(s.clone(), "||");
    let o = unwrap_object(args.pop(), "||");
    if s || o {
        Ok(Value::Top)
    } else {
        Ok(Value::Nil)
    }
}

fn unwrap_subject(s: Value, name: &str) -> bool {
    match s {
        Value::Nil => false,
        Value::Top => true,
        _ => panic!("type missmatched on '{}:{name}'.", TypeId::Bool),
    }
}

fn unwrap_object(s: Option<Value>, name: &str) -> bool {
    match s {
        Some(Value::Nil) => false,
        Some(Value::Top) => true,
        _ => panic!("type missmatched on '{}:{name}'.", TypeId::Bool),
    }
}

fn unwrap_lazy_block(s: Option<Value>, name: &str) -> Vec<Token> {
    if let Some(Value::Lazy(n)) = s {
        n
    } else {
        panic!("type missmatched on '{}:{name}'.", TypeId::Bool);
    }
}
