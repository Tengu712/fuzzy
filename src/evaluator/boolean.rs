use super::*;

pub fn insert_bool_functions(maps: &mut FunctionMap) {
    let map = maps
        .get_mut("bool")
        .unwrap_or_else(|| panic!("function map for 'bool' not found."));
    map.insert(
        ">>".to_string(),
        Function {
            types: vec!["{}".to_string()],
            code: FunctionCode::Builtin(on_then),
        },
    );
    map.insert(
        "!>".to_string(),
        Function {
            types: vec!["{}".to_string()],
            code: FunctionCode::Builtin(on_else),
        },
    );
}

fn on_then(env: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    if unwrap_subject(s.clone(), ">>") {
        let mut o = unwrap_lazy_block(args.pop(), ">>");
        env.vr_map.push(HashMap::new());
        let _ = eval_block(env, &mut o)?;
        env.vr_map.pop();
    }
    Ok(s)
}

fn on_else(env: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    if !unwrap_subject(s.clone(), "!>") {
        let mut o = unwrap_lazy_block(args.pop(), "!>");
        env.vr_map.push(HashMap::new());
        let _ = eval_block(env, &mut o)?;
        env.vr_map.pop();
    }
    Ok(s)
}

fn unwrap_subject(s: Value, name: &str) -> bool {
    match s {
        Value::Nil => false,
        Value::Top => true,
        _ => panic!("type missmatched on 'bool:{name}'."),
    }
}

fn unwrap_lazy_block(s: Option<Value>, name: &str) -> Vec<Token> {
    if let Some(Value::Lazy(n)) = s {
        n
    } else {
        panic!("type missmatched on 'bool:{name}'.");
    }
}
