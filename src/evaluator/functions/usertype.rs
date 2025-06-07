use super::*;

pub fn insert(fm: &mut FunctionMapStack) {
    fm.insert_all(
        &TypeId::Array,
        vec![
            builtin_fn!("=>>", vec![TypeId::Symbol], define_public_user_type),
            builtin_fn!("->>", vec![TypeId::Symbol], define_private_user_type),
        ],
    );
}

fn define_public_user_type(env: &mut Environment, s: Value, args: Vec<Value>) -> RResult<Value> {
    define_user_type(env, s, args, false)
}

fn define_private_user_type(env: &mut Environment, s: Value, args: Vec<Value>) -> RResult<Value> {
    define_user_type(env, s, args, true)
}

fn define_user_type(
    env: &mut Environment,
    s: Value,
    mut args: Vec<Value>,
    private: bool,
) -> RResult<Value> {
    let s = extract_variant!(s, Array);
    let type_name = pop_extract_variant!(args, Symbol);

    let mut fields = Vec::new();
    let mut i = 0;
    while i < s.len() {
        if i + 1 >= s.len() {
            return Err("error: field definition must have both name and type.".into());
        }

        let Value::Symbol(n) = &s[i] else {
            return Err("error: field name must be a symbol.".into());
        };
        let (n, p) = if let Some(n) = n.strip_prefix("::") {
            (n.to_string(), true)
        } else if let Some(n) = n.strip_prefix(":") {
            (n.to_string(), false)
        } else {
            return Err("error: field name must start with ':' or '::'.".into());
        };

        let Value::Symbol(t) = &s[i + 1] else {
            return Err("error: field type must be a symbol.".into());
        };
        let t = TypeId::from_with_user_types(t, &env.user_types)?;

        fields.push((n, t, p));
        i += 2;
    }

    let user_type = UserDefinedType {
        name: type_name.clone(),
        fields,
    };

    env.user_types.insert(type_name.clone(), user_type);

    Ok(Value::Nil)
}
