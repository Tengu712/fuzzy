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
    ($maps: expr, $fn: ident, $op: tt, $ty: ident, $_: ident) => {
        $maps
            .get_mut(stringify!($ty))
            .unwrap_or_else(|| panic!("function map for '{}' not found.", stringify!($ty)))
            .insert(
                stringify!($op).to_string(),
                Function::Builtin(paste::item! {[<$fn $ty>]}),
            );
    };
}

pub fn insert_numeric_functions(maps: &mut FunctionMap) {
    for_all_numeric_types!(insert_numeric_function, maps, add, +);
    for_all_numeric_types!(insert_numeric_function, maps, sub, -);
    for_all_numeric_types!(insert_numeric_function, maps, mul, *);
    for_all_numeric_types!(insert_numeric_function, maps, div, /);
}

macro_rules! define_numeric_function {
    ($fn: ident, $op: tt, $ty: ident, $variant: ident) => {
        paste::item! {
        fn [<$fn $ty>](_: &mut Environment, s: Value, args: &mut Vec<Value>) -> RResult<()> {
            let Value::$variant(s) = s else {
                panic!("subject type missmatch.");
            };
            let Some(Value::$variant(o)) = args.pop() else {
                return Err(format!(
                    "error: no argument passed to '{}:{}'.",
                    stringify!($ty),
                    stringify!($op),
                )
                .into());
            };
            args.push(Value::$variant(s $op o));
            Ok(())
        }
        }
    };
}
for_all_numeric_types!(define_numeric_function, add, +);
for_all_numeric_types!(define_numeric_function, sub, -);
for_all_numeric_types!(define_numeric_function, mul, *);
for_all_numeric_types!(define_numeric_function, div, /);
