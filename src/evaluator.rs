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
    String(String),
    Symbol(String),
}

impl Value {
    fn get_typeid(&self, env: &Environment) -> String {
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
            Self::String(_) => "string".to_string(),
            Self::Symbol(n) => {
                for m in env.vr_map.iter().rev() {
                    if let Some(v) = m.get(n) {
                        return v.value.get_typeid(env);
                    }
                }
                "symbol".to_string()
            }
        }
    }
}

pub struct Variable {
    pub value: Value,
    pub mutable: bool,
}

pub enum Function {
    Builtin(fn(&mut Environment, Value, &mut Vec<Value>) -> RResult<()>),
}

pub type FunctionMap = HashMap<String, HashMap<String, Function>>;
pub type VariableMapStack = Vec<HashMap<String, Variable>>;

pub struct Environment {
    pub fn_map: FunctionMap,
    pub vr_map: VariableMapStack,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            fn_map: function::setup(),
            vr_map: Vec::new(),
        }
    }
}

// TODO: add documentation comment.
pub fn eval_block(env: &mut Environment, tokens: &mut Vec<Token>) -> RResult<Value> {
    loop {
        let value = eval_sentence(env, tokens)?;
        let dotted = eat_dot(tokens);
        if tokens.is_empty() {
            if dotted {
                return Ok(Value::Nil);
            } else {
                return Ok(value);
            }
        }
    }
}

fn eval_sentence(env: &mut Environment, tokens: &mut Vec<Token>) -> RResult<Value> {
    let mut values = Vec::new();
    while is_in_sentence(tokens) {
        values.push(eval_expression(env, tokens)?);
    }
    if values.is_empty() {
        values.push(Value::Nil);
    }
    applicate(env, values)
}

fn eval_expression(env: &mut Environment, tokens: &mut Vec<Token>) -> RResult<Value> {
    match tokens.pop() {
        None => panic!("unexpected error: no token passed to eval_expression."),
        Some(Token::Dot) => panic!("unexpected error: Token::Dot passed to eval_expression."),
        Some(Token::LParen) => {
            let Some(mut inner) = split_at_last_token(tokens, Token::RParen) else {
                return Err("error: unmatchd '(' found.".into());
            };
            assert!(
                matches!(tokens.pop(), Some(Token::RParen)),
                "unexpected error: failed to pop ')' from tokens."
            );
            env.vr_map.push(HashMap::new());
            let result = eval_block(env, &mut inner)?;
            let _ = env.vr_map.pop();
            Ok(result)
        }
        Some(Token::RParen) => Err("error: unmatched ')' found.".into()),
        Some(Token::Symbol(n)) => {
            if let Some(n) = number::parse(&n) {
                Ok(n)
            } else if n.starts_with("\"") && n.ends_with("\"") {
                Ok(Value::String(n[1..n.len() - 1].to_string()))
            } else {
                Ok(Value::Symbol(n))
            }
        }
    }
}

fn eat_dot(tokens: &mut Vec<Token>) -> bool {
    match tokens.pop() {
        None => false,
        Some(Token::Dot) => true,
        n => panic!("unexpected error: '{n:?}' found immediately after sentence."),
    }
}

fn is_in_sentence(tokens: &[Token]) -> bool {
    !matches!(tokens.last(), Some(Token::Dot) | None)
}

fn split_at_last_token(tokens: &mut Vec<Token>, token: Token) -> Option<Vec<Token>> {
    tokens
        .iter()
        .rposition(|n| n == &token)
        .map(|n| tokens.split_off(n + 1))
}

fn applicate(env: &mut Environment, mut values: Vec<Value>) -> RResult<Value> {
    values.reverse();
    let mut itr = applicate_inner(env, &mut values)?.into_iter();
    let r = itr
        .next()
        .expect("unexpected error: the result of application is empty.");
    if !values.is_empty() || itr.next().is_some() {
        println!("warn: unused arguments found.");
    }
    Ok(r)
}

fn applicate_inner(env: &mut Environment, values: &mut Vec<Value>) -> RResult<Vec<Value>> {
    let mut args = Vec::new();

    // get subject
    let s = values
        .pop()
        .expect("unexpected error: no value passed to applicate.");

    // get verb
    let Some(v) = values.pop() else {
        args.push(s);
        return Ok(args);
    };
    let Value::Symbol(v_sym) = &v else {
        values.push(v);
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
        Function::Builtin(f) => (f)(env, s, &mut args)?,
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
        let mut env = Environment::default();
        let values = Vec::from(&[Value::I32(1), Value::I32(0), Value::I32(2)]);
        assert_eq!(applicate(&mut env, values).unwrap(), Value::I32(1));
    }

    #[test]
    fn test_undefined_function() {
        let mut env = Environment::default();
        let values = Vec::from(&[Value::I32(1), Value::Symbol("a".to_string()), Value::I32(2)]);
        assert_eq!(applicate(&mut env, values).unwrap(), Value::I32(1));
    }
}
