mod functions;
mod logic;
mod types;
mod usertype;
mod value;
mod variable;

use crate::{lexer::*, *};
use std::collections::HashMap;

#[derive(Default)]
pub struct EnterLazyParams {
    pub slf: Option<value::Value>,
    pub args: Option<Vec<value::Value>>,
}

#[derive(Default)]
pub struct Environment {
    fn_map: functions::FunctionMapStack,
    vr_map: variable::VariableMapStack,
    ut_map: usertype::UserTypeMapStack,
    args: Vec<Vec<value::Value>>,
}

impl Environment {
    pub fn prepare_block_scope(&mut self, params: EnterLazyParams) {
        self.fn_map.push();
        self.vr_map.push();
        self.ut_map.push();
        if let Some(n) = params.slf {
            self.vr_map.insert_self(n);
        }
        if let Some(n) = params.args {
            self.args.push(n);
        }
    }

    pub fn cleanup_block_scope(&mut self, pop_args: bool) {
        self.fn_map.pop();
        self.vr_map.pop();
        self.ut_map.pop();
        if pop_args {
            self.args.pop();
        }
    }

    fn get_argument(&self, i: usize) -> Option<value::Value> {
        // OPTIMIZE: remove clone.
        self.args
            .last()
            .expect("argument stack is empty.")
            .get(i)
            .cloned()
    }

    fn get_self_type(&self) -> Option<types::TypeId> {
        self.vr_map.get("##").map(|n| n.typeid())
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
/// * `params` - The parameters passed to the block.
///
/// Evaluates all statements and returns their results.
/// If the last statement ends with `.`, `Value::Nil` is appended to the results.
///
/// If the result is `Ok`, it is guaranteed that all `tokens` are consumed.
///
/// NOTE: Only top-level and lazy blocks should be passed `Some` for `params.args`.
///       In other words, evaluating an immediate block doesn't affect the argument list's stack.
pub fn eval_block(
    env: &mut Environment,
    tokens: &mut Vec<Token>,
    params: EnterLazyParams,
) -> RResult<Vec<value::Value>> {
    let pop_args = params.args.is_some();
    env.prepare_block_scope(params);
    let results = eval_block_directly(env, tokens);
    env.cleanup_block_scope(pop_args);
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
        let n = logic::eval_sentence(env, tokens, &mut caches, true)?.unwrap_or_default();
        values.push(n);
        dotted = matches!(tokens.last(), Some(Token::Dot)) && tokens.pop().is_some();
    }
    if dotted {
        values.push(value::Value::Nil);
    }
    Ok(values)
}
