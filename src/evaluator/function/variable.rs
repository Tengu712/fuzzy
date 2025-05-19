use super::*;

pub fn insert_variable_definition(maps: &mut FunctionMap, ty: &str) {
    let map = maps
        .get_mut(ty)
        .unwrap_or_else(|| panic!("function map for '{ty}' not found."));
    map.insert("->".to_string(), Function::Builtin(define_mutable));
    map.insert("=>".to_string(), Function::Builtin(define_immutable));
}

macro_rules! define_variable_definition {
    ($fn: ident, $name: expr, $mutable: expr) => {
        fn $fn(env: &mut Environment, s: Value, values: &mut Vec<Value>) -> RResult<()> {
            let Some(Value::Symbol(n)) = values.pop() else {
                return Err(format!(
                    "error: no symbol argument passed to '{}:{}'.",
                    s.get_typeid(env),
                    $name
                )
                .into());
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
            values.push(Value::Nil);
            Ok(())
        }
    };
}
define_variable_definition!(define_mutable, "->", true);
define_variable_definition!(define_immutable, "=>", false);

fn can_define(env: &mut Environment, name: &str) -> bool {
    env.get_variable(name).map(|n| n.mutable).unwrap_or(true)
}
