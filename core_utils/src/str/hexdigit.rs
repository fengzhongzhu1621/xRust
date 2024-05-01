/// Convert the given codepoint to its corresponding hexadecimal digit.
///
/// # Panics
///
/// This panics if `ch` is not in `[0-9A-Fa-f]`.
pub fn char_to_hexdigit(ch: char) -> u8 {
    u8::try_from(ch.to_digit(16).unwrap()).unwrap()
}

/// Convert the given hexadecimal digit to its corresponding codepoint.
///
/// # Panics
///
/// This panics when `digit > 15`.
pub fn hexdigit_to_char(digit: u8) -> char {
    char::from_digit(u32::from(digit), 16).unwrap().to_ascii_uppercase()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_to_hexdigit() {
        assert_eq!(char_to_hexdigit('A'), 10);
        assert_eq!(char_to_hexdigit('a'), 10);
        assert_eq!(char_to_hexdigit('9'), 9);
    }

    #[test]
    fn test_hexdigit_to_char() {
        assert_eq!(hexdigit_to_char(10), 'A');
        assert_eq!(hexdigit_to_char(9), '9');
    }
}
