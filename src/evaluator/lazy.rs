use super::*;

pub fn insert_lazy_functions(maps: &mut FunctionMap) {
    let map = maps
        .get_mut("{}")
        .unwrap_or_else(|| panic!("function map for 'lazy' not found."));
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
            types: vec!["lazy".to_string()],
            code: FunctionCode::Builtin(generate_function),
        },
    );
}

fn eval_lazy_block(env: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let Value::Lazy(mut s) = s else {
        panic!("type missmatched on '{{}}:@'.");
    };
    env.vr_map.push(HashMap::new());
    let result = eval_block(env, &mut s.tokens)?;
    let _  = env.vr_map.pop();
    Ok(result)
}

fn generate_function(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let Value::Lazy(mut s) = s else {
        panic!("type missmatched on 'function::'.");
    };
    let Some(Value::Lazy(o)) = args.pop() else {
        panic!("type missmatched on 'function::'.");
    };
    let mut args = Vec::new();
    while !s.tokens.is_empty() {
        let Some(Token::Symbol(a)) = s.tokens.pop() else {
            return Err("error: argument name must be a symbol.".into());
        };
        let Some(t) = s.tokens.pop() else {
            return Err("error: argument type not found.".into());
        };
        let t = match t {
            Token::Symbol(t) => t,
            Token::LBrace => panic!("unimplemented"),
            _ => return Err("error: argument type must be a symbol or symbol list.".into()),
        };
        args.push((a, t));
    }
    Ok(Value::Lazy(LazyBlock {
        args,
        tokens: o.tokens,
    }))
}
