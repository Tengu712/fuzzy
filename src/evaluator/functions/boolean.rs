use super::*;

pub fn insert(fm: &mut FunctionMapStack) {
    fm.insert_all(
        &TypeId::Bool,
        vec![
            builtin_fn!("~", vec![], not),
            builtin_fn!(">>", vec![TypeId::Lazy], on_then),
            builtin_fn!("!>", vec![TypeId::Lazy], on_else),
            builtin_fn!("&&", vec![TypeId::Bool], and),
            builtin_fn!("||", vec![TypeId::Bool], or),
        ],
    );
}

fn not(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    if unwrap_subject(&s) {
        Ok(Value::Nil)
    } else {
        Ok(Value::Top)
    }
}

fn on_then(env: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    if unwrap_subject(&s) {
        let mut o = pop_extract_variant!(args, Lazy);
        let params = EnterLazyParams {
            slf: None,
            args: Some(vec![]),
        };
        let _ = eval_block(env, &mut o, params)?;
    }
    Ok(s)
}

fn on_else(env: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    if !unwrap_subject(&s) {
        let mut o = pop_extract_variant!(args, Lazy);
        let params = EnterLazyParams {
            slf: None,
            args: Some(vec![]),
        };
        let _ = eval_block(env, &mut o, params)?;
    }
    Ok(s)
}

fn and(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let s = unwrap_subject(&s);
    let o = unwrap_object(&mut args);
    if s && o {
        Ok(Value::Top)
    } else {
        Ok(Value::Nil)
    }
}

fn or(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let s = unwrap_subject(&s);
    let o = unwrap_object(&mut args);
    if s || o {
        Ok(Value::Top)
    } else {
        Ok(Value::Nil)
    }
}

fn unwrap_subject(s: &Value) -> bool {
    match s {
        Value::Nil => false,
        Value::Top => true,
        _ => panic!("type missmatched."),
    }
}

fn unwrap_object(args: &mut Vec<Value>) -> bool {
    match args.pop() {
        Some(Value::Nil) => false,
        Some(Value::Top) => true,
        _ => panic!("type missmatched."),
    }
}
