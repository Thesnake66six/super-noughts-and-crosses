use anyhow::{bail, Ok, Result};
use raylib::{core::math::Rectangle, ffi::GL_MAX_FRAGMENT_INPUT_COMPONENTS, prelude::*};

use crate::{
    cell::{Cell, Value},
    styles::{BOARD_CELL_MARGIN, BOARD_LINE_THICK, COLOUR_BOARD_BG, COLOUR_BOARD_LINE},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub cells: Vec<Cell>,
}

impl Board {
    /// Creates a new board filled with `Cell::None`
    pub fn new() -> Self {
        Board {
            cells: vec![Cell::None; 9],
        }
    }

    /// Recursively creates a new board, containing levels equal to the specified `depth`  
    pub fn new_depth(depth: usize) -> Self {
        if depth > 1 {
            Board {
                cells: vec![Cell::Board(Board::new_depth(depth - 1)); 9],
            }
        } else {
            Board::new()
        }
    }

    /// Returns the `Cell` at a specified position, taking a slice as input.
    ///
    /// The slice should contain the position of the target cell at each level of recursion - I.e.
    /// `[0]` is the top-left cell of a tic-tac-toe board;
    /// `[0, 1]` is the upper-middle cell in the top-left board of a depth 2 game
    pub fn get(&self, pos: &[usize]) -> Option<Cell> {
        if let Cell::Board(board) = &self.cells[pos[0]] {
            board.get(&pos[1..])
        } else {
            return None;
        }
    }

    pub fn set(&mut self, pos: &[usize], value: Cell) -> Result<()> {
        if pos.len() > 1 {
            if let Cell::Board(x) = &mut self.cells[pos[0]] {
                return x.set(&pos[1..], value);
            } else {
                bail!("No cell at specified depth")
            }
        } else {
            self.cells[pos[0]] = value;
            Ok(())
        }
    }

    pub fn check(&self) -> Value {
        let vals = self
            .cells
            .iter()
            .map(|cell| cell.value())
            .collect::<Vec<Value>>();
        let sets = [[0, 1, 2], [3, 4, 5], [6, 7, 8], [0, 4, 8], [2, 4, 6]];

        for set in sets {
            if vals[set[0]] == vals[set[1]] && vals[set[1]] == vals[set[2]] {
                if [Value::Player1, Value::Player2].contains(&vals[set[0]]) {
                    return vals[set[0]];
                }
            }
        }

        if !vals.contains(&Value::None) {
            return Value::Draw;
        }

        Value::None
    }

    pub fn draw_old<T: RaylibDraw>(&self, rect: Rectangle, d: &mut T) {
        let gap = rect.width * BOARD_CELL_MARGIN;
        let cw = (rect.width - 2.0 * gap) / 3.0;

        self.cells[0].draw_old(
            Rectangle {
                x: rect.x,
                y: rect.y,
                width: cw,
                height: cw,
            },
            d,
        );
        self.cells[1].draw_old(
            Rectangle {
                x: rect.x + cw + gap,
                y: rect.y,
                width: cw,
                height: cw,
            },
            d,
        );
        self.cells[2].draw_old(
            Rectangle {
                x: rect.x + 2.0 * cw + 2.0 * gap,
                y: rect.y,
                width: cw,
                height: cw,
            },
            d,
        );

        self.cells[3].draw_old(
            Rectangle {
                x: rect.x,
                y: rect.y + cw + gap,
                width: cw,
                height: cw,
            },
            d,
        );
        self.cells[4].draw_old(
            Rectangle {
                x: rect.x + cw + gap,
                y: rect.y + cw + gap,
                width: cw,
                height: cw,
            },
            d,
        );
        self.cells[5].draw_old(
            Rectangle {
                x: rect.x + 2.0 * cw + 2.0 * gap,
                y: rect.y + cw + gap,
                width: cw,
                height: cw,
            },
            d,
        );

        self.cells[6].draw_old(
            Rectangle {
                x: rect.x,
                y: rect.y + 2.0 * cw + 2.0 * gap,
                width: cw,
                height: cw,
            },
            d,
        );
        self.cells[7].draw_old(
            Rectangle {
                x: rect.x + cw + gap,
                y: rect.y + 2.0 * cw + 2.0 * gap,
                width: cw,
                height: cw,
            },
            d,
        );
        self.cells[8].draw_old(
            Rectangle {
                x: rect.x + 2.0 * cw + 2.0 * gap,
                y: rect.y + 2.0 * cw + 2.0 * gap,
                width: cw,
                height: cw,
            },
            d,
        );
    }

    pub fn draw<T: RaylibDraw>(&self, rect: Rectangle, d: &mut T) {
        d.draw_rectangle(
            rect.x as i32,
            rect.y as i32,
            rect.width as i32,
            rect.height as i32,
            COLOUR_BOARD_BG,
        );

        let l = rect.width;
        let t = BOARD_LINE_THICK * rect.width;
        let m = BOARD_CELL_MARGIN * rect.width;

        let c = (l - 2.0 * t) / 3.0;
        let g1 = c + 0.5 * t;
        let g2 = c + t;

        d.draw_line_ex(
            // Draw the first vertical line
            Vector2 {
                x: rect.x + g1,
                y: rect.y,
            },
            Vector2 {
                x: rect.x + g1,
                y: rect.y + rect.height,
            },
            t,
            COLOUR_BOARD_LINE,
        );

        d.draw_line_ex(
            // Draw the second vertical line
            Vector2 {
                x: rect.x + g1 + g2,
                y: rect.y,
            },
            Vector2 {
                x: rect.x + g1 + g2,
                y: rect.y + rect.height,
            },
            t,
            COLOUR_BOARD_LINE,
        );

        d.draw_line_ex(
            // Draw the first horizontal line
            Vector2 {
                x: rect.x,
                y: rect.y + g1,
            },
            Vector2 {
                x: rect.x + rect.width,
                y: rect.y + g1,
            },
            t,
            COLOUR_BOARD_LINE,
        );

        d.draw_line_ex(
            // Draw the second horizontal line
            Vector2 {
                x: rect.x,
                y: rect.y + g1 + g2,
            },
            Vector2 {
                x: rect.x + rect.width,
                y: rect.y + g1 + g2,
            },
            t,
            COLOUR_BOARD_LINE,
        );

        for y in 0..3 {
            for x in 0..3 {
                self.cells[3 * y + x].draw(
                    Rectangle {
                        x: rect.x + x as f32 * (c + t) + m,
                        y: rect.y + y as f32 * (c + t) + m,
                        width: c - 2.0 * m,
                        height: c - 2.0 * m,
                    },
                    d,
                )
            }
        }
    }
}
