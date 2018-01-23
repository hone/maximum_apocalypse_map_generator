use board::rand::{Rng, ThreadRng};
use self::super::position::Position;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn value(&self) -> Position {
        match *self {
            Direction::Up => Position { x: 0, y: 1 },
            Direction::Down => Position { x: 0, y: -1 },
            Direction::Left => Position { x: -1, y: 0 },
            Direction::Right => Position { x: 1, y: 0 },
        }
    }

    pub fn shuffled(rng: &mut ThreadRng) -> [Direction; 4] {
        let mut directions: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
        rng.shuffle(&mut directions);
        directions
    }
}

