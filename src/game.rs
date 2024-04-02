use std::ops::Not;

use anyhow::{bail, Ok, Result};
use monte_carlo::{Terminal, WonTerminal};
use raylib::{
    camera::Camera2D,
    drawing::{RaylibDraw, RaylibMode2DExt},
    math::Rectangle,
    prelude::Vector2,
};

use crate::{
    board::Board,
    cell::{Cell, Value},
    common::*,
    styles::*,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl monte_carlo::Turn for Turn {
    fn next(self) -> Self {
        !self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move(
    pub Vec<usize>
);




impl monte_carlo::Move<Game, Turn> for Move {
    fn play(&self, game: &mut Game, _turn: Turn) {
        game.play(self).unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    /// The rectange in which the board is rendered to the camera
    pub rect: Rectangle,
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
    pub moves: Vec<Vec<Move>>,
    /// The current set of legal moves
    pub legal: Move,
}

impl Game {
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
            legal: Move(vec![]),
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

    pub fn draw<T: RaylibDraw>(
        &self,
        rect: Rectangle,
        d: &mut T,
        no_check: bool,
        alpha: bool,
        hover: Option<&Move>,
    ) {
        let m = rect.width * BOARD_CELL_MARGIN;

        let mut c = d.begin_mode2D(self.camera);

        c.draw_rectangle_rec(
            rect,
            if self.board.check() != Value::None {
                match self.board.check() {
                    Value::None => panic!("How the fuck did you manage that"),
                    Value::Draw => COLOUR_BOARD_BG_GREYED,
                    Value::Player1 => COLOUR_BOARD_BG_GREYED_P1,
                    Value::Player2 => COLOUR_BOARD_BG_GREYED_P2,
                }
            } else if self.legal.0.is_empty() {
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

        let legal: Legal = if self.board.check() != Value::None || self.moves.is_empty() {
            Legal::ForceDefaultBg
        } else {
            Legal::Pos(&self.legal.0)
        };

        self.board
            .draw(irect, &mut c, no_check, alpha, hover, legal, self.turn)
    }

    pub fn play(&mut self, pos: &Move) -> Result<()> {
        if !pos.0.starts_with(&self.legal.0) {
            bail!("Illegal move: Move is not within bounds of current play")
        }

        if let Cell::Board(b) = self.board.get(&Move(pos.0[..pos.0.len().saturating_sub(2)].to_vec())).unwrap() {
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
                [Move(pos.0.to_vec()), Move(self.legal.0.clone())].to_vec(),
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

    pub fn unplay(&mut self) -> Result<()> {
        if self.moves.is_empty() {
            bail!("No move to unplay")
        }
        // (The move that was played, The legal at that time)
        let mv = self.moves.pop().unwrap();
        let x = &mv[1];
        let _ = self.board.set(&mv[0], Cell::None);
        self.legal = x.clone();
        self.turn = !self.turn;
        Ok(())
    }

    pub fn get_legal(&self, pos: &Move) -> Move {
        if self.board.check() != Value::None {
            return Move(vec![]);
        }

        let yo = Move(pos.0[..pos.0.len().saturating_sub(1)].to_vec());
        let zo = Move(pos.0[..pos.0.len().saturating_sub(2)].to_vec());
        let x = pos.0.last().unwrap(); // The last position in pos
        let y = if !pos.0.is_empty() {
            &yo
        } else {
            pos
        }; // The penultimate position in pos - correlates to the box that the play was made in
        let z = if pos.0.len() >= 2 {
            &zo
        } else {
            pos
        }; // The position two positions up, gives the depth-two board that the next move will always be in

        // Check to see if the move completed the board (up)
        if let Some(Cell::Board(b)) = self.board.get(y) {
            if b.check() != Value::None {
                // ...if so, check again as if the move made was `up`
                if pos.0.len() >= 3 {
                    return self.get_legal(y);
                }
            }
        }
        // Otherwise, check to make sure the new target board exists
        if let Some(Cell::Board(b)) = self.board.get(&Move([z.0.clone(), Move([*x].to_vec()).0].concat())) {
            // If it's completed, then return the board above (`last`)
            if b.check() != Value::None {
                z.clone()
            // Otherwise, return last, plus `n` to get the board referenced by the previous move
            } else {
                Move([z.0.clone(), [*x].to_vec()].concat())
            }
        // And, if the new target board doesn't exist, meaning that this is the top board, then return everywhere (`[]`).
        } else {
            Move([].to_vec())
        }
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        self.board.get(&self.legal).unwrap().moves(&self.legal)
    }

    pub fn get_cell_from_pixel(&self, point: Vector2, no_check: bool) -> Option<Move> {
        self.board.get_cell_from_pixel(point, no_check)
    }
}

impl monte_carlo::Board<Move, Turn> for Game {
    fn legal_moves(&self) -> Vec<Move> {
        self.legal_moves()
    }

    fn completion_state(&self) -> Option<monte_carlo::Terminal<Turn>> {
        match self.board.check() {
            Value::None => None,
            Value::Draw => Some(Terminal::Drawn),
            Value::Player1 => Some(Terminal::Won(WonTerminal::new(1.0, Turn::Player1))),
            Value::Player2 => Some(Terminal::Won(WonTerminal::new(1.0, Turn::Player2))),
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Game::new_depth(Rectangle::EMPTY, BOARD_DEFAULT_DEPTH, 2)
    }
}