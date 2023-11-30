use raylib::{drawing::RaylibDraw, math::Rectangle};
use anyhow::{Ok, Result, bail};

use crate::{board::Board, styles::{BOARD_CELL_MARGIN, COLOUR_CELL_BG}, cell::Cell};

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

    pub fn draw<T: RaylibDraw>(&self, rect: Rectangle, d: &mut T, no_check: bool, alpha: bool) {
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

        self.board.draw(irect, d, no_check, alpha)
    }

    pub fn play(&mut self, pos: &[usize]) -> Result<()>{
        let mut check_pos = pos;
        for c in &self.legal {
            if pos.starts_with(&[*c]) {
                check_pos = &check_pos[1..];
            } else {
                print!("h");
                bail!("Illegal move")
            }
        }
        
        if let Cell::None = &mut self.board.get(pos).unwrap() {
            let val = if self.turn == 1 { Cell::Player1 } else { Cell::Player2 };
            self.board.set(pos, val)?;
            
            
            
            self.turn = (self.turn + 1) % 2;
            Ok(())
        } else {
            print!("hh");
            bail!("Illegal move")
        }
    }

    pub fn get_legal(&self, pos: &[usize]) -> Vec<usize> {
        // Implement pls
    }
}
