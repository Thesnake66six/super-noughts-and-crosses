use std::{
    fmt::Display, fs::File, sync::mpsc::{self, Receiver, SyncSender}, thread, time::{self, Duration}
};

use anyhow::Result;
use cell::Value;
use ego_tree::Tree;
use game::Game;
use graphvis_ego_tree::TreeWrapper;
use raylib::{core::texture::RaylibTexture2D, prelude::*};
use styles::*;
use ui::{UITab, UI};
use std::io::Write;

use crate::{
    common::*,
    game::{Move, Turn},
    monte_carlo_me::{
        message::Message,
        monte_carlo_me::{MonteCarloManager, MonteCarloPolicy, MonteCarloSettings},
    },
};

mod board;
mod cell;
mod common;
mod game;
mod monte_carlo_me;
mod styles;
mod ui;

#[derive(Debug)]
pub struct Noughbert {}

impl monte_carlo::Game for Noughbert {
    type Turn = Turn;

    type Move = Move;

    type Board = Game;
}

fn write_tree_to_dot(state: &monte_carlo::State<Noughbert>) {
    let mut file = File::create("file.dot").unwrap();
    writeln!(
        file,
        "{}",
        TreeWrapper::new(&state.tree, state.root, |x| format!(
            "{}\nid: {:?}\nmove: {:?}\nplayouts: {}\nscore: {}\nis_complete: {}",
            state.build_board(x.id()).0.board.dbg_repr(),
            x.id(),
            x.value().r#move,
            x.value().playouts,
            x.value().score,
            x.value().is_complete,
        ))
    )
    .unwrap();
}

fn main() -> Result<()> {
    // Main thread comms with Noughbert
    let (tx_0, rx_0) = mpsc::sync_channel::<Message>(0);

    // Noughbert comms witn main thread
    let (tx_1, rx_1) = mpsc::sync_channel::<Move>(1);

    let spawn = thread::spawn(move || {
        let rx = rx_0;
        let tx = tx_1;

        loop {
            let message = rx.recv().unwrap();
            let mc_options = match message {
                Message::Start(x) => x,
                Message::Interrupt => continue,
                Message::GetThoughts() => continue,
                Message::Move(_) => continue,
                Message::GetMoveNow() => continue,
            };
            let start_time = time::Instant::now();
            let mut interrupt = false;
            let mut sims = 0;

            let mut state = monte_carlo::State::new(mc_options.game.turn, mc_options.opt_for, mc_options.game);

            while start_time.elapsed() < mc_options.timeout && sims < mc_options.max_sims
            {
                let message = rx.try_recv();
                match message {
                    Ok(m) => match m {
                        Message::Start(_) => {}
                        Message::Interrupt => {
                            interrupt = true;
                            break;
                        }
                        Message::GetThoughts() => todo!(),
                        Message::Move(_) => {}
                        Message::GetMoveNow() => todo!(),
                    },
                    Err(e) => match e {
                        mpsc::TryRecvError::Empty => {}
                        mpsc::TryRecvError::Disconnected => panic!("Thread disconnected"),
                    },
                }
                state.mcts(1);
                sims += 1;
            }
            if interrupt {
                println!("Exited due to interrupt request");
                continue;
            } else if sims >= mc_options.max_sims {
                println!("Exited due to simulation cap")
            } else if start_time.elapsed() >= mc_options.timeout {
                println!("Exited due to timeout")
            } else {
                println!("Exited due to complete game tree");
            }
            println!("{} sims", sims);

            let best_play = state.suggest_move(monte_carlo::MovePolicy::MaxChild).unwrap();
            tx.send(best_play).unwrap();
            write_tree_to_dot(&state)
        }
    });

    let rx = rx_1;
    let tx = tx_0;

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

    let mon_const = get_current_monitor_index();
    dbg!(mon_const);
    let physical_width = get_monitor_physical_width(mon_const) as f32;
    let width = get_monitor_width(mon_const) as f32;
    dbg!(width / physical_width, width, physical_width);

    let font_path = "./resources/Inter-Regular.ttf";

    // Import the font
    let font_50pt = rl
        .load_font_ex(&thread, font_path, 100, FontLoadEx::Default(0))
        .expect("Couldn't load font oof");

    font_50pt
        .texture()
        .set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);

    // Set some settings for the window
    rl.set_target_fps(120);
    rl.set_window_min_size(UI_PANEL_WIDTH as i32, UI_PANEL_MIN_HEIGHT as i32);

    // Create the game
    let mut g = Game::new_depth(get_board_rect(BOARD_DEFAULT_DEPTH), BOARD_DEFAULT_DEPTH, 1);

    // Create the ui
    let mut ui = UI::new();

    // Get the pixel positions of each cell in the game, and each element in the UI
    g.update_positions();
    ui.update_positions(ui_rect);

    // Set up variables to do with input that are needed between frames
    let mut state = State {
        mouse_prev: Vector2::zero(),
        good_right_click: false,
        show_fps: true,
        waiting_for_move: false,
        response_time: COMPUTER_RESPONSE_DELAY,
        message_queue: vec![],
    };

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
            &mut state,
        );

        if g.turn == Turn::Player2 && g.board.check() == Value::None && g.players == 1 && !state.waiting_for_move {
        // if g.board.check() == Value::None && g.players == 1 && !state.waiting_for_move {
            state.message_queue.insert(
                state.message_queue.len(),
                Message::Start(MonteCarloSettings {
                    game: g.clone(),
                    timeout: Duration::from_secs(DEFAULT_MOVE_TIMEOUT as u64),
                    max_sims: DEFAULT_MAX_SIMS,
                    exploration_factor: DEFAULT_EXPLORATION_FACTOR,
                    opt_for: g.turn,
                    carry_forward: false,
                    policy: MonteCarloPolicy::Robust,
                }),
            );
            state.waiting_for_move = true;
        }

        for message in state.message_queue.drain(0..state.message_queue.len()) {
            tx.send(message).unwrap();
        }

        let mv = rx.try_recv();
        match mv {
            Ok(mv) => {
                if state.waiting_for_move {
                    g.play(&mv).unwrap();
                    state.waiting_for_move = false;
                }
            }
            Err(e) => match e {
                mpsc::TryRecvError::Empty => {}
                mpsc::TryRecvError::Disconnected => panic!("Thread disconnected"),
            },
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        g.draw(
            get_board_rect(g.depth),
            &mut d,
            false,
            true,
            hovered_cell.as_ref(),
        );

        ui.draw(ui_rect, &mut d, &g, &font_50pt);

        if state.show_fps {
            d.draw_fps(10, 10);
        }

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
    state: &mut State,
) -> Option<Move> {
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

    handle_click(
        rl,
        ui_rect,
        mouse_pos,
        &mut state.response_time,
        ui,
        g,
        game_rect,
        &hovered_cell,
    );

    if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
        g.centre_camera(*game_rect);
    }

    if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
        let _ = g.unplay();
        state
            .message_queue
            .insert(state.message_queue.len(), Message::Interrupt)
    }

    if rl.is_key_pressed(KeyboardKey::KEY_SLASH) {
        state.message_queue.insert(
            state.message_queue.len(),
            Message::Start(MonteCarloSettings {
                game: g.clone(),
                timeout: Duration::from_secs(DEFAULT_MOVE_TIMEOUT as u64),
                max_sims: DEFAULT_MAX_SIMS,
                exploration_factor: DEFAULT_EXPLORATION_FACTOR,
                opt_for: g.turn,
                carry_forward: false,
                policy: MonteCarloPolicy::Robust,
            }),
        )
    }

    if rl.is_key_pressed(KeyboardKey::KEY_GRAVE) {
        state.show_fps ^= true;
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
    hovered_cell: &Option<Move>,
) {
    if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
        if ui_rect.check_collision_point_rec(mouse_pos) {
            // This means that the mouse click was in the UI.
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
            // This means that the mouse click was in the game.
            if g.players == 2 || g.turn == Turn::Player1 {
                let _ = g.play(cell);
                let x = fastrand::usize(5..20) as f32;
                *response_time = COMPUTER_RESPONSE_DELAY * x / 10.0;
            }
        }
    }
}
