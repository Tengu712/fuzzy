use crate::*;
use regex::Regex;

#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Dot,
    Comma,
    LParen,
    RParen,
    Symbol(String),
}
impl Token {
    fn from(s: &str) -> Self {
        match s {
            "." => Self::Dot,
            "," => Self::Comma,
            "(" => Self::LParen,
            ")" => Self::RParen,
            _ => Self::Symbol(s.to_string()),
        }
    }
}

pub fn lex(code: &str) -> RResult<Vec<Token>> {
    let tokens = Regex::new(r#""[^"]*"|[()]|\S+|\.|,"#)?
        .find_iter(code)
        .flat_map(|n| split_trailing_signs(n.as_str()))
        .map(Token::from)
        .collect::<Vec<Token>>();
    Ok(tokens)
}

fn split_trailing_signs(s: &str) -> Vec<&str> {
    if is_sign_str(s) {
        return vec![s];
    }
    let pos = s
        .rfind(|c: char| !is_sign_char(c))
        .map(|i| i + 1)
        .unwrap_or(0);
    let mut v = Vec::new();
    v.push(&s[..pos]);
    for i in pos..s.len() {
        v.push(&s[i..i + 1]);
    }
    v
}

fn is_sign_str(s: &str) -> bool {
    if s.len() == 1 {
        is_sign_char(s.chars().next().unwrap())
    } else {
        false
    }
}

fn is_sign_char(c: char) -> bool {
    matches!(c, '.' | ',' | '(' | ')')
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_oneline() {
        let tokens = lex("	  12 	 ->   twelve.  ").unwrap();
        insta::assert_yaml_snapshot!(tokens);
    }

    #[test]
    fn test_float_and_dot() {
        let tokens = lex("1.2f32.").unwrap();
        insta::assert_yaml_snapshot!(tokens);
    }

    #[test]
    fn test_parenthesis() {
        let tokens = lex("1 + (2 * 3).").unwrap();
        insta::assert_yaml_snapshot!(tokens);
    }

    #[test]
    fn test_continuous_parenthesis() {
        let tokens = lex("(1 + (2 * 3)).").unwrap();
        insta::assert_yaml_snapshot!(tokens);
    }

    #[test]
    fn test_comma() {
        let tokens = lex("1 + 2, * 3").unwrap();
        insta::assert_yaml_snapshot!(tokens);
    }
}
