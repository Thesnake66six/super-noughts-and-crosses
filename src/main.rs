use anyhow::{Ok, Result};
use board::Board;
use cell::Cell;
use game::Game;
use raylib::prelude::*;
use styles::{CAMERA_MOVE_SPEED, CAMERA_SCROLL_SPEED};

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

    let mut mouse_prev: Vector2 = Vector2 {
        x: (0.0f32),
        y: (0.0f32),
    };

    let mut g = Game::new_depth(1);

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
    

    g.board.update_positions(Rectangle { 
        x: 150.0, 
        y: 150.0, 
        width: 1000.0, 
        height: 1000.0, 
    });

    // g.board.set(&[4], cell::Cell::Board(x.clone())).unwrap();
    // g.board.set(&[4, 1], cell::Cell::Board(x.clone())).unwrap();

    // g.play(&[2]);
    g.camera.target = Vector2 {
        x: 150.0 + 500.0 ,
        y: 150.0 + 500.0 ,
    };

    while !rl.window_should_close() {
        
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            let cell = g.get_cell_from_pos(rl.get_mouse_position(), false).unwrap_or_default();
            println!("{:#?}", g.get_cell_from_pos(rl.get_mouse_position(), false));
            let _ = g.play(&cell);
        }


        // Increment the zoom based of the mousewheel
        g.camera.zoom += rl.get_mouse_wheel_move() * CAMERA_SCROLL_SPEED * g.camera.zoom;

        // Prevent zoom from being negative
        if g.camera.zoom < 0.0 {
            g.camera.zoom *= -1.0;
        }

        // Support panning
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
            Rectangle { 
                x: 150.0, 
                y: 150.0, 
                width: 1000.0, 
                height: 1000.0,
            }, 
            &mut d,
            false,
            true,
            hovered_cell.as_deref(), 
        );

        d.draw_fps(10, 10)
        
    }

    Ok(())
}

// use raylib::prelude::*;

// fn main() {
//     // Initialize the raylib window and other settings
//     let (mut rl, thread) = raylib::init().size(1600, 1200).title("Camera and UI Example").resizable().build();

//     // Create a Camera2D instance
//     let mut camera = Camera2D::default();
//     camera.target = Vector2::new(0.0, 0.0); // Set initial camera target

//     // Main game loop
//     while !rl.window_should_close() {
//         // Update

//         camera.zoom = 1.0;

//         camera.offset = Vector2 {
//             x: rl.get_screen_width() as f32 / 2.0f32,
//             y: rl.get_screen_height() as f32 / 2.0f32,
//         };

//         // Update camera position based on input or game logic
//         if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
//             camera.target.x += 1.0;
//         }
//         if rl.is_key_down(KeyboardKey::KEY_LEFT) {
//             camera.target.x -= 1.0;
//         }
//         // Update camera position based on input or game logic
//         if rl.is_key_down(KeyboardKey::KEY_UP) {
//             camera.target.y += 1.0;
//         }
//         if rl.is_key_down(KeyboardKey::KEY_DOWN) {
//             camera.target.y -= 1.0;
//         }

//         // Begin drawing with the camera transformation
//         let mut d = rl.begin_drawing(&thread);
//         d.clear_background(Color::RAYWHITE);

//         // Apply the camera transformation to draw game objects
//         let mut c = d.begin_mode2D(camera);

//         // c.clear_background(Color::GREEN);

//         // Draw game objects (in this case, a placeholder square for the camera)
//         // c.draw_rectangle((camera.target.x - 50.0) as i32, (camera.target.y - 50.0) as i32, 100, 100, Color::RED);
//         c.draw_rectangle(50, 50, 100, 100, Color::RED);

//         drop(c);

//         // Draw UI elements in regular screen space (not affected by the camera transformation)
//         d.draw_rectangle(600, 0, 200, 600, Color::DARKGRAY);

//         // Draw UI text or other elements
//         d.draw_text("UI Element 1", 620, 10, 20, Color::WHITE);

//         // Draw additional UI elements as needed
//     }
// }