use crate::{
    RResult,
    evaluator::{self, Environment},
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
    println!("{}", value.format_in_detail(env));

    // to next loop
    Ok(true)
}
