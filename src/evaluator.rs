mod function;
mod number;

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
}

impl Value {
    fn from(s: &str) -> Option<Self> {
        if let Some(n) = number::parse(s) {
            Some(n)
        } else if s.starts_with("\"") && s.ends_with("\"") {
            Some(Value::String(s[1..s.len() - 1].to_string()))
        } else if s.starts_with("'") {
            Some(Value::Symbol(s[1..s.len()].to_string()))
        } else {
            None
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
        }
    }
}

pub struct Variable {
    pub value: Value,
    pub mutable: bool,
}

pub enum FunctionCode {
    Builtin(fn(&mut Environment, Value, Vec<Value>) -> RResult<Value>),
}

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
        Self {
            fn_map: function::setup(),
            vr_map: Vec::new(),
        }
    }
}

impl Environment {
    pub fn get_variable(&self, name: &str) -> Option<&Variable> {
        self.vr_map.iter().rev().filter_map(|n| n.get(name)).next()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Parsed {
    Comma,
    Label(String),
    Value(Value),
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
    let mut parseds = Vec::new();
    while !matches!(tokens.last(), Some(Token::Dot) | None) {
        parseds.push(eval_expression(env, tokens)?);
    }
    parseds.reverse();
    applicate(env, parseds)
}

fn eval_expression(env: &mut Environment, tokens: &mut Vec<Token>) -> RResult<Parsed> {
    match tokens.pop() {
        None => panic!("no token passed to eval_expression."),
        Some(Token::Dot) => panic!("Token::Dot passed to eval_expression."),
        Some(Token::Comma) => Ok(Parsed::Comma),
        Some(Token::LParen) => {
            let Some(mut inner) = extract_parenthesized_content(tokens) else {
                return Err("error: unmatchd '(' found.".into());
            };
            env.vr_map.push(HashMap::new());
            let result = eval_block(env, &mut inner)?;
            let _ = env.vr_map.pop();
            Ok(Parsed::Value(result))
        }
        Some(Token::RParen) => Err("error: unmatched ')' found.".into()),
        Some(Token::Symbol(n)) => {
            if let Some(n) = Value::from(&n) {
                Ok(Parsed::Value(n))
            } else {
                Ok(Parsed::Label(n))
            }
        }
    }
}

fn eat_dot(tokens: &mut Vec<Token>) -> bool {
    match tokens.pop() {
        None => false,
        Some(Token::Dot) => true,
        n => panic!("'{n:?}' found immediately after sentence."),
    }
}

fn extract_parenthesized_content(tokens: &mut Vec<Token>) -> Option<Vec<Token>> {
    let mut depth = 0;
    for (i, t) in tokens.iter().enumerate() {
        match t {
            Token::RParen if depth == 0 => {
                let result = tokens.split_off(i + 1);
                tokens.pop();
                return Some(result);
            }
            Token::RParen => depth -= 1,
            Token::LParen => depth += 1,
            _ => (),
        }
    }
    None
}

fn extract_until_comma(parseds: &mut Vec<Parsed>) -> Option<Vec<Parsed>> {
    parseds
        .iter()
        .rposition(|n| matches!(n, Parsed::Comma))
        .map(|i| {
            let result = parseds.split_off(i + 1);
            parseds.pop();
            result
        })
}

pub fn check_argument_types(env: &Environment, t: &str, v: &str, args: &[Value]) -> RResult<bool> {
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

fn applicate(env: &mut Environment, mut parseds: Vec<Parsed>) -> RResult<Value> {
    loop {
        let result = applicate_inner(env, &mut parseds)?;
        if parseds.is_empty() {
            return Ok(result);
        }
    }
}

fn applicate_inner(env: &mut Environment, parseds: &mut Vec<Parsed>) -> RResult<Value> {
    // get subject
    let s = if let Some(n) = extract_until_comma(parseds) {
        applicate(env, n)?
    } else {
        match parseds.pop() {
            None => Value::Nil,
            Some(Parsed::Comma) => return Err("error: comma cannot be the subject.".into()),
            Some(Parsed::Label(n)) => {
                if let Some(n) = env.get_variable(&n) {
                    // OPTIMIZE: remove clone.
                    n.value.clone()
                } else {
                    return Err(format!("error: undefined variable '{n}' found.").into());
                }
            }
            Some(Parsed::Value(n)) => n,
        }
    };

    // get verb
    let Some(v) = parseds.pop() else {
        return Ok(s);
    };
    let Parsed::Label(v_name) = &v else {
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
        args.push(applicate_inner(env, parseds)?);
    }
    args.reverse();

    // applicate
    let result = match env.fn_map[&t][v_name].code {
        FunctionCode::Builtin(f) => (f)(env, s, args)?,
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
        let mut expect = vec![Token::Symbol("1".to_string())];
        expect.reverse();
        let result = extract_parenthesized_content(&mut tokens).unwrap();
        assert_eq!(result, expect);
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
        let mut expect = vec![
            Token::Symbol("1".to_string()),
            Token::LParen,
            Token::Symbol("2".to_string()),
            Token::RParen,
        ];
        expect.reverse();
        let result = extract_parenthesized_content(&mut tokens).unwrap();
        assert_eq!(result, expect);
    }

    #[test]
    fn test_extract_until_comma() {
        let mut parseds = vec![
            Parsed::Value(Value::I32(1)),
            Parsed::Comma,
            Parsed::Value(Value::I32(2)),
            Parsed::Comma,
        ];
        parseds.reverse();
        let mut expect = vec![Parsed::Value(Value::I32(1))];
        expect.reverse();
        let result = extract_until_comma(&mut parseds).unwrap();
        assert_eq!(result, expect);
    }

    #[test]
    fn test_no_dot_return_last_result() {
        let mut env = Environment::default();
        let mut parseds = vec![
            Parsed::Value(Value::I32(1)),
            Parsed::Value(Value::I32(2)),
            Parsed::Value(Value::I32(3)),
        ];
        parseds.reverse();
        assert_eq!(applicate(&mut env, parseds).unwrap(), Value::I32(3));
    }

    #[test]
    fn test_few_arguments() {
        let mut env = Environment::default();
        let mut parseds = vec![Parsed::Value(Value::I32(1)), Parsed::Label("+".to_string())];
        parseds.reverse();
        applicate(&mut env, parseds).unwrap_err();
    }
}
