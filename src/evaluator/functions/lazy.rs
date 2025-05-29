use super::*;

pub fn insert(fm: &mut FunctionMap) {
    fm.insert_all(
        &TypeId::Lazy,
        vec![
            builtin_fn!("@", vec![], eval_lazy_block),
            builtin_fn!(":", vec![TypeId::Array], define_function),
        ],
    );
}

fn eval_lazy_block(env: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let mut s = extract_variant!(s, Lazy);
    eval(env, &mut s, Some(Vec::new()))
}

fn define_function(env: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, Lazy);
    let o = pop_extract_variant!(args, Array);

    let ts = convert_symbols_to_typeids(o)?;
    let t = TypeId::Function(ts.clone());

    if !env.fn_map.is_defined(&t, "@") {
        env.fn_map.insert_all(&t, vec![builtin_fn!("@", ts, call)]);
        variable::insert(&mut env.fn_map, &t);
    }

    Ok(Value::Function((t, s)))
}

fn convert_symbols_to_typeids(n: Vec<Value>) -> RResult<Vec<TypeId>> {
    let mut v = Vec::new();
    for n in n {
        match n {
            Value::Nil => (),
            Value::Symbol(n) => v.push(TypeId::from(&n)?),
            Value::Array(n) => v.push(TypeId::Function(convert_symbols_to_typeids(n)?)),
            _ => return Err(format!("error: the element of argument list must be symbol or array of symbols but passed '{}'.", n.typeid()).into()),
        }
    }
    Ok(v)
}

fn call(env: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let mut s = extract_variant!(s, Function);
    args.reverse();
    eval(env, &mut s.1, Some(args))
}

fn eval(
    env: &mut Environment,
    tokens: &mut Vec<Token>,
    args: Option<Vec<Value>>,
) -> RResult<Value> {
    let result = eval_block(env, tokens, args)?
        .pop()
        .expect("evaluating block result is empty.");
    Ok(result)
}
