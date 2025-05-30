use super::*;

pub fn insert(fm: &mut FunctionMap) {
    fm.insert_all(
        &TypeId::Array,
        vec![
            builtin_fn!("#", vec![], length),
            builtin_fn!("@", vec![TypeId::I32], at),
            builtin_fn!("@@", vec![TypeId::I32, TypeId::Any], repl),
            builtin_fn!("@<", vec![TypeId::I32, TypeId::Any], ins),
            builtin_fn!("^", vec![], first),
            builtin_fn!("$", vec![], last),
            builtin_fn!("$>", vec![TypeId::Any], push),
            builtin_fn!("@-", vec![TypeId::I32], remove),
            builtin_fn!("$-", vec![], pop),
        ],
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

fn repl(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let mut s = unwrap_subject(s, "@@");
    let o = unwrap_index(args.pop(), "@@");
    let Some(t) = args.pop() else {
        panic!("type missmatched on '{}:@@'.", TypeId::Array);
    };
    let i = convert_index(o, s.len())?;
    s[i] = t;
    Ok(Value::Array(s))
}

fn ins(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let mut s = unwrap_subject(s, "@<");
    let o = unwrap_index(args.pop(), "@<");
    let Some(t) = args.pop() else {
        panic!("type missmatched on '{}:@<'.", TypeId::Array);
    };
    let i = convert_index(o, s.len())?;
    s.insert(i, t);
    Ok(Value::Array(s))
}

fn first(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let s = unwrap_subject(s, "^");
    Ok(s.first().cloned().unwrap_or(Value::Nil))
}

fn last(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let s = unwrap_subject(s, "$");
    Ok(s.last().cloned().unwrap_or(Value::Nil))
}

fn push(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let mut s = unwrap_subject(s, "$>");
    let Some(t) = args.pop() else {
        panic!("type missmatched on '{}:$>'.", TypeId::Array);
    };
    s.push(t);
    Ok(Value::Array(s))
}

fn remove(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let mut s = unwrap_subject(s, "@-");
    let o = unwrap_index(args.pop(), "@-");
    let i = convert_index(o, s.len())?;
    s.remove(i);
    Ok(Value::Array(s))
}

fn pop(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let mut s = unwrap_subject(s, "$-");
    s.pop();
    Ok(Value::Array(s))
}

fn unwrap_subject(s: Value, name: &str) -> Vec<Value> {
    if let Value::Array(s) = s {
        s
    } else {
        panic!("type missmatched on '{}:{name}'.", TypeId::Array);
    }
}

fn unwrap_index(o: Option<Value>, name: &str) -> i32 {
    if let Some(Value::I32(o)) = o {
        o
    } else {
        panic!("type missmatched on '{}:{name}'.", TypeId::Array);
    }
}

fn convert_index(i: i32, l: usize) -> RResult<usize> {
    let l = l as i32;
    if i >= l {
        Err(format!("error: index must be 0 <= index < {l} but passed {i}.").into())
    } else if i >= 0 {
        Ok(i as usize)
    } else if i + l < 0 {
        Err(format!("error: revert index must be -{l} <= index < 0 but passed {i}.").into())
    } else {
        Ok((i + l) as usize)
    }
}
