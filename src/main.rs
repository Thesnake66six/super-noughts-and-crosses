use anyhow::{Error, Ok, Result};
use board::Board;
use cell::{Cell, Value};
use raylib::prelude::*;

mod cell;
mod board;
mod styles;

fn main() -> Result<()>{
    let (mut rl, thread) = raylib::init()
    .size(640 * 2, 640 * 2)
    .resizable()
    .title("Hello, World")
    .msaa_4x()
    .build();

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

        x.draw(Rectangle { x: 10.0, y: 10.0, width: 1000.0, height: 1000.0 }, &mut d)
    }        

    Ok(())
}