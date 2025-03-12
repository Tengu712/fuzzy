use crate::EError;
use std::io::{BufRead, BufReader, Read};

mod number;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Dot,
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenMeta {
    pub token: Token,
    pub ln: usize,
    pub cn: usize,
}
impl TokenMeta {
    fn new(s: &str, ln: usize, cn: usize) -> Self {
        if let Some(token) = number::parse(s) {
            Self { token, ln, cn }
        } else {
            // TODO: parse symbol token.
            panic!("symbol token not implemented.");
        }
    }
    fn dot(ln: usize, cn: usize) -> Self {
        Self {
            token: Token::Dot,
            ln,
            cn,
        }
    }
}

struct TempToken {
    s: String,
    ln: usize,
    cn: usize,
}
impl TempToken {
    fn new(ln: usize, cn: usize) -> Self {
        Self {
            s: String::new(),
            ln,
            cn,
        }
    }
}

enum Mode {
    Blank,
    Token,
}
impl Mode {
    fn parse(
        &mut self,
        c: char,
        ln: usize,
        cn: usize,
        token: &mut TempToken,
        tokens: &mut Vec<TokenMeta>,
    ) {
        match self {
            Mode::Blank => match c {
                ' ' | '\t' | '\r' | '\n' => (),
                '.' => tokens.push(TokenMeta::dot(ln, cn)),
                _ => {
                    *token = TempToken::new(ln, cn);
                    token.s.push(c);
                    *self = Mode::Token;
                }
            },
            Mode::Token => match c {
                ' ' | '\t' | '\r' | '\n' => {
                    tokens.push(TokenMeta::new(&token.s, token.ln, token.cn));
                    *self = Mode::Blank;
                }
                '.' => {
                    tokens.push(TokenMeta::new(&token.s, token.ln, token.cn));
                    tokens.push(TokenMeta::dot(ln, cn));
                    *self = Mode::Blank;
                }
                _ => token.s.push(c),
            },
        }
    }
}

pub fn parse<R: Read>(content: R) -> Result<Vec<TokenMeta>, EError> {
    let mut mode = Mode::Blank;
    let mut token = TempToken {
        s: String::new(),
        ln: 0,
        cn: 0,
    };
    let mut tokens = Vec::new();

    for (ln, l) in BufReader::new(content).lines().enumerate() {
        for (cn, c) in l?.chars().enumerate() {
            mode.parse(c, ln + 1, cn + 1, &mut token, &mut tokens);
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_empty() {
        assert_eq!(parse("".as_bytes()).unwrap(), Vec::new());
    }

    #[test]
    fn test_parse_12() {
        assert_eq!(
            parse("12.".as_bytes()).unwrap(),
            Vec::from(&[
                TokenMeta {
                    token: Token::I32(12),
                    ln: 1,
                    cn: 1
                },
                TokenMeta {
                    token: Token::Dot,
                    ln: 1,
                    cn: 3
                }
            ])
        );
    }
}
