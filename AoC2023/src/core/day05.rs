use std::{ops::Range, str::FromStr};

/// Pretty print the result of the calculations
pub fn day05_part1(lines: &mut dyn Iterator<Item = String>) {
    let total = day05_part1_handler(lines);
    println!("Total: {}", total);
}

/// Procedure
/// 1. Parse the inputs into structs for the mappings and a list for the seeds.
/// 2. Iterate over the seeds to find the location
///     2a. Build the path from seed to location
/// 3. Select the smallest location
fn day05_part1_handler(lines: &mut dyn Iterator<Item = String>) -> u64 {
    let scenario = Scenario::from_str_itr(lines, Scenario::part1_seed_extractor);
    let min_location = scenario
        .seeds
        .iter()
        .map(|seed| location_for_seed(*seed, &scenario))
        .min();
    min_location.expect("There should have been an answer")
}

/// Pretty print the result of the calculations
pub fn day05_part2(lines: &mut dyn Iterator<Item = String>) {
    let total = day05_part2_handler(lines);
    println!("Total: {}", total);
}

/// Procedure
/// 1. Parse the inputs into structs for the mappings and a list for the seeds.
/// 2. Iterate over the seeds to find the location
///     2a. Build the path from seed to location
/// 3. Select the smallest location
fn day05_part2_handler(lines: &mut dyn Iterator<Item = String>) -> u64 {
    let scenario = Scenario::from_str_itr(lines, Scenario::part1_seed_extractor);
    let min_location = scenario
        .seeds
        .iter()
        .map(|seed| location_for_seed(*seed, &scenario))
        .min();
    min_location.expect("There should have been an answer")
}

fn destination_for_source(source: u64, rngs: &[RangeMapping]) -> u64 {
    let destination: Vec<u64> = rngs
        .iter()
        .flat_map(|rng| rng.find_destination(source))
        .collect();
    if destination.is_empty() {
        source
    } else {
        destination[0]
    }
}

fn location_for_seed(seed: u64, scenario: &Scenario) -> u64 {
    let soil = destination_for_source(seed, &scenario.seed_to_soil_mappings);
    let fertilizer = destination_for_source(soil, &scenario.soil_to_fertilizer_mappings);
    let water = destination_for_source(fertilizer, &scenario.fertilizer_to_waters);
    let light = destination_for_source(water, &scenario.water_to_lights);
    let temperature = destination_for_source(light, &scenario.light_to_temperatures);
    let humidity = destination_for_source(temperature, &scenario.temperature_to_humidities);
    destination_for_source(humidity, &scenario.humidity_to_locations)
}

struct Scenario {
    seeds: Vec<u64>,
    seed_to_soil_mappings: Vec<RangeMapping>,
    soil_to_fertilizer_mappings: Vec<RangeMapping>,
    fertilizer_to_waters: Vec<RangeMapping>,
    water_to_lights: Vec<RangeMapping>,
    light_to_temperatures: Vec<RangeMapping>,
    temperature_to_humidities: Vec<RangeMapping>,
    humidity_to_locations: Vec<RangeMapping>,
}

impl Scenario {
    fn new() -> Scenario {
        Scenario {
            seeds: Vec::new(),
            seed_to_soil_mappings: Vec::new(),
            soil_to_fertilizer_mappings: Vec::new(),
            fertilizer_to_waters: Vec::new(),
            water_to_lights: Vec::new(),
            light_to_temperatures: Vec::new(),
            temperature_to_humidities: Vec::new(),
            humidity_to_locations: Vec::new(),
        }
    }

    fn part1_seed_extractor(line: &str) -> Vec<u64> {
        let mut splits = line.split(":");
        splits.next(); // throw away the first part.
                       //
        let parsed: Vec<u64> = splits
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u64>().expect("Should parse into u64"))
            .collect();
        parsed
    }

    fn from_str_itr(
        lines: &mut dyn Iterator<Item = String>,
        seed_extractor: fn(&str) -> Vec<u64>,
    ) -> Scenario {
        let mut s = Scenario::new();

        lines.take_while(|x| !x.is_empty()).for_each(|x| {
            if x.contains("seeds:") {
                s.seeds.append(&mut (seed_extractor)(x.as_str()));
            }
        });
        lines.next(); // advance one.
        lines.take_while(|x| !x.is_empty()).for_each(|x| {
            if !x.contains("seed-to-soil map:") {
                s.seed_to_soil_mappings
                    .push(RangeMapping::from_str(x.as_str()).unwrap());
            }
        });
        lines.take_while(|x| !x.is_empty()).for_each(|x| {
            if !x.contains("soil-to-fertilizer map:") {
                s.soil_to_fertilizer_mappings
                    .push(RangeMapping::from_str(x.as_str()).unwrap());
            }
        });
        lines.take_while(|x| !x.is_empty()).for_each(|x| {
            if !x.contains("fertilizer-to-water map:") {
                s.fertilizer_to_waters
                    .push(RangeMapping::from_str(x.as_str()).unwrap());
            }
        });
        lines.take_while(|x| !x.is_empty()).for_each(|x| {
            if !x.contains("water-to-light map:") {
                s.water_to_lights
                    .push(RangeMapping::from_str(x.as_str()).unwrap());
            }
        });
        lines.take_while(|x| !x.is_empty()).for_each(|x| {
            if !x.contains("light-to-temperature map:") {
                s.light_to_temperatures
                    .push(RangeMapping::from_str(x.as_str()).unwrap());
            }
        });
        lines.take_while(|x| !x.is_empty()).for_each(|x| {
            if !x.contains("temperature-to-humidity map:") {
                s.temperature_to_humidities
                    .push(RangeMapping::from_str(x.as_str()).unwrap());
            }
        });
        lines.take_while(|x| !x.is_empty()).for_each(|x| {
            if !x.contains("humidity-to-location map:") {
                s.humidity_to_locations
                    .push(RangeMapping::from_str(x.as_str()).unwrap());
            }
        });

        s
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct RangeMapping {
    dst_range_start: u64,
    src_range_start: u64,
    range_len: u64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct ParseRangeMappingError {
    message: String,
}

impl FromStr for RangeMapping {
    type Err = ParseRangeMappingError;

    fn from_str(input: &str) -> Result<RangeMapping, Self::Err> {
        // Segment the line
        let mut splits = input.split(" ");
        let dst_range_start = splits.next().and_then(|x| match x.trim().parse::<u64>() {
            Ok(x) => Some(x),
            Err(_) => None,
        });
        let src_range_start = splits.next().and_then(|x| match x.trim().parse::<u64>() {
            Ok(x) => Some(x),
            Err(_) => None,
        });
        let range_len = splits.next().and_then(|x| match x.trim().parse::<u64>() {
            Ok(x) => Some(x),
            Err(_) => None,
        });

        match (dst_range_start, src_range_start, range_len) {
            (Some(dst), Some(src), Some(len)) => Ok(RangeMapping {
                dst_range_start: dst,
                src_range_start: src,
                range_len: len,
            }),
            _ => Err(ParseRangeMappingError {
                message: format!("Failed to parse {} into RangeMapping", input),
            }),
        }
    }
}

impl RangeMapping {
    /// If the input value is in source range, then return destination
    fn find_destination(&self, input: u64) -> Option<u64> {
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
        assert_eq!(Some(13u64), range_mapping.find_destination(3));
        assert_eq!(None, range_mapping.find_destination(5));
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

    fn sample_data() -> Vec<String> {
        let lines: Vec<String> = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            .split('\n')
            .map(|x| x.to_string())
            .collect();
        lines
    }

    #[test]
    fn test_day05_part1_handler() {
        let lines = sample_data();
        let calculated = day05_part1_handler(&mut lines.iter().map(|x| x.to_string()));
        assert_eq!(35, calculated);
    }

    #[test]
    fn test_day05_part2_handler() {
        let lines = sample_data();
        let calculated = day05_part2_handler(&mut lines.iter().map(|x| x.to_string()));
        assert_eq!(46, calculated);
    }
}
