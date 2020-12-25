use std::error::Error;
use std::io;

use common;

pub struct Config {
    filename: String,
    sum: i32,
    num_variables: usize,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        let mut args = args.into_iter();

        let filename = match args.next() {
            Some(arg) => {
                // Check for default arguments and short circuit if matched
                match arg.as_str() {
                    "--part1" => return Ok(Config {
                            filename: String::from("day1/data.txt"),
                            sum: 2020,
                            num_variables: 2,
                        }),
                    "--part2" => return Ok(Config {
                        filename: String::from("day1/data.txt"),
                            sum: 2020,
                            num_variables: 3,
                        }),
                    _ => arg,
                }
            },
            None => return Err("'Filename' parameter not supplied"),
        };

        let sum = match args.next() {
            Some(arg) => arg,
            None => return Err("'Sum' parameter not supplied"),
        }.parse();

        let num_variables = match args.next() {
            Some(arg) => arg,
            None => return Err("'Number of variables' parameter not supplied"),
        }.parse();

        return match (sum, num_variables) {
            (Ok(sum), Ok(num_variables)) => Ok(Config { filename, sum, num_variables }),
            (Err(_), _) => Err("'Sum' parameter must be an integer"),
            (_, Err(_)) => Err("'Number of variables' parameter must be an integer"),
        }
    }
}

enum SumResult {
    TooSmall(Vec<i32>),
    TooBig(Vec<i32>),
    Equal(Vec<i32>),
    Error,
}

pub fn run(config: common::Config) -> Result<(), Box<dyn Error>> {
    let config = Config::new(config.args)?;
    let numbers = read_numbers(&config)?;

    match find_numbers(numbers, &config) {
        Ok(result) => {
            println!("Numbers: {:?} Sum: {} Product: {}", result.numbers, result.sum, result.product);
            Ok(())
        },
        Err(msg) => {
            println!("Error finding numbers: {}", msg);
            Err(msg)?
        },
    }
}

struct SearchResult {
    numbers: Vec<i32>,
    sum: i32,
    product: i32,
}

impl SearchResult {
    fn new(numbers: Vec<i32>) -> SearchResult {
        let sum: i32 = numbers.iter().sum();
        let product: i32 = numbers.iter().product();

        SearchResult {
            numbers,
            sum,
            product,
        }
    }
}

fn find_numbers(numbers: Vec<i32>, config: &Config) -> Result<SearchResult, &'static str> {
    let selected: Vec<i32> = Vec::new();
    return match find_numbers_internal(&numbers, selected, config) {
        SumResult::Equal(numbers) => Ok(SearchResult::new(numbers)),
        _ => Err("Could not find matching numbers"),
    }
}

fn find_numbers_internal(numbers: &Vec<i32>, selected: Vec<i32>, config: &Config) -> SumResult {
    let sum: i32 = selected.iter().sum();

    if selected.len() == config.num_variables {
        if sum == config.sum {
            return SumResult::Equal(selected.clone());
        } else if sum > config.sum {
            return SumResult::TooBig(selected.clone());
        } else {
            return SumResult::TooSmall(selected.clone());
        }
    }

    for x in numbers {
        if selected.contains(x) {
            continue;
        }

        if selected.len() == 0 {
            println!("First number of search: {}", x);
        }

        let mut new_selected = selected.clone();
        new_selected.push(*x);
        
        let internal_result = find_numbers_internal(numbers, new_selected, config);

        match internal_result {
            SumResult::TooBig(_) => break,
            SumResult::Equal(numbers) => return SumResult::Equal(numbers),
            result => result,
        };
    }

    return SumResult::Error;
}

fn read_numbers(config: &Config) -> io::Result<Vec<i32>> {
    let lines = common::read_lines(&config.filename)?;

    let mut numbers: Vec<i32> = Vec::new();
    for line in lines {
        if let Ok(line) = line {
            if let Ok(num) = line.parse() {
                numbers.push(num);
            }
        }
    }

    numbers.sort();
    return Ok(numbers);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_result_new_should_calculate() {
        // arrange
        let numbers = vec![1, 2, 3, 4];
        let numbers_clone = numbers.clone();

        // act
        let result = SearchResult::new(numbers);

        // assert
        assert_eq!(numbers_clone, result.numbers);
        assert_eq!(10, result.sum);
        assert_eq!(24, result.product);
    }

    #[test]
    fn find_numbers_should_find_two() {
        // arrange
        let args = vec![
            String::from("_"),
            String::from("5"),
            String::from("2")];

        let config = Config::new(args).unwrap();

        let numbers = vec![1, 2, 3, 5];

        // act
        let result = find_numbers(numbers, &config).unwrap();

        // assert
        assert_eq!(vec![2, 3], result.numbers);
        assert_eq!(5, result.sum);
    }

    #[test]
    fn find_numbers_should_find_three() {
        // arrange
        let args = vec![
            String::from("_"),
            String::from("9"),
            String::from("3")];

        let config = Config::new(args).unwrap();

        let numbers = vec![1, 2, 3, 4];

        // act
        let result = find_numbers(numbers, &config).unwrap();

        // assert
        assert_eq!(vec![2, 3, 4], result.numbers);
        assert_eq!(9, result.sum);
    }
}
