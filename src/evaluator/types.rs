use crate::RResult;

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
    Args(Vec<TypeId>),
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

    pub fn to_string(&self) -> String {
        match self {
            Self::Any => "_".to_string(),
            Self::Bool => "bool".to_string(),
            Self::I8 => "i8".to_string(),
            Self::U8 => "u8".to_string(),
            Self::I16 => "i16".to_string(),
            Self::U16 => "u16".to_string(),
            Self::I32 => "i32".to_string(),
            Self::U32 => "u32".to_string(),
            Self::I64 => "i64".to_string(),
            Self::U64 => "u64".to_string(),
            Self::I128 => "i128".to_string(),
            Self::U128 => "u128".to_string(),
            Self::F32 => "f32".to_string(),
            Self::F64 => "f64".to_string(),
            Self::String => "string".to_string(),
            Self::Symbol => "symbol".to_string(),
            Self::Array => "[]".to_string(),
            Self::Lazy => "{}".to_string(),
            Self::Args(n) => {
                let mut s = "[".to_string();
                for (i, t) in n.iter().enumerate() {
                    s.push_str(&t.to_string());
                    if i < n.len() - 1 {
                        s.push(' ');
                    }
                }
                s.push(']');
                s
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
