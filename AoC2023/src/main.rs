pub mod core;
pub mod io;
use crate::core::day01;
use crate::core::day02;
use crate::core::day03;
use crate::io::data_loader;
use std::env;
use std::path::Path;

struct ScenarioConfig {
    file_path: String,
    process_fn: fn(&mut dyn Iterator<Item = String>) -> (),
}

// Examines the command line arguments and passes back
// the input file to load.
fn select_scenario() -> ScenarioConfig {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Expected 1 argument like 'day01_part1'");
    }
    match args[1].as_str() {
        "day01_part1" => ScenarioConfig {
            file_path: "./data/day01.txt".to_string(),
            process_fn: day01::day01_part1,
        },
        "day01_part2" => ScenarioConfig {
            file_path: "./data/day01.txt".to_string(),
            process_fn: day01::day01_part2,
        },
        "day02_part1" => ScenarioConfig {
            file_path: "./data/day02.txt".to_string(),
            process_fn: day02::day02_part1,
        },
        "day02_part2" => ScenarioConfig {
            file_path: "./data/day02.txt".to_string(),
            process_fn: day02::day02_part2,
        },
        "day03_part1" => ScenarioConfig {
            file_path: "./data/day03.txt".to_string(),
            process_fn: day03::day03_part1,
        },
        "day03_part2" => ScenarioConfig {
            file_path: "./data/day03.txt".to_string(),
            process_fn: day03::day03_part2,
        },

        _ => {
            panic!("Expected argument like 'day01_part1' and not {}", &args[1]);
        }
    }
}

fn main() {
    // Parse the input arguments
    let scenario = select_scenario();

    // Get the lines iterator
    //let lines = data_loader::read_lines(Path::new(scenario.file_path.as_str())).expect(&format!(
    //    "Expected {} to have readable lines.",
    let lines = data_loader::read_lines(Path::new(scenario.file_path.as_str()))
        .unwrap_or_else(|_| panic!("Expected {} to have readable lines.", scenario.file_path));

    // Only process the 'Ok()' items
    let mut itr = lines.map_while(Result::ok);
    (scenario.process_fn)(&mut itr);
}
