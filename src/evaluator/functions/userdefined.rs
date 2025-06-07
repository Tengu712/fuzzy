use super::*;

pub fn insert(fm: &mut FunctionMapStack, type_name: &str) {
    let type_id = TypeId::UserDefined(type_name.to_string());
    fm.insert_all(
        &type_id,
        vec![
            builtin_fn!(":", vec![TypeId::Symbol], get_public_member),
            builtin_fn!("::", vec![TypeId::Symbol], get_private_member),
        ],
    );
}

fn get_public_member(env: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let (type_name, fields) = extract_variant!(s, UserDefined);
    let member_name = pop_extract_variant!(args, Symbol);
    
    let user_type = env.ut_map.get(&type_name)
        .ok_or_else(|| format!("error: undefined user type {type_name}."))?;
    
    let field_def = user_type.fields.iter()
        .find(|f| f.name == member_name)
        .ok_or_else(|| format!("error: undefined field {member_name} in type {type_name}."))?;
    
    if field_def.private {
        return Err(format!("error: cannot access private field {member_name} outside of type {type_name}.").into());
    }
    
    fields.get(&member_name)
        .cloned()
        .ok_or_else(|| format!("error: field {member_name} not found.").into())
}

fn get_private_member(env: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
    let (type_name, fields) = extract_variant!(s, UserDefined);
    let member_name = pop_extract_variant!(args, Symbol);
    
    let user_type = env.ut_map.get(&type_name)
        .ok_or_else(|| format!("error: undefined user type {type_name}."))?;
    
    let field_def = user_type.fields.iter()
        .find(|f| f.name == member_name)
        .ok_or_else(|| format!("error: undefined field {member_name} in type {type_name}."))?;
    
    if !field_def.private {
        return Err(format!("error: field {member_name} is not private.").into());
    }
    
    if let Some(slf) = &env.vr_map.get("##") {
        if let Value::UserDefined((slf_type, _)) = slf {
            if slf_type != &type_name {
                return Err(format!("error: cannot access private field {member_name} from different type.").into());
            }
        } else {
            return Err("error: ## is not a user-defined type.".into());
        }
    } else {
        return Err("error: cannot access private field outside of instance method.".into());
    }
    
    fields.get(&member_name)
        .cloned()
        .ok_or_else(|| format!("error: field {member_name} not found.").into())
}