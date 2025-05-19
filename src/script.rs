use crate::{
    RResult,
    evaluator::{self, Environment},
    lexer,
};
use std::{collections::HashMap, fs};

pub fn run(path: String) -> RResult<()> {
    let content = fs::read_to_string(&path).map_err(|e| format!("error: {path}: {e}"))?;
    let mut tokens = lexer::lex(&content)?;
    let mut env = Environment::default();
    tokens.reverse();
    env.vr_map.push(HashMap::new());
    evaluator::eval_block(&mut env, &mut tokens)?;
    Ok(())
}
