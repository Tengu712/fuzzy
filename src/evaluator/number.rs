use super::*;
use regex::Regex;

pub fn parse(s: &str) -> Option<Value> {
    let caps = Regex::new(r"(.*?)(i8|u8|i16|u16|i32|u32|i64|u64|i128|u128|f32|f64)?$")
        .ok()?
        .captures(s)?;
    let f = caps.get(1)?.as_str();
    let l = caps.get(2).map(|n| n.as_str());
    match (f, l) {
        (n, None) => n.parse::<i32>().ok().map(Value::I32),
        (n, Some("i8")) => n.parse::<i8>().ok().map(Value::I8),
        (n, Some("u8")) => n.parse::<u8>().ok().map(Value::U8),
        (n, Some("i16")) => n.parse::<i16>().ok().map(Value::I16),
        (n, Some("u16")) => n.parse::<u16>().ok().map(Value::U16),
        (n, Some("i32")) => n.parse::<i32>().ok().map(Value::I32),
        (n, Some("u32")) => n.parse::<u32>().ok().map(Value::U32),
        (n, Some("i64")) => n.parse::<i64>().ok().map(Value::I64),
        (n, Some("u64")) => n.parse::<u64>().ok().map(Value::U64),
        (n, Some("i128")) => n.parse::<i128>().ok().map(Value::I128),
        (n, Some("u128")) => n.parse::<u128>().ok().map(Value::U128),
        (n, Some("f32")) => n.parse::<f32>().ok().map(Value::F32),
        (n, Some("f64")) => n.parse::<f64>().ok().map(Value::F64),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use fake::{Fake, Faker};

    #[test]
    fn test_parse_u32_not_number() {
        assert_eq!(parse("u32"), None);
    }

    #[test]
    fn test_parse_foof64_not_number() {
        assert_eq!(parse("foof64"), None);
    }

    #[test]
    fn test_parse_2147483648_not_number() {
        assert_eq!(parse("2147483648"), None);
    }

    #[test]
    fn test_parse_2147483648u16_not_number() {
        assert_eq!(parse("2147483648u16"), None);
    }

    #[test]
    fn test_parse_2147483648u32_to_u32() {
        assert_eq!(parse("2147483648u32"), Some(Value::U32(2147483648)));
    }

    #[test]
    fn test_parse_1f32_to_f32() {
        assert_eq!(parse("1f32"), Some(Value::F32(1.0)));
    }

    #[test]
    fn test_parse_1_e_minus_2_f32_to_f32() {
        assert_eq!(parse("1e-2f32"), Some(Value::F32(0.01)));
    }

    #[test]
    fn test_parse_no_suffix_i32() {
        let n = Faker.fake::<i32>();
        assert_eq!(parse(&format!("{n}")), Some(Value::I32(n)));
    }

    #[test]
    fn test_parse_u64() {
        let n = Faker.fake::<u64>();
        assert_eq!(parse("{n}u64"), Some(Value::U64(n)));
    }

    #[test]
    fn test_parse_f32() {
        let n = Faker.fake::<f32>();
        assert_eq!(parse(&format!("{n}f32")), Some(Value::F32(n)));
    }
}
