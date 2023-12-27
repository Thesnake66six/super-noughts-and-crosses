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
    styles::{
        BOARD_CELL_MARGIN, COLOUR_BOARD_BG, COLOUR_BOARD_BG_GREYED_P1, COLOUR_BOARD_BG_GREYED_P2,
    },
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
            rect,
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

        c.draw_rectangle_rec(
            rect,
            if self.moves.is_empty() {
                COLOUR_BOARD_BG
            } else if self.turn == 1 {
                COLOUR_BOARD_BG_GREYED_P1
            } else {
                COLOUR_BOARD_BG_GREYED_P2
            },
        );

        let irect = Rectangle {
            x: rect.x + m,
            y: rect.x + m,
            width: rect.width - 2.0 * m,
            height: rect.height - 2.0 * m,
        };

        let legal: Option<&[usize]> = if self.board.check() != Value::None {
            Some(&[13])
        } else {
            Some(&self.legal)
        };

        self.board
            .draw(irect, &mut c, no_check, alpha, hover, legal, self.turn)
    }

    pub fn play(&mut self, pos: &[usize]) -> Result<()> {
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
            dbg!(&self.legal);
            self.turn = (self.turn + 1) % 2;
            self.moves.insert(self.moves.len(), pos.to_vec());
            Ok(())
        } else {
            println!("hh");
            bail!("Illegal move: Cell already filled")
        }
    }

    pub fn get_legal(&self, pos: &[usize]) -> Vec<usize> {
        if self.board.check() != Value::None {
            return vec![];
        }

        let n = pos.last().unwrap(); // The last position in pos
        let up = if !pos.is_empty() {
            &pos[..pos.len().saturating_sub(1)]
        } else {
            pos
        }; // The penultimate position in pos - correlates to the box that the play was made in
        let last = if pos.len() >= 2 {
            &pos[..pos.len().saturating_sub(2)]
        } else {
            pos
        }; // The position two positions up, gives the depth-two board that the next move will always be in

        // Check to see if the move completed the board (up)
        if let Some(Cell::Board(b)) = self.board.get(up) {
            if b.check() != Value::None {
                // ...if so, check again as if the move made was `up`
                if pos.len() >= 3 {
                    return self.get_legal(up);
                }
            }
        }
        // Otherwise, check to make sure `up` exists
        if let Some(Cell::Board(b)) = self.board.get(&[last, &[*n]].concat()) {
            // If it's completed, then return the board above (`last`)
            if b.check() != Value::None {
                last.to_vec()
            // Otherwise, return last, plus `n` to get the board referenced by the previous move
            } else {
                [last, &[*n]].concat()
            }
        // And, if `up` doesn't exist, meaning that this is the top board, then return everywhere (`[]`).
        } else {
            [].to_vec()
        }
    }

    pub fn get_cell_from_pos(&self, point: Vector2, no_check: bool) -> Option<Vec<usize>> {
        self.board.get_cell_from_pos(point, no_check)
    }
}
