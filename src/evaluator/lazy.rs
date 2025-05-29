use super::{types::TypeId, *};

pub fn insert_lazy_functions(maps: &mut FunctionMap) {
    let map = maps
        .get_mut(&TypeId::Lazy)
        .unwrap_or_else(|| panic!("function map for '{}' not found.", TypeId::Lazy));

    map.insert(
        "@".to_string(),
        Function {
            types: Vec::new(),
            code: FunctionCode::Builtin(eval_lazy_block),
        },
    );
    map.insert(
        ":".to_string(),
        Function {
            types: vec![TypeId::Array],
            code: FunctionCode::Builtin(define_function),
        },
    );
}

fn eval_lazy_block(env: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let mut s = unwrap_subject(s, "@");
    let result = eval_block(env, &mut s, Some(Vec::new()))?
        .pop()
        .expect("evaluating block result is empty.");
    Ok(result)
}

fn define_function(env: &mut Environment, s: Value, args: Vec<Value>) -> RResult<Value> {
    let s = unwrap_subject(s, ":");

    let types = convert_symbols_to_typeids(args)?;
    let t = TypeId::Function(types.clone());

    if !env.fn_map.contains_key(&t) {
        let mut n = HashMap::new();
        n.insert(
            "@".to_string(),
            Function {
                types,
                code: FunctionCode::Builtin(call),
            },
        );
        env.fn_map.insert(t.clone(), n);
    }

    Ok(Value::Function((t, s)))
}

fn convert_symbols_to_typeids(n: Vec<Value>) -> RResult<Vec<TypeId>> {
    let mut v = Vec::new();
    for n in n {
        match n {
            Value::Symbol(n) => v.push(TypeId::from(&n)?),
            Value::Array(n) => v.push(TypeId::Function(convert_symbols_to_typeids(n)?)),
            _ => return Err(format!("error: the element of argument list must be symbol or array of symbols but passed '{}'.", n.get_typeid()).into()),
        }
    }
    Ok(v)
}

fn call(env: &mut Environment, s: Value, args: Vec<Value>) -> RResult<Value> {
    let Value::Function((_, mut s)) = s else {
        panic!("type missmatched on '{}:@'.", s.get_typeid());
    };
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
