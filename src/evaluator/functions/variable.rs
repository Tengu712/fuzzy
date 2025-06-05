use super::{super::variable::Variable, *};

pub fn insert(fm: &mut FunctionMapStack, ty: &TypeId) {
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

            // TODO: object
            // TODO: self
            if let Some((private, t, n)) = split_type_and_name(&o) {
                if matches!(s.typeid(), TypeId::Function(_)) {
                    // TODO:
                }
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

fn split_type_and_name(n: &str) -> Option<(bool, &str, &str)> {
    if let Some((n, m)) = n.split_once("::") {
        Some((true, n, m))
    } else if let Some((n, m)) = n.split_once(":") {
        Some((false, n, m))
    } else {
        None
    }
}
