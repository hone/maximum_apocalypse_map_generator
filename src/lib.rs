extern crate rand;

use std::slice::Iter;
use rand::Rng;

#[derive(Clone)]
struct Board {
    pub data: Vec<Option<Tile>>,
    pub num_tiles: usize,
    pub tile_count: usize,
    rng: rand::ThreadRng,
}

impl Board {
    pub fn new(num_tiles: &usize) -> Self {
        let data = vec![None; num_tiles * num_tiles]; 

        let mut board = Board {
            data: data,
            num_tiles: num_tiles.to_owned(),
            rng: rand::thread_rng(),
            tile_count: 0,
        };
        board.generate_board();
        // if not enough tiles, regenerate map
        while board.tile_count < board.num_tiles {
            board.tile_count = 0;
            board.data.clear();
            for _ in 0..(num_tiles * num_tiles) {
                board.data.push(None);
            }
            board.generate_board();
        }

        board
    }

    pub fn display(&self) {
        for y in (0..self.num_tiles).rev() {
            for x in 0..self.num_tiles {
                let index = x + y * self.num_tiles;
                let value = match self.data[index] {
                    Some(tile) => {
                        match tile.value {
                            Value::Van => 1,
                            Value::Normal => 2,
                        }
                    }
                    None => 0,
                };
                print!("{} ", value);
            }
            println!("");
        }
    }

    fn index(&self, position: &Position) -> usize {
        (position.x + position.y * self.num_tiles as isize) as usize
    }

    fn valid_position(&self, position: &Position) -> bool {
        let num_tiles_isize = self.num_tiles as isize;
        position.x >= 0 &&
            position.x < num_tiles_isize &&
            position.y >= 0 &&
            position.y < num_tiles_isize
    }

    fn generate_board(&mut self) {
        let starting_position = self.rng.gen_range(0, self.num_tiles);
        let starting_tile = Tile {
            value: Value::Van,
            position: Position {
                x: starting_position as isize,
                y: 0,
            }
        };
        self.add_tile(starting_tile);
        self.lay_tiles(&starting_tile);
    }

    fn add_tile(&mut self, tile: Tile) -> Option<Tile> {
        let index = self.index(&tile.position);

        if self.tile_count < self.num_tiles && self.data[index].is_none() {
            let result = Some(tile);
            self.data[index] = result;
            self.tile_count += 1;

            result
        } else {
            None
        }
    }


    fn lay_tiles(&mut self, tile: &Tile) {
        for direction in Direction::iter() {
            let new_tile_option = self.lay_tile_direction(&tile, direction);
            if new_tile_option.is_some() && self.tile_count < self.num_tiles {
                self.lay_tiles(&new_tile_option.unwrap());
            }
        }
    }

    fn lay_tile_direction(&mut self, tile: &Tile, direction: &Direction) -> Option<Tile> {
        if self.rng.gen_range(0, 100) > 33 {
            let new_position = tile.position.add(direction.value());
            let index = self.index(&new_position);
            if self.valid_position(&new_position) && self.data[index].is_none() {
                let new_tile = Tile {
                    value: Value::Normal,
                    position: new_position,
                };
                return self.add_tile(new_tile);
            }
        }

        None
    }
}

#[derive(Clone,Copy)]
struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    fn add(&self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn value(&self) -> Position {
        match *self {
            Direction::Up => Position { x: 0, y: 1 },
            Direction::Down => Position { x: 0, y: -1 },
            Direction::Left => Position { x: -1, y: 0 },
            Direction::Right => Position { x: 1, y: 0 },
        }
    }

    fn iter() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
        DIRECTIONS.iter()
    }
}

#[derive(Clone,Copy)]
enum Value {
    Van,
    Normal
}

#[derive(Clone,Copy)]
struct Tile {
    pub value: Value,
    pub position: Position,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let num_tiles = 23;
        let mut board = Board::new(&num_tiles);
        let found_tiles = board.data.iter().filter(|i| i.is_some() ).collect::<Vec<&Option<Tile>>>().len();
        println!("");
        board.display();

        assert_eq!(found_tiles, num_tiles);
    }
}
