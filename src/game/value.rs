use raylib::{drawing::RaylibDraw, math::Rectangle};

use crate::{common::{draw_cross, draw_draw, draw_nought, get_greyed_colour_cell}, styles::{COLOUR_CELL_BG, COLOUR_CROSS_BG, COLOUR_CROSS_BGA, COLOUR_DRAW_BG, COLOUR_DRAW_BGA, COLOUR_NOUGHT_BG, COLOUR_NOUGHT_BGA}};

use super::{game::Turn, legal::Legal};

/// An enum used to differentiate the states of a board.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Value {
    /// The board is still being played
    None,
    /// The board is drawn
    Draw,
    /// The first player has won
    Player1,
    /// The second player has won
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
                greyed = false;
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
