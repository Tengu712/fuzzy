use super::*;

pub fn insert(fm: &mut FunctionMap, ty: &TypeId) {
    fm.insert_all(
        ty,
        vec![
            ("!".to_string(), (Vec::new(), FunctionCode::Builtin(p))),
            ("!!".to_string(), (Vec::new(), FunctionCode::Builtin(pl))),
        ],
    );
}

fn p(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    print!("{s}");
    Ok(s)
}

fn pl(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    println!("{s}");
    Ok(s)
}
