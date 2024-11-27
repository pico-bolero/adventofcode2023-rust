/// Receives input and prints output
pub fn day02_part1(lines: &mut dyn Iterator<Item = String>) {
    let total = day02_part1_handler(lines);
    println!("Sum {}", total);
}

/// Receives input and prints output
pub fn day02_part2(lines: &mut dyn Iterator<Item = String>) {
    let total = day02_part2_handler(lines);
    println!("Power total {}", total);
}

#[derive(Eq, PartialEq, Debug)]
struct CubeCounts {
    red: u32,
    green: u32,
    blue: u32,
}

/// Inspects the data and of the game and if it is possible given the constraint the sum the game number
fn day02_part1_handler(lines: &mut dyn Iterator<Item = String>) -> u32 {
    let constraint = CubeCounts {
        red: 12,
        green: 13,
        blue: 14,
    };
    let total: u32 = lines
        .map(|x| day02_part1_line_handler(x.as_str(), &constraint))
        .sum();
    total
}

/// Parse the input into a PullResult and compare against the constraint
/// Return 0 if the game cannot meet the constraint
fn day02_part1_line_handler(input: &str, constraint: &CubeCounts) -> u32 {
    let (game_id, pulls) = parse_game(input);
    if pulls.iter().all(|x| {
        x.red <= constraint.red && x.blue <= constraint.blue && x.green <= constraint.green
    }) {
        return game_id;
    }
    0
}

/// Sums all the power levels for game
fn day02_part2_handler(lines: &mut dyn Iterator<Item = String>) -> u32 {
    let power = lines
        .map(|line| parse_game(line.as_str()))
        .map(|(_game_id, pulls)| calculate_power(pulls))
        .sum();
    power
}

/// Iterates over the input vector to establish the minimum number of cubes for each color
/// in the bag. Then multiplies those values together to determine the power.
fn calculate_power(pulls: Vec<CubeCounts>) -> u32 {
    let min_possible_cubes_in_bag = pulls.iter().fold(
        CubeCounts {
            red: 0,
            green: 0,
            blue: 0,
        },
        |mut acc, x| {
            if x.red > acc.red {
                acc.red = x.red
            }
            if x.green > acc.green {
                acc.green = x.green
            }
            if x.blue > acc.blue {
                acc.blue = x.blue
            }
            acc
        },
    );
    // Not required: if one of these values is zero that would be bad. if need, put into a vec and filter out the zeros
    //  and calculate the product
    min_possible_cubes_in_bag.red * min_possible_cubes_in_bag.green * min_possible_cubes_in_bag.blue
}

/// Parses the input into a tuple of the game_id and a vector of pulls
fn parse_game(input: &str) -> (u32, Vec<CubeCounts>) {
    // Extract the game id.
    let mut splits = input.split(":");
    let game_str = splits.next().expect("First segments should be Game #");
    let remainder = splits
        .next()
        .expect("Second segment should be a list of Pull");
    let game_id = parse_game_segment(game_str);
    let pulls = parse_pulls_segment(remainder);
    (game_id, pulls)
}

/// Excepts a string in the format of 'Game #' and returns the # as a u32
fn parse_game_segment(game_str: &str) -> u32 {
    let game_str = game_str.trim();
    let mut splits = game_str.split(" ");
    splits.next().expect("Don't need the game segment");
    let game_id = splits.next().expect("This should be the int portion");
    game_id.parse::<u32>().expect("Should parse into int")
}

/// Excepts a str in the format of a repeating '# (red|blue|green),' values separated by a semicolon
fn parse_pulls_segment(game_str: &str) -> Vec<CubeCounts> {
    let pulls = game_str.split(";");
    pulls.map(parse_to_pull_result).collect()
}

/// Except a string of '# (red|blue|green)' separated by a comma and return a PullResult
fn parse_to_pull_result(pull: &str) -> CubeCounts {
    let mut result = CubeCounts {
        red: 0,
        green: 0,
        blue: 0,
    };
    pull.split(",").for_each(|mut x| {
        x = x.trim();
        let mut splits = x.split(" ");
        let value = splits.next().expect("Should be an int").trim();
        let color = splits.next().expect("Should be a color").trim();
        match color {
            "red" => {
                result.red = value.parse::<u32>().expect("Should have been an u32");
            }
            "green" => {
                result.green = value.parse::<u32>().expect("Should have been an u32");
            }
            "blue" => {
                result.blue = value.parse::<u32>().expect("Should have been an u32");
            }
            _ => panic!("Color {} is unhandled", color),
        }
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day02_part1() {
        let v = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ];
        let mut itr = v.into_iter();
        assert_eq!(8u32, day02_part1_handler(&mut itr));
    }

    #[test]
    fn test_day02_part2() {
        let v = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ];
        let mut itr = v.into_iter();
        assert_eq!(2286u32, day02_part2_handler(&mut itr));
    }

    #[test]
    fn test_parse_game() {
        let game = "Game 21: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let (game_id, pull_results) = parse_game(game);
        assert_eq!(21u32, game_id);
        assert_eq!(3, pull_results.len());
    }

    #[test]
    fn test_parse_game_segment() {
        assert_eq!(0u32, parse_game_segment(" Game 0 "));
        assert_eq!(1u32, parse_game_segment("\tGame 1\n"));
        assert_eq!(
            u32::MAX,
            parse_game_segment(format!("Game {}", u32::MAX).as_str())
        );
    }

    #[test]
    fn test_parse_to_pull_result() {
        assert_eq!(
            CubeCounts {
                red: 1,
                green: 0,
                blue: 0
            },
            parse_to_pull_result(" 1 red ")
        );
        assert_eq!(
            CubeCounts {
                red: 0,
                green: 1,
                blue: 0
            },
            parse_to_pull_result(" 1 green ")
        );
        assert_eq!(
            CubeCounts {
                red: 0,
                green: 0,
                blue: 1
            },
            parse_to_pull_result(" 1 blue ")
        );
        assert_eq!(
            CubeCounts {
                red: 13,
                green: 42,
                blue: 69
            },
            parse_to_pull_result("\t69 blue  , 13 red  \t, 42 green \n")
        );
    }
}
