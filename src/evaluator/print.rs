use super::{types::TypeId, *};

pub fn insert_print(maps: &mut FunctionMap, ty: &TypeId) {
    let map = maps
        .get_mut(ty)
        .unwrap_or_else(|| panic!("function map for '{ty}' not found."));

    map.insert(
        "!".to_string(),
        Function {
            types: Vec::new(),
            code: FunctionCode::Builtin(print),
        },
    );
    map.insert(
        "!!".to_string(),
        Function {
            types: Vec::new(),
            code: FunctionCode::Builtin(println),
        },
    );
}

fn print(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    print!("{s}");
    Ok(s)
}

fn println(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    println!("{s}");
    Ok(s)
}
