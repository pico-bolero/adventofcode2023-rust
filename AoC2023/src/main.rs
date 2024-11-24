pub mod io;
use crate::io::data_loader;
use std::path::Path;

fn main() {
    let path = Path::new("./data/day01.txt");
    match data_loader::read_lines(path) {
        Ok(lines) => {
            lines.for_each(|line| match line {
                Ok(x) => {
                    print!("{}\n", x)
                }
                Err(_) => std::process::exit(1),
            });
        }
        Err(_) => {
            std::process::exit(1);
        }
    }
}
