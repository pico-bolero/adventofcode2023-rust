use std::collections::HashSet;

use super::parsers::parse_str_with_separator;

/// Pretty print the result of the calculations
pub fn day04_part1(lines: &mut dyn Iterator<Item = String>) {
    let total = day04_part1_handler(lines);
    println!("Total: {}", total);
}

/// 1. Parse the lines into Cards that have two lists of values
/// 2. Create a set from each list. Find the Union of each set
/// 3. Return 2^<Number of Sets>
fn day04_part1_handler(lines: &mut dyn Iterator<Item = String>) -> u32 {
    let base: u32 = 2;
    let score = lines
        .map(|x| Card::from_str(x.as_str()))
        .map(|x: Card| x.matches())
        .map(|x| {
            if x.len() == 0 {
                0
            } else {
                base.pow((x.len() - 1).try_into().unwrap())
            }
        })
        .sum();
    score
}

#[derive(Eq, PartialEq, Debug)]
struct Card {
    id: u32,
    winners: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn matches(&self) -> Vec<u32> {
        let winners_set: HashSet<u32> = HashSet::from_iter(self.winners.iter().cloned());
        let numbers_set: HashSet<u32> = HashSet::from_iter(self.numbers.iter().cloned());
        let intersection: Vec<u32> = winners_set
            .intersection(&numbers_set)
            .map(|x| x.clone())
            .collect();
        intersection
    }

    fn from_str(input: &str) -> Card {
        // Segment the line
        let mut splits = input.split(":");
        let card_segment = splits.next().expect("First segments should be Card #");
        let numbers_segment = splits
            .next()
            .expect("Second segment should be the remainder");

        // Extract the card id
        let card_id = card_segment
            .replace("Card ", "")
            .trim()
            .parse::<u32>()
            .expect("Should parse to a u32");

        // Extract the winners and numbers
        let mut splits = numbers_segment.split("|");
        let lhs = splits.next().expect("Left hand side: winners");
        let rhs = splits.next().expect("Right hand side: numbers");

        let winners = parse_str_with_separator(lhs, " ");
        let numbers = parse_str_with_separator(rhs, " ");

        let card = Card {
            id: card_id,
            winners,
            numbers,
        };
        card
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day04_part1() {
        let lines: Vec<&str> = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .split('\n')
            .collect();

        let result = day04_part1_handler(&mut lines.iter().map(|x| x.to_string()));
        assert_eq!(13, result);
    }

    #[test]
    fn test_matches() {
        let card = Card {
            id: 0,
            winners: vec![0, 1, 2, 3, 7, 8, 9],
            numbers: vec![0, 3, 7, 9, 10, 11, 12],
        };
        let calculated: HashSet<u32> = HashSet::from_iter(card.matches().iter().cloned());
        let expected: HashSet<u32> = HashSet::from_iter(vec![0, 3, 7, 9].iter().cloned());
        assert_eq!(expected, calculated);
    }

    #[test]
    fn test_card_from_str() {
        let calculated = Card::from_str("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        let expected = Card {
            id: 6,
            winners: vec![31, 18, 13, 56, 72],
            numbers: vec![74, 77, 10, 23, 35, 67, 36, 11],
        };
        assert_eq!(expected, calculated);
    }
}
