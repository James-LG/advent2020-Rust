use std::env;
use std::fs::File;
use std::process;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("In file {}", config.filename);

    let passwords = read_passwords(&config).unwrap_or_else(|err| {
        println!("Problem reading lines: {}", err);
        process::exit(1);
    });

    let count = count_valid_passwords(&passwords);
    println!("Valid passwords {}", count);
}

fn count_valid_passwords(passwords: &Vec<Password>) -> i32 {
    let mut count = 0;
    for password in passwords {
        let letter_count = password.value.matches(password.letter).count();

        if letter_count >= password.min && letter_count <= password.max {
            count += 1;
        }
    }
    return count;
}

fn read_passwords(config: &Config) -> Result<Vec<Password>, String> {
    match read_lines(config.filename.clone()) {
        Ok(lines) => {
            let mut passwords: Vec<Password> = Vec::new();
            
            for line in lines {
                match line {
                    Ok(line_string) => {
                        match Password::new(&line_string) {
                            Ok(password) => passwords.push(password),
                            Err(err) => return Err(String::from(err)),
                        }
                    },
                    Err(err) => return Err(format!("Error reading file {:?}", err.kind())),
                }
            }

            return Ok(passwords);
        },
        Err(err) => return Err(format!("Error reading file {:?}", err.kind())),
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Password {
    min: usize,
    max: usize,
    letter: char,
    value: String,
}

impl Password {
    fn new(line: &String) -> Result<Password, &'static str> {
        let space_split: Vec<&str> = line.split(' ').collect();

        if space_split.len() < 3 {
            return Err("Invalid input format");
        }

        let nums: Vec<&str> = space_split[0].split('-').collect();
        if let Ok(min) = nums[0].parse() {
            if let Ok(max) = nums[1].parse() {
                let letter = match space_split[1].chars().nth(0) {
                    Some(c) => c,
                    None => return Err("Invalid character"),
                };
                
                let value = String::from(space_split[2]);

                return Ok(Password { min, max, letter, value });
            } else {
                return Err("Missing min or max number"); 
            }
        } else {
            return Err("Missing min or max number");
        }

    }
}

struct Config {
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let filename = args[1].clone();

        return Ok(Config{ filename });
    }
}