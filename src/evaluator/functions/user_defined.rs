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
    _is_private: bool,
) -> RResult<Value> {
    let s = extract_variant!(s, Array);
    let type_name = pop_extract_variant!(args, Symbol);

    let mut fields = Vec::new();
    let mut i = 0;
    while i < s.len() {
        if i + 1 >= s.len() {
            return Err(format!("error: field definition must have both name and type.").into());
        }

        let field_name = match &s[i] {
            Value::Symbol(name) => {
                if name.starts_with("::") {
                    (name[2..].to_string(), true)
                } else if name.starts_with(":") {
                    (name[1..].to_string(), false)
                } else {
                    return Err(format!("error: field name must start with ':' or '::'.").into());
                }
            }
            _ => return Err(format!("error: field name must be a symbol.").into()),
        };

        let field_type = match &s[i + 1] {
            Value::Symbol(type_str) => TypeId::from_with_user_types(type_str, &env.user_types)?,
            _ => return Err(format!("error: field type must be a symbol.").into()),
        };

        fields.push((field_name.0, field_type, field_name.1));
        i += 2;
    }

    let user_type = UserDefinedType {
        name: type_name.clone(),
        fields,
    };

    env.user_types.insert(type_name.clone(), user_type);

    Ok(Value::Symbol(type_name))
}
