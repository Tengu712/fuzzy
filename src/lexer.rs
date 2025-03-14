use crate::EError;
use std::{
    io::{BufRead, BufReader, Read},
    iter::Peekable,
    str::CharIndices,
};

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
impl Token {
    fn from(s: String) -> Self {
        match s.as_str() {
            "." => Self::Dot,
            _ => number::lex(&s).unwrap_or(Self::Symbol(s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenView {
    pub token: Token,
    pub ln: usize,
    pub cn: usize,
}

fn eat(chars: &mut Peekable<CharIndices>, ln: usize) -> Vec<TokenView> {
    // skip blank
    while let Some((_, c)) = chars.peek() {
        match c {
            ' ' | '\t' => (),
            _ => break,
        }
        chars.next();
    }

    // check next
    let Some(next) = chars.peek() else {
        return Vec::new();
    };

    // lex token
    let cn = next.0 + 1;
    let mut buf = String::new();
    while let Some((_, c)) = chars.next() {
        match c {
            ' ' | '\t' => break,
            _ => buf.push(c),
        }
    }

    // prepare
    let mut tokens = Vec::new();

    // pop tail .
    while !buf.is_empty() {
        if buf.ends_with(".") {
            tokens.push(TokenView {
                token: Token::from(buf.pop().unwrap().to_string()),
                ln,
                cn: cn + buf.len(),
            });
        } else {
            break;
        }
    }

    // finish
    if !buf.is_empty() {
        tokens.push(TokenView {
            token: Token::from(buf),
            ln,
            cn,
        });
    }
    tokens.reverse();
    tokens
}

pub fn lex<R: Read>(content: R) -> Result<Vec<TokenView>, EError> {
    let mut tokens = Vec::new();
    for (ln, l) in BufReader::new(content).lines().enumerate() {
        let l = l?;
        let mut chars = l.char_indices().peekable();
        while chars.peek().is_some() {
            tokens.append(&mut eat(&mut chars, ln + 1));
        }
    }
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
                    cn: 1,
                },
                TokenView {
                    token: Token::Dot,
                    ln: 1,
                    cn: 3,
                },
            ])
        );
    }

    #[test]
    fn test_lex_foo_dot_minus254i16_dot_dot_bar() {
        assert_eq!(
            lex(" foo   .  -254i16..  \n\nbar \n".as_bytes()).unwrap(),
            Vec::from(&[
                TokenView {
                    token: Token::Symbol("foo".to_string()),
                    ln: 1,
                    cn: 2,
                },
                TokenView {
                    token: Token::Dot,
                    ln: 1,
                    cn: 8,
                },
                TokenView {
                    token: Token::I16(-254),
                    ln: 1,
                    cn: 11,
                },
                TokenView {
                    token: Token::Dot,
                    ln: 1,
                    cn: 18,
                },
                TokenView {
                    token: Token::Dot,
                    ln: 1,
                    cn: 19,
                },
                TokenView {
                    token: Token::Symbol("bar".to_string()),
                    ln: 3,
                    cn: 1,
                },
            ])
        );
    }
}
