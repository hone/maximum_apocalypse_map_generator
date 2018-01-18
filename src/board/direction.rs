use std::slice::Iter;
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

    pub fn iter() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
        DIRECTIONS.iter()
    }
}

