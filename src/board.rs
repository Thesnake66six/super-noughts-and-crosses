use anyhow::{bail, Ok, Result};
use raylib::{core::math::Rectangle, prelude::*};

use crate::{
    cell::{Cell, Value}, common::*, game::Turn, styles::*
};

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub cells: Vec<Cell>,
    pub cell_positions: Vec<Rectangle>,
}

impl Board {
    /// Creates a new board filled with `Cell::None`
    pub fn new() -> Self {
        Board {
            cells: vec![Cell::None; 9],
            cell_positions: vec![Rectangle::new(0.0, 0.0, 0.0, 0.0); 9],
        }
    }

    /// Creates a new board with its cells as the input slice
    pub fn new_cells(cells: [Cell; 9]) -> Self {
        Board {
            cells: cells.to_vec(),
            cell_positions: vec![Rectangle::new(0.0, 0.0, 0.0, 0.0); 9],
        }
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
        if pos.is_empty() {
            Some(Cell::Board(self.clone()))
        } else if pos.len() > 1 {
            if let Cell::Board(board) = &self.cells[pos[0]] {
                return board.get(&pos[1..]);
            } else {
                None
            }
        } else {
            Some(self.cells[pos[0]].clone())
        }
    }

    // pub fn get_mut(mut self, pos: &[usize]) -> Option<&mut Cell> {
    //     if pos.is_empty() {
    //         Some(&mut Cell::Board(self))
    //     } else if pos.len() > 1 {
    //         if let Cell::Board(board) = &self.cells[pos[0]] {
    //             return board.get_mut(&pos[1..]);
    //         } else {
    //             None
    //         }
    //     } else {
    //         Some(&mut self.cells[pos[0]])
    //     }
    // }

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
            if vals[set[0]] == vals[set[1]]
                && vals[set[1]] == vals[set[2]]
                && [Value::Player1, Value::Player2].contains(&vals[set[0]])
            {
                return vals[set[0]];
            }
        }

        if !vals.contains(&Value::None) {
            return Value::Draw;
        }

        Value::None
    }

    /// Returns a Vec of all possible moves in the board
    pub fn legal_moves(&self, pos: &[usize]) -> Vec<Vec<usize>> {
        let mut l = vec![];
        for (i, x) in self.cells.iter().enumerate() {
            let mut v = Vec::with_capacity(pos.len() + 1);
            v.extend_from_slice(pos);
            v.push(i);
            l.append(&mut x.moves(&v))
        }
        l
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

    pub fn get_cell_from_pixel(&self, point: Vector2, no_check: bool) -> Option<Vec<usize>> {
        // Iterate over every cell in the board.
        for ((cell, rect), i) in self.cells.iter().zip(&self.cell_positions).zip(0..9) {
            // If the point collides with the cell...
            if rect.check_collision_point_rec(point) {
                // ...and it is a board...
                if let Cell::Board(b) = cell {
                    // ...and it hasn't been completed (or we don't check)...
                    if (b.check() == Value::None) || no_check {
                        // ...then append the current coordinate...
                        let mut out = vec![i];
                        let x = b.get_cell_from_pixel(point, no_check);
                        match x {
                            Some(mut x) => {
                                out.append(&mut x);
                                return Some(out);
                            }
                            None => return None,
                        }
                    } else {
                        return Some(vec![i]);
                    }
                } else {
                    return Some(vec![i]);
                }
            }
        }

        None
    }

    /// Draws the board in a given `Rectangle`. Automatically checking for wins can be turned off, as well as rendering completed boards under their symbols
    pub fn draw<T: RaylibDraw>(
        &self,
        rect: Rectangle,
        d: &mut T,
        no_check: bool,
        alpha: bool,
        hover: Option<&[usize]>,
        mut legal: Legal,
        turn: Turn,
    ) {
        let mut t: Option<usize> = None;
        let mut ignore = false;
        if legal == Legal::ForceDefaultBg {
            t = Some(13);
            ignore = true
        } else if let Legal::Pos(x) = legal {
            if !x.is_empty() {
                t = Some(x[0]);
                if x.len() == 1 {
                    legal = Legal::Pos(&[]);
                } else {
                    legal = Legal::Pos(&x[1..])
                }
            } else {
                t = Some(10)
            }
        };

        let board_complete = self.check() != Value::None || ignore;

        d.draw_rectangle_rec(
            rect,
            if board_complete {
                COLOUR_BOARD_BG
            } else if INVERT_GREYS {
                if t.is_some() {
                    COLOUR_BOARD_BG
                } else {
                    get_greyed_colour_board(turn)
                }
            } else if let Some(x) = t {
                if x == 10 {
                    get_greyed_colour_board(turn)
                } else {
                    COLOUR_BOARD_BG
                }
            } else {
                COLOUR_BOARD_BG
            },
        );

        let length = rect.width; // Side length of the board
        let thickness = BOARD_LINE_THICK * rect.width; // Thickness of the lines in px

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
            COLOUR_BOARD_FG,
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
            COLOUR_BOARD_FG,
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
            COLOUR_BOARD_FG,
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
            COLOUR_BOARD_FG,
        );

        let mut x = 10;
        if let Some(pos) = hover {
            x = pos[0]
        }

        for i in 0..9 {
            self.cells[i].draw(
                self.cell_positions[i],
                d,
                no_check,
                alpha,
                if i == x {
                    Some(&hover.unwrap()[1..])
                } else {
                    None
                },
                if board_complete {
                    Legal::ForceDefaultBg
                } else if [10, i].contains(&t.unwrap_or(11)) {
                    legal
                } else {
                    Legal::None
                },
                turn,
            )
        }
    }
}
