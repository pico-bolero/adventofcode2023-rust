use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

/// Returns an iterator of each line from the provided file name.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
