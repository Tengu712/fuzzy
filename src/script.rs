use crate::{
    RResult,
    evaluator::{self, Environment},
    lexer,
};
use std::fs;

pub fn run(path: String) -> RResult<()> {
    let content = fs::read_to_string(path)?;
    let mut tokens = lexer::lex(&content)?;
    let mut env = Environment::default();
    tokens.reverse();
    evaluator::eval_block(&mut env, &mut tokens)?;
    Ok(())
}
