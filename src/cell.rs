use crate::board::Board;

pub enum Cell {
    None,
    Player1,
    Player2,
    Board(Board),
}

pub enum Value {
    None,
    Draw, 
    Player1,
    Player2,
}