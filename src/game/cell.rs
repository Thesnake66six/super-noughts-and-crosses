use raylib::{drawing::RaylibDraw, math::Rectangle};
use serde::{Deserialize, Serialize};

use crate::{ common::*, styles::*, };

use super::{board::Board, game::Turn, legal::Legal, value::Value};

/// An enum used to differentiate the states of a cell.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Cell {
    /// An empty cell
    None,
    /// The first player
    Player1,
    /// The second player
    Player2,
    /// Another board
    Board(Board),
}

impl Cell {
    /// Returns the `Value` corresponding to a cell
    pub fn value(&self) -> Value {
        match self {
            Cell::None => Value::None,
            Cell::Player1 => Value::Player1,
            Cell::Player2 => Value::Player2,
            Cell::Board(b) => b.check(), // If the cell is a `Cell::Board`, return the value of the board instead
        }
    }

    /// Returns the possible moves within a cell
    pub fn moves<'a>(&'a self, pos: &'a [usize]) -> Vec<Vec<usize>> {
        match self {
            Cell::None => vec![pos.to_vec()],
            Cell::Player1 => vec![pos.to_vec()],
            Cell::Player2 => vec![pos.to_vec()],
            Cell::Board(b) => b.moves(pos),
        }
    }

    /// Returns the possible legal moves within a cell
    pub fn legal_moves<'a>(&'a self, pos: &'a [usize]) -> Vec<Vec<usize>> {
        match self {
            Cell::None => vec![pos.to_vec()],
            Cell::Player1 => vec![],
            Cell::Player2 => vec![],
            Cell::Board(b) => {
                if b.check() != Value::None {
                    vec![]
                } else {
                    b.legal_moves(pos)
                }
            }
        }
    }

    /// Draws the value onto `T`, inside the given `Rectangle`
    pub fn draw<T: RaylibDraw>(
        &self,
        rect: Rectangle,
        d: &mut T,
        no_check: bool,
        alpha: bool,
        mut hover: Option<&[usize]>,
        legal: Legal,
        turn: Turn,
    ) {
        let mut flag = false;
        if let Some(pos) = hover {
            if pos.is_empty() {
                flag = true;
                hover = None
            }
        }

        let mut greyed = true;
        if let Legal::Pos(x) = legal {
            if x.is_empty() {
                greyed = false
            }
        }

        let no_grey = legal == Legal::ForceDefaultBg;

        let mut board_completed = false;
        if let Cell::Board(x) = self {
            if x.check() != Value::None {
                board_completed = true;
            }
        }

        let draw_as_alpha = if let Cell::Board(_) = self {
            alpha
        } else {
            false
        };

        d.draw_rectangle_rec(
            rect,
            if draw_as_alpha {
                match self {
                    Cell::None => COLOUR_CELL_BG,
                    Cell::Player1 => COLOUR_CROSS_BGA,
                    Cell::Player2 => COLOUR_NOUGHT_BGA,
                    Cell::Board(_) => COLOUR_BOARD_BG,
                }
            } else if no_grey {
                match self {
                    Cell::None => COLOUR_CELL_BG,
                    Cell::Player1 => COLOUR_CROSS_BG,
                    Cell::Player2 => COLOUR_NOUGHT_BG,
                    Cell::Board(_) => COLOUR_BOARD_BG,
                }
            } else if greyed {
                if INVERT_GREYS && !board_completed {
                    get_greyed_colour_cell(turn)
                } else {
                    match self {
                        Cell::None => COLOUR_CELL_BG,
                        Cell::Player1 => COLOUR_CROSS_BG,
                        Cell::Player2 => COLOUR_NOUGHT_BG,
                        Cell::Board(_) => COLOUR_BOARD_BG,
                    }
                }
            } else if INVERT_GREYS || board_completed {
                match self {
                    Cell::None => COLOUR_CELL_BG,
                    Cell::Player1 => COLOUR_CROSS_BG,
                    Cell::Player2 => COLOUR_NOUGHT_BG,
                    Cell::Board(_) => COLOUR_BOARD_BG,
                }
            } else {
                get_greyed_colour_cell(turn)
            },
        );

        match self {
            Cell::None => {}
            Cell::Player1 => draw_cross(rect, d),
            Cell::Player2 => draw_nought(rect, d),
            Cell::Board(b) => {
                if let Value::None = b.check() {
                    b.draw(rect, d, no_check, alpha, hover, legal, turn) // Draw the board, if it is still playable...
                } else if no_check {
                    b.draw(rect, d, no_check, alpha, hover, legal, turn) // ...or if we're told not to check...
                } else {
                    b.draw(rect, d, no_check, alpha, hover, legal, turn);
                    b.check().draw(rect, d, alpha, legal, turn) // ...else draw the corresponding value
                }
            }
        }

        if flag {
            d.draw_rectangle_rec(rect, COLOUR_CELL_HOVER)
        }
    }
}
