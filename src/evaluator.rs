mod functions;
mod logic;
mod types;
mod value;
mod variable;

use crate::{lexer::*, *};
use std::collections::HashMap;

#[derive(Default)]
pub struct Environment {
    fn_map: functions::FunctionMap,
    vr_map: variable::VariableMapStack,
    args: Vec<Vec<value::Value>>,
}

impl Environment {
    pub fn prepare_block_scope(&mut self, args: Option<Vec<value::Value>>) {
        if let Some(n) = args {
            self.args.push(n);
        }
        self.vr_map.push();
    }

    pub fn cleanup_block_scope(&mut self, should_pop_args: bool) {
        if should_pop_args {
            self.args.pop();
        }
        self.vr_map.pop();
    }

    fn get_argument(&self, i: usize) -> Option<value::Value> {
        // OPTIMIZE: remove clone.
        self.args
            .last()
            .expect("argument stack is empty.")
            .get(i)
            .cloned()
    }
}

/// A function to convert command line arguments to Fuzzy values.
pub fn parse_command_line_args(args: Vec<String>) -> Vec<value::Value> {
    args.into_iter()
        .map(value::Value::String)
        .collect::<Vec<_>>()
}

/// A function to evaluate a block.
///
/// * `env` - The current environment.
/// * `tokens` - All tokens in the block in reverse order.
/// * `args` - The list of arguments passed to the block.
///
/// Evaluates all statements and returns their results.
/// If the last statement ends with `.`, `Value::Nil` is appended to the results.
///
/// If the result is `Ok`, it is guaranteed that all `tokens` are consumed.
///
/// NOTE: Only top-level and lazy blocks should be passed `Some` for `args`.
///       In other words, evaluating an immediate block doesn't affect the argument list's stack.
pub fn eval_block(
    env: &mut Environment,
    tokens: &mut Vec<Token>,
    args: Option<Vec<value::Value>>,
) -> RResult<Vec<value::Value>> {
    let should_pop_args = args.is_some();
    env.prepare_block_scope(args);
    let results = eval_block_directly(env, tokens);
    env.cleanup_block_scope(should_pop_args);
    results
}

/// A function to evaluate a block without any environment setup.
///
/// * `env` - The current environment.
/// * `tokens` - All tokens in the block in reverse order.
///
/// NOTE: This function does not manage the environment's variable map stack.
///       The caller is responsible for managing the stack.
///       This is to accommodate the behavior where top-level blocks in a REPL
///       have their environments expanded globally.
pub fn eval_block_directly(
    env: &mut Environment,
    tokens: &mut Vec<Token>,
) -> RResult<Vec<value::Value>> {
    let mut values = Vec::new();
    let mut caches = Vec::new();
    let mut dotted = false;
    while !tokens.is_empty() || !caches.is_empty() {
        values.push(logic::eval_sentence(env, tokens, &mut caches, true)?);
        dotted = matches!(tokens.last(), Some(Token::Dot)) && tokens.pop().is_some();
    }
    if dotted {
        values.push(value::Value::Nil);
    }
    Ok(values)
}
