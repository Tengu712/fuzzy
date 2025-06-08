use super::*;

pub fn insert(fm: &mut FunctionMapStack) {
    fm.insert_builtins(
        &TypeId::Lazy,
        vec![
            builtin_fn!("#", vec![], length),
            builtin_fn!("^", vec![], first),
            builtin_fn!("$", vec![], last),
            builtin_fn!("@", vec![TypeId::I32], at),
            builtin_fn!("@@", vec![TypeId::I32, TypeId::String], replace),
            builtin_fn!("@<", vec![TypeId::I32, TypeId::String], ins),
            builtin_fn!("@-", vec![TypeId::I32], remove),
            builtin_fn!("$>", vec![TypeId::String], push),
            builtin_fn!("$-", vec![], pop),
            builtin_fn!("%", vec![], eval_lazy_block),
            builtin_fn!("%%", vec![TypeId::Lazy], while_loop),
            builtin_fn!(":", vec![TypeId::Array], define_function),
        ],
    );
}

fn length(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, Lazy);
    Ok(Value::U32(s.len() as u32))
}

fn first(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, Lazy);
    Ok(s.back()
        .map(|n| Value::String(n.to_string()))
        .unwrap_or_default())
}

fn last(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, Lazy);
    Ok(s.front()
        .map(|n| Value::String(n.to_string()))
        .unwrap_or_default())
}

fn at(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, Lazy);
    let o = pop_extract_variant!(args, I32);
    let i = convert_index(o, s.len())?;
    let i = s.len() - 1 - i;
    let n = s.get(i).map(|n| Value::String(n.to_string()));
    Ok(n.unwrap_or_default())
}

fn replace(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let mut s = extract_variant!(s, Lazy);
    let o = pop_extract_variant!(args, I32);
    let n = args.pop().expect("type missmatched.");
    let n = extract_variant!(n, String);
    let i = convert_index(o, s.len())?;
    let i = s.len() - 1 - i;
    s[i] = Token::from(&n);
    Ok(Value::Lazy(s))
}

fn ins(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let mut s = extract_variant!(s, Lazy);
    let o = pop_extract_variant!(args, I32);
    let n = args.pop().expect("type missmatched.");
    let n = extract_variant!(n, String);
    let i = convert_index(o, s.len())?;
    let i = s.len() - 1 - i;
    s.insert(i, Token::from(&n));
    Ok(Value::Lazy(s))
}

fn remove(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let mut s = extract_variant!(s, Lazy);
    let o = pop_extract_variant!(args, I32);
    let i = convert_index(o, s.len())?;
    let i = s.len() - 1 - i;
    s.remove(i);
    Ok(Value::Lazy(s))
}

fn push(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let mut s = extract_variant!(s, Lazy);
    let o = args.pop().expect("type missmatched.");
    let o = extract_variant!(o, String);
    s.push_front(Token::from(&o));
    Ok(Value::Lazy(s))
}

fn pop(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let mut s = extract_variant!(s, Lazy);
    s.pop_front();
    Ok(Value::Lazy(s))
}

fn eval_lazy_block(env: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, Lazy);
    eval(env, &mut s.into(), Vec::new())
}

fn while_loop(env: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, Lazy);
    let o = pop_extract_variant!(args, Lazy);
    loop {
        let r = eval(env, &mut s.clone().into(), Vec::new())?;
        if r == Value::Nil {
            break;
        }
        eval(env, &mut o.clone().into(), vec![r])?;
    }
    Ok(Value::Nil)
}

fn define_function(env: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, Lazy);
    let o = pop_extract_variant!(args, Array);

    let ts = convert_symbols_to_typeids(&o)?;
    let t = TypeId::Function(ts.clone());

    if !env.fn_map.is_defined(None, &t, "@") {
        let n = Function {
            mutable: false,
            private: false,
            types: ts,
            code: FunctionCode::Builtin(call),
        };
        env.fn_map.insert_builtins(&t, vec![("@".to_string(), n)]);
        variable::insert(&mut env.fn_map, &t);
    }

    Ok(Value::Function((t, s.into())))
}

fn call(env: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let mut s = extract_variant!(s, Function);
    args.reverse();
    eval(env, &mut s.1, args)
}

fn eval(env: &mut Environment, tokens: &mut Vec<Token>, args: Vec<Value>) -> RResult<Value> {
    let params = EnterLazyParams {
        slf: None,
        args: Some(args),
    };
    let result = eval_block(env, tokens, params)?.pop().unwrap_or_default();
    Ok(result)
}
