use std::error::Error;

use common::{self, Config};
use day1;
use day2;
use day3;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.puzzle_name.as_str() {
        "day1" => day1::run(config),
        "day2" => day2::run(config),
        "day3" => day3::run(config),
        _ => Err("Unknown puzzle name")?,
    }
}