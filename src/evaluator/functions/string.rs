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
    let n = s.chars().next().map(|n| Value::String(n.to_string()));
    Ok(n.unwrap_or_default())
}

fn last(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, String);
    let n = s.chars().last().map(|n| Value::String(n.to_string()));
    Ok(n.unwrap_or_default())
}

fn at(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, String);
    let s = s.chars().collect::<Vec<_>>();
    let o = pop_extract_variant!(args, I32);
    let Ok(i) = convert_index(o, s.len()) else {
        return Ok(Value::Nil);
    };
    Ok(Value::String(s[i].to_string()))
}

fn concat(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, String);
    let o = pop_extract_variant!(args, String);
    Ok(Value::String(format!("{}{}", s, o)))
}
