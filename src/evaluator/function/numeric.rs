use super::*;

macro_rules! insert_numeric_function {
    ($maps: expr, $rsfn: ident, $fn: expr, $rsty: ty) => {
        $maps
            .get_mut(stringify!($rsty))
            .unwrap()
            .insert($fn.to_string(), Function::Builtin($rsfn::<$rsty>));
    };
}

macro_rules! define_numeric_function {
    ($trait: ident, $fn: ident, $name: expr) => {
        fn $fn<T>(_: &mut Environment, s: Value, args: &mut Vec<Value>) -> RResult<()>
        where
            T: FromValue + IntoValue + $trait<Output = T>,
        {
            let Some(f) = T::from_value(&s) else {
                panic!("subject type missmatch.");
            };
            let Some(o) = args.pop() else {
                return Err(format!(
                    "error: no argument passed to '{}:{}'.",
                    stringify!($fn),
                    $name
                )
                .into());
            };
            let Some(l) = T::from_value(&o) else {
                return Err(format!(
                    "error: type missmatched argument passed to '{}:{}'.",
                    stringify!($fn),
                    $name
                )
                .into());
            };
            args.push($trait::$fn(f, l).into_value());
            Ok(())
        }
    };
}
define_numeric_function!(Add, add, "+");
define_numeric_function!(Sub, sub, "-");
define_numeric_function!(Mul, mul, "*");
define_numeric_function!(Div, div, "/");

macro_rules! for_all_numeric_types {
    ($macro: ident $(, $($arg: tt)*)?) => {
        $macro!($($($arg)*, )? i8);
        $macro!($($($arg)*, )? u8);
        $macro!($($($arg)*, )? i16);
        $macro!($($($arg)*, )? u16);
        $macro!($($($arg)*, )? i32);
        $macro!($($($arg)*, )? u32);
        $macro!($($($arg)*, )? i64);
        $macro!($($($arg)*, )? u64);
        $macro!($($($arg)*, )? i128);
        $macro!($($($arg)*, )? u128);
        $macro!($($($arg)*, )? f32);
        $macro!($($($arg)*, )? f64);
    };
}
macro_rules! for_all_numeric_types_and_variants {
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

trait FromValue: Sized {
    fn from_value(value: &Value) -> Option<Self>;
}
macro_rules! implement_fromvalue {
    ($rsty: ty, $var: ident) => {
        impl FromValue for $rsty {
            fn from_value(value: &Value) -> Option<Self> {
                match value {
                    Value::$var(n) => Some(*n),
                    _ => None,
                }
            }
        }
    };
}
for_all_numeric_types_and_variants!(implement_fromvalue);

trait IntoValue: Sized {
    fn into_value(self) -> Value;
}
macro_rules! implement_intovalue {
    ($rsty: ty, $var: ident) => {
        impl IntoValue for $rsty {
            fn into_value(self) -> Value {
                Value::$var(self)
            }
        }
    };
}
for_all_numeric_types_and_variants!(implement_intovalue);

pub fn insert_numeric_functions(maps: &mut FunctionMap) {
    for_all_numeric_types!(insert_numeric_function, maps, add, "+");
    for_all_numeric_types!(insert_numeric_function, maps, sub, "-");
    for_all_numeric_types!(insert_numeric_function, maps, mul, "*");
    for_all_numeric_types!(insert_numeric_function, maps, div, "/");
}
