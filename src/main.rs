use anyhow::{Ok, Result};
use board::Board;
use cell::Cell;
use game::Game;
use raylib::prelude::*;

mod board;
mod cell;
mod game;
mod styles;

fn main() -> Result<()> {
    let (mut rl, thread) = raylib::init()
        .size(650 * 2, 650 * 2)
        .resizable()
        .title("Super Noughts and Crosses")
        .msaa_4x()
        .build();

    rl.set_target_fps(12);

    let mut g = Game::new_depth(2);

    let mut x = Board::new();
    x.set(&[0], Cell::Player1).unwrap();
    x.set(&[3], Cell::Player1).unwrap();
    x.set(&[6], Cell::Player1).unwrap();
    x.set(&[2], Cell::Player2).unwrap();
    x.set(&[5], Cell::Player2).unwrap();
    x.set(&[8], Cell::Player2).unwrap();

    g.board = x.clone();

    g.board.set(&[4], cell::Cell::Board(x.clone())).unwrap();
    g.board.set(&[4, 1], cell::Cell::Board(x.clone())).unwrap();

    // g.play(&[2]);

    while !rl.window_should_close() {
        
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            g.play(&[0])?;
        }
        
        let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(Color::BLACK);

        g.draw(
            Rectangle { 
                x: 150.0, 
                y: 150.0, 
                width: 1000.0, 
                height: 1000.0, 
            }, 
            &mut d,
            false,
            true,
        )
        
    }

    Ok(())
}
