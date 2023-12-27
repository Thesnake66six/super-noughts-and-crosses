use anyhow::{Ok, Result};
use game::Game;
use raylib::prelude::*;
use styles::{BOARD_DEPTH, CAMERA_DEFAULT_ZOOM, CAMERA_MOVE_SPEED, CAMERA_SCROLL_SPEED};

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

    let board_rect = Rectangle {
        x: 150.0,
        y: 150.0,
        width: 60.0 * 3f32.powi(BOARD_DEPTH as i32),
        height: 60.0 * 3f32.powi(BOARD_DEPTH as i32),
    };

    let mut g = Game::new_depth(board_rect, BOARD_DEPTH);

    g.update_positions();

    let mut mouse_prev = Vector2::zero();

    // Centre
    g.camera.target = Vector2 {
        x: board_rect.x + board_rect.width / 2.0f32,
        y: board_rect.y + board_rect.height / 2.0f32,
    };
    g.camera.zoom = f32::max(
        rl.get_screen_width() as f32 / board_rect.width * CAMERA_DEFAULT_ZOOM,
        rl.get_screen_height() as f32 / board_rect.height * CAMERA_DEFAULT_ZOOM,
    );

    rl.set_target_fps(60);

    let mut x = 0;
    while !rl.window_should_close() {
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

        let world_coord = rl.get_screen_to_world2D(rl.get_mouse_position(), g.camera);
        let hovered_cell = g.get_cell_from_pos(world_coord, false);

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            if let Some(ref cell) = hovered_cell {
                let _ = g.play(cell);
                dbg!(g.legal.as_slice());
            }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
            g.camera.target = Vector2 {
                x: board_rect.x + board_rect.width / 2.0f32,
                y: board_rect.y + board_rect.height / 2.0f32,
            };
            g.camera.zoom = f32::max(
                rl.get_screen_width() as f32 / board_rect.width * CAMERA_DEFAULT_ZOOM,
                rl.get_screen_height() as f32 / board_rect.height * CAMERA_DEFAULT_ZOOM,
            );
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        g.draw(board_rect, &mut d, false, true, hovered_cell.as_deref());

        d.draw_fps(10, 10);

        d.draw_text(&x.to_string(), 10, 30, 10, Color::RED);
        x += 1;
    }

    Ok(())
}
