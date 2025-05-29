use crate::{
    RResult,
    evaluator::{self, Environment, Value},
    lexer,
};
use std::fs;

pub fn run(path: &str, args: &[String]) -> RResult<()> {
    // read file
    let content = fs::read_to_string(path).map_err(|e| format!("error: {path}: {e}"))?;

    // lex
    let mut tokens = lexer::lex(&content)?;

    // setup
    let mut env = Environment::default();
    let args = args
        .iter()
        .map(|n| Value::String(n.clone()))
        .collect::<Vec<_>>();
    tokens.reverse();

    // evaluate
    evaluator::eval_block(&mut env, &mut tokens, Some(args))?;

    // finish
    Ok(())
}
