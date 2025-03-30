use crate::*;
use regex::Regex;

#[derive(serde::Serialize, Debug, Clone, PartialEq)]
pub enum Token {
    Dot,
    Symbol(String),
}
impl Token {
    fn from(s: &str) -> Self {
        match s {
            "." => Self::Dot,
            _ => Self::Symbol(s.to_string()),
        }
    }
}

pub fn lex(code: &str) -> RResult<Vec<Token>> {
    let tokens = Regex::new(r#""[^"]*"|\S+|\."#)?
        .find_iter(code)
        .flat_map(|n| split_trailing_signs(n.as_str()))
        .map(Token::from)
        .collect::<Vec<Token>>();
    Ok(tokens)
}

fn split_trailing_signs(s: &str) -> Vec<&str> {
    let pos = s.rfind(|c| c != '.').map(|i| i + 1).unwrap_or(0);
    let mut v = Vec::new();
    v.push(&s[..pos]);
    for i in pos..s.len() {
        v.push(&s[i..i + 1]);
    }
    v
}

#[cfg(test)]
mod test {
    use super::*;
    use insta;

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
}
