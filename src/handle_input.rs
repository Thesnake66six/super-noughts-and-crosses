use std::{fs, time::Duration};

use raylib::{
    camera::Camera2D,
    ffi::{KeyboardKey, MouseButton},
    math::{Rectangle, Vector2},
    RaylibHandle,
};

use crate::{
    common::{get_game_rect, get_ui_rect},
    game::{game::Game, value::Value},
    handle_click::handle_click,
    noughbert::{
        message::Message, monte_carlo_policy::MonteCarloPolicy,
        monte_carlo_settings::MonteCarloSettings,
    },
    state::State,
    styles::{ALLOW_FPS_COUNTER, CAMERA_MOVE_SPEED, CAMERA_SCROLL_SPEED, DEFAULT_EXPLORATION_FACTOR, UI_DIVIDER_THICKNESS, UI_NAVBAR_HEIGHT, UI_SCROLL_SPEED},
    ui::{textbox::Textbox, ui::UI, ui_tab::UITab},
};

pub fn handle_input(
    rl: &mut RaylibHandle,
    g: &mut Game,
    ui: &mut UI,
    state: &mut State,
) -> Option<Vec<usize>> {
    let mouse_pos = rl.get_mouse_position();

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
    if rl.is_mouse_button_pressed(MouseButton::MOUSE_RIGHT_BUTTON)
        && state.game_rect.check_collision_point_rec(mouse_pos)
    {
        state.good_right_click = true;
    }

    if rl.is_mouse_button_released(MouseButton::MOUSE_RIGHT_BUTTON) {
        state.good_right_click = false;
    }

    if rl.is_mouse_button_down(MouseButton::MOUSE_RIGHT_BUTTON) && state.good_right_click {
        g.camera.target.x += (mouse_pos.x - state.mouse_prev.x) * CAMERA_MOVE_SPEED / g.camera.zoom;
        g.camera.target.y += (mouse_pos.y - state.mouse_prev.y) * CAMERA_MOVE_SPEED / g.camera.zoom;
    }
    state.mouse_prev = mouse_pos;

    let world_coord = rl.get_screen_to_world2D(mouse_pos, g.camera);
    let hovered_cell = g.get_cell_from_pixel(world_coord, false);

    handle_click(rl, g, ui, state, mouse_pos, &hovered_cell);

    if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
        g.centre_camera(state.game_rect);
    }

    if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
        match state.typing {
            Textbox::MaxSims => {
                let x = &mut ui.state.max_sims;
                *x /= 10;
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
            }
        }
    }

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
                exploration_factor: DEFAULT_EXPLORATION_FACTOR,
                opt_for: g.turn,
                carry_forward: false,
                policy: MonteCarloPolicy::Robust,
            }),
        );
        state.waiting_for_move = true;
    }

    if rl.is_key_pressed(KeyboardKey::KEY_GRAVE) {
        if ALLOW_FPS_COUNTER {
            state.show_fps ^= true;
        } else {
            state.show_fps = false;
        }
    }

    handle_typing(rl, state, ui);

    if rl.is_file_dropped() {
        let paths = rl.get_dropped_files();
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
                };
                g.update_positions();
                g.centre_camera(state.game_rect);
            }
            Err(_) => {
                println!("Could not read game from file");
            }
        }
        rl.clear_dropped_files();
    }

    hovered_cell    
}

fn handle_typing(rl: &mut RaylibHandle, state: &mut State, ui: &mut UI) {
    if rl.is_key_pressed(KeyboardKey::KEY_ZERO) || rl.is_key_pressed(KeyboardKey::KEY_KP_0) {
        match state.typing {
            Textbox::MaxSims => {
                let x = &mut ui.state.max_sims;
                *x *= 10;
            }
            Textbox::MaxTime => {
                let x = &mut ui.state.max_time;
                *x *= 10;
            }
            Textbox::None => {}
        }
    }
    if rl.is_key_pressed(KeyboardKey::KEY_ONE) || rl.is_key_pressed(KeyboardKey::KEY_KP_1) {
        match state.typing {
            Textbox::MaxSims => {
                let x = &mut ui.state.max_sims;
                *x *= 10;
                *x += 1;
            }
            Textbox::MaxTime => {
                let x = &mut ui.state.max_time;
                *x *= 10;
                *x += 1;
            }
            Textbox::None => {}
        }
    }
    if rl.is_key_pressed(KeyboardKey::KEY_TWO) || rl.is_key_pressed(KeyboardKey::KEY_KP_2) {
        match state.typing {
            Textbox::MaxSims => {
                let x = &mut ui.state.max_sims;
                *x *= 10;
                *x += 2;
            }
            Textbox::MaxTime => {
                let x = &mut ui.state.max_time;
                *x *= 10;
                *x += 2;
            }
            Textbox::None => {}
        }
    }
    if rl.is_key_pressed(KeyboardKey::KEY_THREE) || rl.is_key_pressed(KeyboardKey::KEY_KP_3) {
        match state.typing {
            Textbox::MaxSims => {
                let x = &mut ui.state.max_sims;
                *x *= 10;
                *x += 3;
            }
            Textbox::MaxTime => {
                let x = &mut ui.state.max_time;
                *x *= 10;
                *x += 3;
            }
            Textbox::None => {}
        }
    }
    if rl.is_key_pressed(KeyboardKey::KEY_FOUR) || rl.is_key_pressed(KeyboardKey::KEY_KP_4) {
        match state.typing {
            Textbox::MaxSims => {
                let x = &mut ui.state.max_sims;
                *x *= 10;
                *x += 4;
            }
            Textbox::MaxTime => {
                let x = &mut ui.state.max_time;
                *x *= 10;
                *x += 4;
            }
            Textbox::None => {}
        }
    }
    if rl.is_key_pressed(KeyboardKey::KEY_FIVE) || rl.is_key_pressed(KeyboardKey::KEY_KP_5) {
        match state.typing {
            Textbox::MaxSims => {
                let x = &mut ui.state.max_sims;
                *x *= 10;
                *x += 5;
            }
            Textbox::MaxTime => {
                let x = &mut ui.state.max_time;
                *x *= 10;
                *x += 5;
            }
            Textbox::None => {}
        }
    }
    if rl.is_key_pressed(KeyboardKey::KEY_SIX) || rl.is_key_pressed(KeyboardKey::KEY_KP_6) {
        match state.typing {
            Textbox::MaxSims => {
                let x = &mut ui.state.max_sims;
                *x *= 10;
                *x += 6;
            }
            Textbox::MaxTime => {
                let x = &mut ui.state.max_time;
                *x *= 10;
                *x += 6;
            }
            Textbox::None => {}
        }
    }
    if rl.is_key_pressed(KeyboardKey::KEY_SEVEN) || rl.is_key_pressed(KeyboardKey::KEY_KP_7) {
        match state.typing {
            Textbox::MaxSims => {
                let x = &mut ui.state.max_sims;
                *x *= 10;
                *x += 7;
            }
            Textbox::MaxTime => {
                let x = &mut ui.state.max_time;
                *x *= 10;
                *x += 7;
            }
            Textbox::None => {}
        }
    }
    if rl.is_key_pressed(KeyboardKey::KEY_EIGHT) || rl.is_key_pressed(KeyboardKey::KEY_KP_8) {
        match state.typing {
            Textbox::MaxSims => {
                let x = &mut ui.state.max_sims;
                *x *= 10;
                *x += 8;
            }
            Textbox::MaxTime => {
                let x = &mut ui.state.max_time;
                *x *= 10;
                *x += 8;
            }
            Textbox::None => {}
        }
    }
    if rl.is_key_pressed(KeyboardKey::KEY_NINE) || rl.is_key_pressed(KeyboardKey::KEY_KP_9) {
        match state.typing {
            Textbox::MaxSims => {
                let x = &mut ui.state.max_sims;
                *x *= 10;
                *x += 9;
            }
            Textbox::MaxTime => {
                let x = &mut ui.state.max_time;
                *x *= 10;
                *x += 9;
            }
            Textbox::None => {}
        }
    }
    }
