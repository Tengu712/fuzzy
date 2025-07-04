use super::{Environment, types::TypeId};
use crate::{RResult, lexer::Token};
use std::{
    collections::{HashMap, VecDeque},
    fmt::{Display, Result},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub private: bool,
    pub value: Value,
}

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
    Lazy(VecDeque<Token>),
    Function((TypeId, Vec<Token>)),
    UserType((TypeId, HashMap<String, Object>)),
}

impl Default for Value {
    fn default() -> Self {
        Self::Nil
    }
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
            Self::UserType((_, n)) => {
                let mut s = "[".to_string();
                let mut keys = n.keys().collect::<Vec<_>>();
                keys.sort();
                for (i, k) in keys.into_iter().enumerate() {
                    let v = n.get(k).unwrap();
                    let p = if v.private { "::" } else { ":" };
                    s.push_str(&format!("{p}{k} {}", v.value));
                    if i < n.len() - 1 {
                        s.push(' ');
                    }
                }
                s.push(']');
                write!(f, "{s}")
            }
        }
    }
}

impl Value {
    pub fn from(env: &Environment, token: Token) -> RResult<Self> {
        match token {
            Token::Top => Ok(Self::Top),
            Token::I8(n) => Ok(Self::I8(n)),
            Token::U8(n) => Ok(Self::U8(n)),
            Token::I16(n) => Ok(Self::I16(n)),
            Token::U16(n) => Ok(Self::U16(n)),
            Token::I32(n) => Ok(Self::I32(n)),
            Token::U32(n) => Ok(Self::U32(n)),
            Token::I64(n) => Ok(Self::I64(n)),
            Token::U64(n) => Ok(Self::U64(n)),
            Token::I128(n) => Ok(Self::I128(n)),
            Token::U128(n) => Ok(Self::U128(n)),
            Token::F32(n) => Ok(Self::F32(n)),
            Token::F64(n) => Ok(Self::F64(n)),
            Token::String(n) => Ok(Self::String(n)),
            Token::Symbol(n) => Ok(Self::Symbol(n)),
            Token::Label(n) => env.vr_map.get_unwrap(env.get_self_type(), &n),
            _ => panic!("tried to create value from non-atom token."),
        }
    }

    pub fn typeid(&self) -> TypeId {
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
            Self::UserType((n, _)) => n.clone(),
        }
    }

    pub fn format_in_detail(&self, env: &Environment) -> String {
        match self {
            Self::Nil => self.to_string(),
            Self::Top => self.to_string(),
            Self::Array(_) => self.to_string(),
            Self::Lazy(_) => self.to_string(),
            Self::Symbol(n) if env.vr_map.get(n).is_some() => {
                let v = env.vr_map.get(n).unwrap();
                let s = v.format_in_detail(env);
                let a = if env.vr_map.is_mutable(n).unwrap() {
                    "<-"
                } else {
                    "<="
                };
                format!("{n} {a} {s}")
            }
            _ => format!("{self} ({})", self.typeid()),
        }
    }

    pub fn equal(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nil, Self::Nil) => true,
            (Self::Top, Self::Top) => true,
            (Self::Nil, Self::Top) => false,
            (Self::Top, Self::Nil) => false,
            (Self::I8(a), Self::I8(b)) => a == b,
            (Self::U8(a), Self::U8(b)) => a == b,
            (Self::I16(a), Self::I16(b)) => a == b,
            (Self::U16(a), Self::U16(b)) => a == b,
            (Self::I32(a), Self::I32(b)) => a == b,
            (Self::U32(a), Self::U32(b)) => a == b,
            (Self::I64(a), Self::I64(b)) => a == b,
            (Self::U64(a), Self::U64(b)) => a == b,
            (Self::I128(a), Self::I128(b)) => a == b,
            (Self::U128(a), Self::U128(b)) => a == b,
            (Self::F32(a), Self::F32(b)) => a == b,
            (Self::F64(a), Self::F64(b)) => a == b,
            (Self::String(a), Self::String(b)) => a == b,
            (Self::Symbol(a), Self::Symbol(b)) => a == b,
            (Self::Array(a), Self::Array(b)) => {
                a.len() == b.len()
                    && a.iter()
                        .zip(b.iter())
                        .all(|(x, y)| x.typeid() == y.typeid() && x.equal(y))
            }
            (Self::Lazy(a), Self::Lazy(b)) => a == b,
            (Self::UserType((at, av)), Self::UserType((bt, bv))) => at == bt && av == bv,
            _ => panic!("tried to compare {} and {}", self.typeid(), other.typeid(),),
        }
    }

    pub fn l(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nil, Self::Nil) => false,
            (Self::Top, Self::Top) => false,
            (Self::Nil, Self::Top) => true,
            (Self::Top, Self::Nil) => false,
            (Self::I8(a), Self::I8(b)) => a < b,
            (Self::U8(a), Self::U8(b)) => a < b,
            (Self::I16(a), Self::I16(b)) => a < b,
            (Self::U16(a), Self::U16(b)) => a < b,
            (Self::I32(a), Self::I32(b)) => a < b,
            (Self::U32(a), Self::U32(b)) => a < b,
            (Self::I64(a), Self::I64(b)) => a < b,
            (Self::U64(a), Self::U64(b)) => a < b,
            (Self::I128(a), Self::I128(b)) => a < b,
            (Self::U128(a), Self::U128(b)) => a < b,
            (Self::F32(a), Self::F32(b)) => a < b,
            (Self::F64(a), Self::F64(b)) => a < b,
            (Self::String(a), Self::String(b)) => a < b,
            (Self::Symbol(a), Self::Symbol(b)) => a < b,
            _ => panic!("tried to compare {} and {}", self.typeid(), other.typeid(),),
        }
    }

    pub fn g(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nil, Self::Nil) => false,
            (Self::Top, Self::Top) => false,
            (Self::Nil, Self::Top) => false,
            (Self::Top, Self::Nil) => true,
            (Self::I8(a), Self::I8(b)) => a > b,
            (Self::U8(a), Self::U8(b)) => a > b,
            (Self::I16(a), Self::I16(b)) => a > b,
            (Self::U16(a), Self::U16(b)) => a > b,
            (Self::I32(a), Self::I32(b)) => a > b,
            (Self::U32(a), Self::U32(b)) => a > b,
            (Self::I64(a), Self::I64(b)) => a > b,
            (Self::U64(a), Self::U64(b)) => a > b,
            (Self::I128(a), Self::I128(b)) => a > b,
            (Self::U128(a), Self::U128(b)) => a > b,
            (Self::F32(a), Self::F32(b)) => a > b,
            (Self::F64(a), Self::F64(b)) => a > b,
            (Self::String(a), Self::String(b)) => a > b,
            (Self::Symbol(a), Self::Symbol(b)) => a > b,
            _ => panic!("tried to compare {} and {}", self.typeid(), other.typeid(),),
        }
    }
}
