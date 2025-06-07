use super::*;

pub fn insert(fm: &mut FunctionMapStack) {
    fm.insert_all(
        &TypeId::Lazy,
        vec![
            builtin_fn!("%", vec![], eval_lazy_block),
            builtin_fn!("%%", vec![TypeId::Lazy], while_loop),
            builtin_fn!(":", vec![TypeId::Array], define_function),
            builtin_fn!("::", vec![TypeId::Array, TypeId::Symbol], define_user_defined_function),
        ],
    );
}

fn eval_lazy_block(env: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let mut s = extract_variant!(s, Lazy);
    eval(env, &mut s, Vec::new())
}

fn while_loop(env: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, Lazy);
    let o = pop_extract_variant!(args, Lazy);
    loop {
        let r = eval(env, &mut s.clone(), Vec::new())?;
        if r == Value::Nil {
            break;
        }
        eval(env, &mut o.clone(), vec![r])?;
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
        env.fn_map.insert_new_type(t.clone());
        env.fn_map.insert_user_defined(&t, "@".to_string(), n)?;
        variable::insert(&mut env.fn_map, &t);
    }

    Ok(Value::Function((t, s)))
}

fn call(env: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let mut s = extract_variant!(s, Function);
    args.reverse();
    eval(env, &mut s.1, args)
}

fn define_user_defined_function(env: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, Lazy);
    let o = pop_extract_variant!(args, Array);
    let type_symbol = pop_extract_variant!(args, Symbol);
    
    if !type_symbol.contains(':') {
        return Err("error: user defined function must be in format 'typename:functionname'.".into());
    }
    
    let parts: Vec<&str> = type_symbol.splitn(2, ':').collect();
    let type_name = parts[0];
    let func_name = parts[1];
    
    if env.ut_map.get(type_name).is_none() {
        return Err(format!("error: undefined user type {type_name}.").into());
    }
    
    let ts = convert_symbols_to_typeids(&o)?;
    let t = TypeId::UserDefined(type_name.to_string());
    
    let n = Function {
        mutable: false,
        private: false,
        types: ts,
        code: FunctionCode::UserDefined(s.clone()),
    };
    env.fn_map.insert_new_type(t.clone());
    env.fn_map.insert_user_defined(&t, func_name.to_string(), n)?;
    
    Ok(Value::Function((t, s)))
}

fn eval(env: &mut Environment, tokens: &mut Vec<Token>, args: Vec<Value>) -> RResult<Value> {
    let params = EnterLazyParams {
        slf: None,
        args: Some(args),
    };
    let result = eval_block(env, tokens, params)?.pop().unwrap_or_default();
    Ok(result)
}
