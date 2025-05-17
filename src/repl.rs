use crate::{
    RResult,
    evaluator::{self, Environment, Value},
    lexer,
};
use std::{
    collections::HashMap,
    io::{self, Write},
};

pub fn run() {
    let mut env = Environment::default();
    env.vr_map.push(HashMap::new());
    loop {
        match run_inner(&mut env) {
            Ok(true) => (),
            Ok(false) => break,
            Err(n) => println!("{n}"),
        }
    }
}

fn run_inner(env: &mut Environment) -> RResult<bool> {
    // show prompt
    print!(">> ");
    io::stdout().flush()?;

    // read
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    // exit?
    if input == "#exit" {
        return Ok(false);
    }

    // eval
    let mut tokens = lexer::lex(input)?;
    tokens.reverse();
    let value = evaluator::eval_block(env, &mut tokens)?;

    // print
    println!("{}", value.format_for_repl(env));

    // to next loop
    Ok(true)
}

impl Value {
    fn format_for_repl(&self, env: &Environment) -> String {
        match self {
            Self::Nil => "()".to_string(),
            Self::I8(n) => format!("{n} (i8)"),
            Self::U8(n) => format!("{n} (u8)"),
            Self::I16(n) => format!("{n} (i16)"),
            Self::U16(n) => format!("{n} (u16)"),
            Self::I32(n) => format!("{n} (i32)"),
            Self::U32(n) => format!("{n} (u32)"),
            Self::I64(n) => format!("{n} (i64)"),
            Self::U64(n) => format!("{n} (u64)"),
            Self::I128(n) => format!("{n} (i128)"),
            Self::U128(n) => format!("{n} (u128)"),
            Self::F32(n) => format!("{n} (f32)"),
            Self::F64(n) => format!("{n} (f64)"),
            Self::String(n) => format!("{n} (string)"),
            Self::Symbol(n) => {
                for m in env.vr_map.iter().rev() {
                    if let Some(v) = m.get(n) {
                        let a = if v.mutable { "<-" } else { "<=" };
                        return format!("{n} {a} {}", v.value.format_for_repl(env));
                    }
                }
                format!("{n} (symbol)")
            }
        }
    }
}
