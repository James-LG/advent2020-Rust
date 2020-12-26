pub struct Config {
    pub filename: String,
    pub sum: i32,
    pub num_variables: usize,
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