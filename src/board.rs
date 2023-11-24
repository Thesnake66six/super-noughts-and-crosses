use anyhow::{Error, Ok};

use crate::cell::{Cell, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub cells: Vec<Cell>
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: vec![Cell::None; 9]
        }
    }

    pub fn new_depth(depth: usize) -> Self {
        if depth > 1 {
            Board {
                cells: vec![Cell::Board(Board::new_depth(depth - 1)); 9],
            }
        } else {
            Board::new()
        }
    }

    pub fn get(&self, pos: &[usize]) -> Option<Cell> {
        if let Cell::Board(board) = &self.cells[pos[0]] {
            board.get(&pos[1..])
        } else {
            return None;
        }
    }

    pub fn set(&mut self, pos: &[usize], value: Cell) -> Result<(), Error> {
        if pos.len() > 1 {
            if let Cell::Board(x) = &mut self.cells[pos[0]] {
                return x.set(&pos[1..], value);
            } else {
                panic!()
            }
        } else {
            self.cells[pos[0]] = value;
            Ok(())
        }
    }

    pub fn check(&self) -> Value {
        let vals = self.cells.iter().map(|cell| cell.value()).collect::<Vec<Value>>();
        let sets = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];

        for set in sets {
            if vals[set[0]] == vals[set[1]] && vals[set[1]] == vals[set[2]] {
                if [Value::Player1, Value::Player2].contains(&vals[set[0]]) {
                    return vals[set[0]];
                }
            }
        }

        if !vals.contains(&Value::None) {
            return Value::Draw;
        }

        Value::None
    }
}