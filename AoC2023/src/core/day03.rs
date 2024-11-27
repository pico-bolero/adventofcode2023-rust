use std::ops::Range;

pub fn day03_part1(lines: &mut dyn Iterator<Item = String>) -> () {
    let total = day03_part1_handler(lines);
    print!("Total: {}\n", total);
}

pub fn day03_part1_handler(lines: &mut dyn Iterator<Item = String>) -> u32 {
    let mut scan_area = ScanArea {
        prev: None,
        current: None,
        next: lines.next(),
    };

    let mut total = 0u32;
    loop {
        let line = lines.next();
        scan_area = scan_area.shift_new(line);

        // Processed the last item
        if scan_area.current.is_none() {
            break;
        }
        total = total + scan_area_handler(scan_area.clone());
    }
    total
}

fn scan_area_handler(scan_area: ScanArea) -> u32 {
    let current_line = scan_area.current.expect("There should always be a current");
    let part_number_locations = extract_part_number_locations(&current_line);
    let prev_gears = if scan_area.prev.is_some() {
        extract_gears(scan_area.prev.unwrap().as_str())
    } else {
        Vec::new()
    };

    let curr_gears = extract_gears(&current_line);

    let next_gears = if scan_area.next.is_some() {
        extract_gears(scan_area.next.unwrap().as_str())
    } else {
        Vec::new()
    };
    let part_numbers: Vec<&PartNumberLocation> = part_number_locations
        .iter()
        .filter(|x| {
            let mut gear_chain = prev_gears
                .iter()
                .chain(curr_gears.iter())
                .chain(next_gears.iter());
            part_intersects_gears(*x, &mut gear_chain)
        })
        .collect();
    let result: u32 = part_numbers.iter().map(|x| x.value).sum();
    result
}

/// Accepts a part location and gears, returns true if the part intersects any gears
fn part_intersects_gears(
    part_number_location: &PartNumberLocation,
    gears: &mut dyn Iterator<Item = &GearLocation>,
) -> bool {
    let extended_range = Range {
        start: if part_number_location.index.start == 0 {
            0
        } else {
            part_number_location.index.start - 1
        }, // usize is going to hurt here
        end: part_number_location.index.end + 1,
    };
    // Lesson learned: gears is a gear chain, so you cannot call `any` on it? Correct.
    let result: bool = gears.fold(false, |prev, gear| {
        prev || extended_range.contains(&gear.index)
    });
    result
}

// ScanArea
#[derive(Debug, Eq, PartialEq, Clone)]
struct ScanArea {
    prev: Option<String>,
    current: Option<String>,
    next: Option<String>,
}

impl ScanArea {
    /// Returns a new ScanArea by cloning the elements of the current
    /// ScanArea and shifting all the elements to the next position.
    fn shift_new(&self, line: Option<String>) -> ScanArea {
        ScanArea {
            prev: self.current.clone(),
            current: self.next.clone(),
            next: line.clone(),
        }
    }
}

#[derive(Debug)]
struct PartNumberLocation {
    index: Range<usize>,
    value: u32,
}

#[derive(Debug)]
struct GearLocation {
    symbol: char,
    index: usize,
}

fn extract_gears(input: &str) -> Vec<GearLocation> {
    let gears: Vec<GearLocation> = input
        .chars()
        .enumerate()
        .filter(|(_idx, x)| !(x.is_ascii_alphanumeric() || *x == '.'))
        .map(|(idx, x)| GearLocation {
            symbol: x,
            index: idx,
        })
        .collect();
    gears
}

fn extract_part_number_locations(input: &str) -> Vec<PartNumberLocation> {
    // replace all non-digits with space in the input
    let mut only_the_digits: String = input
        .chars()
        .map(|x| if x.is_ascii_digit() { x } else { ' ' })
        .collect();

    // The string should only contain digits and spaces, split on spaces and trim
    //   to get a list of strings to find indices for.
    let number_strings: Vec<String> = only_the_digits
        .split(" ")
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty())
        .collect();

    // Iterate through the numbers strings. Find the range indices of the string.
    //  Replace the number string with spaces so it is not detected again. Repeat.
    let part_numbers: Vec<PartNumberLocation> = number_strings
        .iter()
        .map(|x| {
            let idx = only_the_digits.find(x).expect(
                format!(
                    "This was already found! Looking for {}; {}!",
                    x, only_the_digits
                )
                .as_str(),
            );
            let value: u32 = x.parse().expect("Should parse into a u32");
            let replacement = " ".repeat(x.len());
            only_the_digits = only_the_digits.replacen(x, replacement.as_str(), 1);
            PartNumberLocation {
                value: value,
                index: Range {
                    start: idx,
                    end: idx + x.len(),
                },
            }
        })
        .collect();
    part_numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day03_part1() {
        let lines = vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];
        let result = day03_part1_handler(&mut lines.iter().map(|x| x.to_string()));
        assert_eq!(4361, result);
    }

    #[test]
    fn test_scan_area() {
        let mut scan_area = ScanArea {
            prev: None,
            current: None,
            next: None,
        };

        {
            scan_area = scan_area.shift_new(Some("lineA".to_string()));
            let expected = ScanArea {
                prev: None,
                current: None,
                next: Some("lineA".to_string()),
            };
            assert_eq!(expected, scan_area);
        }

        {
            scan_area = scan_area.shift_new(Some("lineB".to_string()));
            let expected = ScanArea {
                prev: None,
                current: Some("lineA".to_string()),
                next: Some("lineB".to_string()),
            };
            assert_eq!(expected, scan_area);
        }

        scan_area = scan_area.shift_new(Some("lineC".to_string()));
        let expected = ScanArea {
            prev: Some("lineA".to_string()),
            current: Some("lineB".to_string()),
            next: Some("lineC".to_string()),
        };
        assert_eq!(expected, scan_area);

        scan_area = scan_area.shift_new(None);
        let expected = ScanArea {
            prev: Some("lineB".to_string()),
            current: Some("lineC".to_string()),
            next: None,
        };
        assert_eq!(expected, scan_area);

        scan_area = scan_area.shift_new(None);
        let expected = ScanArea {
            prev: Some("lineC".to_string()),
            current: None,
            next: None,
        };
        assert_eq!(expected, scan_area);

        scan_area = scan_area.shift_new(None);
        let expected = ScanArea {
            prev: None,
            current: None,
            next: None,
        };
        assert_eq!(expected, scan_area);
    }

    #[test]
    fn test_scan_area_handler() {
        let scan_area = ScanArea {
            prev: None,
            current: Some("..123..".to_string()),
            next: Some(".....*.".to_string()),
        };
        assert_eq!(123, scan_area_handler(scan_area));

        let scan_area = ScanArea {
            prev: Some(".*.....".to_string()),
            current: Some("..123..".to_string()),
            next: None,
        };
        assert_eq!(123, scan_area_handler(scan_area));

        let scan_area = ScanArea {
            prev: None,
            current: Some("..123..".to_string()),
            next: Some(".......".to_string()),
        };
        assert_eq!(0, scan_area_handler(scan_area));

        let scan_area = ScanArea {
            prev: Some(".......".to_string()),
            current: Some("..123..".to_string()),
            next: None,
        };
        assert_eq!(0, scan_area_handler(scan_area));

        let scan_area = ScanArea {
            prev: Some("..999.*..................".to_string()),
            current: Some("..111..222.*333.444*.555.".to_string()),
            next: Some("..999...................+".to_string()),
        };
        assert_eq!(222 + 333 + 444 + 555, scan_area_handler(scan_area));
    }

    #[test]
    fn test_part_intersects_gears_1() {
        let part_number = PartNumberLocation {
            index: Range { start: 0, end: 3 },
            value: 123,
        };
        let gear = GearLocation {
            index: 0,
            symbol: '*',
        };
        // should intersect
        let gears = vec![gear];
        assert!(part_intersects_gears(&part_number, &mut gears.iter()));
    }

    #[test]
    fn test_part_intersects_gears_2() {
        let part_number = PartNumberLocation {
            index: Range { start: 0, end: 3 },
            value: 123,
        };
        let gear = GearLocation {
            index: 3,
            symbol: '+',
        };
        // should intersect
        let gears = vec![gear];
        assert!(part_intersects_gears(&part_number, &mut gears.iter()));
    }

    #[test]
    fn test_part_intersects_gears_3() {
        let part_number = PartNumberLocation {
            index: Range { start: 5, end: 8 },
            value: 123,
        };
        // should intersect
        let gears = vec![
            GearLocation {
                index: 3,
                symbol: '+',
            },
            GearLocation {
                index: 8,
                symbol: '*',
            },
            GearLocation {
                index: 11,
                symbol: '#',
            },
        ];
        assert!(part_intersects_gears(&part_number, &mut gears.iter()));
    }

    #[test]
    fn test_extract_part_number_locations() {
        assert!(extract_part_number_locations("!@#$%^&*()-+=.").is_empty());
        assert_eq!(
            5,
            extract_part_number_locations("123..1..456..654..789").len()
        );
    }

    #[test]
    fn test_extract_gears() {
        assert!(extract_gears(".0123456789").is_empty());
        assert_eq!(13, extract_gears("!@#$%^&*()-+=").len());
    }
}