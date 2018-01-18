use self::super::position::Position;

#[derive(Clone,Copy)]
pub struct Tile {
    pub value: Value,
    pub position: Position,
}

#[derive(Clone,Copy)]
pub enum Value {
    Van,
    Normal
}
