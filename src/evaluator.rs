mod functions;
mod types;
mod value;

use crate::{lexer::*, *};
use std::collections::HashMap;

pub struct Variable {
    pub value: value::Value,
    pub mutable: bool,
}

pub type VariableMapStack = Vec<HashMap<String, Variable>>;

pub struct Environment {
    pub fn_map: functions::FunctionMap,
    pub vr_map: VariableMapStack,
    pub args: Vec<Vec<value::Value>>,
    pub evaluated: Vec<Option<value::Value>>,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            fn_map: functions::FunctionMap::default(),
            vr_map: Vec::new(),
            args: Vec::new(),
            evaluated: Vec::new(),
        }
    }
}

impl Environment {
    fn get_variable_mut(&mut self, name: &str) -> Option<&mut Variable> {
        self.vr_map.iter_mut().rev().find_map(|n| n.get_mut(name))
    }

    pub fn get_variable(&self, name: &str) -> Option<&Variable> {
        self.vr_map.iter().rev().find_map(|n| n.get(name))
    }

    pub fn get_variable_unwrap(&self, name: &str) -> RResult<value::Value> {
        if let Some(n) = self.get_variable(name) {
            // OPTIMIZE: remove clone.
            Ok(n.value.clone())
        } else {
            Err(format!("error: undefined variable '{name}' found.").into())
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

    fn take_evaluated(&mut self) -> Option<value::Value> {
        self.evaluated.last_mut().and_then(|n| n.take())
    }

    fn set_evaluated(&mut self, v: value::Value) {
        if let Some(n) = self.evaluated.last_mut() {
            *n = Some(v);
        }
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
    let args_is_some = args.is_some();
    if args_is_some {
        env.args.push(args.unwrap());
    };
    env.vr_map.push(HashMap::new());
    env.evaluated.push(None);

    let results = eval_block_directly(env, tokens);

    if args_is_some {
        env.args.pop();
    }
    env.vr_map.pop();
    env.evaluated.pop();

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
    let mut dotted = true;
    while !tokens.is_empty() || env.evaluated.last().map(|n| n.is_some()).unwrap_or(false) {
        values.push(eval_sentence(env, tokens)?);
        dotted = eat_dot(tokens);
    }
    if dotted {
        values.push(value::Value::Nil);
    }
    Ok(values)
}

fn eat_dot(tokens: &mut Vec<Token>) -> bool {
    matches!(tokens.last(), Some(Token::Dot)) && tokens.pop().is_some()
}

/// A function to evaluate a sentence.
///
/// * `env` - The current environment.
/// * `tokens` - All tokens in the block in reverse order.
fn eval_sentence(env: &mut Environment, tokens: &mut Vec<Token>) -> RResult<value::Value> {
    let mut s = env.take_evaluated();
    loop {
        let n = eval_sentence_inner(env, tokens, s)?;
        if matches!(tokens.last(), Some(Token::Comma)) {
            tokens.pop();
            s = Some(n);
        } else {
            return Ok(n);
        }
    }
}

fn eval_sentence_inner(
    env: &mut Environment,
    tokens: &mut Vec<Token>,
    s: Option<value::Value>,
) -> RResult<value::Value> {
    // end?
    if is_end_sentence(tokens) {
        return Ok(s.unwrap_or(value::Value::Nil));
    }

    // get subject
    let s = s.map_or_else(|| eval_expression(env, tokens), Ok)?;
    let s = expand_label(env, s)?;

    // end?
    if is_end_sentence(tokens) {
        return Ok(s);
    }

    // get verb
    let v = eval_expression(env, tokens)?;
    let value::Value::Label(vn) = &v else {
        env.set_evaluated(v);
        return Ok(s);
    };

    // get verb function
    let t = s.get_typeid();
    if !env.fn_map.is_defined(&t, vn) {
        env.set_evaluated(v);
        return Ok(s);
    };

    // collect arguments
    let mut args = Vec::new();
    while !check_argument_types(env, &t, vn, &args)? {
        if let Some(arg) = env.take_evaluated() {
            args.push(arg);
            continue;
        }
        if is_end_sentence(tokens) {
            return Err(format!("error: too few arguments passed to '{vn}' on '{t}'.").into());
        }
        let arg = eval_sentence_inner(env, tokens, None)?;
        let arg = expand_label(env, arg)?;
        args.push(arg);
    }
    args.reverse();

    // applicate
    let result = match env.fn_map.get_code(&t, vn) {
        functions::FunctionCode::Builtin(f) => (f)(env, s, args)?,
    };
    Ok(result)
}

fn is_end_sentence(tokens: &[Token]) -> bool {
    matches!(tokens.last(), None | Some(Token::Dot) | Some(Token::Comma))
}

fn expand_label(env: &Environment, n: value::Value) -> RResult<value::Value> {
    match n {
        value::Value::Label(n) => env.get_variable_unwrap(&n),
        n => Ok(n),
    }
}

fn check_argument_types(
    env: &Environment,
    t: &types::TypeId,
    v: &str,
    args: &[value::Value],
) -> RResult<bool> {
    let ts = env.fn_map.get_types(t, v);
    if ts.len() > args.len() {
        Ok(false)
    } else if ts
        .iter()
        .zip(args.iter())
        .all(|(n, m)| n == &m.get_typeid() || n == &types::TypeId::Any)
    {
        Ok(true)
    } else {
        let expected = ts
            .iter()
            .enumerate()
            .map(|(i, n)| format!("({}) {n}", i + 1))
            .collect::<Vec<_>>()
            .join(", ");
        let instead = args
            .iter()
            .enumerate()
            .map(|(i, n)| format!("({}) {}", i + 1, n.get_typeid()))
            .collect::<Vec<_>>()
            .join(", ");
        Err(
            format!("error: '{v}' on '{t}' expected arguments {expected}, but passed {instead}.")
                .into(),
        )
    }
}

/// A function to evaluate an atom or a block.
///
/// * `env` - The current environment.
/// * `tokens` - All tokens in the block in reverse order.
fn eval_expression(env: &mut Environment, tokens: &mut Vec<Token>) -> RResult<value::Value> {
    match tokens.pop() {
        None => panic!("no token passed to eval_expression."),
        Some(Token::Dot) => panic!("Token::Dot passed to eval_expression."),
        Some(Token::Comma) => panic!("Token::Comma passed to eval_expression."),
        Some(Token::LParen) => {
            let Some(mut n) = extract_brackets_content(tokens, Token::LParen, Token::RParen) else {
                return Err("error: unmatched '(' found.".into());
            };
            let result = eval_block(env, &mut n, None)?
                .pop()
                .expect("evaluating block result is empty.");
            Ok(result)
        }
        Some(Token::RParen) => Err("error: unmatched ')' found.".into()),
        Some(Token::LBrace) => {
            let Some(n) = extract_brackets_content(tokens, Token::LBrace, Token::RBrace) else {
                return Err("error: unmatched '{' found.".into());
            };
            Ok(value::Value::Lazy(n))
        }
        Some(Token::RBrace) => Err("error: unmatched '}' found.".into()),
        Some(Token::LBracket) => {
            let Some(mut n) = extract_brackets_content(tokens, Token::LBracket, Token::RBracket)
            else {
                return Err("error: unmatched '[' found.".into());
            };
            let results = eval_block(env, &mut n, None)?;
            Ok(value::Value::Array(results))
        }
        Some(Token::RBracket) => Err("error: unmatched ']' found.".into()),
        Some(Token::Argument(n)) => env
            .get_argument(n)
            .ok_or(format!("error: argument at {n} not found.").into()),
        Some(n) => Ok(value::Value::from(n)),
    }
}

fn extract_brackets_content(tokens: &mut Vec<Token>, l: Token, r: Token) -> Option<Vec<Token>> {
    let mut depth = 0;
    for i in (0..tokens.len()).rev() {
        if tokens[i] == r && depth == 0 {
            let mut result = tokens.split_off(i);
            result.remove(0);
            return Some(result);
        } else if tokens[i] == r {
            depth -= 1;
        } else if tokens[i] == l {
            depth += 1;
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parenthesis() {
        let mut tokens = vec![
            // Token::LParen,
            Token::Symbol("1".to_string()),
            Token::RParen,
            Token::Symbol("2".to_string()),
        ];
        tokens.reverse();
        let result_expect = vec![Token::Symbol("1".to_string())];
        let tokens_expect = vec![Token::Symbol("2".to_string())];
        let result = extract_brackets_content(&mut tokens, Token::LParen, Token::RParen).unwrap();
        assert_eq!(tokens, tokens_expect);
        assert_eq!(result, result_expect);
    }

    #[test]
    fn test_multiple_parenthesis() {
        let mut tokens = vec![
            // Token::LParen,
            Token::Symbol("1".to_string()),
            Token::LParen,
            Token::Symbol("2".to_string()),
            Token::RParen,
            Token::RParen,
            Token::Symbol("3".to_string()),
        ];
        tokens.reverse();
        let result_expect = vec![
            Token::RParen,
            Token::Symbol("2".to_string()),
            Token::LParen,
            Token::Symbol("1".to_string()),
        ];
        let tokens_expect = vec![Token::Symbol("3".to_string())];
        let result = extract_brackets_content(&mut tokens, Token::LParen, Token::RParen).unwrap();
        assert_eq!(tokens, tokens_expect);
        assert_eq!(result, result_expect);
    }

    #[test]
    fn test_continuous_parenthesis() {
        let mut tokens = vec![
            // Token::LParen,
            Token::Symbol("1".to_string()),
            Token::RParen,
            Token::LParen,
            Token::Symbol("3".to_string()),
            Token::RParen,
        ];
        tokens.reverse();
        let result_expect = vec![Token::Symbol("1".to_string())];
        let tokens_expect = vec![Token::RParen, Token::Symbol("3".to_string()), Token::LParen];
        let result = extract_brackets_content(&mut tokens, Token::LParen, Token::RParen).unwrap();
        assert_eq!(tokens, tokens_expect);
        assert_eq!(result, result_expect);
    }
}
