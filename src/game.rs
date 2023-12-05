use raylib::{drawing::RaylibDraw, math::Rectangle, prelude::Vector2};
use anyhow::{Ok, Result, bail};

use crate::{board::Board, styles::{BOARD_CELL_MARGIN, COLOUR_CELL_BG, USE_OLD_RENDERER}, cell::{Cell, Value}};

pub struct Game {
    pub board: Board,
    pub turn: u8,
    pub moves: Vec<Vec<usize>>,
    legal: Vec<usize>,
}

impl Game {
    pub fn new_depth(depth: usize) -> Self {
        Game { 
            board: Board::new_depth(depth), 
            turn: 1, 
            moves: [].into(),
            legal: vec![],
        }
    }

    pub fn draw<T: RaylibDraw>(&self, rect: Rectangle, d: &mut T, no_check: bool, alpha: bool, hover: Option<&[usize]>) {
        let m = rect.width * BOARD_CELL_MARGIN;

        d.draw_rectangle(
            rect.x as i32,
            rect.y as i32,
            rect.width as i32,
            rect.height as i32,
            COLOUR_CELL_BG,
        );

        let irect = Rectangle {
            x: rect.x + m,
            y: rect.x + m,
            width: rect.width - 2.0 * m,
            height: rect.height - 2.0 * m,
        }; 

        if USE_OLD_RENDERER {
            self.board.draw_old(irect, d, no_check, alpha)
        } else {
            self.board.draw(irect, d, no_check, alpha, hover)
        }

    }

    pub fn play(&mut self, pos: &[usize]) -> Result<()>{
        let mut check_pos = pos;
        for c in &self.legal {
            if pos.starts_with(&[*c]) {
                check_pos = &check_pos[1..];
            } else {
                bail!("Illegal move: Move is not within bounds of current play")
            }
        }

        if let Cell::Board(b) = self.board.get(&pos[..pos.len().saturating_sub(2)]).unwrap() {
            if b.check() != Value::None {
                bail!("Illegal move: Board already completed")
            }
        }

        if let Cell::None = &mut self.board.get(pos).unwrap() {

            // Play the move
            let val = if self.turn == 1 { Cell::Player1 } else { Cell::Player2 };
            self.board.set(pos, val)?;
            self.legal = self.get_legal(pos);
            self.turn = (self.turn + 1) % 2;
            Ok(())
        } else {
            print!("hh");
            bail!("Illegal move: Cell already filled")
        }
    }

    pub fn get_legal(&self, pos: &[usize]) -> Vec<usize> {
        if pos.len() == 0 {
            return vec![];
        }

        // Check to see if the move completed the board, ...
        if let Some(Cell::Board(b)) = self.board.get(&pos[..pos.len().saturating_sub(2)]) {
            if b.check() != Value::None {
                // ...if so, re-target to a higher board
                return self.get_legal(&pos[..pos.len().saturating_sub(2)]);
            }
        }
        
        let n = pos[pos.len() - 1];
        
        if let Some(Cell::Board(b)) = self.board.get(&[&pos[..pos.len().saturating_sub(3)], &[n]].concat()) {
            if b.check() != Value::None {
                // ...if so, re-target to a higher board
                return pos[..pos.len().saturating_sub(3)].to_vec();
            } else {
                return [&pos[..pos.len().saturating_sub(3)], &[n]].concat().to_vec();
            }
        }

        return pos[..pos.len().saturating_sub(2)].to_vec();
    }

    pub fn get_cell_from_pos(&self, point: Vector2, no_check: bool) -> Option<Vec<usize>> {
        self.board.get_cell_from_pos(point, no_check)
    }
    
}
