mod numeric;
mod print;
mod symbol;
mod variable;

use crate::{lexer::*, *};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
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
    Function(Function),
    Label(String),
}

impl Value {
    fn from(token: Token) -> Self {
        match token {
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
            Self::Nil => "nil".to_string(),
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
            Self::Function(_) => "function".to_string(),
            Self::Label(_) => panic!("tried to get type of label."),
        }
    }
}

const ALL_TYPES: &[&str] = &[
    "nil", "i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64", "i128", "u128", "f32", "f64",
    "string", "symbol",
];

pub struct Variable {
    pub value: Value,
    pub mutable: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionCode {
    Builtin(fn(&mut Environment, Value, Vec<Value>) -> RResult<Value>),
    LazyBlock(Vec<Token>),
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
}

impl Default for Environment {
    fn default() -> Self {
        let mut fn_map = HashMap::new();
        for n in ALL_TYPES {
            fn_map.insert(n.to_string(), HashMap::new());
            print::insert_print(&mut fn_map, n);
            variable::insert_variable_definition(&mut fn_map, n);
        }
        numeric::insert_numeric_functions(&mut fn_map);
        symbol::insert_symbol_value(&mut fn_map);

        Self {
            fn_map,
            vr_map: Vec::new(),
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
/// Returns the evaluation result.
/// If the result is `Ok`, it is guaranteed that all `tokens` are consumed.
///
/// NOTE: This function does not manage the environment's variable map stack.
///       The caller is responsible for managing the stack.
///       This is to accommodate the behavior where top-level blocks in a REPL
///       have their environments expanded globally.
pub fn eval_block(env: &mut Environment, tokens: &mut Vec<Token>) -> RResult<Value> {
    loop {
        let value = eval_sentence(env, tokens)?;
        let dotted = eat_dot(tokens);
        if tokens.is_empty() {
            if dotted {
                return Ok(Value::Nil);
            } else {
                return Ok(value);
            }
        }
    }
}

fn eval_sentence(env: &mut Environment, tokens: &mut Vec<Token>) -> RResult<Value> {
    let mut temp = None;
    loop {
        // handover temp
        let mut values = Vec::new();
        if let Some(n) = temp {
            values.push(n);
        }

        // evaluate until separator
        while !matches!(tokens.last(), Some(Token::Dot) | Some(Token::Comma) | None) {
            values.push(eval_expression(env, tokens)?);
        }
        values.reverse();
        let result = applicate(env, values)?;

        // continue?
        if matches!(tokens.last(), Some(Token::Comma)) {
            tokens.pop().unwrap();
            temp = Some(result);
        } else {
            return Ok(result);
        }
    }
}

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
            let result = eval_block(env, &mut n)?;
            let _ = env.vr_map.pop();
            Ok(result)
        }
        Some(Token::RParen) => Err("error: unmatched ')' found.".into()),
        Some(Token::LBrace) => {
            let Some(n) = extract_brackets_content(tokens, Token::LBrace, Token::RBrace) else {
                return Err("error: unmatched '{' found.".into());
            };
            Ok(Value::Function(Function {
                types: Vec::new(),
                code: FunctionCode::LazyBlock(n),
            }))
        }
        Some(Token::RBrace) => Err("error: unmatched '}' found.".into()),
        Some(n) => Ok(Value::from(n)),
    }
}

fn eat_dot(tokens: &mut Vec<Token>) -> bool {
    match tokens.pop() {
        None => false,
        Some(Token::Dot) => true,
        n => panic!("'{n:?}' found immediately after sentence."),
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
        .all(|(n, m)| n == &m.get_typeid())
    {
        Ok(true)
    } else {
        Err(format!("error: type missmatched arguments passed to '{v}' on '{t}'.").into())
    }
}

fn applicate(env: &mut Environment, mut parseds: Vec<Value>) -> RResult<Value> {
    loop {
        let result = applicate_inner(env, &mut parseds)?;
        if parseds.is_empty() {
            return Ok(result);
        }
    }
}

fn applicate_inner(env: &mut Environment, parseds: &mut Vec<Value>) -> RResult<Value> {
    // get subject
    let s = if let Some(n) = parseds.pop() {
        expand_label(env, n)?
    } else {
        Value::Nil
    };

    // get verb
    let Some(v) = parseds.pop() else {
        return Ok(s);
    };
    let Value::Label(v_name) = &v else {
        parseds.push(v);
        return Ok(s);
    };

    // get verb function
    let t = s.get_typeid();
    if !env.fn_map.contains_key(&t) || !env.fn_map[&t].contains_key(v_name) {
        parseds.push(v);
        return Ok(s);
    };

    // collect arguments
    let mut args = Vec::new();
    loop {
        if check_argument_types(env, &t, v_name, &args)? {
            break;
        }
        if parseds.is_empty() {
            return Err(format!("error: too few arguments passed to '{t}' on '{v_name}'.").into());
        }
        let arg = applicate_inner(env, parseds)?;
        let arg = expand_label(env, arg)?;
        args.push(arg);
    }
    args.reverse();

    // applicate
    let result = match env.fn_map[&t][v_name].code {
        FunctionCode::Builtin(f) => (f)(env, s, args)?,
        FunctionCode::LazyBlock(_) => panic!("unimplemented"),
    };
    Ok(result)
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

    #[test]
    fn test_no_dot_return_last_result() {
        let mut env = Environment::default();
        let mut parseds = vec![Value::I32(1), Value::I32(2), Value::I32(3)];
        parseds.reverse();
        assert_eq!(applicate(&mut env, parseds).unwrap(), Value::I32(3));
    }

    #[test]
    fn test_few_arguments() {
        let mut env = Environment::default();
        let mut parseds = vec![Value::I32(1), Value::Label("+".to_string())];
        parseds.reverse();
        applicate(&mut env, parseds).unwrap_err();
    }
}
