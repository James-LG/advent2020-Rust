use std::error::Error;

mod config;
use config::Config;

mod pattern;
use pattern::{Pattern, Tile};

pub fn run(config: common::Config) -> Result<(), Box<dyn Error>> {
    let config = Config::new(config.args)?;

    let pattern = read_pattern(&config)?;

    let result = find_blocking_trees(&pattern, &config)?;

    println!("Blocking trees {:?}", result);
    
    if result.len() > 1 {
        let multiplication: i64 = result.into_iter().map(|x| i64::from(x)).product();

        println!("Multiplication {}", multiplication);
    }
    
    Ok(())
}

fn read_pattern(config: &Config) -> Result<Pattern, Box<dyn Error>> {
    let lines = common::read_lines(&config.filename)?;
    let pattern = Pattern::new(lines)?;

    Ok(pattern)
}

fn find_blocking_trees(pattern: &Pattern, config: &Config) -> Result<Vec<i32>, &'static str> {
    let mut results: Vec<i32> = Vec::new();

    for slope in &config.slopes {
        let mut x = 0;
        let mut y = 0;
        let mut count = 0;
    
        while y < pattern.height {
            if let Tile::Tree = pattern.get_tile(x, y) {
                count += 1;
            }
    
            x += slope.x_move;
            y += slope.y_move;
        }

        results.push(count);
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use config::Slope;
    use super::*;

    #[test]
    fn find_blocking_trees_should_increment_on_tree() {
        // arrange
        let text = vec![
            String::from("#..."),
            String::from(".#.."),
            String::from("...."),
            String::from("...#"),
        ].into_iter().map(|t| Ok(t));

        let pattern = Pattern::new(text).unwrap();

        let config = Config{ 
            filename: String::from("_"),
            slopes: vec![Slope {x_move: 1, y_move: 1 }],
        };

        // act
        let result = find_blocking_trees(&pattern, &config).unwrap();

        // assert
        assert_eq!(3, result[0]);
    }

    #[test]
    fn find_blocking_trees_should_work_past_width() {
        // arrange
        let text = vec![
            String::from("#"),
            String::from("."),
            String::from("#"),
            String::from("."),
        ].into_iter().map(|t| Ok(t));

        let pattern = Pattern::new(text).unwrap();

        let config = Config{ 
            filename: String::from("_"),
            slopes: vec![Slope {x_move: 1, y_move: 1 }],
        };

        // act
        let result = find_blocking_trees(&pattern, &config).unwrap();

        // assert
        assert_eq!(2, result[0]);
    }

    #[test]
    fn find_blocking_trees_should_solve_multiple_slopes() {
        // arrange
        let text = vec![
            String::from("#..."),
            String::from(".#.."),
            String::from(".##."),
            String::from("...#"),
        ].into_iter().map(|t| Ok(t));

        let pattern = Pattern::new(text).unwrap();

        let config = Config{ 
            filename: String::from("_"),
            slopes: vec![
                Slope {x_move: 1, y_move: 1 },
                Slope {x_move: 1, y_move: 2 },
            ],
        };

        // act
        let result = find_blocking_trees(&pattern, &config).unwrap();

        // assert
        assert_eq!(4, result[0]);
        assert_eq!(2, result[1]);
    }
}
