use super::*;

pub fn insert(fm: &mut FunctionMapStack) {
    fm.insert_all(
        &TypeId::Array,
        vec![
            builtin_fn!("#", vec![], length),
            builtin_fn!("^", vec![], first),
            builtin_fn!("$", vec![], last),
            builtin_fn!("@", vec![TypeId::I32], at),
            builtin_fn!("@@", vec![TypeId::I32, TypeId::Any], replace),
            builtin_fn!("@<", vec![TypeId::I32, TypeId::Any], ins),
            builtin_fn!("@-", vec![TypeId::I32], remove),
            builtin_fn!("$>", vec![TypeId::Any], push),
            builtin_fn!("$-", vec![], pop),
        ],
    );
}

fn length(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, Array);
    Ok(Value::U32(s.len() as u32))
}

fn first(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, Array);
    // OPTIMIZE: remove clone.
    Ok(s.first().cloned().unwrap_or_default())
}

fn last(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, Array);
    // OPTIMIZE: remove clone.
    Ok(s.last().cloned().unwrap_or_default())
}

fn at(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, Array);
    let o = pop_extract_variant!(args, I32);
    let n = if o >= 0 {
        s.get(o as usize)
    } else {
        let i = o + s.len() as i32;
        if i >= 0 { s.get(i as usize) } else { None }
    };
    // OPTIMIZE: remove clone.
    Ok(n.cloned().unwrap_or_default())
}

fn replace(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let mut s = extract_variant!(s, Array);
    let o = pop_extract_variant!(args, I32);
    let n = args.pop().expect("type missmatched.");
    let i = convert_index(o, s.len())?;
    s[i] = n;
    Ok(Value::Array(s))
}

fn ins(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let mut s = extract_variant!(s, Array);
    let o = pop_extract_variant!(args, I32);
    let n = args.pop().expect("type missmatched.");
    let i = convert_index(o, s.len())?;
    s.insert(i, n);
    Ok(Value::Array(s))
}

fn remove(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let mut s = extract_variant!(s, Array);
    let o = pop_extract_variant!(args, I32);
    let i = convert_index(o, s.len())?;
    s.remove(i);
    Ok(Value::Array(s))
}

fn push(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let mut s = extract_variant!(s, Array);
    let o = args.pop().expect("type missmatched.");
    s.push(o);
    Ok(Value::Array(s))
}

fn pop(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let mut s = extract_variant!(s, Array);
    s.pop();
    Ok(Value::Array(s))
}

fn convert_index(i: i32, l: usize) -> RResult<usize> {
    let l = l as i32;
    if i >= l {
        Err(format!("error: index must be 0 <= index < {l} but passed {i}.").into())
    } else if i >= 0 {
        Ok(i as usize)
    } else if i + l < 0 {
        Err(format!("error: reverse order index must be -{l} <= index < 0 but passed {i}.").into())
    } else {
        Ok((i + l) as usize)
    }
}
