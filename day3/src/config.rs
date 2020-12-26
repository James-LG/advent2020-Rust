pub struct Config {
    pub filename: String,
    pub slopes: Vec<Slope>,
}

pub struct Slope {
    pub x_move: usize,
    pub y_move: usize,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        let mut args = args.into_iter();

        let filename = match args.next() {
            Some(arg) => {
                // Check for default arguments and short circuit if matched
                match arg.as_str() {
                    "--part1" => return Ok(Config {
                            filename: String::from("day3/data.txt"),
                            slopes: vec![
                                Slope{ x_move: 3, y_move: 1},],
                        }),
                    "--part2" => return Ok(Config {
                            filename: String::from("day3/data.txt"),
                            slopes: vec![
                                Slope{ x_move: 1, y_move: 1},
                                Slope{ x_move: 3, y_move: 1},
                                Slope{ x_move: 5, y_move: 1},
                                Slope{ x_move: 7, y_move: 1},
                                Slope{ x_move: 1, y_move: 2},],
                        }),
                    _ => arg,
                }
            },
            None => return Err("'Filename' parameter not supplied"),
        };

        let mut slopes: Vec<Slope> = Vec::new();
        for arg in args {
            let pair: Vec<&str> = arg.split(',').collect();

            if pair.len() < 2 || pair.len() > 2 {
                return Err("Invalid slope pair, use format '{X},{Y}'");
            }
    
            let x_move: usize = match pair[0].parse() {
                Ok(p) => p,
                Err(_) => return Err("'X-Move' parameter must be an integer"),
            };
    
            let y_move: usize = match pair[1].parse() {
                Ok(p) => p,
                Err(_) => return Err("'Y-Move' parameter must be an integer"),
            };

            slopes.push(Slope { x_move, y_move });
        }

        return Ok(Config{ filename, slopes });
    }
}