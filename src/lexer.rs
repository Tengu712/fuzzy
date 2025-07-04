use crate::*;
use regex::Regex;
use std::fmt::{Display, Result};

#[derive(serde::Serialize, Debug, Clone, PartialEq)]
pub enum Token {
    // signs
    Dot,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    // atoms
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
    Argument(usize),
    Label(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        match self {
            Self::Dot => write!(f, "."),
            Self::Comma => write!(f, ","),
            Self::Semicolon => write!(f, ";"),
            Self::LParen => write!(f, "("),
            Self::RParen => write!(f, ")"),
            Self::LBrace => write!(f, "{{"),
            Self::RBrace => write!(f, "}}"),
            Self::LBracket => write!(f, "["),
            Self::RBracket => write!(f, "]"),
            Self::Top => write!(f, "T"),
            Self::I8(n) => write!(f, "{n}"),
            Self::U8(n) => write!(f, "{n}"),
            Self::I16(n) => write!(f, "{n}"),
            Self::U16(n) => write!(f, "{n}"),
            Self::I32(n) => write!(f, "{n}"),
            Self::U32(n) => write!(f, "{n}"),
            Self::I64(n) => write!(f, "{n}"),
            Self::U64(n) => write!(f, "{n}"),
            Self::I128(n) => write!(f, "{n}"),
            Self::U128(n) => write!(f, "{n}"),
            Self::F32(n) => write!(f, "{n}"),
            Self::F64(n) => write!(f, "{n}"),
            Self::String(n) => write!(f, "\"{n}\""),
            Self::Symbol(n) => write!(f, "'{n}"),
            Self::Argument(n) => write!(f, "#{n}"),
            Self::Label(n) => write!(f, "{n}"),
        }
    }
}

impl Token {
    pub fn from(s: &str) -> Self {
        if s == "." {
            Self::Dot
        } else if s == "," {
            Self::Comma
        } else if s == ";" {
            Self::Semicolon
        } else if s == "(" {
            Self::LParen
        } else if s == ")" {
            Self::RParen
        } else if s == "{" {
            Self::LBrace
        } else if s == "}" {
            Self::RBrace
        } else if s == "[" {
            Self::LBracket
        } else if s == "]" {
            Self::RBracket
        } else if s == "T" {
            Self::Top
        } else if let Some(n) = parse_number(s) {
            n
        } else if s.starts_with("\"") && s.ends_with("\"") {
            Self::String(parse_string_literal(&s[1..s.len() - 1]))
        } else if s.starts_with("'") {
            Self::Symbol(s[1..s.len()].to_string())
        } else if let Some(n) = parse_argument(s) {
            n
        } else {
            Self::Label(s.to_string())
        }
    }
}

pub fn lex(code: &str) -> RResult<Vec<Token>> {
    let mut tokens = Vec::new();
    let regex = Regex::new(r#""(?:[^"\\]|\\.)*"|[(\{\[)\}\]]|\S+|\.|,|;"#)?;
    for l in code.lines() {
        let l = l.find("--").map(|n| &l[..n]).unwrap_or(l);
        let i = regex
            .find_iter(l)
            .flat_map(|n| split_trailing_signs(n.as_str()))
            .filter(|n| !n.is_empty())
            .map(Token::from);
        tokens.extend(i);
    }
    Ok(tokens)
}

fn split_trailing_signs(s: &str) -> Vec<&str> {
    if is_sign_str(s) {
        return vec![s];
    }
    let spos = if s.starts_with("'[]") || s.starts_with("'{}") {
        3
    } else {
        0
    };
    let pos = s[spos..]
        .rfind(|c: char| !is_sign_char(c))
        .map(|i| spos + i + 1)
        .unwrap_or(spos);
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
    matches!(c, '.' | ',' | ';' | '(' | ')' | '{' | '}' | '[' | ']')
}

fn parse_number(s: &str) -> Option<Token> {
    let caps = Regex::new(r"(.*?)(i8|u8|i16|u16|i32|u32|i64|u64|i128|u128|f32|f64)?$")
        .ok()?
        .captures(s)?;
    let f = caps.get(1)?.as_str();
    let l = caps.get(2).map(|n| n.as_str());
    match (f, l) {
        (n, None) => n.parse::<i32>().ok().map(Token::I32),
        (n, Some("i8")) => n.parse::<i8>().ok().map(Token::I8),
        (n, Some("u8")) => n.parse::<u8>().ok().map(Token::U8),
        (n, Some("i16")) => n.parse::<i16>().ok().map(Token::I16),
        (n, Some("u16")) => n.parse::<u16>().ok().map(Token::U16),
        (n, Some("i32")) => n.parse::<i32>().ok().map(Token::I32),
        (n, Some("u32")) => n.parse::<u32>().ok().map(Token::U32),
        (n, Some("i64")) => n.parse::<i64>().ok().map(Token::I64),
        (n, Some("u64")) => n.parse::<u64>().ok().map(Token::U64),
        (n, Some("i128")) => n.parse::<i128>().ok().map(Token::I128),
        (n, Some("u128")) => n.parse::<u128>().ok().map(Token::U128),
        (n, Some("f32")) => n.parse::<f32>().ok().map(Token::F32),
        (n, Some("f64")) => n.parse::<f64>().ok().map(Token::F64),
        _ => None,
    }
}

fn parse_argument(s: &str) -> Option<Token> {
    s.strip_prefix('#')?
        .parse::<usize>()
        .ok()
        .map(Token::Argument)
}

fn parse_string_literal(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some('r') => result.push('\r'),
                Some('\\') => result.push('\\'),
                Some('"') => result.push('"'),
                Some('0') => result.push('\0'),
                Some(c) => {
                    result.push('\\');
                    result.push(c);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(c);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use fake::{Fake, Faker};

    #[test]
    fn test_oneline() {
        let tokens = lex("	  12 	 ->   'twelve.  ").unwrap();
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

    #[test]
    fn test_braces() {
        let tokens = lex("{ 1 + 2 } -> f.").unwrap();
        insta::assert_yaml_snapshot!(tokens);
    }

    #[test]
    fn test_brackets() {
        let tokens = lex("[1 + 2].").unwrap();
        insta::assert_yaml_snapshot!(tokens);
    }

    #[test]
    fn test_top() {
        let tokens = lex("foo B T 'T").unwrap();
        insta::assert_yaml_snapshot!(tokens);
    }

    #[test]
    fn test_argument() {
        let tokens = lex("foo 12 #012 T").unwrap();
        insta::assert_yaml_snapshot!(tokens);
    }

    #[test]
    fn test_lex_array_type() {
        let tokens = lex("['i32 '[] '[]]").unwrap();
        insta::assert_yaml_snapshot!(tokens);
    }

    #[test]
    fn test_lex_lazy_type() {
        let tokens = lex("['i32 '{} '{}]").unwrap();
        insta::assert_yaml_snapshot!(tokens);
    }

    #[test]
    fn test_lex_semicolon() {
        let tokens = lex("1 + 2; * 5").unwrap();
        insta::assert_yaml_snapshot!(tokens);
    }

    #[test]
    fn test_string() {
        let tokens = lex(r#""hello world""#).unwrap();
        insta::assert_yaml_snapshot!(tokens);
    }

    #[test]
    fn test_string_literal_escape() {
        let tokens = lex(r#""hell\\o\nworld\t\"test\"""#).unwrap();
        insta::assert_yaml_snapshot!(tokens);
    }

    #[test]
    fn test_comment() {
        let tokens = lex("-- head\n1 + 2. -- middle\n3 * 4 --tail").unwrap();
        insta::assert_yaml_snapshot!(tokens);
    }

    #[test]
    fn test_parse_u32_not_number() {
        assert_eq!(parse_number("u32"), None);
    }

    #[test]
    fn test_parse_foof64_not_number() {
        assert_eq!(parse_number("foof64"), None);
    }

    #[test]
    fn test_parse_2147483648_not_number() {
        assert_eq!(parse_number("2147483648"), None);
    }

    #[test]
    fn test_parse_2147483648u16_not_number() {
        assert_eq!(parse_number("2147483648u16"), None);
    }

    #[test]
    fn test_parse_2147483648u32_to_u32() {
        assert_eq!(parse_number("2147483648u32"), Some(Token::U32(2147483648)));
    }

    #[test]
    fn test_parse_1f32_to_f32() {
        assert_eq!(parse_number("1f32"), Some(Token::F32(1.0)));
    }

    #[test]
    fn test_parse_1_e_minus_2_f32_to_f32() {
        assert_eq!(parse_number("1e-2f32"), Some(Token::F32(0.01)));
    }

    #[test]
    fn test_parse_no_suffix_i32() {
        let n = Faker.fake::<i32>();
        assert_eq!(parse_number(&format!("{n}")), Some(Token::I32(n)));
    }

    #[test]
    fn test_parse_u64() {
        let n = Faker.fake::<u64>();
        assert_eq!(parse_number(&format!("{n}u64")), Some(Token::U64(n)));
    }

    #[test]
    fn test_parse_f32() {
        let n = Faker.fake::<f32>();
        assert_eq!(parse_number(&format!("{n}f32")), Some(Token::F32(n)));
    }

    #[test]
    fn test_hash_and_not_number_not_argument() {
        assert_eq!(parse_argument("#foo"), None);
    }

    #[test]
    fn test_parse_argument() {
        let n = Faker.fake::<usize>();
        assert_eq!(parse_argument(&format!("#{n}")), Some(Token::Argument(n)));
    }
}
