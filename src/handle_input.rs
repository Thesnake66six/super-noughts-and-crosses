use std::{fs, time::Duration};

use raylib::{
    camera::Camera2D,
    ffi::{KeyboardKey, MouseButton},
    math::{Rectangle, Vector2},
    RaylibHandle, RaylibThread,
};

use crate::{
    common::{get_game_rect, get_ui_rect, update_window_title},
    game::{game::Game, value::Value},
    handle_click::handle_click,
    noughbert::{
        message::Message, monte_carlo_policy::MonteCarloPolicy,
        monte_carlo_settings::MonteCarloSettings,
    },
    state::State,
    styles::{
        ALLOW_FPS_COUNTER, CAMERA_MOVE_SPEED, CAMERA_SCROLL_SPEED, DEFAULT_EXPLORATION_FACTOR,
        UI_DIVIDER_THICKNESS, UI_NAVBAR_HEIGHT, UI_SCROLL_SPEED,
    },
    ui::{textbox::Textbox, ui::UI, ui_tab::UITab},
};

pub fn handle_input(
    rl: &mut RaylibHandle,
    rlthread: &mut RaylibThread,
    g: &mut Game,
    ui: &mut UI,
    state: &mut State,
) -> Option<Vec<usize>> {
    // Get the mouse position
    let mouse_pos = rl.get_mouse_position();

    // Check if the window has been resized and update
    if rl.is_window_resized() {
        state.game_rect = get_game_rect(rl);
        state.ui_rect = get_ui_rect(rl);
        ui.update_positions(state.ui_rect);
    }

    // Centre the camera
    g.camera.offset = Vector2 {
        x: state.game_rect.width / 2.0,
        y: state.game_rect.height / 2.0,
    };

    // Increment the zoom based of the mouse wheel and mouse position
    let x = rl.get_mouse_wheel_move();
    if state.ui_rect.check_collision_point_rec(mouse_pos) {
        // If the mouse is over the UI...
        match ui.tab {
            // ...and is in the Game tab...
            UITab::Game => {
                // ...and is in the Moves display...
                if ui.game_elements.moves.check_collision_point_rec(mouse_pos) {
                    // ...increment the scroll offset.
                    ui.scroll_offset_game += x * UI_SCROLL_SPEED;
                    if ui.scroll_offset_game > 0.0 {
                        ui.scroll_offset_game = 0.0;
                    }
                }
            }
            // ...and is in the Settings tab...
            UITab::Settings => {
                // ... and is over the content...
                let content_rec = Rectangle {
                    x: state.ui_rect.x,
                    y: state.ui_rect.y + UI_NAVBAR_HEIGHT as f32 + UI_DIVIDER_THICKNESS as f32,
                    width: state.ui_rect.width,
                    height: state.ui_rect.height
                        - UI_NAVBAR_HEIGHT as f32
                        - UI_DIVIDER_THICKNESS as f32,
                };
                if content_rec.check_collision_point_rec(mouse_pos) {
                    // ...increment the scroll offset.
                    ui.scroll_offset_settings += x * UI_SCROLL_SPEED;
                    if ui.scroll_offset_settings > 0.0 {
                        ui.scroll_offset_settings = 0.0;
                    }
                }
            }
            UITab::Keybinds => {
                // ... and is over the content...
                let content_rec = Rectangle {
                    x: state.ui_rect.x,
                    y: state.ui_rect.y + (ui.keybinds_elements.binds.y - state.ui_rect.y),
                    width: state.ui_rect.width,
                    height: state.ui_rect.height
                        - UI_NAVBAR_HEIGHT as f32
                        - UI_DIVIDER_THICKNESS as f32,
                };
                if content_rec.check_collision_point_rec(mouse_pos) {
                    // ...increment the scroll offset.
                    ui.scroll_offset_keybinds += x * UI_SCROLL_SPEED;
                    if ui.scroll_offset_keybinds > 0.0 {
                        ui.scroll_offset_keybinds = 0.0;
                    }
                }
            }
            UITab::Symbols => {}
            UITab::None => {}
        }
    } else {
        // If the mouse is over the Game, increment the Camera zoom
        g.camera.zoom += x * CAMERA_SCROLL_SPEED * g.camera.zoom;
        if g.camera.zoom < 0.0 {
            g.camera.zoom *= -1.0;
        }
    }

    // Small check to see whether the right-click was on the Game, if so, as long as it's held, pan the camera.
    // Stops a bug where if the cursor was over the UI window or outside the window, the camera wouldn't pan
    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT)
        && state.game_rect.check_collision_point_rec(mouse_pos)
    {
        state.good_right_click = true;
    }

    // Stop the drag when the right-click is released
    if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_RIGHT) {
        state.good_right_click = false;
    }

    // Pan when a good right-click is held
    if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) && state.good_right_click {
        g.camera.target.x += (mouse_pos.x - state.mouse_prev.x) * CAMERA_MOVE_SPEED / g.camera.zoom;
        g.camera.target.y += (mouse_pos.y - state.mouse_prev.y) * CAMERA_MOVE_SPEED / g.camera.zoom;
    }
    state.mouse_prev = mouse_pos;

    // Get the vurrently hovered-over cell
    let world_coord = rl.get_screen_to_world2D(mouse_pos, g.camera);
    let hovered_cell = g.get_cell_from_pixel(world_coord, false);

    // Handle left-click inputs
    handle_click(rl, rlthread, g, ui, state, mouse_pos, &hovered_cell);

    // Re-centre the camera when enter is pressed
    if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
        g.centre_camera(state.game_rect);
    }

    // When the backspace key is pressed, either delete the last character, or unplay the last move
    if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
        match state.typing {
            Textbox::MaxSims => {
                let x = &mut ui.state.max_sims;
                *x /= 10;
                ui.state.is_ai_modified = true
            }
            Textbox::MaxTime => {
                let x = &mut ui.state.max_time;
                *x /= 10;
            }
            Textbox::None => {
                let _ = g.unplay();
                state
                    .message_queue
                    .insert(state.message_queue.len(), Message::Interrupt);
                ui.state.is_ai_modified = true
            }
        }
    }

    // Queue a computer move when slash is pressed
    if rl.is_key_pressed(KeyboardKey::KEY_SLASH)
        && g.board.check() == Value::None
        && !state.waiting_for_move
    {
        state.message_queue.insert(
            state.message_queue.len(),
            Message::Start(MonteCarloSettings {
                game: g.clone(),
                timeout: Duration::from_secs(ui.state.max_time as u64),
                max_sims: ui.state.max_sims,
                threads: ui.state.ai_threads,
                exploration_factor: DEFAULT_EXPLORATION_FACTOR,
                opt_for: g.turn,
                carry_forward: false,
                policy: MonteCarloPolicy::Robust,
            }),
        );
        state.waiting_for_move = true;
    }

    // Toggle the FPS counter when the grave key is pressed
    if rl.is_key_pressed(KeyboardKey::KEY_GRAVE) {
        if ALLOW_FPS_COUNTER {
            state.show_fps ^= true;
        } else {
            state.show_fps = false;
        }
    }

    // Handle all typing inputs
    handle_typing(rl, state, ui);

    // Handle the deserialisation of a dropped file
    if rl.is_file_dropped() {
        let paths = rl.load_dropped_files();
        let paths = paths.paths();
        let path = paths.last().unwrap();
        let json = fs::read(path).unwrap();
        match serde_json::from_slice::<Game>(&json) {
            Ok(new_game) => {
                *g = Game {
                    rect: new_game.rect,
                    camera: Camera2D {
                        zoom: 1.0,
                        ..Default::default()
                    },
                    board: new_game.board,
                    depth: new_game.depth,
                    turn: new_game.turn,
                    players: new_game.players,
                    moves: new_game.moves,
                    legal: new_game.legal,
                    player_1: new_game.player_1,
                    player_2: new_game.player_2,
                };

                // Update the state to reflect the new game
                g.update_positions();
                g.centre_camera(state.game_rect);
                ui.state.is_ai_modified = true;
                ui.state.player_1 = g.player_1.symbol;
                ui.state.player_2 = g.player_2.symbol;
                update_window_title(rl, rlthread, g);
            }
            Err(_) => {
                println!("Could not read game from file");
                ui.state.is_ai_modified = true
            }
        }
    }

    hovered_cell
}

/// Handle typing inputs
fn handle_typing(rl: &mut RaylibHandle, state: &mut State, ui: &mut UI) {
    // Handle the 0 key
    if rl.is_key_pressed(KeyboardKey::KEY_ZERO) || rl.is_key_pressed(KeyboardKey::KEY_KP_0) {
        match state.typing {
            Textbox::MaxSims => {
                let x = &mut ui.state.max_sims;
                *x = x.saturating_mul(10);
                ui.state.is_ai_modified = true
            }
            Textbox::MaxTime => {
                let x = &mut ui.state.max_time;
                *x = x.saturating_mul(10);
            }
            Textbox::None => {}
        }
    }

    // Handle the 1 key
    if rl.is_key_pressed(KeyboardKey::KEY_ONE) || rl.is_key_pressed(KeyboardKey::KEY_KP_1) {
        match state.typing {
            Textbox::MaxSims => {
                let x = &mut ui.state.max_sims;
                *x = x.saturating_mul(10);
                *x = x.saturating_add(1);
                ui.state.is_ai_modified = true
            }
            Textbox::MaxTime => {
                let x = &mut ui.state.max_time;
                *x = x.saturating_mul(10);
                *x = x.saturating_add(1);
            }
            Textbox::None => {}
        }
    }

    // Handle the 2 key
    if rl.is_key_pressed(KeyboardKey::KEY_TWO) || rl.is_key_pressed(KeyboardKey::KEY_KP_2) {
        match state.typing {
            Textbox::MaxSims => {
                let x = &mut ui.state.max_sims;
                *x = x.saturating_mul(10);
                *x = x.saturating_add(2);
                ui.state.is_ai_modified = true
            }
            Textbox::MaxTime => {
                let x = &mut ui.state.max_time;
                *x = x.saturating_mul(10);
                *x = x.saturating_add(2);
            }
            Textbox::None => {}
        }
    }

    // Handle the 3 key
    if rl.is_key_pressed(KeyboardKey::KEY_THREE) || rl.is_key_pressed(KeyboardKey::KEY_KP_3) {
        match state.typing {
            Textbox::MaxSims => {
                let x = &mut ui.state.max_sims;
                *x = x.saturating_mul(10);
                *x = x.saturating_add(3);
                ui.state.is_ai_modified = true
            }
            Textbox::MaxTime => {
                let x = &mut ui.state.max_time;
                *x = x.saturating_mul(10);
                *x = x.saturating_add(3);
            }
            Textbox::None => {}
        }
    }
    // Handle the 4 key
    if rl.is_key_pressed(KeyboardKey::KEY_FOUR) || rl.is_key_pressed(KeyboardKey::KEY_KP_4) {
        match state.typing {
            Textbox::MaxSims => {
                let x = &mut ui.state.max_sims;
                *x = x.saturating_mul(10);
                *x = x.saturating_add(4);
                ui.state.is_ai_modified = true
            }
            Textbox::MaxTime => {
                let x = &mut ui.state.max_time;
                *x = x.saturating_mul(10);
                *x = x.saturating_add(4);
            }
            Textbox::None => {}
        }
    }

    // Handle the 5 key
    if rl.is_key_pressed(KeyboardKey::KEY_FIVE) || rl.is_key_pressed(KeyboardKey::KEY_KP_5) {
        match state.typing {
            Textbox::MaxSims => {
                let x = &mut ui.state.max_sims;
                *x = x.saturating_mul(10);
                *x = x.saturating_add(5);
                ui.state.is_ai_modified = true
            }
            Textbox::MaxTime => {
                let x = &mut ui.state.max_time;
                *x = x.saturating_mul(10);
                *x = x.saturating_add(5);
            }
            Textbox::None => {}
        }
    }

    // Handle the 6 key
    if rl.is_key_pressed(KeyboardKey::KEY_SIX) || rl.is_key_pressed(KeyboardKey::KEY_KP_6) {
        match state.typing {
            Textbox::MaxSims => {
                let x = &mut ui.state.max_sims;
                *x = x.saturating_mul(10);
                *x = x.saturating_add(6);
                ui.state.is_ai_modified = true
            }
            Textbox::MaxTime => {
                let x = &mut ui.state.max_time;
                *x = x.saturating_mul(10);
                *x = x.saturating_add(6);
            }
            Textbox::None => {}
        }
    }

    // Handle the 7 key
    if rl.is_key_pressed(KeyboardKey::KEY_SEVEN) || rl.is_key_pressed(KeyboardKey::KEY_KP_7) {
        match state.typing {
            Textbox::MaxSims => {
                let x = &mut ui.state.max_sims;
                *x = x.saturating_mul(10);
                *x = x.saturating_add(7);
                ui.state.is_ai_modified = true
            }
            Textbox::MaxTime => {
                let x = &mut ui.state.max_time;
                *x = x.saturating_mul(10);
                *x = x.saturating_add(7);
            }
            Textbox::None => {}
        }
    }

    // Handle the 8 key
    if rl.is_key_pressed(KeyboardKey::KEY_EIGHT) || rl.is_key_pressed(KeyboardKey::KEY_KP_8) {
        match state.typing {
            Textbox::MaxSims => {
                let x = &mut ui.state.max_sims;
                *x = x.saturating_mul(10);
                *x = x.saturating_add(8);
                ui.state.is_ai_modified = true
            }
            Textbox::MaxTime => {
                let x = &mut ui.state.max_time;
                *x = x.saturating_mul(10);
                *x = x.saturating_add(8);
            }
            Textbox::None => {}
        }
    }

    // Handle the 9 key
    if rl.is_key_pressed(KeyboardKey::KEY_NINE) || rl.is_key_pressed(KeyboardKey::KEY_KP_9) {
        match state.typing {
            Textbox::MaxSims => {
                let x = &mut ui.state.max_sims;
                *x = x.saturating_mul(10);
                *x = x.saturating_add(9);
                ui.state.is_ai_modified = true
            }
            Textbox::MaxTime => {
                let x = &mut ui.state.max_time;
                *x = x.saturating_mul(10);
                *x = x.saturating_add(9);
            }
            Textbox::None => {}
        }
    }
}
