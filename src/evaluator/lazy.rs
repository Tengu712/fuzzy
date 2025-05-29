use super::*;

pub fn insert_lazy_functions(maps: &mut FunctionMap) {
    let map = maps
        .get_mut(&TypeId::Unit("{}".to_string()))
        .unwrap_or_else(|| panic!("function map for 'lazy' not found."));
    map.insert(
        "@".to_string(),
        Function {
            types: Vec::new(),
            code: FunctionCode::Builtin(eval_lazy_block),
        },
    );
}

fn eval_lazy_block(env: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let Value::Lazy(mut s) = s else {
        panic!("type missmatched on '{{}}:@'.");
    };
    let result = eval_block(env, &mut s)?
        .pop()
        .expect("evaluating block result is empty.");
    Ok(result)
}
