use super::{super::variable::Variable, *};

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
                return Err(format!("error: cannot redefine T.").into());
            }

            let v = Variable {
                value: s,
                mutable: $mutable,
            };
            env.vr_map.insert(o, v)?;

            Ok(Value::Nil)
        }
    };
}
define_variable_definition!(define_mutable, "->", true);
define_variable_definition!(define_immutable, "=>", false);
