use std::fs::File;
use std::error::Error;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Config {
    pub filename: String,
    pub sum: i32,
    pub variables: usize,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("Not enough arguments");
        }

        let filename = args[1].clone();
        let sum_result = args[2].parse();
        let variables_result = args[3].parse();

        return match (sum_result, variables_result) {
            (Ok(sum), Ok(variables)) => Ok(Config { filename, sum, variables }),
            (Err(_), _) => Err("Sum must be an integer"),
            (_, Err(_)) => Err("Variables must be an integer"),
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let numbers = read_numbers(&config)?;

    match find_numbers_for(&numbers, &config) {
        Ok(numbers) => {
            let sum: i32 = numbers.iter().sum();
            let mut product = 1;
            for num in &numbers {
                product *= num;
            }
            println!("{:?} {} {}", numbers, sum, product);
        },
        Err(msg) => println!("Error finding numbers: {}", msg),
    };

    Ok(())
}

fn find_numbers_for(numbers: &Vec<i32>, config: &Config) -> Result<Vec<i32>, &'static str> {
    let selected: Vec<i32> = Vec::new();
    return match find_numbers_internal(numbers, &selected, config) {
        SumResult::Equal(numbers) => Ok(numbers),
        _ => Err("Could not find matching numbers"),
    }
}

fn find_numbers_internal(numbers: &Vec<i32>, selected: &Vec<i32>, config: &Config) -> SumResult {
    let sum: i32 = selected.iter().sum();

    if selected.len() == config.variables {
        if sum == config.sum {
            return SumResult::Equal(selected.clone());
        } else if sum > config.sum {
            return SumResult::TooBig(selected.clone());
        } else {
            return SumResult::TooSmall;
        }
    }

    for x in numbers {
        if selected.len() == 0 {
            println!("First number of search: {}", x);
        }

        let mut new_selected = selected.clone();
        new_selected.push(*x);
        
        let internal_result = find_numbers_internal(numbers, &new_selected, config);
        match internal_result {
            SumResult::TooBig(_) => break,
            SumResult::Equal(numbers) => return SumResult::Equal(numbers),
            SumResult::TooSmall => (),
        }
    }

    return SumResult::TooSmall;
}

enum SumResult {
    TooSmall,
    TooBig(Vec<i32>),
    Equal(Vec<i32>),
}

fn read_numbers(config: &Config) -> io::Result<Vec<i32>> {
    let lines = read_lines(&config.filename)?;

    let mut numbers: Vec<i32> = Vec::new();
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        if let Ok(ip) = line {
            if let Ok(num) = ip.parse() {
                numbers.push(num);
            }
        }
    }

    numbers.sort();
    return Ok(numbers);
}

fn read_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}