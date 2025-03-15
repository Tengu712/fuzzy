use crate::{EError, lexer::*};

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
    Imm(Vec<Sentence>),
}

#[derive(Debug, PartialEq)]
pub struct ExprView {
    expr: Expr,
    ln: usize,
    cn: usize,
}

#[derive(Debug, PartialEq)]
pub struct Sentence {
    pub subject: ExprView,
    pub verb: Option<ExprView>,
    pub objects: Vec<ExprView>,
}

pub fn parse(tokens: Vec<TokenView>) -> Result<ExprView, EError> {
    let mut parser = Parser { tokens, idx: 0 };
    let sentences = parser.parse_block();
    if parser.is_end() {
        Ok(ExprView {
            expr: Expr::Imm(sentences),
            ln: 1,
            cn: 1,
        })
    } else {
        // TODO: write better message.
        Err("unexpected".into())
    }
}

struct Parser {
    tokens: Vec<TokenView>,
    idx: usize,
}
impl Parser {
    fn parse_block(&mut self) -> Vec<Sentence> {
        let mut sentences = Vec::new();
        while let Some(n) = self.parse_sentence() {
            sentences.push(n);
            if self.parse_dot().is_none() {
                break;
            }
        }
        sentences
    }

    fn parse_sentence(&mut self) -> Option<Sentence> {
        let subject = self.parse_expr()?;
        let verb = self.parse_expr();
        let mut objects = Vec::new();
        while let Some(n) = self.parse_expr() {
            objects.push(n);
        }
        Some(Sentence {
            subject,
            verb,
            objects,
        })
    }

    fn parse_expr(&mut self) -> Option<ExprView> {
        let expr = match &self.tokens.get(self.idx)?.token {
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
            _ => return None,
        };
        let ln = self.tokens[self.idx].ln;
        let cn = self.tokens[self.idx].cn;
        self.idx += 1;
        Some(ExprView { expr, ln, cn })
    }

    fn parse_dot(&mut self) -> Option<()> {
        match self.tokens.get(self.idx)?.token {
            Token::Dot => {
                self.idx += 1;
                Some(())
            }
            _ => None,
        }
    }

    fn is_end(&self) -> bool {
        self.idx >= self.tokens.len()
    }
}
