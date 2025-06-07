use super::{
    super::usertype::{UserType, UserTypeField},
    *,
};

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
    mutable: bool,
) -> RResult<Value> {
    let s = extract_variant!(s, Array);
    let o = pop_extract_variant!(args, Symbol);

    let mut fields = Vec::new();
    let mut i = 0;
    while i < s.len() {
        if i + 1 >= s.len() {
            return Err("error: field definition must have both name and type.".into());
        }

        let Value::Symbol(n) = &s[i] else {
            return Err("error: field name must be a symbol.".into());
        };
        let (p, n) = if let Some(n) = n.strip_prefix("::") {
            (true, n.to_string())
        } else if let Some(n) = n.strip_prefix(":") {
            (false, n.to_string())
        } else {
            return Err("error: field name must start with ':' or '::'.".into());
        };

        let t = if let Value::Symbol(t) = &s[i + 1] {
            TypeId::from(t)
        } else if let Value::Array(a) = &s[i + 1] {
            TypeId::Function(convert_symbols_to_typeids(a)?)
        } else {
            return Err("error: field type must be a symbol.".into());
        };

        fields.push(UserTypeField {
            private: p,
            name: n,
            ty: t,
        });
        i += 2;
    }

    env.ut_map.insert(o, UserType { mutable, fields })?;
    Ok(Value::Nil)
}
