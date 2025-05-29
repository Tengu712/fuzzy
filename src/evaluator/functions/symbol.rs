use super::*;

pub fn insert(fm: &mut FunctionMap) {
    fm.insert_all(
        &TypeId::Symbol,
        vec![(
            "$".to_string(),
            (Vec::new(), FunctionCode::Builtin(symbol_value)),
        )],
    );
}

fn symbol_value(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    if let Value::Symbol(n) = s {
        Ok(Value::Label(n))
    } else {
        panic!("type missmatched on '{}:$'.", TypeId::Symbol);
    }
}
