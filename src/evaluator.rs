mod number;

use crate::{lexer::*, *};

pub fn eval(tokens: &Vec<Token>) -> RResult<()> {
    let mut env = Environment {
        tokens,
        index: 0,
        stack: Vec::new(),
    };
    eval_block(&mut env)?;
    if env.index >= env.tokens.len() {
        println!("{:?}", env.stack.last().unwrap());
        Ok(())
    } else {
        Err("error: some tokens not evaluated.".into())
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Value {
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
}

struct Environment<'a> {
    tokens: &'a Vec<Token>,
    index: usize,
    stack: Vec<Value>,
}

fn eval_block(env: &mut Environment) -> RResult<()> {
    eval_sentence(env)?;
    Ok(())
}

fn eval_sentence(env: &mut Environment) -> RResult<()> {
    eval_expression(env)?;
    Ok(())
}

fn eval_expression(env: &mut Environment) -> RResult<()> {
    match env.tokens.get(env.index) {
        None => env.stack.push(Value::Nil),
        _ => panic!("not implemented"),
    }
    Ok(())
}
