use anyhow::{Error, Ok, Result};
use board::Board;
use cell::{Cell, Value};
use raylib::prelude::*;

mod board;
mod cell;
mod game;
mod styles;

fn main() -> Result<()> {
    let (mut rl, thread) = raylib::init()
        .size(600 * 2, 600 * 2)
        .resizable()
        .title("It's beginning")
        .msaa_4x()
        .build();

    rl.set_target_fps(12);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        let mut x = Board::new();
        x.set(&[0], Cell::Player1).unwrap();
        x.set(&[3], Cell::Player1).unwrap();
        x.set(&[6], Cell::Player1).unwrap();
        x.set(&[2], Cell::Player2).unwrap();
        x.set(&[5], Cell::Player2).unwrap();
        x.set(&[8], Cell::Player2).unwrap();
        let y = x.clone();
        x.set(&[4], Cell::Board(y.clone())).unwrap();
        x.set(&[4, 4], Cell::Board(y.clone())).unwrap();
        x.set(&[4, 4, 4], Cell::Board(y.clone())).unwrap();
        x.set(&[4, 4, 4, 4], Cell::Board(y.clone())).unwrap();

        // let x = Board::new_depth(3);

        x.draw(
            Rectangle {
                x: 100.0,
                y: 100.0,
                width: 1000.0,
                height: 1000.0,
            },
            &mut d,
        )
    }

    Ok(())
}
