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
    Symbol(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenView {
    pub token: Token,
    pub ln: usize,
    pub cn: usize,
}
impl TokenView {
    fn new(s: &str, ln: usize, cn: usize) -> Self {
        if let Some(token) = number::lex(s) {
            Self { token, ln, cn }
        } else {
            let token = Token::Symbol(s.to_string());
            Self { token, ln, cn }
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
    fn dispatch(
        &mut self,
        c: char,
        ln: usize,
        cn: usize,
        token: &mut TempToken,
        tokens: &mut Vec<TokenView>,
    ) {
        match self {
            Mode::Blank => match c {
                ' ' | '\t' => (),
                '.' => tokens.push(TokenView::dot(ln, cn)),
                _ => {
                    *token = TempToken::new(ln, cn);
                    token.s.push(c);
                    *self = Mode::Token;
                }
            },
            Mode::Token => match c {
                ' ' | '\t' | '.' => {
                    tokens.push(TokenView::new(&token.s, token.ln, token.cn));
                    if c == '.' {
                        tokens.push(TokenView::dot(ln, cn));
                    }
                    *self = Mode::Blank;
                }
                _ => token.s.push(c),
            },
        }
    }

    fn dispatch_before_eol(&self, _: &TempToken, _: &mut Vec<TokenView>) {
        match self {
            _ => (),
        }
    }

    fn dispatch_before_eof(&self, token: &TempToken, tokens: &mut Vec<TokenView>) {
        match self {
            Mode::Token => tokens.push(TokenView::new(&token.s, token.ln, token.cn)),
            _ => (),
        }
    }
}

pub fn lex<R: Read>(content: R) -> Result<Vec<TokenView>, EError> {
    let mut mode = Mode::Blank;
    let mut token = TempToken::new(0, 0);
    let mut tokens = Vec::new();
    for (ln, l) in BufReader::new(content).lines().enumerate() {
        for (cn, c) in l?.chars().enumerate() {
            mode.dispatch(c, ln + 1, cn + 1, &mut token, &mut tokens);
        }
        mode.dispatch_before_eol(&token, &mut tokens);
    }
    mode.dispatch_before_eof(&token, &mut tokens);
    Ok(tokens)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lex_empty() {
        assert_eq!(lex("".as_bytes()).unwrap(), Vec::new());
    }

    #[test]
    fn test_lex_12() {
        assert_eq!(
            lex("12.".as_bytes()).unwrap(),
            Vec::from(&[
                TokenView {
                    token: Token::I32(12),
                    ln: 1,
                    cn: 1
                },
                TokenView {
                    token: Token::Dot,
                    ln: 1,
                    cn: 3
                }
            ])
        );
    }
}
