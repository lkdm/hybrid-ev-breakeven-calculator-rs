use leptos::logging::log;
use std::iter::FromIterator;

pub fn parse_decimal(input: String) -> String {
    let mut iter = input.chars().peekable();
    let mut valid_chars = Vec::new();
    let mut has_decimal: bool = false;

    // Iterate over the remaining characters
    while let Some(c) = iter.next() {
        match c {
            '.' => {
                if !has_decimal {
                    valid_chars.push(c);
                    has_decimal = true;
                }
                if valid_chars.len() == 1 {
                    // Add a leading 0 if we just added a .
                    valid_chars.insert(0, '0');
                }
            }
            '0'..='9' => {
                // Out of bounds, length is 0 but index is 0.
                if valid_chars.len() == 2 && valid_chars[0] == '-' && valid_chars[1] == '0' {
                    // Remove the leading 0 after the sign '-', if we've inserted a number after it.
                    valid_chars.pop();
                } else if valid_chars.len() == 1 && valid_chars[0] == '0' {
                    // Remove the leading 0, if we've inserted a number after it.
                    valid_chars.pop();
                }
                valid_chars.push(c);
            }
            '-' => {
                if valid_chars.is_empty() {
                    valid_chars.push(c);
                    if iter.peek() == Some(&'.') {
                        // Add a 0 between the - and .
                        valid_chars.insert(1, '0')
                    }
                }
            }
            _ => (),
        }
    }

    log!("{}", String::from_iter(&valid_chars));
    String::from_iter(valid_chars)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_decimal_one_zero() {
        assert_eq!(parse_decimal(String::from("0")), "0");
    }

    #[test]
    fn test_parse_decimal_positive_integer() {
        assert_eq!(parse_decimal(String::from("123")), "123");
    }

    #[test]
    fn test_parse_decimal_positive_float() {
        assert_eq!(parse_decimal(String::from("123.456")), "123.46");
    }

    #[test]
    fn test_parse_decimal_negative_integer() {
        assert_eq!(parse_decimal(String::from("-123")), "-123");
    }

    #[test]
    fn test_parse_decimal_negative_float() {
        assert_eq!(parse_decimal(String::from("-123.456")), "-123.46");
    }

    #[test]
    fn test_parse_decimal_starting_with_dot() {
        assert_eq!(parse_decimal(String::from(".123")), "0.12");
    }

    #[test]
    fn test_parse_decimal_starting_with_minus_dot() {
        assert_eq!(parse_decimal(String::from("-.123")), "-0.12");
    }

    #[test]
    fn test_parse_decimal_leading_zero_float() {
        assert_eq!(parse_decimal(String::from("0.123")), "0.12");
    }

    #[test]
    fn test_parse_decimal_leading_zero_integer() {
        assert_eq!(parse_decimal(String::from("0123")), "123");
    }

    #[test]
    fn test_parse_decimal_leading_zero_float_single_digit() {
        assert_eq!(parse_decimal(String::from("0.1")), "0.1");
    }

    #[test]
    fn test_parse_decimal_leading_zero_integer_single_digit() {
        assert_eq!(parse_decimal(String::from("01")), "1");
    }

    #[test]
    fn test_parse_decimal_leading_zero_float_multiple_zeros() {
        assert_eq!(parse_decimal(String::from("000.123")), "0.12");
    }

    #[test]
    fn test_parse_decimal_leading_zero_integer_multiple_zeros() {
        assert_eq!(parse_decimal(String::from("0000123")), "123");
    }

    #[test]
    fn test_parse_decimal_strip_alphabetic_characters() {
        assert_eq!(parse_decimal(String::from("123abc")), "123");
    }

    #[test]
    fn test_parse_decimal_strip_alphabetic_characters_with_dot() {
        assert_eq!(parse_decimal(String::from("123.abc12")), "123.12");
    }
}
