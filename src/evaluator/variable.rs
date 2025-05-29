use super::{types::TypeId, *};

pub fn insert_variable_definition(maps: &mut FunctionMap, ty: &TypeId) {
    let map = maps
        .get_mut(ty)
        .unwrap_or_else(|| panic!("function map for '{}' not found.", ty.to_string()));

    map.insert(
        "->".to_string(),
        Function {
            types: vec![TypeId::Symbol],
            code: FunctionCode::Builtin(define_mutable),
        },
    );
    map.insert(
        "=>".to_string(),
        Function {
            types: vec![TypeId::Symbol],
            code: FunctionCode::Builtin(define_immutable),
        },
    );
}

macro_rules! define_variable_definition {
    ($fn: ident, $name: expr, $mutable: expr) => {
        fn $fn(env: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
            let Some(Value::Symbol(o)) = args.pop() else {
                panic!(
                    "type missmatched on '{}:{}'.",
                    s.get_typeid().to_string(),
                    $name
                );
            };
            if o == "T" {
                return Err(format!("error: cannot redefine 'T'.").into());
            }
            let n = env.get_variable_mut(&o);
            if !n.as_ref().map(|n| n.mutable).unwrap_or(true) {
                return Err(format!("error: cannot redefine variable '{o}'.").into());
            }
            let v = Variable {
                value: s,
                mutable: $mutable,
            };
            if let Some(n) = n {
                *n = v;
            } else {
                env.vr_map
                    .last_mut()
                    .expect("variable map stack is empty.")
                    .insert(o, v);
            }
            Ok(Value::Nil)
        }
    };
}
define_variable_definition!(define_mutable, "->", true);
define_variable_definition!(define_immutable, "=>", false);
