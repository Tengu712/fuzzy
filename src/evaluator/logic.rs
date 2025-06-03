use super::{
    functions::{FunctionCode, TypesCheckResult},
    types::TypeId,
    value::Value,
    *,
};

pub fn eval_sentence(
    env: &mut Environment,
    tokens: &mut Vec<Token>,
    caches: &mut Vec<Value>,
) -> RResult<Value> {
    let mut s = caches.pop();
    loop {
        if let Some(s) = s.take() {
            caches.push(s);
        }
        s = Some(eval_clause(env, tokens, caches)?.unwrap_or_default());
        if matches!(tokens.last(), Some(Token::Comma)) {
            tokens.pop();
        }
        if is_sentence_end(tokens, caches) {
            break;
        }
        if let Some(vn) = take_verb(env, tokens, caches, &s.as_ref().unwrap().typeid())? {
            caches.push(Value::Symbol(vn));
        } else {
            break;
        }
    }
    Ok(s.unwrap_or_default())
}

fn is_sentence_end(tokens: &[Token], caches: &[Value]) -> bool {
    matches!(tokens.last(), None | Some(Token::Dot)) && caches.is_empty()
}

fn eval_clause(
    env: &mut Environment,
    tokens: &mut Vec<Token>,
    caches: &mut Vec<Value>,
) -> RResult<Option<Value>> {
    if is_clause_end(tokens, caches) {
        return Ok(None);
    }
    let s = pop_cache_or_eval_element(env, tokens, caches)?;
    if is_clause_end(tokens, caches) {
        return Ok(Some(s));
    }
    let ty = &s.typeid();
    let Some(vn) = take_verb(env, tokens, caches, ty)? else {
        return Ok(Some(s));
    };
    let args = collect_args(env, tokens, caches, ty, vn.as_str())?;
    let result = appplicate(env, s, ty, vn.as_str(), args)?;
    Ok(Some(result))
}

fn is_clause_end(tokens: &[Token], caches: &[Value]) -> bool {
    matches!(tokens.last(), None | Some(Token::Dot) | Some(Token::Comma)) && caches.is_empty()
}

fn take_verb(
    env: &mut Environment,
    tokens: &mut Vec<Token>,
    caches: &mut Vec<Value>,
    ty: &TypeId,
) -> RResult<Option<String>> {
    if let Some(n) = caches.pop() {
        if is_verb_name_value(env, ty, &n) {
            let Value::Symbol(n) = n else {
                panic!("failed to extract symbol.");
            };
            Ok(Some(n))
        } else {
            caches.push(n);
            Ok(None)
        }
    } else if let Some(Token::Label(_)) = tokens.last() {
        let Token::Label(n) = tokens.pop().unwrap() else {
            panic!("failed to extract label.");
        };
        if is_verb_name(env, ty, n.as_str()) {
            Ok(Some(n))
        } else {
            tokens.push(Token::Label(n));
            Ok(None)
        }
    } else {
        let n = eval_element(env, tokens)?;
        if is_verb_name_value(env, ty, &n) {
            let Value::Symbol(n) = n else {
                panic!("failed to extract symbol.");
            };
            Ok(Some(n))
        } else {
            caches.push(n);
            Ok(None)
        }
    }
}

fn is_verb_name_value(env: &Environment, ty: &TypeId, v: &Value) -> bool {
    if let Value::Symbol(n) = v {
        is_verb_name(env, ty, n.as_str())
    } else {
        false
    }
}

fn is_verb_name(env: &Environment, ty: &TypeId, vn: &str) -> bool {
    env.fn_map.is_defined(ty, vn) || is_symbol_value(ty, vn)
}

fn collect_args(
    env: &mut Environment,
    tokens: &mut Vec<Token>,
    caches: &mut Vec<Value>,
    ty: &TypeId,
    vn: &str,
) -> RResult<Vec<Value>> {
    if is_symbol_value(ty, vn) {
        return Ok(Vec::new());
    }
    let mut args = Vec::new();
    loop {
        match env.fn_map.check_types(ty, vn, &args) {
            TypesCheckResult::Undecided => (),
            TypesCheckResult::Err(n) => return Err(n.into()),
            TypesCheckResult::Ok => break,
        }
        let Some(arg) = eval_clause(env, tokens, caches)? else {
            return Err(format!("error: too few arguments passed to {} on {}.", vn, ty).into());
        };
        args.push(arg);
    }
    args.reverse();
    Ok(args)
}

fn appplicate(
    env: &mut Environment,
    s: Value,
    ty: &TypeId,
    vn: &str,
    args: Vec<Value>,
) -> RResult<Value> {
    if is_symbol_value(ty, vn) {
        if let Value::Symbol(n) = s {
            return env.vr_map.get_unwrap(&n);
        } else {
            panic!("failed to extract symbol.");
        }
    }
    match env.fn_map.get_code(ty, vn) {
        FunctionCode::Builtin(f) => (f)(env, s, args),
    }
}

fn pop_cache_or_eval_element(
    env: &mut Environment,
    tokens: &mut Vec<Token>,
    caches: &mut Vec<Value>,
) -> RResult<Value> {
    if let Some(n) = caches.pop() {
        Ok(n)
    } else {
        eval_element(env, tokens)
    }
}

fn is_symbol_value(ty: &TypeId, vn: &str) -> bool {
    matches!(ty, TypeId::Symbol) && vn == "$"
}

fn eval_element(env: &mut Environment, tokens: &mut Vec<Token>) -> RResult<Value> {
    match tokens.pop() {
        None => panic!("no token passed to eval_element."),
        Some(Token::Dot) => panic!("Token::Dot passed to eval_element."),
        Some(Token::Comma) => panic!("Token::Comma passed to eval_element."),
        Some(Token::LParen) => {
            let mut n = extract_brackets_content(tokens, Token::LParen, Token::RParen)?;
            let result = eval_block(env, &mut n, None)?.pop().unwrap_or_default();
            Ok(result)
        }
        Some(Token::RParen) => Err("error: unmatched ')' found.".into()),
        Some(Token::LBrace) => {
            let n = extract_brackets_content(tokens, Token::LBrace, Token::RBrace)?;
            Ok(Value::Lazy(n))
        }
        Some(Token::RBrace) => Err("error: unmatched '}' found.".into()),
        Some(Token::LBracket) => {
            let mut n = extract_brackets_content(tokens, Token::LBracket, Token::RBracket)?;
            let results = eval_block(env, &mut n, None)?;
            Ok(Value::Array(results))
        }
        Some(Token::RBracket) => Err("error: unmatched ']' found.".into()),
        Some(Token::Argument(n)) => env
            .get_argument(n)
            .ok_or(format!("error: argument at {n} not found.").into()),
        Some(n) => Value::from(env, n),
    }
}

fn extract_brackets_content(tokens: &mut Vec<Token>, l: Token, r: Token) -> RResult<Vec<Token>> {
    let mut depth = 0;
    for i in (0..tokens.len()).rev() {
        if tokens[i] == r && depth == 0 {
            let mut result = tokens.split_off(i);
            result.remove(0);
            return Ok(result);
        } else if tokens[i] == r {
            depth -= 1;
        } else if tokens[i] == l {
            depth += 1;
        }
    }
    Err(format!("error: unmatched {l} found.").into())
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
