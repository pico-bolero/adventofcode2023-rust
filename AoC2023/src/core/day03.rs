use std::ops::Range;

/// Pretty print the result of the calculations
pub fn day03_part1(lines: &mut dyn Iterator<Item = String>) {
    let total = day03_part1_handler(lines);
    println!("Total: {}", total);
}

fn day03_part1_handler(lines: &mut dyn Iterator<Item = String>) -> u32 {
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
        total += scan_area_handler(scan_area.clone());
    }
    total
}

pub fn day03_part2(lines: &mut dyn Iterator<Item = String>) {
    let total = day03_part2_handler(lines);
    println!("Total: {}", total);
}

fn day03_part2_handler(lines: &mut dyn Iterator<Item = String>) -> u32 {
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
        total += scan_area_handler2(scan_area.clone());
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
            part_intersects_gears(x, &mut gear_chain)
        })
        .collect();
    let result: u32 = part_numbers.iter().map(|x| x.value).sum();
    result
}

fn scan_area_handler2(scan_area: ScanArea) -> u32 {
    let current_line = scan_area.current.expect("There should always be a current");

    let gears = extract_gears(&current_line);
    let prev_parts = if scan_area.prev.is_some() {
        extract_part_number_locations(scan_area.prev.unwrap().as_str())
    } else {
        Vec::new()
    };
    let curr_parts = extract_part_number_locations(&current_line);
    let next_parts = if scan_area.next.is_some() {
        extract_part_number_locations(scan_area.next.unwrap().as_str())
    } else {
        Vec::new()
    };

    let gear_ratios: Vec<u32> = gears
        .iter()
        .map(|gear| {
            let mut part_chain = prev_parts
                .iter()
                .chain(curr_parts.iter())
                .chain(next_parts.iter());
            gear_ratio(gear, &mut part_chain)
        })
        .collect();

    let result: u32 = gear_ratios.iter().map(|x| x).sum();
    result
}

/// Accepts a part location and gears, returns true if the part intersects any gears
fn part_intersects_gears(
    part_number_location: &PartNumberLocation,
    gears: &mut dyn Iterator<Item = &GearLocation>,
) -> bool {
    // Lesson learned: gears is a gear chain, so you cannot call `any` on it? Correct.
    #[allow(clippy::unnecessary_fold)]
    let result: bool = gears.fold(false, |prev, gear| {
        prev || part_number_location.extended_range().contains(&gear.index)
    });
    result
}

/// Accepts a gear, returns the parts that it is adjacent to.
fn gear_ratio(
    gear: &GearLocation,
    part_numbers: &mut dyn Iterator<Item = &PartNumberLocation>,
) -> u32 {
    // Ignore non-gear symbols
    if gear._symbol != '*' {
        return 0;
    }

    // Lesson learned: part_numbers is a iter chain, so you cannot call `any` on it? Correct.
    #[allow(clippy::unnecessary_fold)]
    let parts_touching_gears: Vec<&PartNumberLocation> = part_numbers
        .filter(|part| part.extended_range().contains(&gear.index))
        .collect();

    if parts_touching_gears.len() <= 1 {
        return 0;
    }
    parts_touching_gears.iter().fold(1, |x, y| x * y.value)
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

impl PartNumberLocation {
    /// Helper to get the PartNumberLocations' range expanded by one on each size
    ///   for comparison
    fn extended_range(&self) -> Range<usize> {
        Range {
            start: if self.index.start == 0 {
                0
            } else {
                self.index.start - 1
            },
            end: self.index.end + 1,
        }
    }
}

#[derive(Debug)]
struct GearLocation {
    _symbol: char,
    index: usize,
}

fn extract_gears(input: &str) -> Vec<GearLocation> {
    let gears: Vec<GearLocation> = input
        .chars()
        .enumerate()
        .filter(|(_idx, x)| !(x.is_ascii_alphanumeric() || *x == '.'))
        .map(|(idx, x)| GearLocation {
            _symbol: x,
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
            let idx = only_the_digits.find(x).unwrap_or_else(|| {
                panic!(
                    "This was already found! Looking for {}; {}!",
                    x, only_the_digits
                )
            });
            let value: u32 = x.parse().expect("Should parse into a u32");
            let replacement = " ".repeat(x.len());
            only_the_digits = only_the_digits.replacen(x, replacement.as_str(), 1);
            PartNumberLocation {
                value,
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
        let lines: Vec<&str> = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .split('\n')
            .collect();

        let result = day03_part1_handler(&mut lines.iter().map(|x| x.to_string()));
        assert_eq!(4361, result);
    }

    #[test]
    fn test_day03_part2() {
        let lines: Vec<&str> = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .split('\n')
            .collect();
        let result = day03_part2_handler(&mut lines.iter().map(|x| x.to_string()));
        assert_eq!(467835, result);
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
        let mut scan_area = ScanArea {
            prev: None,
            current: Some("..123..".to_string()),
            next: Some(".....*.".to_string()),
        };
        assert_eq!(123, scan_area_handler(scan_area));

        scan_area = ScanArea {
            prev: Some(".*.....".to_string()),
            current: Some("..123..".to_string()),
            next: None,
        };
        assert_eq!(123, scan_area_handler(scan_area));

        scan_area = ScanArea {
            prev: None,
            current: Some("..123..".to_string()),
            next: Some(".......".to_string()),
        };
        assert_eq!(0, scan_area_handler(scan_area));

        {
            scan_area = ScanArea {
                prev: Some(".......".to_string()),
                current: Some("..123..".to_string()),
                next: None,
            };
            assert_eq!(0, scan_area_handler(scan_area));
        }

        let scan_area = ScanArea {
            prev: Some("..999.*..................".to_string()),
            current: Some("..111..222.*333.444*.555.".to_string()),
            next: Some("..999...................+".to_string()),
        };
        assert_eq!(222 + 333 + 444 + 555, scan_area_handler(scan_area));
    }

    #[test]
    fn test_part_number_location_extend_range() {
        let part = PartNumberLocation {
            index: Range { start: 0, end: 2 },
            value: 13,
        };
        assert_eq!(Range { start: 0, end: 3 }, part.extended_range());

        let part = PartNumberLocation {
            index: Range { start: 10, end: 15 },
            value: 1234,
        };
        assert_eq!(Range { start: 9, end: 16 }, part.extended_range());
    }

    #[test]
    fn test_part_intersects_gears_1() {
        let part_number = PartNumberLocation {
            index: Range { start: 0, end: 3 },
            value: 123,
        };
        let gear = GearLocation {
            index: 0,
            _symbol: '*',
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
            _symbol: '+',
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
                _symbol: '+',
            },
            GearLocation {
                index: 8,
                _symbol: '*',
            },
            GearLocation {
                index: 11,
                _symbol: '#',
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
