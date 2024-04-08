use std::{
    fs,
    path::Path,
    process::Command,
    sync::mpsc,
    thread,
    time::{self, Duration},
};

use anyhow::Result;
use cell::Value;
use game::Game;
use raylib::{core::texture::RaylibTexture2D, prelude::*};
use styles::*;
use ui::{UITab, UI};

use crate::{
    common::*,
    game::Turn,
    monte_carlo::{
        message::Message,
        monte_carlo::{MonteCarloManager, MonteCarloPolicy, MonteCarloSettings},
    },
};

mod board;
mod cell;
mod common;
mod game;
mod monte_carlo;
mod styles;
mod ui;

fn main() -> Result<()> {
    // Main thread comms with Noughbert
    let (tx_0, rx_0) = mpsc::sync_channel::<Message>(0);

    // Noughbert comms witn main thread
    let (tx_1, rx_1) = mpsc::sync_channel::<Option<Vec<usize>>>(1);

    let spawn = thread::spawn(move || {
        let rx = rx_0;
        let tx = tx_1;
        let mut runs = 0;

        if OUTPUT_GRAPHVIS_FILES {
            let _ = fs::remove_dir_all("./outs");
            let _ = fs::create_dir("./outs");
        }

        loop {
            let message = rx.recv().unwrap();
            let mc_options = match message {
                Message::Start(x) => x,
                Message::Interrupt => continue,
                Message::GetThoughts() => continue,
                Message::Move(_) => continue,
                Message::GetMoveNow() => continue,
            };

            println!("Simulation requested");

            let mut noughbert = MonteCarloManager::new(mc_options.game, mc_options.opt_for);
            let start_time = time::Instant::now();
            let mut interrupt = false;

            assert_eq!(noughbert.g.board.check(), Value::None);

            if noughbert.g.board.check() != Value::None {
                interrupt = true
            }

            while start_time.elapsed() < mc_options.timeout
                && noughbert.sims < mc_options.max_sims
                && !interrupt
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
                let x = noughbert.select(mc_options.exploration_factor, mc_options.opt_for);
                if x.is_none() {
                    break;
                }
                let x = noughbert.expand(x.unwrap());
                let (x, val) = noughbert.simulate(x, mc_options.opt_for);
                noughbert.backpropogate(x, val);
                noughbert.sims += 1;
            }
            if interrupt {
                println!("Exited due to interrupt request");
                continue;
            } else if noughbert.sims >= mc_options.max_sims {
                println!("Exited due to simulation cap")
            } else if start_time.elapsed() >= mc_options.timeout {
                println!("Exited due to timeout")
            } else {
                println!("Exited due to complete game tree");
            }
            println!("{} sims", noughbert.sims);

            // Assertions true
            // assert_eq!(noughbert.g.board, sgame.board);
            // assert_eq!(noughbert.g.turn, sgame.turn);
            // assert_eq!(noughbert.g.moves, sgame.moves);

            let best_play = noughbert.best(
                mc_options.policy,
                mc_options.opt_for,
                mc_options.exploration_factor,
            );

            tx.send(best_play).unwrap();
            runs += 1;

            if OUTPUT_GRAPHVIS_FILES {
                let _ = fs::write(Path::new(&format!("./outs/{}.dot", runs)), {
                    let s = format!(
                        "{}",
                        graphvis_ego_tree::TreeWrapper::new(
                            &noughbert.tree,
                            noughbert.tree.root().id(),
                            |node| {
                                let mut board = noughbert.g.clone();
                                let id = format!("Node ID: {:?}", node.id());
                                let play = {
                                    if node.id() == noughbert.tree.root().id() {
                                        format!(
                                            "Starting Board: {}",
                                            match noughbert.g.turn {
                                                Turn::Player1 => "Crosses".to_owned(),
                                                Turn::Player2 => "Noughts".to_owned(),
                                            }
                                        )
                                    } else {
                                        match node.value().turn {
                                            Turn::Player1 => {
                                                format!("Crosses' turn: {:?}", &node.value().play)
                                            }
                                            Turn::Player2 => {
                                                format!("Noughts' turn: {:?}", &node.value().play)
                                            }
                                        }
                                    }
                                };

                                for x in node.ancestors().collect::<Vec<_>>().iter().rev().skip(1) {
                                    if board.board.check() != Value::None {
                                        // println!(
                                        //     "{:?}",
                                        //     node.ancestors().collect::<Vec<_>>().iter().rev().skip(1)
                                        // )
                                    }
                                    board.play(&x.value().play).unwrap();
                                }
                                if !node.value().play.is_empty() {
                                    board.play(&node.value().play).unwrap();
                                }
                                let repr = board.board.dbg_repr();

                                let done = match board.board.check() {
                                    Value::None => " ".to_owned(),
                                    Value::Draw => "Draw".to_owned(),
                                    Value::Player1 => "Crosses".to_owned(),
                                    Value::Player2 => "Noughts".to_owned(),
                                };

                                let score = {
                                    if node.id() == noughbert.tree.root().id() {
                                        format!(
                                            "{} / {} = {}",
                                            node.value().score(!mc_options.opt_for),
                                            node.value().playouts,
                                            if node.id() == noughbert.tree.root().id() {
                                                "".to_owned()
                                            } else {
                                                (node.value().score(!mc_options.opt_for)
                                                    / node.value().playouts)
                                                    .to_string()
                                            },
                                        )
                                    } else {
                                        format!(
                                            "{} / {} = {}",
                                            node.value().score(mc_options.opt_for),
                                            node.value().playouts,
                                            if node.id() == noughbert.tree.root().id() {
                                                "".to_owned()
                                            } else {
                                                (node.value().score(mc_options.opt_for)
                                                    / node.value().playouts)
                                                    .to_string()
                                            },
                                        )
                                    }
                                };

                                let ucb1 = {
                                    if node.id() == noughbert.tree.root().id() {
                                        "".to_owned()
                                    } else {
                                        format!(
                                            "{}",
                                            node.value().ucb1(
                                                mc_options.exploration_factor,
                                                match node.parent() {
                                                    Some(n) => n.value().playouts,
                                                    None => node.value().playouts,
                                                },
                                                mc_options.opt_for
                                            )
                                        )
                                    }
                                };

                                format!(
                                    "{}\n{}\n{}\n{}\n{}\nucb1 = {}",
                                    id, play, repr, done, score, ucb1
                                )
                            }
                        )
                    );
                    s.replace("\"]", "\" fontname = \"Consolas\"]")
                });
                if AUTOCOMPILE_GRAPHVIS_FILES {
                    let _ = Command::new("dot")
                        .args(["-T", "svg", "-O", &format!("./outs/{}.dot", runs)])
                        .spawn();
                }
            }
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
    rl.set_window_icon(
        raylib::texture::Image::load_image("./resources/icon.png").expect("Couldn't load icon oof"),
    );

    // Create the game
    let mut g = Game::new_depth(
        get_board_rect(BOARD_DEFAULT_DEPTH),
        BOARD_DEFAULT_DEPTH,
        BOARD_DEFAULT_PLAYERS,
    );

    // Create the ui
    let mut ui = UI::new();

    // Get the pixel positions of each cell in the game, and each element in the UI
    g.update_positions();
    ui.update_positions(ui_rect);

    // Set up variables to do with input that are needed between frames
    let mut state = State {
        mouse_prev: Vector2::zero(),
        good_right_click: false,
        show_fps: DEFAULT_SHOW_FPS_COUNTER,
        waiting_for_move: false,
        response_time: COMPUTER_RESPONSE_DELAY,
        message_queue: vec![],
        move_queue: vec![],
        typing: Textbox::None,
        can_export: true,
    };

    // Centre
    g.centre_camera(game_rect);

    println!("//------Look Ma, I'm a hacker now!------//");

    while !rl.window_should_close() {
        let delta = rl.get_frame_time();

        //----------// Handle input //----------//

        let hovered_cell = handle_input(
            &mut rl,
            &mut game_rect,
            &mut ui_rect,
            &mut ui,
            &mut g,
            &mut state,
        );

        if (g.players == 0 || (g.players == 1 && g.turn == Turn::Player2))
            && g.board.check() == Value::None
            && !state.waiting_for_move
        {
            state.message_queue.insert(
                state.message_queue.len(),
                Message::Start(MonteCarloSettings {
                    game: g.clone(),
                    timeout: Duration::from_secs(ui.state["Max Time"] as u64),
                    max_sims: ui.state["Max Sims"],
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
                    if let Some(y) = mv {
                        state.move_queue.insert(0, y);
                        println!("{:?}", state.move_queue)
                    }
                }
            }
            Err(e) => match e {
                mpsc::TryRecvError::Empty => {}
                mpsc::TryRecvError::Disconnected => panic!("Thread disconnected"),
            },
        }

        if state.response_time <= 0.0 {
            if let Some(mv) = state.move_queue.pop() {
                println!("Some move");
                dbg!(&state.move_queue);
                g.play(&mv).unwrap();
                state.waiting_for_move = false;
            }
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

        ui.draw(ui_rect, &mut d, &g, &font_50pt, &state);

        if state.show_fps {
            d.draw_fps(10, 10);
        }

        state.response_time -= delta;
        if state.response_time < 0.0 {
            state.response_time = 0.0
        }
    }

    Ok(())
}

fn handle_input(
    rl: &mut RaylibHandle,
    game_rect: &mut Rectangle,
    ui_rect: &mut Rectangle,
    ui: &mut UI<'_>,
    g: &mut Game,
    state: &mut State,
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

    // Increment the zoom based of the mouse wheel and mouse position
    let x = rl.get_mouse_wheel_move();
    if ui_rect.check_collision_point_rec(mouse_pos) {
        // If the mouse is over the UI...
        match ui.tab {
            // ...and is in the Game tab...
            UITab::Game => {
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
            UITab::Settings => {
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
            UITab::None => {}
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
        ui,
        g,
        state,
        game_rect,
        &hovered_cell,
    );

    if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
        g.centre_camera(*game_rect);
    }

    if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
        match state.typing {
            Textbox::MaxSims => {
                let x = ui.state.get_mut("Max Sims").unwrap();
                *x /= 10;
            }
            Textbox::MaxTime => {
                let x = ui.state.get_mut("Max Time").unwrap();
                *x /= 10;
            }
            Textbox::None => {
                let _ = g.unplay();
                state
                    .message_queue
                    .insert(state.message_queue.len(), Message::Interrupt)
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
                timeout: Duration::from_secs(DEFAULT_MAX_TIME as u64),
                max_sims: DEFAULT_MAX_SIMS,
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

    if rl.is_key_pressed(KeyboardKey::KEY_ZERO) || rl.is_key_pressed(KeyboardKey::KEY_KP_0) {
        match state.typing {
            Textbox::MaxSims => {
                let x = ui.state.get_mut("Max Sims").unwrap();
                *x *= 10;
            }
            Textbox::MaxTime => {
                let x = ui.state.get_mut("Max Time").unwrap();
                *x *= 10;
            }
            Textbox::None => {}
        }
    }
    if rl.is_key_pressed(KeyboardKey::KEY_ONE) || rl.is_key_pressed(KeyboardKey::KEY_KP_1) {
        match state.typing {
            Textbox::MaxSims => {
                let x = ui.state.get_mut("Max Sims").unwrap();
                *x *= 10;
                *x += 1
            }
            Textbox::MaxTime => {
                let x = ui.state.get_mut("Max Time").unwrap();
                *x *= 10;
                *x += 1
            }
            Textbox::None => {}
        }
    }
    if rl.is_key_pressed(KeyboardKey::KEY_TWO) || rl.is_key_pressed(KeyboardKey::KEY_KP_2) {
        match state.typing {
            Textbox::MaxSims => {
                let x = ui.state.get_mut("Max Sims").unwrap();
                *x *= 10;
                *x += 2
            }
            Textbox::MaxTime => {
                let x = ui.state.get_mut("Max Time").unwrap();
                *x *= 10;
                *x += 2
            }
            Textbox::None => {}
        }
    }
    if rl.is_key_pressed(KeyboardKey::KEY_THREE) || rl.is_key_pressed(KeyboardKey::KEY_KP_3) {
        match state.typing {
            Textbox::MaxSims => {
                let x = ui.state.get_mut("Max Sims").unwrap();
                *x *= 10;
                *x += 3
            }
            Textbox::MaxTime => {
                let x = ui.state.get_mut("Max Time").unwrap();
                *x *= 10;
                *x += 3
            }
            Textbox::None => {}
        }
    }
    if rl.is_key_pressed(KeyboardKey::KEY_FOUR) || rl.is_key_pressed(KeyboardKey::KEY_KP_4) {
        match state.typing {
            Textbox::MaxSims => {
                let x = ui.state.get_mut("Max Sims").unwrap();
                *x *= 10;
                *x += 4
            }
            Textbox::MaxTime => {
                let x = ui.state.get_mut("Max Time").unwrap();
                *x *= 10;
                *x += 4
            }
            Textbox::None => {}
        }
    }
    if rl.is_key_pressed(KeyboardKey::KEY_FIVE) || rl.is_key_pressed(KeyboardKey::KEY_KP_5) {
        match state.typing {
            Textbox::MaxSims => {
                let x = ui.state.get_mut("Max Sims").unwrap();
                *x *= 10;
                *x += 5
            }
            Textbox::MaxTime => {
                let x = ui.state.get_mut("Max Time").unwrap();
                *x *= 10;
                *x += 5
            }
            Textbox::None => {}
        }
    }
    if rl.is_key_pressed(KeyboardKey::KEY_SIX) || rl.is_key_pressed(KeyboardKey::KEY_KP_6) {
        match state.typing {
            Textbox::MaxSims => {
                let x = ui.state.get_mut("Max Sims").unwrap();
                *x *= 10;
                *x += 6
            }
            Textbox::MaxTime => {
                let x = ui.state.get_mut("Max Time").unwrap();
                *x *= 10;
                *x += 6
            }
            Textbox::None => {}
        }
    }
    if rl.is_key_pressed(KeyboardKey::KEY_SEVEN) || rl.is_key_pressed(KeyboardKey::KEY_KP_7) {
        match state.typing {
            Textbox::MaxSims => {
                let x = ui.state.get_mut("Max Sims").unwrap();
                *x *= 10;
                *x += 7
            }
            Textbox::MaxTime => {
                let x = ui.state.get_mut("Max Time").unwrap();
                *x *= 10;
                *x += 7
            }
            Textbox::None => {}
        }
    }
    if rl.is_key_pressed(KeyboardKey::KEY_EIGHT) || rl.is_key_pressed(KeyboardKey::KEY_KP_8) {
        match state.typing {
            Textbox::MaxSims => {
                let x = ui.state.get_mut("Max Sims").unwrap();
                *x *= 10;
                *x += 8
            }
            Textbox::MaxTime => {
                let x = ui.state.get_mut("Max Time").unwrap();
                *x *= 10;
                *x += 8
            }
            Textbox::None => {}
        }
    }
    if rl.is_key_pressed(KeyboardKey::KEY_NINE) || rl.is_key_pressed(KeyboardKey::KEY_KP_9) {
        match state.typing {
            Textbox::MaxSims => {
                let x = ui.state.get_mut("Max Sims").unwrap();
                *x *= 10;
                *x += 9
            }
            Textbox::MaxTime => {
                let x = ui.state.get_mut("Max Time").unwrap();
                *x *= 10;
                *x += 9
            }
            Textbox::None => {}
        }
    }

    if rl.is_file_dropped() {
        let paths = rl.get_dropped_files();
        let path = paths.last().unwrap();
        let json = fs::read(path).unwrap();
        match serde_json::from_slice::<Game>(&json) {
            Ok(new_game) => {
                // Game {
                //     rect,
                //     camera: Camera2D {
                //         zoom: 1.0,
                //         ..Default::default()
                //     },
                //     board: Board::new_depth(depth),
                //     depth,
                //     turn: Turn::Player1,
                //     players,
                //     moves: [].into(),
                //     legal: vec![],
                // }
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
                g.centre_camera(*game_rect);
            }
            Err(_) => {
                println!("Could not read game from file")
            }
        }
        rl.clear_dropped_files();
    }

    hovered_cell
}

fn handle_click(
    rl: &RaylibHandle,
    ui_rect: &mut Rectangle,
    mouse_pos: Vector2,
    ui: &mut UI<'_>,
    g: &mut Game,
    state: &mut State,
    game_rect: &mut Rectangle,
    hovered_cell: &Option<Vec<usize>>,
) {
    if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
        state.typing = Textbox::None;
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
                        // Export the game to a file if Export is clicked
                        if ui.game_elements["Export"].check_collision_point_rec(mouse_pos) {
                            let game_serial = serde_json::to_string(g).unwrap();
                            let _ = fs::create_dir("./exports");
                            let filename =
                                &format!("{:x}", md5::compute(game_serial.clone()))[..16];
                            match fs::write(format!("./exports/{}", filename), game_serial) {
                                Ok(_) => {
                                    println!("Game exported as file \"./exports/{}.xo\"", filename)
                                }
                                Err(_) => {
                                    eprintln!("Game export failed");
                                    state.can_export = false;
                                }
                            }
                        }
                    }
                    UITab::Settings => {
                        // Account for scroll offset
                        let offset = Vector2 {
                            x: mouse_pos.x,
                            y: mouse_pos.y - ui.scroll_offset_settings,
                        };
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
                        } else if ui.settings_elements["0 Players"]
                            .check_collision_point_rec(offset)
                        {
                            *ui.state.get_mut("Players").unwrap() = 0;

                        // Set the player to 1 if Player 1 is clicked
                        } else if ui.settings_elements["1 Player"].check_collision_point_rec(offset)
                        {
                            *ui.state.get_mut("Players").unwrap() = 1;

                        // Set the player to 2 if Player 2 is clicked
                        } else if ui.settings_elements["2 Players"]
                            .check_collision_point_rec(offset)
                        {
                            *ui.state.get_mut("Players").unwrap() = 2;

                        // Start a new Game with the selected settings if New Game is clicked
                        } else if ui.settings_elements["New Game"].check_collision_point_rec(offset)
                        {
                            // Stop any currently calculating moves
                            state
                                .message_queue
                                .insert(state.message_queue.len(), Message::Interrupt);
                            // Stop waiting to receive a move
                            state.waiting_for_move = false;
                            // Set a new game based on the current UI state
                            *g = Game::new_depth(
                                get_board_rect(ui.state["Depth"]),
                                ui.state["Depth"],
                                ui.state["Players"],
                            );
                            // Re-initialise the game
                            g.update_positions();
                            g.centre_camera(*game_rect);
                            g.camera.offset = Vector2 {
                                x: game_rect.width / 2.0f32,
                                y: game_rect.height,
                            };
                        } else if ui.settings_elements["AI 1"].check_collision_point_rec(offset) {
                            *ui.state.get_mut("AI Strength").unwrap() = 1;
                        } else if ui.settings_elements["AI 2"].check_collision_point_rec(offset) {
                            *ui.state.get_mut("AI Strength").unwrap() = 2;
                        } else if ui.settings_elements["AI 3"].check_collision_point_rec(offset) {
                            *ui.state.get_mut("AI Strength").unwrap() = 3;
                        } else if ui.settings_elements["AI Max Sims"]
                            .check_collision_point_rec(offset)
                        {
                            state.typing = Textbox::MaxSims
                        } else if ui.settings_elements["AI Max Time"]
                            .check_collision_point_rec(offset)
                        {
                            state.typing = Textbox::MaxTime
                        }
                    }
                    UITab::None => {}
                }
            }
        } else if let Some(ref cell) = *hovered_cell {
            // This means that the mouse click was in the game.
            if g.players == 2 || (g.players == 1 && g.turn == Turn::Player1) {
                let _ = g.play(cell);
                state
                    .message_queue
                    .insert(state.message_queue.len(), Message::Interrupt);
                let x = fastrand::usize(5..20) as f32;
                state.response_time = COMPUTER_RESPONSE_DELAY * x / 10.0;
            }
        }
    }
}
