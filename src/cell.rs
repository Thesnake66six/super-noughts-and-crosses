use raylib::{drawing::RaylibDraw, math::Rectangle};

use crate::{board::Board, common::*, game::{Move, Turn}, styles::*};

#[derive(Debug, Copy, Clone, PartialEq)]
/// An enum used to differentiate the states of a board, namely:
/// `Player1`: The first player has won;
/// `Player2`: The second player has won;
/// `Draw`: The board has drawn;
/// `None`: The board may still be played into.
pub enum Value {
    None,
    Draw,
    Player1,
    Player2,
}

impl Value {
    /// Draws the value onto `T`, inside the given `Rectangle`
    pub fn draw<T: RaylibDraw>(
        &self,
        rect: Rectangle,
        d: &mut T,
        alpha: bool,
        legal: Legal,
        turn: Turn,
    ) {
        let mut greyed = true;
        if let Legal::Pos(x) = legal {
            if x.is_empty() {
                greyed = false
            }
        }

        // Draw background for the cell
        d.draw_rectangle_rec(
            rect,
            if alpha {
                match self {
                    Value::None => COLOUR_CELL_BG,
                    Value::Player1 => COLOUR_CROSS_BGA,
                    Value::Player2 => COLOUR_NOUGHT_BGA,
                    Value::Draw => COLOUR_DRAW_BGA,
                }
            } else if greyed {
                get_greyed_colour_cell(turn)
            } else {
                match self {
                    Value::None => COLOUR_CELL_BG,
                    Value::Player1 => COLOUR_CROSS_BG,
                    Value::Player2 => COLOUR_NOUGHT_BG,
                    Value::Draw => COLOUR_DRAW_BG,
                }
            },
        );

        // Draw the symbol atop the background
        match self {
            Value::None => {}
            Value::Draw => draw_draw(rect, d),
            Value::Player1 => draw_cross(rect, d),
            Value::Player2 => draw_nought(rect, d),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// An enum used to differentiate the states of a cell, namely:
/// `None`: An empty cell;
/// `Player1`: The first player;
/// `Player2`: The second player;
/// `Board(Board)`: Another board.
pub enum Cell {
    None,
    Player1,
    Player2,
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
    pub fn moves<'a>(&'a self, pos: &'a Move) -> Vec<Move> {
        match self {
            Cell::None => vec![pos.clone()],
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
        mut hover: Option<&Move>,
        legal: Legal,
        turn: Turn,
    ) {
        let mut flag = false;
        if let Some(pos) = hover {
            if pos.0.is_empty() {
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
