use std::ops::Range;

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

struct RangeMapping {
    dst_range_start: u32,
    src_range_start: u32,
    range_len: u32,
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
}
