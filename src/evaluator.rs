mod array;
mod lazy;
mod numeric;
mod print;
mod symbol;
mod variable;

use crate::{lexer::*, *};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Top,
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    I128(i128),
    U128(u128),
    F32(f32),
    F64(f64),
    String(String),
    Symbol(String),
    Array(Vec<Value>),
    Lazy(Vec<Token>),
    Label(String),
}

impl Value {
    fn from(token: Token) -> Self {
        match token {
            Token::Top => Self::Top,
            Token::I8(n) => Self::I8(n),
            Token::U8(n) => Self::U8(n),
            Token::I16(n) => Self::I16(n),
            Token::U16(n) => Self::U16(n),
            Token::I32(n) => Self::I32(n),
            Token::U32(n) => Self::U32(n),
            Token::I64(n) => Self::I64(n),
            Token::U64(n) => Self::U64(n),
            Token::I128(n) => Self::I128(n),
            Token::U128(n) => Self::U128(n),
            Token::F32(n) => Self::F32(n),
            Token::F64(n) => Self::F64(n),
            Token::String(n) => Self::String(n),
            Token::Symbol(n) => Self::Symbol(n),
            Token::Label(n) => Self::Label(n),
            _ => panic!("tried to create value from non-atom token."),
        }
    }

    fn get_typeid(&self) -> String {
        match self {
            Self::Nil => "bool".to_string(),
            Self::Top => "bool".to_string(),
            Self::I8(_) => "i8".to_string(),
            Self::U8(_) => "u8".to_string(),
            Self::I16(_) => "i16".to_string(),
            Self::U16(_) => "u16".to_string(),
            Self::I32(_) => "i32".to_string(),
            Self::U32(_) => "u32".to_string(),
            Self::I64(_) => "i64".to_string(),
            Self::U64(_) => "u64".to_string(),
            Self::I128(_) => "i128".to_string(),
            Self::U128(_) => "u128".to_string(),
            Self::F32(_) => "f32".to_string(),
            Self::F64(_) => "f64".to_string(),
            Self::String(_) => "string".to_string(),
            Self::Symbol(_) => "symbol".to_string(),
            Self::Array(_) => "[]".to_string(),
            Self::Lazy(_) => "{}".to_string(),
            Self::Label(_) => panic!("tried to get type of label."),
        }
    }
}

const ALL_TYPES: &[&str] = &[
    "bool", "i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64", "i128", "u128", "f32", "f64",
    "string", "symbol", "[]", "{}",
];

pub struct Variable {
    pub value: Value,
    pub mutable: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionCode {
    Builtin(fn(&mut Environment, Value, Vec<Value>) -> RResult<Value>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub types: Vec<String>,
    pub code: FunctionCode,
}

pub type FunctionMap = HashMap<String, HashMap<String, Function>>;
pub type VariableMapStack = Vec<HashMap<String, Variable>>;

pub struct Environment {
    pub fn_map: FunctionMap,
    pub vr_map: VariableMapStack,
    evaluated: Option<Value>,
}

impl Default for Environment {
    fn default() -> Self {
        let mut fn_map = HashMap::new();
        for n in ALL_TYPES {
            fn_map.insert(n.to_string(), HashMap::new());
            print::insert_print(&mut fn_map, n);
            variable::insert_variable_definition(&mut fn_map, n);
        }
        array::insert_array_functions(&mut fn_map);
        lazy::insert_lazy_functions(&mut fn_map);
        numeric::insert_numeric_functions(&mut fn_map);
        symbol::insert_symbol_value(&mut fn_map);

        Self {
            fn_map,
            vr_map: Vec::new(),
            evaluated: None,
        }
    }
}

impl Environment {
    pub fn get_variable(&self, name: &str) -> Option<&Variable> {
        self.vr_map.iter().rev().filter_map(|n| n.get(name)).next()
    }

    pub fn get_variable_unwrap(&self, name: &str) -> RResult<Value> {
        if let Some(n) = self.get_variable(name) {
            // OPTIMIZE: remove clone.
            Ok(n.value.clone())
        } else {
            Err(format!("error: undefined variable '{name}' found.").into())
        }
    }
}

/// A function to evaluate a block.
///
/// * `env` - The current environment.
/// * `tokens` - All tokens in the block in reverse order.
///
/// Evaluates all statements and returns their results.
/// If the last statement ends with `.`, `Value::Nil` is appended to the results.
///
/// If the result is `Ok`, it is guaranteed that all `tokens` are consumed.
///
/// NOTE: This function does not manage the environment's variable map stack.
///       The caller is responsible for managing the stack.
///       This is to accommodate the behavior where top-level blocks in a REPL
///       have their environments expanded globally.
pub fn eval_block(env: &mut Environment, tokens: &mut Vec<Token>) -> RResult<Vec<Value>> {
    let mut values = Vec::new();
    let mut dotted = true;
    while !tokens.is_empty() || env.evaluated.is_some() {
        values.push(eval_sentence(env, tokens)?);
        dotted = eat_dot(tokens);
    }
    if dotted {
        values.push(Value::Nil);
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
fn eval_sentence(env: &mut Environment, tokens: &mut Vec<Token>) -> RResult<Value> {
    let mut s = env.evaluated.take();
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
    s: Option<Value>,
) -> RResult<Value> {
    // end?
    if is_end_sentence(tokens) {
        return Ok(s.unwrap_or(Value::Nil));
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
    let Value::Label(vn) = &v else {
        env.evaluated = Some(v);
        return Ok(s);
    };

    // get verb function
    let t = s.get_typeid();
    if !env
        .fn_map
        .get(&t)
        .map(|n| n.contains_key(vn))
        .unwrap_or(false)
    {
        env.evaluated = Some(v);
        return Ok(s);
    };

    // collect arguments
    let mut args = Vec::new();
    while !check_argument_types(env, &t, vn, &args)? {
        if let Some(arg) = env.evaluated.take() {
            args.push(arg);
            continue;
        }
        if is_end_sentence(tokens) {
            return Err(format!("error: too few arguments passed to '{t}' on '{vn}'.").into());
        }
        let arg = eval_sentence_inner(env, tokens, None)?;
        let arg = expand_label(env, arg)?;
        args.push(arg);
    }
    args.reverse();

    // applicate
    let result = match env.fn_map[&t][vn].code {
        FunctionCode::Builtin(f) => (f)(env, s, args)?,
    };
    Ok(result)
}

fn is_end_sentence(tokens: &[Token]) -> bool {
    matches!(tokens.last(), None | Some(Token::Dot) | Some(Token::Comma))
}

fn expand_label(env: &Environment, n: Value) -> RResult<Value> {
    match n {
        Value::Label(n) => env.get_variable_unwrap(&n),
        n => Ok(n),
    }
}

fn check_argument_types(env: &Environment, t: &str, v: &str, args: &[Value]) -> RResult<bool> {
    let f = env
        .fn_map
        .get(t)
        .and_then(|n| n.get(v))
        .unwrap_or_else(|| panic!("tried to get undefined function '{v}' on '{t}'"));
    if f.types.len() > args.len() {
        Ok(false)
    } else if f
        .types
        .iter()
        .zip(args.iter())
        .all(|(n, m)| n == &m.get_typeid() || n == "_")
    {
        Ok(true)
    } else {
        Err(format!("error: type missmatched arguments passed to '{v}' on '{t}'.").into())
    }
}

/// A function to evaluate an atom or a block.
///
/// * `env` - The current environment.
/// * `tokens` - All tokens in the block in reverse order.
fn eval_expression(env: &mut Environment, tokens: &mut Vec<Token>) -> RResult<Value> {
    match tokens.pop() {
        None => panic!("no token passed to eval_expression."),
        Some(Token::Dot) => panic!("Token::Dot passed to eval_expression."),
        Some(Token::Comma) => panic!("Token::Comma passed to eval_expression."),
        Some(Token::LParen) => {
            let Some(mut n) = extract_brackets_content(tokens, Token::LParen, Token::RParen) else {
                return Err("error: unmatched '(' found.".into());
            };
            env.vr_map.push(HashMap::new());
            let result = eval_block(env, &mut n)?
                .pop()
                .expect("evaluating block result is empty.");
            env.vr_map.pop();
            Ok(result)
        }
        Some(Token::RParen) => Err("error: unmatched ')' found.".into()),
        Some(Token::LBrace) => {
            let Some(n) = extract_brackets_content(tokens, Token::LBrace, Token::RBrace) else {
                return Err("error: unmatched '{' found.".into());
            };
            Ok(Value::Lazy(n))
        }
        Some(Token::RBrace) => Err("error: unmatched '}' found.".into()),
        Some(Token::LBracket) => {
            let Some(mut n) = extract_brackets_content(tokens, Token::LBracket, Token::RBracket)
            else {
                return Err("error: unmatched '[' found.".into());
            };
            env.vr_map.push(HashMap::new());
            let results = eval_block(env, &mut n)?;
            env.vr_map.pop();
            Ok(Value::Array(results))
        }
        Some(Token::RBracket) => Err("error: unmatched ']' found.".into()),
        Some(n) => Ok(Value::from(n)),
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
