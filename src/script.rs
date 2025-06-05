use crate::{
    RResult,
    evaluator::{self, EnterLazyParams, Environment},
    lexer,
};
use std::fs;

pub fn run(path: String, args: Vec<String>) -> RResult<()> {
    // read file
    let content = fs::read_to_string(&path).map_err(|e| format!("error: {path}: {e}"))?;

    // lex
    let mut tokens = lexer::lex(&content)?;

    // setup
    let mut env = Environment::default();
    let args = evaluator::parse_command_line_args(args);
    let params = EnterLazyParams {
        slf: None,
        args: Some(args),
    };
    tokens.reverse();

    // evaluate
    evaluator::eval_block(&mut env, &mut tokens, params)?;

    // finish
    Ok(())
}
