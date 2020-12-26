use std::error::Error;

struct Config {
    filename: String,
    part: i32,
}

impl Config {
    fn new(args: Vec<String>) -> Result<Config, &'static str> {
        let mut args = args.into_iter();

        let filename = match args.next() {
            Some(arg) => {
                // Check for default arguments and short circuit if matched
                match arg.as_str() {
                    "--part1" => return Ok(Config {
                            filename: String::from("day2/data.txt"),
                            part: 1
                        }),
                    "--part2" => return Ok(Config {
                        filename: String::from("day2/data.txt"),
                            part: 2
                        }),
                    _ => arg,
                }
            },
            None => return Err("'Filename' parameter not supplied"),
        };

        let part = match args.next() {
            Some(arg) => arg,
            None => return Err("'Part' parameter not supplied"),
        }.parse::<i32>();

        let part = match part {
            Ok(p) => p,
            Err(_) => return Err("'Part' parameter must be an integer"),
        };

        return Ok(Config{ filename, part });
    }
}

pub fn run(config: common::Config) -> Result<(), Box<dyn Error>> {
    let config = Config::new(config.args)?;

    let passwords = read_passwords(&config)?;

    let count = match config.part {
        1 => count_valid_passwords(&passwords),
        2 => count_valid_passwords2(&passwords),
        _ => return Err("Invalid 'Part' parameter")?
    };
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

fn count_valid_passwords2(passwords: &Vec<Password>) -> i32 {
    let mut count = 0;
    for password in passwords {
        let mut value = password.value.chars();

        let min_letter = match value.nth(password.min - 1) {
            Some(c) => c,
            None => continue,
        };
        let max_letter = match value.nth(password.max - password.min - 1) {
            Some(c) => c,
            None => continue,
        };

        if min_letter == password.letter && max_letter != password.letter {
            count += 1;
        } else if min_letter != password.letter && max_letter == password.letter {
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
    use super::*;

    #[test]
    fn count_valid_passwords_should_be_valid() {
        // arrange
        let passwords = vec![
            Password::new(String::from("1-3 a: abcde")).unwrap()];

        // act
        let result = count_valid_passwords(&passwords);

        // assert
        assert_eq!(1, result);
    }

    #[test]
    fn count_valid_passwords_should_be_invalid_below_min() {
        // arrange
        let passwords = vec![
            Password::new(String::from("1-3 b: cdefg")).unwrap()];

        // act
        let result = count_valid_passwords(&passwords);

        // assert
        assert_eq!(0, result);
    }

    #[test]
    fn count_valid_passwords_should_be_invalid_above_max() {
        // arrange
        let passwords = vec![
            Password::new(String::from("2-8 c: ccccccccc")).unwrap()];

        // act
        let result = count_valid_passwords(&passwords);

        // assert
        assert_eq!(0, result);
    }

    #[test]
    fn count_valid_passwords2_should_be_valid() {
        // arrange
        let passwords = vec![
            Password::new(String::from("1-3 a: abcde")).unwrap()];

        // act
        let result = count_valid_passwords2(&passwords);

        // assert
        assert_eq!(1, result);
    }

    #[test]
    fn count_valid_passwords2_should_be_invalid_neither() {
        // arrange
        let passwords = vec![
            Password::new(String::from("1-3 b: cdefg")).unwrap()];

        // act
        let result = count_valid_passwords2(&passwords);

        // assert
        assert_eq!(0, result);
    }

    #[test]
    fn count_valid_passwords2_should_be_invalid_both() {
        // arrange
        let passwords = vec![
            Password::new(String::from("2-9 c: ccccccccc")).unwrap()];

        // act
        let result = count_valid_passwords2(&passwords);

        // assert
        assert_eq!(0, result);
    }
}
