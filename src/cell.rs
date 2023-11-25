use raylib::{drawing::RaylibDraw, math::Rectangle, color::Color};

use crate::{board::Board, styles::{draw_none, draw_cross, draw_nought, draw_draw}};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Value {
    None,
    Draw, 
    Player1,
    Player2,
}

impl Value {
    pub fn draw<T:RaylibDraw>(&self, rect: Rectangle, d: &mut T) {
        match self {
            Value::None => {
                draw_none(rect, d)
            },
            Value::Player1 => {
                draw_cross(rect, d)
            },
            Value::Player2 => {
                draw_nought(rect, d)
            },
            Value::Draw => {
                draw_draw(rect, d)
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    None,
    Player1,
    Player2,
    Board(Board),
}

impl Cell {
    pub fn value(&self) -> Value{
        match self {
            Cell::None => Value::None,
            Cell::Player1 => Value::Player1,
            Cell::Player2 => Value::Player2,
            Cell::Board(b) => b.check(),
        }
    }

    pub fn draw<T:RaylibDraw>(&self, rect: Rectangle, d: &mut T) {
        match self {
            Cell::None => {
                draw_none(rect, d)
            },
            Cell::Player1 => {
                draw_cross(rect, d)
            },
            Cell::Player2 => {
                draw_nought(rect, d)
            },
            Cell::Board(b) => {
                if let Value::None = b.check() {
                    b.draw(rect, d)
                } else {
                    b.check().draw(rect, d)
                }
            },
        }
    }
}