use super::parsers::parse_str_with_separator;
use std::{cell::Cell, ops::Range, str::FromStr};

/// Pretty print the result of the calculations
pub fn day05_part1(lines: &mut dyn Iterator<Item = String>) {
    let total = day05_part1_handler(lines);
    println!("Total: {}", total);
}

/// Procedure
/// 1. Parse the inputs into structs for the mappings and a list for the seeds.
/// 2. Build the path from seed to location
/// 3. Iterate over the seeds to find the location
/// 4. Select the smallest location
fn day05_part1_handler(lines: &mut dyn Iterator<Item = String>) -> u32 {
    todo!()
}

struct Something {
    seeds: Vec<u32>,
    seed_to_soil_mappings: Vec<RangeMapping>,
    soil_to_fertilizer_mappings: Vec<RangeMapping>,
    fertilizer_to_waters: Vec<RangeMapping>,
    water_to_lights: Vec<RangeMapping>,
    light_to_temperatures: Vec<RangeMapping>,
    temperature_to_humiditys: Vec<RangeMapping>,
    humidity_to_locations: Vec<RangeMapping>,
}

impl Something {
    fn new() -> Something {
        Something {
            seeds: Vec::new(),
            seed_to_soil_mappings: Vec::new(),
            soil_to_fertilizer_mappings: Vec::new(),
            fertilizer_to_waters: Vec::new(),
            water_to_lights: Vec::new(),
            light_to_temperatures: Vec::new(),
            temperature_to_humiditys: Vec::new(),
            humidity_to_locations: Vec::new(),
        }
    }
    /*
        fn from_str_itr(lines: &mut dyn Iterator<Item = String>) -> Something {
            let mut s = Something::new();
            let r: Cell<&Vec<RangeMapping>> = Cell::new(&s.seed_to_soil_mappings);

            lines.for_each(|line| match line {
                x if x.contains("seeds:") => {
                    let mut splits = x.split(":");
                    splits.next(); // throw away the first part.
                    s.seeds
                        .append(&mut parse_str_with_separator(splits.next().unwrap(), ","));
                }
                x if x.contains("seed-to-soil map:") => {
                    r.replace(&mut s.seed_to_soil_mappings);
                }
                x if x.contains("soil-to-fertilizer map:") => {}
                x if x.contains("fertilizer-to-water map:") => {}
                x if x.contains("water-to-light map:") => {}
                x if x.contains("light-to-temperature map:") => {}
                x if x.contains("temperature-to-humitity map:") => {}
                x if x.contains("humidity-to-location map:") => {}
                x if x.is_empty() => {}
                _ => { /* parse range */ }
            });

            s
        }
    */
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct RangeMapping {
    dst_range_start: u32,
    src_range_start: u32,
    range_len: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct ParseRangeMappingError {
    message: String,
}

impl FromStr for RangeMapping {
    type Err = ParseRangeMappingError;

    /// There has got to be a cleaner way of doing this.
    fn from_str(input: &str) -> Result<RangeMapping, Self::Err> {
        // Segment the line
        let mut splits = input.split(" ");
        let dst_range_start = splits
            .next()
            .ok_or(ParseRangeMappingError {
                message: "Failed to get the first chunk".to_string(),
            })
            .map(|x| {
                x.trim().parse::<u32>().map_err(|_| ParseRangeMappingError {
                    message: "Failed to parse string into u32".to_string(),
                })
            })?;
        let src_range_start = splits
            .next()
            .ok_or(ParseRangeMappingError {
                message: "Failed to get the first chunk".to_string(),
            })
            .map(|x| {
                x.trim().parse::<u32>().map_err(|_| ParseRangeMappingError {
                    message: "Failed to parse string into u32".to_string(),
                })
            })?;
        let range_len = splits
            .next()
            .ok_or(ParseRangeMappingError {
                message: "Failed to get the first chunk".to_string(),
            })
            .map(|x| {
                x.trim().parse::<u32>().map_err(|_| ParseRangeMappingError {
                    message: "Failed to parse string into u32".to_string(),
                })
            })?;
        match (dst_range_start, src_range_start, range_len) {
            (Ok(dst), Ok(src), Ok(len)) => Ok(RangeMapping {
                dst_range_start: dst,
                src_range_start: src,
                range_len: len,
            }),
            _ => Err(ParseRangeMappingError {
                message: "Failed to parse".to_string(),
            }),
        }
    }
}

impl RangeMapping {
    /// If the input value is in source range, then return destination
    fn find_destination(&self, input: u32) -> Option<u32> {
        let rng = Range {
            start: self.src_range_start,
            end: self.src_range_start + self.range_len,
        };
        if rng.contains(&input) {
            // what is the difference from source to the start?
            let delta = input - self.src_range_start;
            return Some(self.dst_range_start + delta);
        }
        None
    }
}

fn get_destination_for_ranges(source: u32, ranges: &Vec<RangeMapping>) -> u32 {
    let rng_mapping = ranges.iter().find(|x| x.find_destination(source).is_some());
    match rng_mapping {
        Some(x) => x.find_destination(source).unwrap(),
        None => source,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_mapping_get_destination() {
        let range_mapping = RangeMapping {
            src_range_start: 0,
            dst_range_start: 10,
            range_len: 5,
        };
        assert_eq!(Some(13u32), range_mapping.find_destination(3));
        assert_eq!(None, range_mapping.find_destination(5));
    }

    #[test]
    fn test_get_destination_for_ranges() {
        let range_mappings = vec![
            RangeMapping {
                src_range_start: 0,
                dst_range_start: 10,
                range_len: 5,
            },
            RangeMapping {
                src_range_start: 20,
                dst_range_start: 30,
                range_len: 5,
            },
        ];

        // inside the first range
        assert_eq!(13, get_destination_for_ranges(3, &range_mappings));
        // inside the second range
        assert_eq!(30, get_destination_for_ranges(20, &range_mappings));
        // Outside the mappings returns the same value
        assert_eq!(100, get_destination_for_ranges(100, &range_mappings));
        // outside the first range, but before the second range
        assert_eq!(5, get_destination_for_ranges(5, &range_mappings));
    }

    #[test]
    fn test_parse_range_mapping_from_str() -> Result<(), ParseRangeMappingError> {
        let calculated = match RangeMapping::from_str("3154320624 3939365694 227285246") {
            Ok(it) => it,
            Err(err) => return Err(err),
        };
        let expected = RangeMapping {
            dst_range_start: 3154320624,
            src_range_start: 3939365694,
            range_len: 227285246,
        };
        assert_eq!(expected, calculated);
        Ok(())
    }
}
