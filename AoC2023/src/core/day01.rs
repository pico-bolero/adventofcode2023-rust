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

pub fn word_to_digit(input: &str) -> &str {
    match input.to_lowercase().as_str() {
        "0" | "zero" => "0",
        "1" | "one" => "1",
        "2" | "two" => "2",
        "3" | "three" => "3",
        "4" | "four" => "4",
        "5" | "five" => "5",
        "6" | "six" => "6",
        "7" | "seven" => "7",
        "8" | "eight" => "8",
        "9" | "nine" => "9",
        _ => panic!("'{}' was not an expected digit word", input),
    }
}

pub fn get_index_of_digit(input: &str) -> (usize, &str) {
    let digits = vec![
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine",
    ];
    // locates the indicies of every digit and word
    let indices: Vec<(usize, &str)> = digits
        .iter()
        .flat_map(|x| input.match_indices(*x).collect::<Vec<(usize, &str)>>())
        .collect();

    // reduces down to the first index and word
    let min_index = indices
        .iter()
        .reduce(|a, b| if a.0 < b.0 { a } else { b })
        .expect("did not find an digit in the string");

    (min_index.0, min_index.1)
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

    #[test]
    fn test_get_index_of_digit() {
        assert_eq!((1, "1"), get_index_of_digit("a1aaaaa"));
        assert_eq!((2, "two"), get_index_of_digit("abtwoaaaaa"));
        assert_eq!((9, "3"), get_index_of_digit("abtw_othr3aa"));
    }

    #[test]
    fn test_word_to_digit() {
        assert_eq!("3", word_to_digit("three"));
        assert_eq!("3", word_to_digit("3"));
    }
}
