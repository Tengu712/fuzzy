use super::*;

pub fn insert(fm: &mut FunctionMap, ty: &TypeId) {
    fm.insert_all(
        ty,
        vec![
            builtin_fn!("->", vec![TypeId::Symbol], define_mutable),
            builtin_fn!("=>", vec![TypeId::Symbol], define_immutable),
        ],
    );
}

macro_rules! define_variable_definition {
    ($fn: ident, $name: expr, $mutable: expr) => {
        fn $fn(env: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
            let o = pop_extract_variant!(args, Symbol);
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
