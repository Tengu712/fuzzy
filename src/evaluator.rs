mod number;

use crate::{lexer::*, *};

pub fn eval(tokens: &Vec<Token>) -> RResult<()> {
    let mut env = Environment {
        tokens,
        index: 0,
        stack: Vec::new(),
    };
    Ok(())
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

fn eval_block(tokens: &mut Environment) -> RResult<()> {
    Ok(())
}

fn eval_sentence(tokens: &mut Environment) -> RResult<()> {
    Ok(())
}
