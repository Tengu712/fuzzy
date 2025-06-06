use crate::RResult;
use std::{fmt::{Display, Result}, collections::HashMap};

#[derive(Debug, Clone, PartialEq)]
pub struct UserDefinedType {
    pub name: String,
    pub fields: Vec<(String, TypeId, bool)>,
}

pub type UserDefinedTypeMap = HashMap<String, UserDefinedType>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeId {
    Any,
    Bool,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    I128,
    U128,
    F32,
    F64,
    String,
    Symbol,
    Array,
    Lazy,
    Function(Vec<TypeId>),
    UserDefined(String),
}

impl Display for TypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        match self {
            Self::Any => write!(f, "_"),
            Self::Bool => write!(f, "bool"),
            Self::I8 => write!(f, "i8"),
            Self::U8 => write!(f, "u8"),
            Self::I16 => write!(f, "i16"),
            Self::U16 => write!(f, "u16"),
            Self::I32 => write!(f, "i32"),
            Self::U32 => write!(f, "u32"),
            Self::I64 => write!(f, "i64"),
            Self::U64 => write!(f, "u64"),
            Self::I128 => write!(f, "i128"),
            Self::U128 => write!(f, "u128"),
            Self::F32 => write!(f, "f32"),
            Self::F64 => write!(f, "f64"),
            Self::String => write!(f, "string"),
            Self::Symbol => write!(f, "symbol"),
            Self::Array => write!(f, "[]"),
            Self::Lazy => write!(f, "{{}}"),
            Self::Function(n) => {
                let mut s = "@[".to_string();
                for (i, t) in n.iter().enumerate() {
                    s.push_str(&t.to_string());
                    if i < n.len() - 1 {
                        s.push(' ');
                    }
                }
                s.push(']');
                write!(f, "{s}")
            }
            Self::UserDefined(n) => write!(f, "{n}"),
        }
    }
}

impl TypeId {
    pub fn from(s: &str) -> RResult<Self> {
        match s {
            "_" => Ok(Self::Any),
            "bool" => Ok(Self::Bool),
            "i8" => Ok(Self::I8),
            "u8" => Ok(Self::U8),
            "i16" => Ok(Self::I16),
            "u16" => Ok(Self::U16),
            "i32" => Ok(Self::I32),
            "u32" => Ok(Self::U32),
            "i64" => Ok(Self::I64),
            "u64" => Ok(Self::U64),
            "i128" => Ok(Self::I128),
            "u128" => Ok(Self::U128),
            "f32" => Ok(Self::F32),
            "f64" => Ok(Self::F64),
            "string" => Ok(Self::String),
            "symbol" => Ok(Self::Symbol),
            "[]" => Ok(Self::Array),
            "{}" => Ok(Self::Lazy),
            _ => Err(format!("error: typename '{s}' not defined.").into()),
        }
    }

    pub fn from_with_user_types(s: &str, user_types: &UserDefinedTypeMap) -> RResult<Self> {
        match Self::from(s) {
            Ok(t) => Ok(t),
            Err(_) => {
                if user_types.contains_key(s) {
                    Ok(Self::UserDefined(s.to_string()))
                } else {
                    Err(format!("error: typename '{s}' not defined.").into())
                }
            }
        }
    }
}

pub const ALL_PREMITIVE_TYPES: &[TypeId] = &[
    TypeId::Bool,
    TypeId::I8,
    TypeId::U8,
    TypeId::I16,
    TypeId::U16,
    TypeId::I32,
    TypeId::U32,
    TypeId::I64,
    TypeId::U64,
    TypeId::I128,
    TypeId::U128,
    TypeId::F32,
    TypeId::F64,
    TypeId::String,
    TypeId::Symbol,
    TypeId::Array,
    TypeId::Lazy,
];
