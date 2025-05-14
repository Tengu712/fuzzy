mod function;
mod number;

use crate::{lexer::*, *};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    I128(i128),
    U128(u128),
    F32(f32),
    F64(f64),
    Symbol(String),
}

impl Value {
    fn get_typeid(&self, _: &Environment) -> String {
        match self {
            Self::Nil => "nil".to_string(),
            Self::I8(_) => "i8".to_string(),
            Self::U8(_) => "u8".to_string(),
            Self::I16(_) => "i16".to_string(),
            Self::U16(_) => "u16".to_string(),
            Self::I32(_) => "i32".to_string(),
            Self::U32(_) => "u32".to_string(),
            Self::I64(_) => "i64".to_string(),
            Self::U64(_) => "u64".to_string(),
            Self::I128(_) => "i128".to_string(),
            Self::U128(_) => "u128".to_string(),
            Self::F32(_) => "f32".to_string(),
            Self::F64(_) => "f64".to_string(),
            // TODO: get variable type.
            Self::Symbol(_) => panic!("unimplemented"),
        }
    }
}

enum Function {
    Builtin(fn(Value, &mut Vec<Value>) -> RResult<()>),
}

struct Environment {
    tokens: Vec<Token>,
    fn_map: HashMap<String, HashMap<String, Function>>,
    // TODO: add variable.
}

pub fn eval(tokens: Vec<Token>) -> RResult<Value> {
    let mut env = Environment {
        tokens,
        fn_map: function::setup(),
    };
    let value = eval_block(&mut env)?;
    if env.tokens.is_empty() {
        Ok(value)
    } else {
        Err("error: some tokens not evaluated.".into())
    }
}

fn eval_block(env: &mut Environment) -> RResult<Value> {
    loop {
        let value = eval_sentence(env)?;
        let dotted = eat_dot(env);
        // TODO: consider ) and }.
        if env.tokens.is_empty() {
            if dotted {
                return Ok(Value::Nil);
            } else {
                return Ok(value);
            }
        }
    }
}

fn eval_sentence(env: &mut Environment) -> RResult<Value> {
    let mut values = Vec::new();
    while is_in_sentence(env) {
        values.push(eval_expression(env)?);
    }
    if values.is_empty() {
        values.push(Value::Nil);
    }
    values.reverse();
    applicate(env, &mut values)
}

fn eat_dot(env: &mut Environment) -> bool {
    match env.tokens.pop() {
        None => false,
        Some(Token::Dot) => true,
        n => panic!("unexpected error: '{n:?}' found immediately after sentence."),
    }
}

fn is_in_sentence(env: &Environment) -> bool {
    !matches!(env.tokens.last(), Some(Token::Dot) | None)
}

fn eval_expression(env: &mut Environment) -> RResult<Value> {
    match env.tokens.pop() {
        None => panic!("unexpected error: no token passed to eval_expression."),
        Some(Token::Dot) => panic!("unexpected error: Token::Dot passed to eval_expression."),
        Some(Token::Symbol(n)) => {
            if let Some(n) = number::parse(&n) {
                Ok(n)
            } else {
                Ok(Value::Symbol(n))
            }
        }
    }
}

fn applicate(env: &mut Environment, values: &mut Vec<Value>) -> RResult<Value> {
    let mut itr = applicate_inner(env, values)?.into_iter();
    let Some(r) = itr.next() else {
        panic!("unexpected error: the result of application is empty.");
    };
    if !values.is_empty() || itr.next().is_some() {
        println!("warn: unused arguments found.");
    }
    Ok(r)
}

fn applicate_inner(env: &mut Environment, values: &mut Vec<Value>) -> RResult<Vec<Value>> {
    let mut args = Vec::new();

    // get subject
    let Some(s) = values.pop() else {
        panic!("unexpected error: no value passed to applicate.");
    };

    // get verb
    let Some(v) = values.pop() else {
        args.push(s);
        return Ok(args);
    };
    let Value::Symbol(v_sym) = &v else {
        args.push(s);
        return Ok(args);
    };

    // get verb function
    let t = s.get_typeid(env);
    if !env.fn_map.contains_key(&t) || !env.fn_map[&t].contains_key(v_sym) {
        values.push(v);
        args.push(s);
        return Ok(args);
    };

    // collect arguments
    while !values.is_empty() {
        let mut result = applicate_inner(env, values)?;
        args.append(&mut result);
    }
    args.reverse();

    // applicate
    match env.fn_map[&t][v_sym] {
        Function::Builtin(f) => (f)(s, &mut args)?,
    }

    // finish
    args.reverse();
    Ok(args)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_not_symbol_not_function() {
        let mut env = Environment {
            tokens: Vec::new(),
            fn_map: function::setup(),
        };
        let mut values = Vec::from(&[Value::I32(1), Value::I32(0), Value::I32(2)]);
        values.reverse();
        assert_eq!(applicate(&mut env, &mut values).unwrap(), Value::I32(1));
    }

    #[test]
    fn test_undefined_function() {
        let mut env = Environment {
            tokens: Vec::new(),
            fn_map: function::setup(),
        };
        let mut values = Vec::from(&[Value::I32(1), Value::Symbol("a".to_string()), Value::I32(2)]);
        values.reverse();
        assert_eq!(applicate(&mut env, &mut values).unwrap(), Value::I32(1));
    }
}
