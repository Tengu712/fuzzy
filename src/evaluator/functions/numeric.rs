use super::*;

macro_rules! for_all_numeric_types {
    ($macro: ident $(, $($arg: tt)*)?) => {
        $macro!($($($arg)*, )? i8, I8);
        $macro!($($($arg)*, )? u8, U8);
        $macro!($($($arg)*, )? i16, I16);
        $macro!($($($arg)*, )? u16, U16);
        $macro!($($($arg)*, )? i32, I32);
        $macro!($($($arg)*, )? u32, U32);
        $macro!($($($arg)*, )? i64, I64);
        $macro!($($($arg)*, )? u64, U64);
        $macro!($($($arg)*, )? i128, I128);
        $macro!($($($arg)*, )? u128, U128);
        $macro!($($($arg)*, )? f32, F32);
        $macro!($($($arg)*, )? f64, F64);
    };
}

macro_rules! insert_numeric_function {
    ($fm: expr, $fn: ident, $op: tt, $ty: ident, $_: ident) => {
        let ty = TypeId::from(stringify!($ty))
            .unwrap_or_else(|_| panic!("failed to get typeid from str '{}'.", stringify!($ty)));
        $fm.insert_all(
            &ty,
            vec![(
                stringify!($op).to_string(),
                (
                    vec![ty.clone()],
                    FunctionCode::Builtin(paste::item! {[<$fn $ty>]}),
                ),
            )],
        );
    };
}

macro_rules! insert_cast {
    ($fm: expr, $ty: ident, $_: ident) => {
        let ty = TypeId::from(stringify!($ty))
            .unwrap_or_else(|_| panic!("failed to get typeid from str '{}'.", stringify!($ty)));
        $fm.insert_all(
            &ty,
            vec![(
                ":".to_string(),
                (
                    vec![TypeId::Symbol],
                    FunctionCode::Builtin(paste::item! {[<cast $ty>]}),
                ),
            )],
        );
    };
}

pub fn insert(maps: &mut FunctionMap) {
    for_all_numeric_types!(insert_numeric_function, maps, add, +);
    for_all_numeric_types!(insert_numeric_function, maps, sub, -);
    for_all_numeric_types!(insert_numeric_function, maps, mul, *);
    for_all_numeric_types!(insert_numeric_function, maps, div, /);
    for_all_numeric_types!(insert_cast, maps);
}

macro_rules! define_numeric_function {
    ($fn: ident, $op: tt, $ty: ident, $variant: ident) => {
        paste::item! {
            fn [<$fn $ty>](_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
                let (Value::$variant(s), Some(Value::$variant(o))) = (s, args.pop()) else {
                    panic!("type missmatched on '{}:{}'", stringify!($ty), stringify!($op));
                };
                Ok(Value::$variant(s $op o))
            }
        }
    };
}
for_all_numeric_types!(define_numeric_function, add, +);
for_all_numeric_types!(define_numeric_function, sub, -);
for_all_numeric_types!(define_numeric_function, mul, *);
for_all_numeric_types!(define_numeric_function, div, /);

macro_rules! define_cast {
    ($ty: ident, $variant: ident) => {
        paste::item! {
            fn [<cast $ty>](_: &mut Environment, s: Value, mut args: Vec<Value>) -> RResult<Value> {
                let (Value::$variant(s), Some(Value::Symbol(o))) = (s, args.pop()) else {
                    panic!("type missmatched on '{}::'", stringify!($ty));
                };
                let n = match o.as_str() {
                    "i8" => Value::I8(s as i8),
                    "u8" => Value::U8(s as u8),
                    "i16" => Value::I16(s as i16),
                    "u16" => Value::U16(s as u16),
                    "i32" => Value::I32(s as i32),
                    "u32" => Value::U32(s as u32),
                    "i64" => Value::I64(s as i64),
                    "u64" => Value::U64(s as u64),
                    "i128" => Value::I128(s as i128),
                    "u128" => Value::U128(s as u128),
                    "f32" => Value::F32(s as f32),
                    "f64" => Value::F64(s as f64),
                    n => return Err(format!("error: {} cannot cast to {n}.", stringify!($ty)).into()),
                };
                Ok(n)
            }
        }
    };
}
for_all_numeric_types!(define_cast);
