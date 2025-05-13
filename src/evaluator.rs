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

fn eat_dot(env: &mut Environment) -> bool {
    match env.tokens.pop() {
        None => false,
        Some(Token::Dot) => true,
        n => panic!("unexpected error: '{n:?}' found immediately after sentence."),
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
    while values.len() > 1 {
        applicate(env, &mut values)?;
    }
    Ok(values.pop().unwrap())
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

fn applicate(env: &mut Environment, values: &mut Vec<Value>) -> RResult<()> {
    let Some(s) = values.pop() else {
        panic!("unexpected error: no value passed to applicate.");
    };
    let Some(v) = values.pop() else {
        values.push(s);
        return Ok(());
    };
    let Value::Symbol(v) = v else {
        return Err(format!("error: function must be a symbol but {v:?} provided.").into());
    };
    let t = s.get_typeid(env);
    let Some(f) = env.fn_map.get(&t).and_then(|n| n.get(&v)) else {
        return Err(format!("error: function '{v}' not defined on {t}.").into());
    };
    match f {
        Function::Builtin(f) => (f)(s, values)?,
    }
    Ok(())
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
        applicate(&mut env, &mut values).unwrap_err();
    }

    #[test]
    fn test_undefined_function() {
        let mut env = Environment {
            tokens: Vec::new(),
            fn_map: function::setup(),
        };
        let mut values = Vec::from(&[Value::I32(1), Value::Symbol("a".to_string()), Value::I32(2)]);
        applicate(&mut env, &mut values).unwrap_err();
    }
}
