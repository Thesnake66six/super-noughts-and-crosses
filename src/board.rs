use anyhow::{bail, Ok, Result};
use raylib::{core::math::Rectangle, prelude::*};

use crate::{
    cell::{Cell, Value},
    styles::{BOARD_CELL_MARGIN, BOARD_LINE_THICK, COLOUR_BOARD_BG, COLOUR_BOARD_LINE},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub cells: Vec<Cell>,
    pub cell_positions: Vec<Rectangle>,
}

impl Board {
    /// Creates a new board filled with `Cell::None`
    pub fn new() -> Self {
        let mut x = Board {
            cells: vec![Cell::None; 9],
            cell_positions: vec![Rectangle::new(0.0, 0.0, 0.0, 0.0); 9],
        };
        x
    }

    /// Creates a new board with its cells as the input slice
    pub fn new_cells(cells: [Cell; 9]) -> Self {
        let mut x = Board {
            cells: cells.to_vec(),
            cell_positions: vec![Rectangle::new(0.0, 0.0, 0.0, 0.0); 9],
        };
        x
    }
    
    /// Recursively creates a new board, containing levels equal to the specified `depth`  
    pub fn new_depth(depth: usize) -> Self {
        if depth > 1 {
            Board {
                cells: vec![Cell::Board(Board::new_depth(depth - 1)); 9],
                cell_positions: vec![Rectangle::new(0.0, 0.0, 0.0, 0.0); 9],
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
        if &pos.len() == &0 {
            return Some(Cell::Board(self.clone()));
        }
        else if &pos.len() > &1 {
            if let Cell::Board(board) = &self.cells[pos[0]] {
                return board.get(&pos[1..]);
            } else {
                return None;
            }
        } else {
            Some(self.cells[pos[0]].clone())
        }
    }

    /// Changes the `Cell` at a given position to the given `Value`
    pub fn set(&mut self, pos: &[usize], value: Cell) -> Result<()> {
        if pos.len() > 1 {
            if let Cell::Board(x) = &mut self.cells[pos[0]] {
                x.set(&pos[1..], value)
            } else {
                bail!("No cell at specified depth")
            }
        } else {
            self.cells[pos[0]] = value;
            Ok(())
        }
    }

    /// Recursively checks the board to see if it has been won or drawn, and returns the corresponding `Value`
    pub fn check(&self) -> Value {
        let vals = self
            .cells
            .iter()
            .map(|cell| cell.value())
            .collect::<Vec<Value>>();
        let sets = [
            [0, 1, 2], 
            [3, 4, 5], 
            [6, 7, 8], 
            [0, 3, 6], 
            [1, 4, 7], 
            [2, 5, 8], 
            [0, 4, 8], 
            [2, 4, 6],
        ];

        for set in sets {
            if vals[set[0]] == vals[set[1]] && vals[set[1]] == vals[set[2]] && [Value::Player1, Value::Player2].contains(&vals[set[0]]) {
                return vals[set[0]];
            }
        }

        if !vals.contains(&Value::None) {
            return Value::Draw;
        }

        Value::None
    }

    /// Updates the positions of all cells within the board based on a given rectangle, which will then be used for drawing
    pub fn update_positions(&mut self, rect: Rectangle) {
        let length = rect.width;
        let thickness = BOARD_LINE_THICK * length;
        let margin = BOARD_CELL_MARGIN * length;
        let column_size = (length - (2.0 * thickness)) / 3.0;

        for y in 0..3 {
            for x in 0..3 {
                self.cell_positions[3 * y + x] = Rectangle {
                    x: rect.x + x as f32 * (column_size + thickness) + margin,
                    y: rect.y + y as f32 * (column_size + thickness) + margin,
                    width: column_size - 2.0 * margin,
                    height: column_size - 2.0 * margin,
                }
            }
        }

        for i in 0..9 {
            if let Cell::Board(b) = &mut self.cells[i] {
                b.update_positions(self.cell_positions[i])
            }
        }
    }

    pub fn get_cell_from_pos(&self, point: Vector2, no_check: bool) -> Option<Vec<usize>> {
        for ((cell, rect), i) in self.cells.iter().zip(&self.cell_positions).zip(0..9) {
            if rect.check_collision_point_rec(point) {
                if let Cell::Board(b) = cell {
                    if (b.check() == Value::None) || no_check {
                        let mut out = vec![i];
                        out.append(&mut b.get_cell_from_pos(point, no_check).unwrap_or(vec![]));
                        return Some(out);
                    } else {
                        return Some(vec![i]);
                    }
                } else {
                    return Some(vec![i]);
                }
            }
        }

        return None;
    }

    /// Alternate `draw` function
    pub fn draw_old<T: RaylibDraw>(&self, rect: Rectangle, d: &mut T, no_check: bool, alpha: bool) {
        let gap = rect.width * BOARD_CELL_MARGIN;
        let cw = (rect.width - 2.0 * gap) / 3.0;
    
        for r in 0..3 {
            for c in 0..3 {
                let x = rect.x + c as f32 * (cw + gap);
                let y = rect.y + r as f32 * (cw + gap);
    
                self.cells[3 * r + c].draw_old(
                    Rectangle {
                        x,
                        y,
                        width: cw,
                        height: cw,
                    },
                    d,
                    no_check,
                    alpha,
                );
            }
        }
    }

    /// Draws the board in a given `Rectangle`. Automatically checking for wins can be turned off, as well as rendering completed boards under their symbols
    pub fn draw<T: RaylibDraw>(&self, rect: Rectangle, d: &mut T, no_check: bool, alpha: bool, hover: Option<&[usize]>) {
        d.draw_rectangle_rec(
            rect,
            COLOUR_BOARD_BG,
        );

        let length = rect.width; // Side length of the board
        let thickness = BOARD_LINE_THICK * rect.width; // Thickness of the lines in px
        let margin = BOARD_CELL_MARGIN * rect.width; // Size of margin in px

        let column_size = (length - 2.0 * thickness) / 3.0;
        let g1 = column_size + 0.5 * thickness;
        let g2 = column_size + thickness;

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
            thickness,
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
            thickness,
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
            thickness,
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
            thickness,
            COLOUR_BOARD_LINE,
        );

        let mut x = 10;
        if let Some(pos) = hover {
            x = pos[0]
        }

        for i in 0..9 {
            if i == x {
                self.cells[i].draw(self.cell_positions[i], d, no_check, alpha, Some(&hover.unwrap()[1..]))
            } else {
                self.cells[i].draw(self.cell_positions[i], d, no_check, alpha, None)
            }
        }
    }
}
