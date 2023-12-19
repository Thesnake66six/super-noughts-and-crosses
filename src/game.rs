use anyhow::{bail, Ok, Result};
use raylib::{
    camera::Camera2D,
    drawing::{RaylibDraw, RaylibMode2DExt},
    math::Rectangle,
    prelude::Vector2,
};

use crate::{
    board::Board,
    cell::{Cell, Value},
    styles::{BOARD_CELL_MARGIN, BOARD_DEPTH, COLOUR_CELL_BG, USE_OLD_RENDERER},
};

pub struct Game {
    pub rect: Rectangle,
    pub camera: Camera2D,
    pub board: Board,
    pub turn: u8,
    pub moves: Vec<Vec<usize>>,
    pub legal: Vec<usize>,
}

impl Game {
    pub fn new_depth(rect: Rectangle, depth: usize) -> Self {
        Game {
            rect: rect,
            camera: Camera2D {
                zoom: 1.0,
                ..Default::default()
            },
            board: Board::new_depth(depth),
            turn: 1,
            moves: [].into(),
            legal: vec![],
        }
    }

    pub fn update_positions(&mut self) {
        let m = self.rect.width * BOARD_CELL_MARGIN;

        let irect = Rectangle {
            x: self.rect.x + m,
            y: self.rect.x + m,
            width: self.rect.width - 2.0 * m,
            height: self.rect.height - 2.0 * m,
        };

        self.board.update_positions(irect)
    }

    pub fn draw<T: RaylibDraw>(
        &self,
        rect: Rectangle,
        d: &mut T,
        no_check: bool,
        alpha: bool,
        hover: Option<&[usize]>,
    ) {
        let m = rect.width * BOARD_CELL_MARGIN;

        let mut c = d.begin_mode2D(self.camera);

        c.draw_rectangle_rec(rect, COLOUR_CELL_BG);

        let irect = Rectangle {
            x: rect.x + m,
            y: rect.x + m,
            width: rect.width - 2.0 * m,
            height: rect.height - 2.0 * m,
        };

        if USE_OLD_RENDERER {
            self.board.draw_old(irect, &mut c, no_check, alpha)
        } else {
            self.board.draw(irect, &mut c, no_check, alpha, hover)
        }
    }

    pub fn play(&mut self, pos: &[usize]) -> Result<()> {
        let mut check_pos = pos;

            if !pos.starts_with(&self.legal) {
                bail!("Illegal move: Move is not within bounds of current play")
            }


        if let Cell::Board(b) = self.board.get(&pos[..pos.len().saturating_sub(2)]).unwrap() {
            if b.check() != Value::None {
                bail!("Illegal move: Board already completed")
            }
        }

        if let Cell::None = &mut self.board.get(pos).unwrap() {
            // Play the move
            let val = if self.turn == 1 {
                Cell::Player1
            } else {
                Cell::Player2
            };
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
        let n = pos.last().unwrap(); // The last position in pos
        let up = if pos.len() >= 2 { &pos[..pos.len().saturating_sub(1)] } else { pos }; // The penultimate position in pos - correlates to the box that the play was made in
        let last = if pos.len() >= 2 { &pos[..pos.len().saturating_sub(2)] } else { pos }; // The position two positions up, gives the depth-two board that the next move will always be in

        // Check to see if the move completed the board (up)
        if let Some(Cell::Board(b)) = self.board.get(up) {
            if b.check() != Value::None {
                // ...if so, check again as if the move made was `up`
                return self.get_legal(up);
            }
        }
        // Otherwise, check to make sure `up` exists
        if let Some(Cell::Board(b)) = self.board.get(&[last, &[*n]].concat()){
            // If it's completed, then return the board above (`last`)
            if b.check() != Value::None {
                return last.to_vec()
            // Otherwise, return last, plus `n` to get the board referenced by the previous move
            } else {
                return [last, &[*n]].concat();
            }
        // And, if `up` doesn't exist, return the targetted board as a failsafe (though, this shouldn't happen sp )
        } else {
            eprintln!("Didn't know this could happen, please check circumstances");
            return [last, &[*n]].concat();
        }


        // if pos.len() == 0 {

        // }

        // // Check to see if the move completed the board, ...
        // if let Some(Cell::Board(b)) = self.board.get(&pos[..pos.len().saturating_sub(2)]) {
        //     if b.check() != Value::None {
        //         // ...if so, re-target to a higher board
        //         return self.get_legal(&pos[..pos.len().saturating_sub(2)]);
        //     }
        // }

        // let n = pos[pos.len() - 1];

        // if let Some(Cell::Board(b)) = self
        //     .board
        //     .get(&[&pos[..pos.len().saturating_sub(2)], &[n]].concat())
        // {
        //     if b.check() != Value::None {
        //         // ...if so, re-target to a higher board
        //         return pos[..pos.len().saturating_sub(2)].to_vec();
        //     } else {
        //         return [&pos[..pos.len().saturating_sub(2)], &[n]]
        //             .concat()
        //             .to_vec();
        //     }
        // }

        // return pos[..pos.len().saturating_sub(2)].to_vec();
    }

    pub fn get_cell_from_pos(&self, point: Vector2, no_check: bool) -> Option<Vec<usize>> {
        self.board.get_cell_from_pos(point, no_check)
    }
}
