use super::*;

pub fn insert(fm: &mut FunctionMapStack) {
    fm.insert_builtins(
        &TypeId::String,
        vec![
            builtin_fn!("#", vec![], length),
            builtin_fn!("^", vec![], first),
            builtin_fn!("$", vec![], last),
            builtin_fn!("@", vec![TypeId::I32], at),
            builtin_fn!("@<", vec![TypeId::I32, TypeId::String], ins),
            builtin_fn!("@-", vec![TypeId::I32], remove),
            builtin_fn!("$>", vec![TypeId::String], push),
            builtin_fn!("$-", vec![], pop),
            builtin_fn!("=@", vec![TypeId::String, TypeId::String], replace),
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

fn ins(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let mut s = extract_variant!(s, String);

    let o = pop_extract_variant!(args, I32);
    let i = convert_index(o, s.len())?;

    let n = pop_extract_variant!(args, String);
    if n.chars().count() != 1 {
        return Err(format!(
            "error: the replacement string must be a single character, but '{n}' was provided."
        )
        .into());
    }
    let n = n.chars().next().unwrap();

    s.insert(i, n);
    Ok(Value::String(s))
}

fn remove(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let mut s = extract_variant!(s, String);
    let o = pop_extract_variant!(args, I32);
    let i = convert_index(o, s.len())?;
    s.remove(i);
    Ok(Value::String(s))
}

fn push(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, String);
    let o = pop_extract_variant!(args, String);
    Ok(Value::String(format!("{}{}", s, o)))
}

fn pop(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let mut s = extract_variant!(s, String);
    s.pop();
    Ok(Value::String(s))
}

fn replace(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, String);
    let o = pop_extract_variant!(args, String);
    let n = pop_extract_variant!(args, String);
    Ok(Value::String(s.replace(&o, &n)))
}
