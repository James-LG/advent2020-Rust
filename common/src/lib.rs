use std::fs::File;
use std::io::{self, BufRead, Lines, BufReader};
use std::path::Path;

mod config;

pub use config::Config;

pub fn read_lines<P>(filename: &P) -> io::Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
