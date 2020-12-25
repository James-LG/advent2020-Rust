use std::error::Error;

struct Config {
    filename: String,
}

impl Config {
    fn new(args: Vec<String>) -> Result<Config, &'static str> {
        let mut args = args.into_iter();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("'Filename' parameter not supplied"),
        };

        return Ok(Config{ filename });
    }
}

pub fn run(config: common::Config) -> Result<(), Box<dyn Error>> {
    let config = Config::new(config.args)?;

    let passwords = read_passwords(&config)?;

    let count = count_valid_passwords(&passwords);
    println!("Valid passwords {}", count);

    Ok(())
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

fn read_passwords(config: &Config) -> Result<Vec<Password>, Box<dyn Error>> {
    match common::read_lines(&config.filename) {
        Ok(lines) => {
            let mut passwords: Vec<Password> = Vec::new();
            
            for line in lines {
                passwords.push(Password::new(line?)?);
            }

            return Ok(passwords);
        },
        Err(err) => return Err(err)?,
    }
}

struct Password {
    min: usize,
    max: usize,
    letter: char,
    value: String,
}

impl Password {
    fn new(line: String) -> Result<Password, &'static str> {
        let space_split: Vec<&str> = line.split(' ').collect();

        if space_split.len() < 3 {
            return Err("Invalid input format");
        }

        let nums: Vec<&str> = space_split[0].split('-').collect();

        let min = nums[0].parse::<usize>();
        let max = nums[1].parse::<usize>();

        match (min, max) {
            (Ok(min), Ok(max)) => {
                let letter = match space_split[1].chars().nth(0) {
                    Some(c) => c,
                    None => return Err("Invalid character"),
                };

                let value = String::from(space_split[2]);

                Ok(Password { min, max, letter, value })
            }
            _ => Err("Missing min or max value")
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
