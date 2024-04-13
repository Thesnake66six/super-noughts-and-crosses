use raylib::{drawing::RaylibDraw, math::Rectangle};
use serde::{Deserialize, Serialize};

use crate::{common::get_greyed_colour_cell, styles::{COLOUR_BOARD_BG, COLOUR_CELL_BG, COLOUR_CELL_HOVER, INVERT_GREYS}};

use super::{board::Board, game::Turn, legal::Legal, player::{self, Player}, value::Value};

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
        player_1: &Player,
        player_2: &Player,
    ) {
        let mut flag = false;
        if let Some(pos) = hover {
            if pos.is_empty() {
                flag = true;
                hover = None;
            }
        }

        let mut greyed = true;
        if let Legal::Pos(x) = legal {
            if x.is_empty() {
                greyed = false;
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
                    Cell::Player1 => player_1.background_alpha,
                    Cell::Player2 => player_2.background_alpha,
                    Cell::Board(_) => COLOUR_BOARD_BG,
                }
            } else if no_grey {
                match self {
                    Cell::None => COLOUR_CELL_BG,
                    Cell::Player1 => player_1.background,
                    Cell::Player2 => player_2.background,
                    Cell::Board(_) => COLOUR_BOARD_BG,
                }
            } else if greyed {
                if INVERT_GREYS && !board_completed {
                    get_greyed_colour_cell(turn, player_1, player_2)
                } else {
                    match self {
                        Cell::None => COLOUR_CELL_BG,
                        Cell::Player1 => player_1.background,
                        Cell::Player2 => player_2.background,
                        Cell::Board(_) => COLOUR_BOARD_BG,
                    }
                }
            } else if INVERT_GREYS || board_completed {
                match self {
                    Cell::None => COLOUR_CELL_BG,
                    Cell::Player1 => player_1.background,
                    Cell::Player2 => player_2.background,
                    Cell::Board(_) => COLOUR_BOARD_BG,
                }
            } else {
                get_greyed_colour_cell(turn, player_1, player_2)
            },
        );

        match self {
            Cell::None => {}
            Cell::Player1 => player_1.symbol.draw(player_1, rect, d), 
            Cell::Player2 => player_2.symbol.draw(player_2, rect, d),
            Cell::Board(b) => {
                if let Value::None = b.check() {
                    b.draw(rect, d, no_check, alpha, hover, legal, turn, player_1, player_2); // Draw the board, if it is still playable...
                } else if no_check {
                    b.draw(rect, d, no_check, alpha, hover, legal, turn, player_1, player_2); // ...or if we're told not to check...
                } else {
                    b.draw(rect, d, no_check, alpha, hover, legal, turn, player_1, player_2);
                    b.check().draw(rect, d, alpha, legal, turn, player_1, player_2); // ...else draw the corresponding value
                }
            }
        }

        if flag {
            d.draw_rectangle_rec(rect, COLOUR_CELL_HOVER);
        }
    }
}
