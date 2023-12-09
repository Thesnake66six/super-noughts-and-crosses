use anyhow::{Ok, Result};
use board::Board;
use cell::Cell;
use game::Game;
use raylib::prelude::*;
use styles::{CAMERA_MOVE_SPEED, CAMERA_SCROLL_SPEED, BOARD_DEPTH};

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

    let mut g = Game::new_depth(BOARD_DEPTH);

    // let mut x = Board::new();
    // x.set(&[0], Cell::Player1).unwrap();
    // x.set(&[1], Cell::Player1).unwrap();
    // x.set(&[5], Cell::Player1).unwrap();
    // x.set(&[6], Cell::Player1).unwrap();
    // x.set(&[7], Cell::Player1).unwrap();
    // x.set(&[2], Cell::Player2).unwrap();
    // x.set(&[3], Cell::Player2).unwrap();
    // x.set(&[4], Cell::Player2).unwrap();
    // x.set(&[8], Cell::Player2).unwrap();

    // g.board.cells[1] = cell::Cell::Board(Board::new_cells([Cell::Player1, Cell::Player1, Cell::Player1, Cell::Player1, Cell::Player1, Cell::Player1, Cell::Player1, Cell::Player1, Cell::Player1]));
    // g.board.cells[4] = cell::Cell::Board(x.clone());
    // g.board.cells[7] = cell::Cell::Board(Board::new_cells([Cell::Player2, Cell::Player2, Cell::Player2, Cell::Player2, Cell::Player2, Cell::Player2, Cell::Player2, Cell::Player2, Cell::Player2]));
    
    // g.board = Board::new_cells([Cell::Player1, Cell::Player1, Cell::Player1, Cell::Player1, Cell::Player1, Cell::Player1, Cell::Player1, Cell::Player1, Cell::Player1]);
    // g.board.cells[4] = cell::Cell::Board(Board::new_cells([Cell::Player1, Cell::Player1, Cell::Player1, Cell::Player1, Cell::Player1, Cell::Player1, Cell::Player1, Cell::Player1, Cell::Player1]));
    
    let board_rect = Rectangle { 
        x: 150.0, 
        y: 150.0, 
        width: 60.0 * 3f32.powi(BOARD_DEPTH as i32), 
        height: 60.0 * 3f32.powi(BOARD_DEPTH as i32),
    };

    g.board.update_positions(board_rect);

    // g.board.set(&[4], cell::Cell::Board(x.clone())).unwrap();
    // g.board.set(&[4, 1], cell::Cell::Board(x.clone())).unwrap();

    // g.play(&[2]);

    let mut mouse_prev = Vector2::zero();

    while !rl.window_should_close() {
        
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            let cell = g.get_cell_from_pos(rl.get_mouse_position(), false).unwrap_or_default();
            println!("{:#?}", g.get_cell_from_pos(rl.get_mouse_position(), false));
            let _ = g.play(&cell);
        }

            // Centre the camera
            g.camera.offset = Vector2 {
                x: rl.get_screen_width() as f32 / 2.0f32,
                y: rl.get_screen_height() as f32 / 2.0f32,
            };
    
            // Increment the zoom based of the mousewheel
            g.camera.zoom += rl.get_mouse_wheel_move() * CAMERA_SCROLL_SPEED * g.camera.zoom;
    

        // Prevent zoom from being negative
        if g.camera.zoom < 0.0 {
            g.camera.zoom *= -1.0;
        }


        if rl.is_mouse_button_down(MouseButton::MOUSE_RIGHT_BUTTON) {
            let offset = rl.get_mouse_position();
            g.camera.target.x += (offset.x - mouse_prev.x) * CAMERA_MOVE_SPEED / g.camera.zoom;
            g.camera.target.y += (offset.y - mouse_prev.y) * CAMERA_MOVE_SPEED / g.camera.zoom;
            mouse_prev = offset;
        } else {
            mouse_prev = rl.get_mouse_position();
        }
 
        
        let hovered_cell = g.get_cell_from_pos(rl.get_mouse_position(), false);
        
        let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(Color::BLACK);


        g.draw(
            board_rect, 
            &mut d,
            false,
            true,
            hovered_cell.as_deref(), 
        );

        d.draw_fps(10, 10)
        
    }

    Ok(())
}
