mod numeric;
mod print;
mod variable;

use super::*;
use std::ops::*;

const ALL_TYPES: &[&str] = &[
    "nil", "i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64", "i128", "u128", "f32", "f64",
    "string", "symbol",
];

pub fn setup() -> FunctionMap {
    let mut maps = HashMap::new();
    for n in ALL_TYPES {
        maps.insert(n.to_string(), HashMap::new());
    }

    for n in ALL_TYPES {
        print::insert_print(&mut maps, n);
        variable::insert_variable_definition(&mut maps, n);
    }

    numeric::insert_numeric_functions(&mut maps);

    maps
}
