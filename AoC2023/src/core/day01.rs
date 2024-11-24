extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn combine_first_and_last_digit(input: &str) -> i32 {
    let first_digit_char = get_first_digit_char(input.to_string());
    let last_digit_char = get_first_digit_char(input.graphemes(true).rev().collect());
    let number_str = format!("{}{}", first_digit_char, last_digit_char);
    match number_str.parse::<i32>() {
        Ok(x) => x,
        Err(_) => 0,
    }
}

pub fn get_first_digit_char(input: String) -> char {
    let digit_opt = input.chars().find(|x| char::is_ascii_digit(x));
    match digit_opt {
        Some(x) => x,
        None => '\0',
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_digit_char() {
        assert_eq!('1', get_first_digit_char("1abc2".to_string()));
        assert_eq!('3', get_first_digit_char("pqr3stu8vwx".to_string()));
        assert_eq!('\0', get_first_digit_char("pqrstuvwx".to_string()));
    }

    #[test]
    fn test_combine_first_and_last_digit() {
        assert_eq!(12, combine_first_and_last_digit("1abc2"));
        assert_eq!(38, combine_first_and_last_digit("pqr3stu8vwx"));
        assert_eq!(15, combine_first_and_last_digit("a1b2c3d4e5f"));
        assert_eq!(77, combine_first_and_last_digit("treb7uchet"));
    }
}
