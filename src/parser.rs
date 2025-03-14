use crate::{EError, lexer::*};
use std::{iter::Peekable, slice::Iter};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Nil,
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
    Var(String),
    Imm(Box<Sentence>),
    Lazy(Box<Sentence>),
}

#[derive(Debug, PartialEq)]
pub struct ExprView {
    expr: Expr,
    ln: usize,
    cn: usize,
}
impl ExprView {
    fn from(tokens: &mut Peekable<Iter<TokenMeta>>) -> Option<Self> {
        match tokens.peek()?.token {
            Token::Dot => return None,
            _ => (),
        }
        let token = tokens.next()?;
        let expr = match &token.token {
            Token::I8(n) => Expr::I8(*n),
            Token::U8(n) => Expr::U8(*n),
            Token::I16(n) => Expr::I16(*n),
            Token::U16(n) => Expr::U16(*n),
            Token::I32(n) => Expr::I32(*n),
            Token::U32(n) => Expr::U32(*n),
            Token::I64(n) => Expr::I64(*n),
            Token::U64(n) => Expr::U64(*n),
            Token::I128(n) => Expr::I128(*n),
            Token::U128(n) => Expr::U128(*n),
            Token::Symbol(n) => Expr::Var(n.clone()),
            _ => panic!("unexpected"),
        };
        let ln = token.ln;
        let cn = token.cn;
        Some(Self { expr, ln, cn })
    }

    fn nil(ln: usize, cn: usize) -> Self {
        Self {
            expr: Expr::Nil,
            ln,
            cn,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Sentence {
    pub subject: ExprView,
    pub verb: Option<ExprView>,
    pub objects: Vec<ExprView>,
}
impl Sentence {
    fn from(tokens: &mut Peekable<Iter<TokenMeta>>) -> Option<Self> {
        let subject = ExprView::from(tokens)?;
        let verb = ExprView::from(tokens);
        let mut objects = Vec::new();
        while let Some(n) = ExprView::from(tokens) {
            objects.push(n);
        }
        Some(Self {
            subject,
            verb,
            objects,
        })
    }

    fn nil(ln: usize, cn: usize) -> Self {
        Self {
            subject: ExprView::nil(ln, cn),
            verb: None,
            objects: Vec::new(),
        }
    }
}

pub fn parse(tokens: Vec<TokenMeta>) -> Result<Vec<Sentence>, EError> {
    let mut sentences = Vec::new();
    let mut tokens = tokens.iter().peekable();
    let mut dot = None;
    while let Some(n) = Sentence::from(&mut tokens) {
        sentences.push(n);
        match tokens.peek() {
            Some(token) if token.token == Token::Dot => {
                dot = Some((token.ln, token.cn));
                tokens.next();
            }
            // TODO: what is this case?
            Some(_) => panic!("unexpected"),
            None => dot = None,
        }
    }
    if let Some((ln, cn)) = dot {
        sentences.push(Sentence::nil(ln, cn));
    }
    if let Some(n) = tokens.next() {
        let ln = n.ln;
        let cn = n.cn;
        Err(format!("syntax error: unexpected token found: {ln} line, {cn} char.",).into())
    } else {
        Ok(sentences)
    }
}
