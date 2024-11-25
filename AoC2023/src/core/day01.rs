extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

/// Processes a string according to the rules for Day 1 Part 1
pub fn day01_part1(input: &str) -> i32 {
    let first_digit_char = get_first_digit_char(input);
    let last_digit_char = get_first_digit_char(reverse_graphemes(input).as_str());
    format_char_digits_to_int(first_digit_char, last_digit_char)
}

/// Processes a string according to the rules for Day 1 Part 2
pub fn day01_part2(input: &str) -> i32 {
    let indices_and_digits = get_index_and_digit(input);
    let first_digit_char = word_to_digit_char(indices_and_digits.1);
    let last_digit_char = word_to_digit_char(indices_and_digits.3);
    format_char_digits_to_int(first_digit_char, last_digit_char)
}

/// Reverse the graphemes of the input and returns a new String
pub fn reverse_graphemes(input: &str) -> String {
    let reversed = input.graphemes(true).rev().collect::<String>();
    reversed.to_string()
}

/// Smooshes to chars together and parses it as an int
pub fn format_char_digits_to_int(a: char, b: char) -> i32 {
    let number_str = format!("{}{}", a, b);
    match number_str.parse::<i32>() {
        Ok(x) => x,
        Err(_) => 0,
    }
}

/// Returns the first ascii digit as a char from the string
pub fn get_first_digit_char(input: &str) -> char {
    let digit_opt = input.chars().find(|x| char::is_ascii_digit(x));
    match digit_opt {
        Some(x) => x,
        None => '\0',
    }
}

/// When a string represents a word or a number return it as a digit
pub fn word_to_digit_char(input: &str) -> char {
    match input.to_lowercase().as_str() {
        "0" | "zero" => '0',
        "1" | "one" => '1',
        "2" | "two" => '2',
        "3" | "three" => '3',
        "4" | "four" => '4',
        "5" | "five" => '5',
        "6" | "six" => '6',
        "7" | "seven" => '7',
        "8" | "eight" => '8',
        "9" | "nine" => '9',
        _ => panic!("'{}' was not an expected digit word", input),
    }
}

pub fn get_index_and_digit(input: &str) -> (usize, &str, usize, &str) {
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
    let max_index = indices
        .iter()
        .reduce(|a, b| if a.0 < b.0 { b } else { a })
        .expect("did not find an digit in the string");

    (min_index.0, min_index.1, max_index.0, max_index.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01_part1() {
        assert_eq!(12, day01_part1("1abc2"));
        assert_eq!(38, day01_part1("pqr3stu8vwx"));
        assert_eq!(15, day01_part1("a1b2c3d4e5f"));
        assert_eq!(77, day01_part1("treb7uchet"));
    }

    #[test]
    fn test_day01_part2() {
        assert_eq!(29, day01_part2("two1nine"));
        assert_eq!(83, day01_part2("eightwothree"));
        assert_eq!(13, day01_part2("abcone2threexyz"));
        assert_eq!(24, day01_part2("xtwone3four"));
        assert_eq!(42, day01_part2("4nineeightseven2"));
        assert_eq!(14, day01_part2("zoneight234"));
        assert_eq!(76, day01_part2("7pqrstsixteen"));
    }

    #[test]
    fn test_get_digit_char() {
        assert_eq!('1', get_first_digit_char("1abc2"));
        assert_eq!('3', get_first_digit_char("pqr3stu8vwx"));
        assert_eq!('\0', get_first_digit_char("pqrstuvwx"));
    }

    #[test]
    fn test_get_index_of_digit() {
        assert_eq!((1, "1", 1, "1"), get_index_and_digit("a1aaaaa"));
        assert_eq!((2, "two", 6, "one"), get_index_and_digit("abtwoaoneaaaa"));
        assert_eq!((9, "3", 12, "7"), get_index_and_digit("abtw_othr3aa7bas"));
    }

    #[test]
    fn test_word_to_digit() {
        assert_eq!('3', word_to_digit_char("three"));
        assert_eq!('3', word_to_digit_char("3"));
    }
}
