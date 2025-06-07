use super::*;

pub fn insert(fm: &mut FunctionMapStack) {
    fm.insert_builtins(
        &TypeId::String,
        vec![
            builtin_fn!("#", vec![], length),
            builtin_fn!("^", vec![], first),
            builtin_fn!("$", vec![], last),
            builtin_fn!("@", vec![TypeId::I32], at),
            builtin_fn!("+", vec![TypeId::String], concat),
        ],
    );
}

fn length(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, String);
    Ok(Value::U32(s.chars().count() as u32))
}

fn first(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, String);
    Ok(Value::String(s.chars().next().unwrap_or('\0').to_string()))
}

fn last(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, String);
    Ok(Value::String(s.chars().last().unwrap_or('\0').to_string()))
}

fn at(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, String);
    let o = pop_extract_variant!(args, I32);
    let chars: Vec<char> = s.chars().collect();
    let n = if o >= 0 {
        chars.get(o as usize)
    } else {
        let i = o + chars.len() as i32;
        if i >= 0 { chars.get(i as usize) } else { None }
    };
    Ok(Value::String(n.unwrap_or(&'\0').to_string()))
}

fn concat(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, String);
    let o = pop_extract_variant!(args, String);
    Ok(Value::String(format!("{}{}", s, o)))
}
