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
    env.args.push(Vec::new());
    env.evaluated.push(None);
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
    let value = evaluator::eval_block_directly(env, &mut tokens)?
        .pop()
        .expect("evaluating block result is empty.");

    // print
    println!("{}", value.format_for_repl(env));

    // to next loop
    Ok(true)
}

impl Value {
    fn format_for_repl(&self, env: &Environment) -> String {
        match self {
            Self::Nil => "()".to_string(),
            Self::Top => "T".to_string(),
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
                if let Some(v) = env.get_variable(n) {
                    let a = if v.mutable { "<-" } else { "<=" };
                    format!("{n} {a} {}", v.value.format_for_repl(env))
                } else {
                    format!("{n} (symbol)")
                }
            }
            Self::Array(n) => {
                let mut s = "[".to_string();
                for (i, m) in n.iter().enumerate() {
                    s.push_str(&m.format_for_repl(env));
                    if i < n.len() - 1 {
                        s.push(' ');
                    }
                }
                s.push(']');
                s
            }
            Self::Lazy(_) => "{}".to_string(),
            Self::Function(_) => panic!("unimplemented."),
            Self::Label(_) => panic!("tried to format label."),
        }
    }
}
