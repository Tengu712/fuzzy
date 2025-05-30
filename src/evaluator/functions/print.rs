use super::*;

pub fn insert(fm: &mut FunctionMap, ty: &TypeId) {
    fm.insert_all(
        ty,
        vec![
            builtin_fn!("!", vec![], print),
            builtin_fn!("!!", vec![], println),
        ],
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
