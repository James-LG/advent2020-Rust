pub struct Config {
    pub filename: String,
    pub part: i32,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
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