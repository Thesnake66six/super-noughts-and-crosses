use raylib::{color::Color};
use serde::{Deserialize, Serialize};

use crate::common::get_rgb_from_rgba;

use super::symbol::Symbol;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player<> {
    /// Foreground colour of the symbol
    pub foreground: Color,
    /// Background colour of the cell
    pub background: Color,
    /// Transparrent background colour of the cell
    pub background_alpha: Color,
    /// Function to draw the symbol
    pub symbol: Symbol,
}

impl Player {
    pub fn get_greyed_colour(&self) -> Color {
        get_rgb_from_rgba(self.background_alpha, Color::WHITE)
    }
}

// /// Foreground colour of the cross symbol.
// pub const COLOUR_CROSS_FG: Color = Color {
//     r: 230,
//     g: 41,
//     b: 55,
//     a: 255,
// };

// /// Specific background colour of cross cells.
// pub const COLOUR_CROSS_BG: Color = COLOUR_CELL_BG;

// /// Specific transparent background colour of cross cells.
// pub const COLOUR_CROSS_BGA: Color = Color {
//     r: 230,
//     g: 41,
//     b: 55,
//     a: 127,
// };

// /// Colourful background colour of a greyed cell on crosses' turn.
// pub const COLOUR_CELL_BG_GREYED_P1: Color = Color {
//     r: 243,
//     g: 148,
//     b: 155,
//     a: 255,
// };
