use super::{Environment, types::TypeId};
use crate::lexer::Token;
use std::fmt::{Display, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
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
    Array(Vec<Value>),
    Lazy(Vec<Token>),
    Function((TypeId, Vec<Token>)),
    Label(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        match self {
            Self::Nil => write!(f, "()"),
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
            Self::String(n) => write!(f, "{n}"),
            Self::Symbol(n) => write!(f, "{n}"),
            Self::Array(n) => {
                let mut s = "[".to_string();
                for (i, m) in n.iter().enumerate() {
                    s.push_str(&m.to_string());
                    if i < n.len() - 1 {
                        s.push(' ');
                    }
                }
                s.push(']');
                write!(f, "{s}")
            }
            Self::Lazy(_) => write!(f, "{{}}"),
            Self::Function(_) => write!(f, "{{}}"),
            Self::Label(_) => panic!("tried to format label."),
        }
    }
}

impl Value {
    pub fn from(token: Token) -> Self {
        match token {
            Token::Top => Self::Top,
            Token::I8(n) => Self::I8(n),
            Token::U8(n) => Self::U8(n),
            Token::I16(n) => Self::I16(n),
            Token::U16(n) => Self::U16(n),
            Token::I32(n) => Self::I32(n),
            Token::U32(n) => Self::U32(n),
            Token::I64(n) => Self::I64(n),
            Token::U64(n) => Self::U64(n),
            Token::I128(n) => Self::I128(n),
            Token::U128(n) => Self::U128(n),
            Token::F32(n) => Self::F32(n),
            Token::F64(n) => Self::F64(n),
            Token::String(n) => Self::String(n),
            Token::Symbol(n) => Self::Symbol(n),
            Token::Label(n) => Self::Label(n),
            _ => panic!("tried to create value from non-atom token."),
        }
    }

    pub fn get_typeid(&self) -> TypeId {
        match self {
            Self::Nil => TypeId::Bool,
            Self::Top => TypeId::Bool,
            Self::I8(_) => TypeId::I8,
            Self::U8(_) => TypeId::U8,
            Self::I16(_) => TypeId::I16,
            Self::U16(_) => TypeId::U16,
            Self::I32(_) => TypeId::I32,
            Self::U32(_) => TypeId::U32,
            Self::I64(_) => TypeId::I64,
            Self::U64(_) => TypeId::U64,
            Self::I128(_) => TypeId::I128,
            Self::U128(_) => TypeId::U128,
            Self::F32(_) => TypeId::F32,
            Self::F64(_) => TypeId::F64,
            Self::String(_) => TypeId::String,
            Self::Symbol(_) => TypeId::Symbol,
            Self::Array(_) => TypeId::Array,
            Self::Lazy(_) => TypeId::Lazy,
            Self::Function((n, _)) => n.clone(),
            Self::Label(_) => panic!("tried to get type of label."),
        }
    }

    pub fn format_in_detail(&self, env: &Environment) -> String {
        match self {
            Self::Nil => self.to_string(),
            Self::Top => self.to_string(),
            Self::Array(_) => self.to_string(),
            Self::Lazy(_) => self.to_string(),
            Self::Symbol(n) if env.get_variable(n).is_some() => {
                let v = env.get_variable(n).unwrap();
                let s = v.value.format_in_detail(env);
                let a = if v.mutable { "<-" } else { "<=" };
                format!("{n} {a} {s}")
            }
            Self::Label(_) => panic!("tried to format label."),
            _ => format!("{self} ({})", self.get_typeid()),
        }
    }
}
