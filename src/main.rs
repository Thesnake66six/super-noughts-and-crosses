use std::{fs::File, io::Write};

use anyhow::Result;
use cell::Value;
use game::Game;
use rand::{thread_rng, Rng};
use raylib::prelude::*;
use styles::*;
use ui::{UITab, UI};

mod board;
mod cell;
mod common;
mod game;
mod styles;
mod ui;

fn main() -> Result<()> {
    // Initialise Raylib
    let (mut rl, thread) = raylib::init()
        .size(650 * 2, 650 * 2)
        .resizable()
        .title("Super Noughts and Crosses")
        .msaa_4x()
        .build();

    // Allocate the space on the screen for the game and the UI
    let mut game_rect = get_game_rect(&rl);
    let mut ui_rect = get_ui_rect(&rl);

    let font_path = "./resources/Inter-Regular.ttf";

    // Import the font
    let font_50pt = rl
        .load_font_ex(&thread, font_path, 50, FontLoadEx::Default(0))
        .expect("Couldn't load font oof");

    // Set some settings for the window
    rl.set_target_fps(120);
    rl.set_window_min_size(UI_PANEL_WIDTH as i32, UI_PANEL_MIN_HEIGHT as i32);

    // Create the game
    let mut g = Game::new_depth(get_board_rect(BOARD_DEFAULT_DEPTH), BOARD_DEFAULT_DEPTH, 2);

    // Create the ui
    let mut ui = UI::new();

    // Get the pixel positions of each cell in the game, and each element in the UI
    g.update_positions();
    ui.update_positions(ui_rect);

    // Set up variables to do with input that are needed between frames
    let mut mouse_prev = Vector2::zero();
    let mut good_right_click = false;

    let mut response_time = COMPUTER_RESPONSE_DELAY;

    // Centre
    g.centre_camera(game_rect);

    println!("//------Look Ma, I'm a hacker now!------//");

    while !rl.window_should_close() {
        let delta = rl.get_frame_time();

        //----------// Handle input //----------//

        let hovered_cell = handle_input(
            &rl,
            &mut game_rect,
            &mut ui_rect,
            &mut ui,
            &mut g,
            &mut good_right_click,
            &mut mouse_prev,
            &mut response_time,
        );

        if g.turn == 0 && g.board.check() == Value::None && g.players == 1 && response_time <= 0.0 {
            let moves = g.legal_moves();
            let mut rng = rand::thread_rng();
            let i = if moves.len() == 1 {
                0
            } else {
                rng.gen_range(0..(moves.len() - 1))
            };
            let x = &moves[i];
            let _ = g.play(x);
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        g.draw(
            get_board_rect(g.depth),
            &mut d,
            false,
            true,
            hovered_cell.as_deref(),
        );

        ui.draw(ui_rect, &mut d, &g, &font_50pt);

        d.draw_fps(10, 10);

        response_time -= delta;
        if response_time < 0.0 {
            response_time = 0.0
        }
    }

    Ok(())
}

fn handle_input(
    rl: &RaylibHandle,
    game_rect: &mut Rectangle,
    ui_rect: &mut Rectangle,
    ui: &mut UI<'_>,
    g: &mut Game,
    good_right_click: &mut bool,
    mouse_prev: &mut Vector2,
    response_time: &mut f32,
) -> Option<Vec<usize>> {
    let mouse_pos = rl.get_mouse_position();

    if rl.is_window_resized() {
        *game_rect = get_game_rect(rl);
        *ui_rect = get_ui_rect(rl);
        ui.update_positions(*ui_rect);
    }

    // Centre the camera
    g.camera.offset = Vector2 {
        x: game_rect.width / 2.0,
        y: game_rect.height / 2.0,
    };

    // Increment the zoom based of the mousewheel and mouse position
    let x = rl.get_mouse_wheel_move();
    if ui_rect.check_collision_point_rec(mouse_pos) {
        // If the mouse is over the UI...
        match ui.tab {
            // ...and is in the Game tab...
            ui::UITab::Game => {
                // ...and is in the Moves display...
                if ui.game_elements["Moves"].check_collision_point_rec(mouse_pos) {
                    // ...increment the scroll offset.
                    ui.scroll_offset_game += x * UI_SCROLL_SPEED;
                    if ui.scroll_offset_game > 0.0 {
                        ui.scroll_offset_game = 0.0
                    }
                }
            }
            // ...and is in the Settings tab...
            ui::UITab::Settings => {
                // ... and is over the content...
                let content_rec = Rectangle {
                    x: ui_rect.x,
                    y: ui_rect.y + UI_NAVBAR_HEIGHT as f32 + UI_DIVIDER_THICKNESS as f32,
                    width: ui_rect.width,
                    height: ui_rect.height - UI_NAVBAR_HEIGHT as f32 - UI_DIVIDER_THICKNESS as f32,
                };
                if content_rec.check_collision_point_rec(mouse_pos) {
                    // ...increment the scroll offset.
                    ui.scroll_offset_settings += x * UI_SCROLL_SPEED;
                    if ui.scroll_offset_settings > 0.0 {
                        ui.scroll_offset_settings = 0.0
                    }
                }
            }
        }
    } else {
        // If the mouse is over the Game, increment the Camera zoom
        g.camera.zoom += x * CAMERA_SCROLL_SPEED * g.camera.zoom;
        if g.camera.zoom < 0.0 {
            g.camera.zoom *= -1.0
        }
    }

    // Small check to see whether the right-click was on the Game, if so, as long as it's held, pan the camera.
    // Stops a bug where if the cursor was over the UI window or outside the window, the camera wouldn't pan
    if rl.is_mouse_button_pressed(MouseButton::MOUSE_RIGHT_BUTTON)
        && game_rect.check_collision_point_rec(mouse_pos)
    {
        *good_right_click = true;
    }

    if rl.is_mouse_button_released(MouseButton::MOUSE_RIGHT_BUTTON) {
        *good_right_click = false;
    }

    if rl.is_mouse_button_down(MouseButton::MOUSE_RIGHT_BUTTON) && *good_right_click {
        let offset = mouse_pos;
        g.camera.target.x += (offset.x - mouse_prev.x) * CAMERA_MOVE_SPEED / g.camera.zoom;
        g.camera.target.y += (offset.y - mouse_prev.y) * CAMERA_MOVE_SPEED / g.camera.zoom;
        *mouse_prev = offset;
    } else {
        *mouse_prev = mouse_pos;
    }

    let world_coord = rl.get_screen_to_world2D(mouse_pos, g.camera);
    let hovered_cell = g.get_cell_from_pixel(world_coord, false);

    handle_click(
        rl,
        ui_rect,
        mouse_pos,
        response_time,
        ui,
        g,
        game_rect,
        &hovered_cell,
    );

    if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
        g.centre_camera(*game_rect);
    }
    hovered_cell
}

fn handle_click(
    rl: &RaylibHandle,
    ui_rect: &mut Rectangle,
    mouse_pos: Vector2,
    response_time: &mut f32,
    ui: &mut UI<'_>,
    g: &mut Game,
    game_rect: &mut Rectangle,
    hovered_cell: &Option<Vec<usize>>,
) {
    if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
        if ui_rect.check_collision_point_rec(mouse_pos) {
            // Oh boy, get ready
            if ui.constant_elements["Game"].check_collision_point_rec(mouse_pos) {
                // If the Game tab was clicked, change the tab to Game
                ui.tab = UITab::Game;
            } else if ui.constant_elements["Settings"].check_collision_point_rec(mouse_pos) {
                // If the Settings tab was clicked, change the tab to Settings
                ui.tab = UITab::Settings;
            } else if ui.constant_elements["Inner Content"].check_collision_point_rec(mouse_pos) {
                // If the tab content was clicked...
                match ui.tab {
                    UITab::Game => {
                        // Nothing, yet
                    }
                    UITab::Settings => {
                        // Account for scroll offset
                        let offset = mouse_pos + ui.scroll_offset_settings;

                        // Increment the depth if Depth Plus is clicked
                        if ui.settings_elements["Depth Plus"].check_collision_point_rec(offset) {
                            *ui.state.get_mut("Depth").unwrap() += 1;

                        // Decrement the depth if Depth Minus is clicked, saturating at 1
                        } else if ui.settings_elements["Depth Minus"]
                            .check_collision_point_rec(offset)
                        {
                            *ui.state.get_mut("Depth").unwrap() -= 1;
                            if ui.state["Depth"] < 1 {
                                *ui.state.get_mut("Depth").unwrap() = 1
                            };

                        // Set the player to 1 if Player 1 is clicked
                        } else if ui.settings_elements["Player 1"].check_collision_point_rec(offset)
                        {
                            *ui.state.get_mut("Players").unwrap() = 1;

                        // Set the player to 2 if Player 2 is clicked
                        } else if ui.settings_elements["Player 2"].check_collision_point_rec(offset)
                        {
                            *ui.state.get_mut("Players").unwrap() = 2;

                        // Start a new Game with the selected settings if New Game is clicked
                        } else if ui.settings_elements["New Game"].check_collision_point_rec(offset)
                        {
                            *g = Game::new_depth(
                                get_board_rect(ui.state["Depth"]),
                                ui.state["Depth"],
                                ui.state["Players"],
                            );
                            g.update_positions();
                            g.centre_camera(*game_rect);
                            g.camera.offset = Vector2 {
                                x: game_rect.width / 2.0f32,
                                y: game_rect.height,
                            };
                        }
                    }
                }
            }
        } else if let Some(ref cell) = *hovered_cell {
            if g.players == 2 || g.turn == 1 {
                let _ = g.play(cell);
                let mut rng = thread_rng();
                let x = rng.gen_range(5..20) as f32;
                *response_time = COMPUTER_RESPONSE_DELAY * x / 10.0;
            }
        }
    }
}

/// Returns the rectangle in which the game should be drawn
fn get_game_rect(rl: &RaylibHandle) -> Rectangle {
    Rectangle {
        x: 0.0,
        y: 0.0,
        width: (rl.get_screen_width() - UI_PANEL_WIDTH as i32) as f32,
        height: rl.get_screen_height() as f32,
    }
}

/// Returns the rectangle in which the UI panel should be drawn
fn get_ui_rect(rl: &RaylibHandle) -> Rectangle {
    let r = get_game_rect(rl);
    Rectangle {
        x: r.width,
        y: 0.0,
        width: UI_PANEL_WIDTH as f32,
        height: (rl.get_screen_height()) as f32,
    }
}

/// Returns an appropriately-sized rectangle for drawing the board
fn get_board_rect(depth: usize) -> Rectangle {
    Rectangle {
        x: 0.0,
        y: 0.0,
        width: 60.0 * 3f32.powi(depth as i32),
        height: 60.0 * 3f32.powi(depth as i32),
    }
}
