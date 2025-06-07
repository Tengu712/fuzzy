use super::{
    super::usertype::{UserType, UserTypeField},
    variable, *,
};

pub fn insert(fm: &mut FunctionMapStack) {
    fm.insert_builtins(
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
        if i + 2 >= s.len() {
            return Err("error: field definition must have mutability, name and type.".into());
        }

        let m = match &s[i] {
            Value::Top => true,
            Value::Nil => false,
            n => {
                return Err(format!(
                    "error: field mutability must be a bool but {} passed.",
                    n.typeid()
                )
                .into());
            }
        };

        i += 1;

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

        i += 1;

        let t = if let Value::Symbol(t) = &s[i] {
            TypeId::from(t)
        } else if let Value::Array(a) = &s[i] {
            TypeId::Function(convert_symbols_to_typeids(a)?)
        } else {
            return Err("error: field type must be a symbol.".into());
        };

        i += 1;

        fields.push(UserTypeField {
            mutable: m,
            private: p,
            name: n,
            ty: t,
        });
    }

    env.ut_map.insert(o.clone(), UserType { mutable, fields })?;

    let t = TypeId::UserDefined(o);
    env.fn_map.insert_new_type(t.clone());
    variable::insert(&mut env.fn_map, &t);

    Ok(Value::Nil)
}
