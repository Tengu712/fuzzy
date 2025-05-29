use super::*;

pub fn insert(fm: &mut FunctionMap) {
    fm.insert_all(
        &TypeId::Lazy,
        vec![
            (
                "@".to_string(),
                (Vec::new(), FunctionCode::Builtin(eval_lazy_block)),
            ),
            (
                ":".to_string(),
                (vec![TypeId::Array], FunctionCode::Builtin(define_function)),
            ),
        ],
    );
}

fn eval_lazy_block(env: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let mut s = unwrap_subject(s, "@");
    let result = eval_block(env, &mut s, Some(Vec::new()))?
        .pop()
        .expect("evaluating block result is empty.");
    Ok(result)
}

fn define_function(env: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let s = unwrap_subject(s, ":");
    let Some(Value::Array(o)) = args.pop() else {
        panic!("type missmatched on '{}::'.", TypeId::Lazy);
    };

    let types = convert_symbols_to_typeids(o)?;
    let t = TypeId::Function(types.clone());

    if !env.fn_map.is_defined(&t, "@") {
        env.fn_map
            .insert(&t, "@".to_string(), (types, FunctionCode::Builtin(call)));
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
    let Value::Function((_, mut s)) = s else {
        panic!("type missmatched on '{}:@'.", s.typeid());
    };
    args.reverse();
    let result = eval_block(env, &mut s, Some(args))?
        .pop()
        .expect("evaluating block result is empty.");
    Ok(result)
}

fn unwrap_subject(s: Value, name: &str) -> Vec<Token> {
    if let Value::Lazy(s) = s {
        s
    } else {
        panic!("type missmatched on '{}:{name}'.", TypeId::Lazy);
    }
}
