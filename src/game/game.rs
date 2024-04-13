use std::ops::{Deref, Not};

use anyhow::{bail, Ok, Result};
use raylib::{
    camera::Camera2D,
    drawing::{RaylibDraw, RaylibMode2DExt},
    math::Rectangle,
    prelude::Vector2,
};

use serde::{Deserialize, Serialize};

use crate::{common::*, styles::*};

use super::{board::Board, cell::Cell, legal::Legal, value::Value};

pub struct Move(Vec<usize>);

impl Deref for Move {
    type Target = Vec<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Turn {
    Player1,
    Player2,
}

impl Turn {
    pub fn val(&self) -> Value {
        match self {
            Turn::Player1 => Value::Player1,
            Turn::Player2 => Value::Player2,
        }
    }
}

impl Not for Turn {
    type Output = Turn;
    fn not(self) -> Self::Output {
        match self {
            Turn::Player1 => Turn::Player2,
            Turn::Player2 => Turn::Player1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    /// The rectangle in which the board is rendered to the camera
    pub rect: Rectangle,
    #[serde(skip)]
    /// The camera
    pub camera: Camera2D,
    /// The top level board
    pub board: Board,
    /// The depth of the game
    pub depth: usize,
    /// The current turn - 1 for Crosses, 2 for Noughts
    pub turn: Turn,
    /// The number of human players
    pub players: usize,
    /// A list of all previous moves, and the legal moves that could have been made on that turn
    pub moves: Vec<Vec<Vec<usize>>>,
    /// The current set of legal moves
    pub legal: Vec<usize>,
}

impl Game {
    /// Constructs a new game
    pub fn new_depth(rect: Rectangle, depth: usize, players: usize) -> Self {
        Game {
            rect,
            camera: Camera2D {
                zoom: 1.0,
                ..Default::default()
            },
            board: Board::new_depth(depth),
            depth,
            turn: Turn::Player1,
            players,
            moves: [].into(),
            legal: vec![],
        }
    }

    /// Updates the positions of each cell
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

    /// Centres the game camera
    pub fn centre_camera(&mut self, rect: Rectangle) {
        self.camera.target = Vector2 {
            x: self.rect.x + self.rect.width / 2.0f32,
            y: self.rect.y + self.rect.height / 2.0f32,
        };
        self.camera.zoom = f32::min(
            rect.width / self.rect.width * CAMERA_DEFAULT_ZOOM,
            rect.height / self.rect.height * CAMERA_DEFAULT_ZOOM,
        );
    }

    /// Draws the game into the rectangle
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

        // Draws the background
        c.draw_rectangle_rec(
            rect,
            if self.board.check() != Value::None {
                match self.board.check() {
                    Value::None => panic!("How the fuck did you manage that"),
                    Value::Draw => COLOUR_BOARD_BG_GREYED,
                    Value::Player1 => COLOUR_BOARD_BG_GREYED_P1,
                    Value::Player2 => COLOUR_BOARD_BG_GREYED_P2,
                }
            } else if self.legal.is_empty() {
                COLOUR_BOARD_BG
            } else if self.turn == Turn::Player1 {
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

        // Draws the background for the board
        let legal: Legal = if self.board.check() != Value::None || self.moves.is_empty() {
            Legal::ForceDefaultBg
        } else {
            Legal::Pos(&self.legal)
        };

        // Draws the board
        self.board
            .draw(irect, &mut c, no_check, alpha, hover, legal, self.turn)
    }

    /// Makes a move
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
            let val = if self.turn == Turn::Player1 {
                Cell::Player1
            } else {
                Cell::Player2
            };
            self.board.set(pos, val)?;
            self.moves.insert(
                self.moves.len(),
                [pos.to_vec(), self.legal.to_vec()].to_vec(),
            );
            self.legal = self.get_legal(pos);
            self.turn = !self.turn;
            Ok(())
        } else if let Cell::Board(_) = &mut self.board.get(pos).unwrap() {
            bail!("Illegal move: That's a board")
        } else {
            bail!("Illegal move: Cell already filled")
        }
    }

    /// Removes the last played move
    pub fn unplay(&mut self) -> Result<()> {
        if self.moves.is_empty() {
            bail!("No move to unplay")
        }
        // (The move that was played, The legal at that time)
        let mv = self.moves.pop().unwrap();
        let x = &mv[1];
        let _ = self.board.set(&mv[0], Cell::None);
        self.legal = x.to_vec();
        self.turn = !self.turn;
        Ok(())
    }

    /// Gets the coordinate of the next legal move board
    pub fn get_legal(&self, pos: &[usize]) -> Vec<usize> {
        if self.board.check() != Value::None {
            return vec![];
        }

        let x = pos.last().unwrap(); // The last position in pos
        let y = if !pos.is_empty() {
            &pos[..pos.len().saturating_sub(1)]
        } else {
            pos
        }; // The penultimate position in pos - correlates to the box that the play was made in
        let z = if pos.len() >= 2 {
            &pos[..pos.len().saturating_sub(2)]
        } else {
            pos
        }; // The position two positions up, gives the depth-two board that the next move will always be in

        // Check to see if the move completed the board (up)
        if let Some(Cell::Board(b)) = self.board.get(y) {
            if b.check() != Value::None {
                // ...if so, check again as if the move made was `up`
                if pos.len() >= 3 {
                    return self.get_legal(y);
                }
            }
        }
        // Otherwise, check to make sure the new target board exists
        if let Some(Cell::Board(b)) = self.board.get(&[z, &[*x]].concat()) {
            // If it's completed, then return the board above (`last`)
            if b.check() != Value::None {
                z.to_vec()
            // Otherwise, return last, plus `n` to get the board referenced by the previous move
            } else {
                [z, &[*x]].concat()
            }
        // And, if the new target board doesn't exist, meaning that this is the top board, then return everywhere (`[]`).
        } else {
            [].to_vec()
        }
    }

    /// Returns a list of the legal moves
    pub fn legal_moves(&self) -> Vec<Vec<usize>> {
        self.board
            .get(&self.legal)
            .unwrap()
            .legal_moves(&self.legal)
    }

    /// Wrapper function
    pub fn get_cell_from_pixel(&self, point: Vector2, no_check: bool) -> Option<Vec<usize>> {
        self.board.get_cell_from_pixel(point, no_check)
    }
}
