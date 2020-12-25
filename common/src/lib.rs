use std::fs::File;
use std::io::{self, BufRead, Lines, BufReader};
use std::path::Path;

pub struct Config {
    pub puzzle_name: String,
    pub args: Vec<String>,
}

impl Config {
    pub fn new<T>(mut args: T) -> Result<Config, &'static str>
    where T : Iterator<Item = String> {
        args.next(); // skip program name

        let puzzle_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Must supply a puzzle name"),
        };

        let args: Vec<String> = args.collect();

        Ok(Config {
            puzzle_name,
            args,
        })
    }
}

pub fn read_lines<P>(filename: &P) -> io::Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Must supply a puzzle name")]
    fn new_should_return_err_for_missing_args() {
        // arrange
        let args: Vec<String> = vec![];
        let args = args.into_iter();

        // act
        // assert
        Config::new(args).unwrap();
    }

    #[test]
    fn new_should_get_puzzle_name() {
        // arrange
        let args: Vec<String> = vec![String::from("_"), String::from("puzzle")];
        let args = args.into_iter();

        // act
        let config = Config::new(args).unwrap();

        // assert
        assert_eq!("puzzle", config.puzzle_name);
    }

    #[test]
    fn new_should_collect_remaining_args() {
        // arrange
        let args: Vec<String> = vec![String::from("_"), String::from("_"), String::from("arg1"), String::from("arg2")];
        let args = args.into_iter();

        // act
        let config = Config::new(args).unwrap();

        // assert
        assert_eq!("arg1", config.args[0]);
        assert_eq!("arg2", config.args[1]);
    }
}
