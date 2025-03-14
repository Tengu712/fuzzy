use super::*;

pub fn lex(s: &str) -> Option<Token> {
    let splitted = s
        .char_indices()
        .rev()
        .find(|(_, c)| !c.is_ascii_digit() && *c != '-')
        .map(|(i, _)| (&s[..i], &s[i..]));
    match splitted {
        None => s.parse::<i32>().ok().map(Token::I32),
        Some((body, "i8")) => body.parse::<i8>().ok().map(Token::I8),
        Some((body, "u8")) => body.parse::<u8>().ok().map(Token::U8),
        Some((body, "i16")) => body.parse::<i16>().ok().map(Token::I16),
        Some((body, "u16")) => body.parse::<u16>().ok().map(Token::U16),
        Some((body, "i32")) => body.parse::<i32>().ok().map(Token::I32),
        Some((body, "u32")) => body.parse::<u32>().ok().map(Token::U32),
        Some((body, "i64")) => body.parse::<i64>().ok().map(Token::I64),
        Some((body, "u64")) => body.parse::<u64>().ok().map(Token::U64),
        Some((body, "i128")) => body.parse::<i128>().ok().map(Token::I128),
        Some((body, "u128")) => body.parse::<u128>().ok().map(Token::U128),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lex_0_to_i32() {
        assert_eq!(lex("0"), Some(Token::I32(0)));
    }

    #[test]
    fn test_lex_minus1_to_i32() {
        assert_eq!(lex("-1"), Some(Token::I32(-1)));
    }

    #[test]
    fn test_lex_2147483648_not_number() {
        assert_eq!(lex("2147483648"), None);
    }

    #[test]
    fn test_lex_2147483648u32_to_u32() {
        assert_eq!(lex("2147483648u32"), Some(Token::U32(2147483648)));
    }

    #[test]
    fn test_lex_0u16_to_u16() {
        assert_eq!(lex("0u16"), Some(Token::U16(0)));
    }

    #[test]
    fn test_lex_foo_not_number() {
        assert_eq!(lex("foo"), None);
    }

    #[test]
    fn test_lex_minusminus1_not_number() {
        assert_eq!(lex("--1"), None);
    }
}
