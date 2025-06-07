use super::{super::variable::Variable, *};

pub fn insert(fm: &mut FunctionMapStack, ty: &TypeId) {
    fm.insert_builtins(
        ty,
        vec![
            builtin_fn!("->", vec![TypeId::Symbol], define_mutable),
            builtin_fn!("=>", vec![TypeId::Symbol], define_immutable),
        ],
    );
}

fn define_mutable(env: &mut Environment, s: Value, args: Vec<Value>) -> RResult<Value> {
    define(env, s, args, "->", true)
}

fn define_immutable(env: &mut Environment, s: Value, args: Vec<Value>) -> RResult<Value> {
    define(env, s, args, "=>", false)
}

fn define(
    env: &mut Environment,
    s: Value,
    mut args: Vec<Value>,
    arrow: &str,
    mutable: bool,
) -> RResult<Value> {
    let o = pop_extract_variant!(args, Symbol);
    if o == "##" {
        return Err("error: cannot redefine ##.".into());
    }
    if o == "T" {
        return Err("error: cannot redefine T.".into());
    }

    if let Some((private, trg_ty, trg)) = split_type_and_name(&o) {
        let Value::Function((ty, tokens)) = s else {
            return Err(format!("error: A : is used in the object of a {arrow}, which is treated as a function definition, but the subject is not a function type.").into());
        };

        // check if it can redefine?
        let trg_ty = TypeId::from(trg_ty);
        if env
            .fn_map
            .get(&trg_ty, trg)
            .map(|n| !n.mutable)
            .unwrap_or(false)
        {
            return Err(format!(
                "error: cannot redefine {trg} on {trg_ty} because it's immutable."
            )
            .into());
        }

        // get types
        let TypeId::Function(types) = ty else {
            panic!("failed to extract function.");
        };

        // insert
        let f = Function {
            mutable,
            private,
            types,
            code: FunctionCode::UserDefined(tokens),
        };
        env.fn_map
            .insert_user_defined(&trg_ty, trg.to_string(), f)?;

        // finish
        return Ok(Value::Nil);
    }

    let v = Variable { value: s, mutable };
    env.vr_map.insert(o, v)?;

    Ok(Value::Nil)
}

fn split_type_and_name(n: &str) -> Option<(bool, &str, &str)> {
    if let Some((n, m)) = n.split_once("::") {
        Some((true, n, m))
    } else if let Some((n, m)) = n.split_once(":") {
        Some((false, n, m))
    } else {
        None
    }
}
