use raylib::{drawing::RaylibDraw, math::Rectangle, color::Color};

use crate::{
    board::Board,
    styles::{draw_cross, draw_draw, draw_none, draw_nought, draw_draw_alpha, draw_nought_alpha, draw_cross_alpha},
};

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
    pub fn draw<T: RaylibDraw>(&self, rect: Rectangle, d: &mut T, alpha: bool) {
        if alpha {
            match self {
                Value::None => draw_none(rect, d),
                Value::Player1 => draw_cross_alpha(rect, d),
                Value::Player2 => draw_nought_alpha(rect, d),
                Value::Draw => draw_draw_alpha(rect, d),
            }
        } else {
            match self {
                Value::None => draw_none(rect, d),
                Value::Player1 => draw_cross(rect, d),
                Value::Player2 => draw_nought(rect, d),
                Value::Draw => draw_draw(rect, d),
            }
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

    /// An alternate draw function
    pub fn draw_old<T: RaylibDraw>(&self, rect: Rectangle, d: &mut T, no_check: bool, alpha: bool) {
        match self {
            Cell::None => draw_none(rect, d),
            Cell::Player1 => draw_cross(rect, d),
            Cell::Player2 => draw_nought(rect, d),
            Cell::Board(b) => {
                if let Value::None = b.check() {
                    b.draw_old(rect, d, no_check, alpha) // Draw the board, if it is still playable...
                } else {
                    if no_check {
                        b.draw_old(rect, d, no_check, alpha) // ...or if we're told not to check...
                    } else {
                        b.check().draw(rect, d, alpha) // ...else draw the corresponding value
                    }
                }
            }
        }
    }

    /// Draws the value onto `T`, inside the given `Rectangle`
    pub fn draw<T: RaylibDraw>(&self, rect: Rectangle, d: &mut T, no_check: bool, alpha: bool) {
        match self {
            Cell::None => draw_none(rect, d),
            Cell::Player1 => draw_cross(rect, d),
            Cell::Player2 => draw_nought(rect, d),
            Cell::Board(b) => {
                if let Value::None = b.check() {
                    b.draw(rect, d, no_check, alpha) // Draw the board, if it is still playable...
                } else {
                    if no_check {
                        b.draw(rect, d, no_check, alpha) // ...or if we're told not to check...
                    } else {
                        b.draw(rect, d, no_check, alpha);
                        b.check().draw(rect, d, alpha) // ...else draw the corresponding value
                    }
                }
            }
        }
    }
}
