#[derive(Clone,Copy)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn add(&self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
