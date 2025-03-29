use regex::Regex;
use crate::*;

#[derive(serde::Serialize, Debug, Clone, PartialEq)]
pub enum Token {
    Symbol(String),
}
impl Token {
    fn from(s: &str) -> Self {
        Self::Symbol(s.to_string())
    }
}

pub fn lex(code: &str) -> RResult<Vec<Token>> {
    let tokens = Regex::new(r#""[^"]*"|\S+"#)?
        .find_iter(code)
        .map(|n| Token::from(n.as_str()))
        .collect::<Vec<Token>>();
    Ok(tokens)
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
}
