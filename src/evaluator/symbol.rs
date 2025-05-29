use super::*;

pub fn insert_symbol_value(maps: &mut FunctionMap) {
    let map = maps
        .get_mut(&TypeId::Unit("symbol".to_string()))
        .unwrap_or_else(|| panic!("function map for 'symbol' not found."));
    map.insert(
        "$".to_string(),
        Function {
            types: Vec::new(),
            code: FunctionCode::Builtin(symbol_value),
        },
    );
}

fn symbol_value(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    if let Value::Symbol(n) = s {
        Ok(Value::Label(n))
    } else {
        panic!("type missmatched on 'symbol:$'.");
    }
}
