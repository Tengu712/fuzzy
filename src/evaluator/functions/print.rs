use super::*;

pub fn insert(fm: &mut FunctionMapStack, ty: &TypeId) {
    fm.insert_builtins(
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
