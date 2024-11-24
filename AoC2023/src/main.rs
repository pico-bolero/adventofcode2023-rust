pub mod core;
pub mod io;
use crate::core::day01;
use crate::io::data_loader;
use std::env;
use std::path::Path;

struct ScenarioConfig {
    file_path: String,
    process_fn: fn(&str) -> i32,
}

// Examines the command line arguments and passes back
// the input file to load.
fn select_scenario() -> ScenarioConfig {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Expected 1 argument like 'day01'");
    }
    match args[1].as_str() {
        "day01" => ScenarioConfig {
            file_path: "./data/day01.txt".to_string(),
            process_fn: day01::combine_first_and_last_digit,
        },
        _ => {
            panic!("Expected argument like 'day01' and not {}", &args[1]);
        }
    }
}

fn main() {
    // Parse the input arguments
    let scenario = select_scenario();

    // Get the lines iterator
    let lines = data_loader::read_lines(Path::new(scenario.file_path.as_str())).expect(&format!(
        "Expected {} to have readable lines.",
        scenario.file_path
    ));

    // Do something with each of the lines
    let total: i32 = lines
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .map(|x| (scenario.process_fn)(x.as_str()))
        .sum();
    print!("Sum {}\n", total);
}
