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

pub fn lex(code: String) -> Vec<TokenView> {
    Lexer {
        code,
        idx: 0,
        ln: 1,
        cn: 1,
    }
    .lex()
}

struct Lexer {
    code: String,
    idx: usize,
    ln: usize,
    cn: usize,
}
impl Lexer {
    fn lex(mut self) -> Vec<TokenView> {
        let mut tokens = Vec::new();
        while self.idx < self.code.len() {
            self.eat_blank();
            tokens.append(&mut self.eat_token());
        }
        tokens
    }

    fn eat_blank(&mut self) {
        while let Some(c) = self.code.chars().nth(self.idx) {
            match c {
                ' ' | '\t' => {
                    self.idx += 1;
                    self.cn += 1;
                }
                '\r' | '\n' => {
                    self.idx += 1;
                    self.ln += 1;
                    self.cn = 1;
                }
                _ => break,
            }
        }
    }

    fn eat_token(&mut self) -> Vec<TokenView> {
        let ln = self.ln;
        let cn = self.cn;
        let mut buf = String::new();
        while let Some(c) = self.code.chars().nth(self.idx) {
            match c {
                ' ' | '\t' | '\r' | '\n' => break,
                _ => {
                    buf.push(c);
                    self.idx += 1;
                    self.cn += 1;
                }
            }
        }
        let mut tokens = Vec::new();
        while !buf.is_empty() {
            if buf.ends_with(".") {
                tokens.push(TokenView {
                    token: Token::from(buf.pop().unwrap().to_string()),
                    ln,
                    cn: cn + buf.len(),
                });
            } else {
                tokens.push(TokenView {
                    token: Token::from(buf),
                    ln,
                    cn,
                });
                break;
            }
        }
        tokens.reverse();
        tokens
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lex_empty() {
        assert_eq!(lex("".to_string()), Vec::new());
    }

    #[test]
    fn test_lex_12() {
        assert_eq!(
            lex("12.".to_string()),
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
            lex(" foo   .  -254i16..  \n\nbar \n".to_string()),
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
