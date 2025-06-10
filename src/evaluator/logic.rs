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
    is_toplevel: bool,
) -> RResult<Option<Value>> {
    let mut s = caches.pop();
    let mut first = true;
    loop {
        // take over subject
        if let Some(s) = s.take() {
            caches.push(s);
        } else if !first {
            caches.push(Value::Nil);
        }

        // eval clause
        s = eval_clause(env, tokens, caches)?;
        first = false;

        // consume comma
        //
        // NOTE: If this sentence is a clause, this ends here.
        //       This occurs in structures like `S V So1 Vo1, So2`.
        if matches!(tokens.last(), Some(Token::Comma)) {
            tokens.pop();
            if !is_toplevel {
                break;
            }
        }

        // consume semicolon
        //
        // NOTE: Unlike commas, semicolons are only consumed at the top level.
        if matches!(tokens.last(), Some(Token::Semicolon)) {
            if is_toplevel {
                tokens.pop();
            } else {
                break;
            }
        }

        // end?
        if is_sentence_end(tokens, caches) {
            break;
        }

        // check if the next value is a valid verb
        //
        // NOTE: In an `S V O V' O'` sentence structure,
        //       - if `V'` is the verb of `S`, then the process should carry
        //         over to the next loop.
        //       - Otherwise, the sentence ends here, meaning `V'` is considered
        //         the subject of the next sentence.
        let ty = s.as_ref().unwrap_or(&Value::Nil).typeid();
        if let Some(vn) = take_verb_name(env, tokens, &ty) {
            tokens.push(Token::Label(vn));
        } else {
            break;
        }
    }
    Ok(s)
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
    let Some(vn) = take_verb_name(env, tokens, ty) else {
        return Ok(Some(s));
    };

    let args = collect_args(env, tokens, caches, ty, vn.as_str())?;
    let result = applicate(env, s, ty, vn.as_str(), args)?;
    Ok(Some(result))
}

fn is_clause_end(tokens: &[Token], caches: &[Value]) -> bool {
    matches!(
        tokens.last(),
        None | Some(Token::Dot) | Some(Token::Comma) | Some(Token::Semicolon)
    ) && caches.is_empty()
}

fn take_verb_name(env: &Environment, tokens: &mut Vec<Token>, ty: &TypeId) -> Option<String> {
    match tokens.pop() {
        Some(Token::Label(vn)) if is_valid_verb(env, ty, &vn) => Some(vn),
        Some(n) => {
            tokens.push(n);
            None
        }
        None => None,
    }
}

fn is_valid_verb(env: &Environment, ty: &TypeId, vn: &str) -> bool {
    is_symbol_value(ty, vn) || env.fn_map.is_defined(env.get_self_type(), ty, vn)
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
        if let Some(n) = caches.pop() {
            args.push(n);
            continue;
        }
        let Some(n) = eval_sentence(env, tokens, caches, false)? else {
            return Err(format!("error: too few arguments passed to {} on {}.", vn, ty).into());
        };
        args.push(n);
    }
    args.reverse();
    Ok(args)
}

fn applicate(
    env: &mut Environment,
    s: Value,
    ty: &TypeId,
    vn: &str,
    args: Vec<Value>,
) -> RResult<Value> {
    if is_symbol_value(ty, vn) {
        let Value::Symbol(n) = s else {
            panic!("failed to extract symbol.");
        };
        return env.vr_map.get_unwrap(env.get_self_type(), &n);
    }
    match env.fn_map.get_code(ty, vn) {
        FunctionCode::Builtin(f) => (f)(env, s, args),
        FunctionCode::UserDefined(mut tokens) => {
            let params = EnterLazyParams {
                slf: Some(s),
                args: Some(args),
            };
            let mut results = eval_block(env, &mut tokens, params)?;
            let result = results.pop().unwrap_or_default();
            Ok(result)
        }
    }
}

fn is_symbol_value(ty: &TypeId, vn: &str) -> bool {
    matches!(ty, TypeId::Symbol) && vn == "%"
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

fn eval_element(env: &mut Environment, tokens: &mut Vec<Token>) -> RResult<Value> {
    match tokens.pop() {
        None => panic!("no token passed to eval_element."),
        Some(Token::Dot) => panic!("Token::Dot passed to eval_element."),
        Some(Token::Comma) => panic!("Token::Comma passed to eval_element."),
        Some(Token::Semicolon) => panic!("Token::Semicolon passed to eval_element."),
        Some(Token::LParen) => {
            let mut n = extract_brackets_content(tokens, Token::LParen, Token::RParen)?;
            let result = eval_block(env, &mut n, EnterLazyParams::default())?
                .pop()
                .unwrap_or_default();
            Ok(result)
        }
        Some(Token::RParen) => Err("error: unmatched ')' found.".into()),
        Some(Token::LBrace) => {
            let n = extract_brackets_content(tokens, Token::LBrace, Token::RBrace)?;
            Ok(Value::Lazy(n.into()))
        }
        Some(Token::RBrace) => Err("error: unmatched '}' found.".into()),
        Some(Token::LBracket) => {
            let mut n = extract_brackets_content(tokens, Token::LBracket, Token::RBracket)?;
            let results = eval_block(env, &mut n, EnterLazyParams::default())?;
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
