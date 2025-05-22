use super::*;

pub fn insert_function_functions(maps: &mut FunctionMap) {
    let map = maps
        .get_mut("function")
        .unwrap_or_else(|| panic!("function map for 'function' not found."));
    map.insert(
        ":".to_string(),
        Function {
            types: vec!["symbol".to_string()],
            code: FunctionCode::Builtin(add_argument_type),
        },
    );
}

fn add_argument_type(_: &mut Environment, mut s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let Value::Function(n) = &mut s else {
        panic!("type missmatched on 'function::'.");
    };
    let Some(Value::Symbol(o)) = args.pop() else {
        panic!("type missmatched on 'function::'.");
    };
    n.types.push(o);
    Ok(s)
}
