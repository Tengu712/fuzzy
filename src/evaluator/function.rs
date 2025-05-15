use super::*;

pub fn setup() -> HashMap<String, HashMap<String, Function>> {
    let mut i8_map = HashMap::new();
    let mut u8_map = HashMap::new();
    let mut i16_map = HashMap::new();
    let mut u16_map = HashMap::new();
    let mut i32_map = HashMap::new();
    let mut u32_map = HashMap::new();
    let mut i64_map = HashMap::new();
    let mut u64_map = HashMap::new();
    let mut i128_map = HashMap::new();
    let mut u128_map = HashMap::new();
    let mut f32_map = HashMap::new();
    let mut f64_map = HashMap::new();

    i8_map.insert("+".to_string(), Function::Builtin(i8_add));
    u8_map.insert("+".to_string(), Function::Builtin(u8_add));
    i16_map.insert("+".to_string(), Function::Builtin(i16_add));
    u16_map.insert("+".to_string(), Function::Builtin(u16_add));
    i32_map.insert("+".to_string(), Function::Builtin(i32_add));
    u32_map.insert("+".to_string(), Function::Builtin(u32_add));
    i64_map.insert("+".to_string(), Function::Builtin(i64_add));
    u64_map.insert("+".to_string(), Function::Builtin(u64_add));
    i128_map.insert("+".to_string(), Function::Builtin(i128_add));
    u128_map.insert("+".to_string(), Function::Builtin(u128_add));
    f32_map.insert("+".to_string(), Function::Builtin(f32_add));
    f64_map.insert("+".to_string(), Function::Builtin(f64_add));

    i8_map.insert("-".to_string(), Function::Builtin(i8_sub));
    u8_map.insert("-".to_string(), Function::Builtin(u8_sub));
    i16_map.insert("-".to_string(), Function::Builtin(i16_sub));
    u16_map.insert("-".to_string(), Function::Builtin(u16_sub));
    i32_map.insert("-".to_string(), Function::Builtin(i32_sub));
    u32_map.insert("-".to_string(), Function::Builtin(u32_sub));
    i64_map.insert("-".to_string(), Function::Builtin(i64_sub));
    u64_map.insert("-".to_string(), Function::Builtin(u64_sub));
    i128_map.insert("-".to_string(), Function::Builtin(i128_sub));
    u128_map.insert("-".to_string(), Function::Builtin(u128_sub));
    f32_map.insert("-".to_string(), Function::Builtin(f32_sub));
    f64_map.insert("-".to_string(), Function::Builtin(f64_sub));

    i8_map.insert("*".to_string(), Function::Builtin(i8_mul));
    u8_map.insert("*".to_string(), Function::Builtin(u8_mul));
    i16_map.insert("*".to_string(), Function::Builtin(i16_mul));
    u16_map.insert("*".to_string(), Function::Builtin(u16_mul));
    i32_map.insert("*".to_string(), Function::Builtin(i32_mul));
    u32_map.insert("*".to_string(), Function::Builtin(u32_mul));
    i64_map.insert("*".to_string(), Function::Builtin(i64_mul));
    u64_map.insert("*".to_string(), Function::Builtin(u64_mul));
    i128_map.insert("*".to_string(), Function::Builtin(i128_mul));
    u128_map.insert("*".to_string(), Function::Builtin(u128_mul));
    f32_map.insert("*".to_string(), Function::Builtin(f32_mul));
    f64_map.insert("*".to_string(), Function::Builtin(f64_mul));

    i8_map.insert("/".to_string(), Function::Builtin(i8_div));
    u8_map.insert("/".to_string(), Function::Builtin(u8_div));
    i16_map.insert("/".to_string(), Function::Builtin(i16_div));
    u16_map.insert("/".to_string(), Function::Builtin(u16_div));
    i32_map.insert("/".to_string(), Function::Builtin(i32_div));
    u32_map.insert("/".to_string(), Function::Builtin(u32_div));
    i64_map.insert("/".to_string(), Function::Builtin(i64_div));
    u64_map.insert("/".to_string(), Function::Builtin(u64_div));
    i128_map.insert("/".to_string(), Function::Builtin(i128_div));
    u128_map.insert("/".to_string(), Function::Builtin(u128_div));
    f32_map.insert("/".to_string(), Function::Builtin(f32_div));
    f64_map.insert("/".to_string(), Function::Builtin(f64_div));

    HashMap::from([
        ("i8".to_string(), i8_map),
        ("u8".to_string(), u8_map),
        ("i16".to_string(), i16_map),
        ("u16".to_string(), u16_map),
        ("i32".to_string(), i32_map),
        ("u32".to_string(), u32_map),
        ("i64".to_string(), i64_map),
        ("u64".to_string(), u64_map),
        ("i128".to_string(), i128_map),
        ("u128".to_string(), u128_map),
        ("f32".to_string(), f32_map),
        ("f64".to_string(), f64_map),
    ])
}

fn pop_argument(name: &'static str, values: &mut Vec<Value>) -> RResult<Value> {
    values
        .pop()
        .ok_or(format!("error: no argument passed to '{name}'.").into())
}

macro_rules! define_numeric_function {
($fn_name:ident, $fn_display_name:expr, $op:tt, $variant:ident, $inner_type:ty) => {
    fn $fn_name(s: Value, values: &mut Vec<Value>) -> RResult<()> {
        const FN_NAME: &str = $fn_display_name;
        match (s, pop_argument(FN_NAME, values)?) {
            (Value::$variant(f), Value::$variant(l)) => {
                let f = f as $inner_type;
                let l = l as $inner_type;
                values.push(Value::$variant(f $op l));
            }
            (Value::$variant(_), o) => return Err(format!("error: '{o:?}' passed to '{FN_NAME}'.").into()),
            _ => panic!("unexpected error: subject type missmatch."),
        }
        Ok(())
    }
};
}

define_numeric_function!(i8_add, "i8:+", +, I8, i8);
define_numeric_function!(u8_add, "u8:+", +, U8, u8);
define_numeric_function!(i16_add, "i16:+", +, I16, i16);
define_numeric_function!(u16_add, "u16:+", +, U16, u16);
define_numeric_function!(i32_add, "i32:+", +, I32, i32);
define_numeric_function!(u32_add, "u32:+", +, U32, u32);
define_numeric_function!(i64_add, "i64:+", +, I64, i64);
define_numeric_function!(u64_add, "u64:+", +, U64, u64);
define_numeric_function!(i128_add, "i128:+", +, I128, i128);
define_numeric_function!(u128_add, "u128:+", +, U128, u128);
define_numeric_function!(f32_add, "f32:+", +, F32, f32);
define_numeric_function!(f64_add, "f64:+", +, F64, f64);

define_numeric_function!(i8_sub, "i8:-", -, I8, i8);
define_numeric_function!(u8_sub, "u8:-", -, U8, u8);
define_numeric_function!(i16_sub, "i16:-", -, I16, i16);
define_numeric_function!(u16_sub, "u16:-", -, U16, u16);
define_numeric_function!(i32_sub, "i32:-", -, I32, i32);
define_numeric_function!(u32_sub, "u32:-", -, U32, u32);
define_numeric_function!(i64_sub, "i64:-", -, I64, i64);
define_numeric_function!(u64_sub, "u64:-", -, U64, u64);
define_numeric_function!(i128_sub, "i128:-", -, I128, i128);
define_numeric_function!(u128_sub, "u128:-", -, U128, u128);
define_numeric_function!(f32_sub, "f32:-", -, F32, f32);
define_numeric_function!(f64_sub, "f64:-", -, F64, f64);

define_numeric_function!(i8_mul, "i8:*", *, I8, i8);
define_numeric_function!(u8_mul, "u8:*", *, U8, u8);
define_numeric_function!(i16_mul, "i16:*", *, I16, i16);
define_numeric_function!(u16_mul, "u16:*", *, U16, u16);
define_numeric_function!(i32_mul, "i32:*", *, I32, i32);
define_numeric_function!(u32_mul, "u32:*", *, U32, u32);
define_numeric_function!(i64_mul, "i64:*", *, I64, i64);
define_numeric_function!(u64_mul, "u64:*", *, U64, u64);
define_numeric_function!(i128_mul, "i128:*", *, I128, i128);
define_numeric_function!(u128_mul, "u128:*", *, U128, u128);
define_numeric_function!(f32_mul, "f32:*", *, F32, f32);
define_numeric_function!(f64_mul, "f64:*", *, F64, f64);

define_numeric_function!(i8_div, "i8:/", /, I8, i8);
define_numeric_function!(u8_div, "u8:/", /, U8, u8);
define_numeric_function!(i16_div, "i16:/", /, I16, i16);
define_numeric_function!(u16_div, "u16:/", /, U16, u16);
define_numeric_function!(i32_div, "i32:/", /, I32, i32);
define_numeric_function!(u32_div, "u32:/", /, U32, u32);
define_numeric_function!(i64_div, "i64:/", /, I64, i64);
define_numeric_function!(u64_div, "u64:/", /, U64, u64);
define_numeric_function!(i128_div, "i128:/", /, I128, i128);
define_numeric_function!(u128_div, "u128:/", /, U128, u128);
define_numeric_function!(f32_div, "f32:/", /, F32, f32);
define_numeric_function!(f64_div, "f64:/", /, F64, f64);
