use crate::cell::{Cell, self};

pub struct Board {
    cells: Vec<Cell>
}

impl Board {
    pub fn get<'a>(&self, pos: &[usize]) -> Option<Cell> {
        if let Cell::Board(board) = &self.cells[pos[0]] {
            board.get(&pos[1..])
        } else {
            return None;
        }
    }

    pub fn set<'a>(&self, pos: &[usize], value: Cell) -> Option<Cell> {
        if pos.len() > 1 {
            if let Cell::Board(board) = &self.cells[pos[0]] {
                board.set(&pos[1..], value)
            } else {
                return None;
            }
        } else {
            None
        }
    }
}