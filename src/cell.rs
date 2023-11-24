use crate::board::Board;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Value {
    None,
    Draw, 
    Player1,
    Player2,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    None,
    Player1,
    Player2,
    Board(Board),
}

impl Cell {
    pub fn value(&self) -> Value{
        match self {
            Cell::None => Value::None,
            Cell::Player1 => Value::Player1,
            Cell::Player2 => Value::Player2,
            Cell::Board(b) => b.check(),
        }
    }
}