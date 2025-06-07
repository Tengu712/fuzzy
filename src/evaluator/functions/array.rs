use super::{super::usertype::UserTypeField, value::Object, *};

pub fn insert(fm: &mut FunctionMapStack) {
    fm.insert_builtins(
        &TypeId::Array,
        vec![
            builtin_fn!("#", vec![], length),
            builtin_fn!("^", vec![], first),
            builtin_fn!("$", vec![], last),
            builtin_fn!("@", vec![TypeId::I32], at),
            builtin_fn!("@@", vec![TypeId::I32, TypeId::Any], replace),
            builtin_fn!("@<", vec![TypeId::I32, TypeId::Any], ins),
            builtin_fn!("@-", vec![TypeId::I32], remove),
            builtin_fn!("$>", vec![TypeId::Any], push),
            builtin_fn!("$-", vec![], pop),
            builtin_fn!("|>", vec![TypeId::Symbol], define_user_type),
            builtin_fn!(":", vec![TypeId::Symbol], cast_to_user_type),
        ],
    );
}

fn length(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, Array);
    Ok(Value::U32(s.len() as u32))
}

fn first(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, Array);
    // OPTIMIZE: remove clone.
    Ok(s.first().cloned().unwrap_or_default())
}

fn last(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, Array);
    // OPTIMIZE: remove clone.
    Ok(s.last().cloned().unwrap_or_default())
}

fn at(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, Array);
    let o = pop_extract_variant!(args, I32);
    let n = if o >= 0 {
        s.get(o as usize)
    } else {
        let i = o + s.len() as i32;
        if i >= 0 { s.get(i as usize) } else { None }
    };
    // OPTIMIZE: remove clone.
    Ok(n.cloned().unwrap_or_default())
}

fn replace(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let mut s = extract_variant!(s, Array);
    let o = pop_extract_variant!(args, I32);
    let n = args.pop().expect("type missmatched.");
    let i = convert_index(o, s.len())?;
    s[i] = n;
    Ok(Value::Array(s))
}

fn ins(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let mut s = extract_variant!(s, Array);
    let o = pop_extract_variant!(args, I32);
    let n = args.pop().expect("type missmatched.");
    let i = convert_index(o, s.len())?;
    s.insert(i, n);
    Ok(Value::Array(s))
}

fn remove(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let mut s = extract_variant!(s, Array);
    let o = pop_extract_variant!(args, I32);
    let i = convert_index(o, s.len())?;
    s.remove(i);
    Ok(Value::Array(s))
}

fn push(_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let mut s = extract_variant!(s, Array);
    let o = args.pop().expect("type missmatched.");
    s.push(o);
    Ok(Value::Array(s))
}

fn pop(_: &mut Environment, s: Value, _: Vec<Value>) -> RResult<Value> {
    let mut s = extract_variant!(s, Array);
    s.pop();
    Ok(Value::Array(s))
}

fn define_user_type(env: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
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

    env.ut_map.insert(o.clone(), fields)?;

    let t = TypeId::UserDefined(o);
    env.fn_map.insert_new_type(t.clone());
    variable::insert(&mut env.fn_map, &t);

    Ok(Value::Nil)
}

fn cast_to_user_type(env: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let s = extract_variant!(s, Array);
    let o = pop_extract_variant!(args, Symbol);

    let mut fields = HashMap::new();
    let mut i = 0;
    while i < s.len() {
        if i + 1 > s.len() {
            return Err("error: field definition must have both name and value.".into());
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

        i += 1;

        // OPTIMIZE: remove clone.
        let v = s[i].clone();

        i += 1;

        fields.insert(
            n,
            Object {
                mutable: false,
                private: p,
                value: v,
            },
        );
    }

    let Some(ut_fields) = env.ut_map.get(&o) else {
        return Err(format!("error: the type {o} not defined.").into());
    };

    if fields.len() != ut_fields.len() {
        return Err("error: The provided array does not match the type definition.".into());
    }

    for ut in ut_fields.iter() {
        let Some(field) = fields.get_mut(&ut.name) else {
            return Err(format!(
                "error: {} not found in user-type variable definition.",
                ut.name
            )
            .into());
        };
        if field.private != ut.private {
            let e = if ut.private { "private" } else { "public" };
            let r = if field.private { "private" } else { "public" };
            return Err(format!("error: {} defined as {e} but specified {r}.", ut.name).into());
        }
        if ut.ty != field.value.typeid() {
            return Err(format!(
                "error: field {} expects type {} but {} provided.",
                ut.name,
                ut.ty,
                field.value.typeid()
            )
            .into());
        }
        field.mutable = ut.mutable;
    }

    Ok(Value::UserType((TypeId::UserDefined(o), fields)))
}

fn convert_index(i: i32, l: usize) -> RResult<usize> {
    let l = l as i32;
    if i >= l {
        Err(format!("error: index must be 0 <= index < {l} but passed {i}.").into())
    } else if i >= 0 {
        Ok(i as usize)
    } else if i + l < 0 {
        Err(format!("error: reverse order index must be -{l} <= index < 0 but passed {i}.").into())
    } else {
        Ok((i + l) as usize)
    }
}
