/// Parses a string by a delimiter and converts the values into u32
pub fn parse_str_with_separator(input: &str, delimiter: &str) -> Vec<u32> {
    let parsed: Vec<u32> = input
        .trim()
        .split(delimiter)
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u32>().expect("Should parse into u32"))
        .collect();
    parsed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_str_with_separator() {
        let expected: Vec<u32> = vec![13, 42, 69];
        assert_eq!(expected, parse_str_with_separator("13 42 69", " "));
        assert_eq!(expected, parse_str_with_separator(" 13    42 69 ", " "));
        assert_eq!(expected, parse_str_with_separator("13, 42, 69", ","));
    }
}
