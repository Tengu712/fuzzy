use super::*;

pub fn insert(fm: &mut FunctionMap) {
    fm.insert_all(
        &TypeId::Bool,
        vec![
            ("~".to_string(), (Vec::new(), FunctionCode::Builtin(not))),
            (
                ">>".to_string(),
                (vec![TypeId::Lazy], FunctionCode::Builtin(on_then)),
            ),
            (
                "!>".to_string(),
                (vec![TypeId::Lazy], FunctionCode::Builtin(on_else)),
            ),
            (
                "&&".to_string(),
                (vec![TypeId::Bool], FunctionCode::Builtin(and)),
            ),
            (
                "||".to_string(),
                (vec![TypeId::Bool], FunctionCode::Builtin(or)),
            ),
        ],
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
        let _ = eval_block(env, &mut o, Some(Vec::new()))?;
    }
    Ok(s)
}

fn on_else(env: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    if !unwrap_subject(s.clone(), "!>") {
        let mut o = unwrap_lazy_block(args.pop(), "!>");
        let _ = eval_block(env, &mut o, Some(Vec::new()))?;
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
