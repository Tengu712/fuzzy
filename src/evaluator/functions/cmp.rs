use super::*;

pub fn insert(fm: &mut FunctionMapStack, ty: &TypeId) {
    fm.insert_all(
        ty,
        vec![
            builtin_fn!("==", vec![ty.clone()], equal),
            builtin_fn!("!=", vec![ty.clone()], not_equal),
        ],
    );

    // NOTE: some types don't have inequality comparation.
    //       If this language matures, we might consider supporting it
    //       with algorithms similar to those used for JavaScript.
    if matches!(ty, TypeId::Array | TypeId::Lazy) {
        return;
    }

    fm.insert_all(
        ty,
        vec![
            builtin_fn!("<", vec![ty.clone()], l),
            builtin_fn!(">", vec![ty.clone()], g),
            builtin_fn!("<=", vec![ty.clone()], le),
            builtin_fn!(">=", vec![ty.clone()], ge),
        ],
    );
}

fn equal(_: &mut Environment, s: Value, args: Vec<Value>) -> RResult<Value> {
    let o = pop_object(args);
    if s.equal(&o) {
        Ok(Value::Top)
    } else {
        Ok(Value::Nil)
    }
}

fn not_equal(_: &mut Environment, s: Value, args: Vec<Value>) -> RResult<Value> {
    let o = pop_object(args);
    if !s.equal(&o) {
        Ok(Value::Top)
    } else {
        Ok(Value::Nil)
    }
}

fn l(_: &mut Environment, s: Value, args: Vec<Value>) -> RResult<Value> {
    let o = pop_object(args);
    if s.l(&o) {
        Ok(Value::Top)
    } else {
        Ok(Value::Nil)
    }
}

fn g(_: &mut Environment, s: Value, args: Vec<Value>) -> RResult<Value> {
    let o = pop_object(args);
    if s.g(&o) {
        Ok(Value::Top)
    } else {
        Ok(Value::Nil)
    }
}

fn le(_: &mut Environment, s: Value, args: Vec<Value>) -> RResult<Value> {
    let o = pop_object(args);
    if s.l(&o) || s.equal(&o) {
        Ok(Value::Top)
    } else {
        Ok(Value::Nil)
    }
}

fn ge(_: &mut Environment, s: Value, args: Vec<Value>) -> RResult<Value> {
    let o = pop_object(args);
    if s.g(&o) || s.equal(&o) {
        Ok(Value::Top)
    } else {
        Ok(Value::Nil)
    }
}

fn pop_object(mut args: Vec<Value>) -> Value {
    if let Some(o) = args.pop() {
        o
    } else {
        panic!("type missmatched.");
    }
}
