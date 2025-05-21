use super::*;

pub fn insert_variable_definition(maps: &mut FunctionMap, ty: &str) {
    let map = maps
        .get_mut(ty)
        .unwrap_or_else(|| panic!("function map for '{ty}' not found."));
    map.insert(
        "->".to_string(),
        Function {
            types: vec!["symbol".to_string()],
            code: FunctionCode::Builtin(define_mutable),
        },
    );
    map.insert(
        "=>".to_string(),
        Function {
            types: vec!["symbol".to_string()],
            code: FunctionCode::Builtin(define_immutable),
        },
    );
}

macro_rules! define_variable_definition {
    ($fn: ident, $name: expr, $mutable: expr) => {
        fn $fn(env: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
            let Some(Value::Symbol(n)) = args.pop() else {
                panic!("type missmatched on '{}:{}'.", s.get_typeid(), $name);
            };
            if !can_define(env, &n) {
                return Err(format!("error: cannot redefine variable '{n}'.").into());
            }
            let v = Variable {
                value: s,
                mutable: $mutable,
            };
            env.vr_map
                .last_mut()
                .expect("variable map stack is empty.")
                .insert(n, v);
            Ok(Value::Nil)
        }
    };
}
define_variable_definition!(define_mutable, "->", true);
define_variable_definition!(define_immutable, "=>", false);

fn can_define(env: &mut Environment, name: &str) -> bool {
    env.get_variable(name).map(|n| n.mutable).unwrap_or(true)
}
