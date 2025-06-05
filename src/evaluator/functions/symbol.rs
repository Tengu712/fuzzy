use super::*;

pub fn insert(fm: &mut FunctionMapStack) {
    fm.insert_all(
        &TypeId::Symbol,
        vec![builtin_fn!("$", vec![], symbol_value)],
    );
}

fn symbol_value(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    Ok(Value::Label(extract_variant!(s, Symbol)))
}
