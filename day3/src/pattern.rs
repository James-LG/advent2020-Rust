use std::error::Error;
use std::io;

pub enum Tile {
    Tree,
    Empty,
}

pub struct Pattern {
    rows: Vec<Vec<Tile>>,
    width: usize,
    pub height: usize,
}

impl Pattern {
    pub fn new<T>(text: T) -> Result<Pattern, Box<dyn Error>>
    where T : Iterator<Item = Result<String, io::Error>> {
        let mut rows: Vec<Vec<Tile>> = Vec::new();

        let mut width = 0;
        for line in text {
            match line {
                Ok(line) => {
                    let mut row: Vec<Tile> = Vec::new();
                    for c in line.chars() {
                        match c {
                            '#' => row.push(Tile::Tree),
                            '.' => row.push(Tile::Empty),
                            _ => return Err("Invalid character in pattern text".into()),
                        };
                    }

                    if width == 0 {
                        width = row.len();
                    } else if width != row.len() {
                        return Err("Pattern is not of a fixed width".into());
                    }

                    rows.push(row);
                },
                Err(err) => return Err(Box::new(err)),
            }
        }

        let height = rows.len();
        Ok(Pattern{ rows, width, height: height })
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        &self.rows[y][x % self.width]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_create_pattern() {
        // arrange
        let text = vec![
            String::from("#..."),
            String::from(".#.."),
            String::from("..#."),
            String::from("...#"),
        ].into_iter().map(|t| Ok(t));

        // act
        let pattern = Pattern::new(text).unwrap();

        // assert
        assert_eq!(4, pattern.width);
        assert!(matches!(pattern.get_tile(0, 0), Tile::Tree));
        assert!(matches!(pattern.get_tile(1, 0), Tile::Empty));
    }

    #[test]
    fn get_tile_should_repeat_pattern() {
        // arrange
        let text = vec![
            String::from("#..."),
            String::from(".#.."),
            String::from("..#."),
            String::from("...#"),
        ].into_iter().map(|t| Ok(t));

        // act
        let pattern = Pattern::new(text).unwrap();

        // assert
        assert_eq!(4, pattern.width);
        assert!(matches!(pattern.get_tile(4, 0), Tile::Tree));
        assert!(matches!(pattern.get_tile(4, 1), Tile::Empty));

        assert!(matches!(pattern.get_tile(8, 0), Tile::Tree));
        assert!(matches!(pattern.get_tile(8, 1), Tile::Empty));
    }
}
