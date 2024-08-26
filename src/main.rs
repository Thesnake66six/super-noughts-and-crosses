use std::{
    sync::mpsc,
    thread,
    time::Duration,
};

use anyhow::Result;
use raylib::{core::texture::RaylibTexture2D, prelude::*};
use styles::{BOARD_DEFAULT_DEPTH, BOARD_DEFAULT_PLAYERS, COMPUTER_RESPONSE_DELAY, DEFAULT_EXPLORATION_FACTOR, DEFAULT_MAX_TIME, DEFAULT_SHOW_FPS_COUNTER, UI_PANEL_MIN_HEIGHT, UI_PANEL_WIDTH};

use crate::{
    ai_thread::noughbert, common::{get_board_rect, get_game_rect, get_player_from_symbol, get_ui_rect, update_window_title}, fonts::Fonts, game::{
        game::{Game, Turn},
        value::Value,
    }, handle_input::handle_input, noughbert::{
        message::Message,  monte_carlo_policy::MonteCarloPolicy,
        monte_carlo_settings::MonteCarloSettings,
    }, state::State, ui::{textbox::Textbox, ui::UI}
};

mod common;
mod game;
mod noughbert;
mod state;
mod styles;
mod ui;
mod ai_thread;
mod handle_click;
mod handle_input;
mod fonts;

fn main() -> Result<()> {
    // Main thread comms with Noughbert
    let (tx_0, rx_0) = mpsc::sync_channel::<Message>(0);

    // Noughbert comms with main thread
    let (tx_1, rx_1) = mpsc::sync_channel::<Message>(1);

    let _thread = thread::spawn(move || {
        noughbert(rx_0, tx_1);
    });

    let rx = rx_1;
    let tx = tx_0;

    // Initialise Raylib
    let (mut rl, mut thread) = raylib::init()
        .size(650 * 2, 650 * 2)
        .resizable()
        .msaa_4x()
        .build();

    let mon_const = get_current_monitor_index();
    dbg!(mon_const);
    let physical_width = get_monitor_physical_width(mon_const) as f32;
    let width = get_monitor_width(mon_const) as f32;
    dbg!(width / physical_width, width, physical_width);

    let font_path = "./resources/Inter-Regular.ttf";
    let bold_font_path = "./resources/Inter-Medium.ttf";

    // Import the regular font
    let font_50pt = rl
        .load_font_ex(&thread, font_path, 100, None)
        .expect("Couldn't load font oof");
    font_50pt
        .texture()
        .set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);

    // Import the bold font
    let font_50pt_bold = rl
        .load_font_ex(&thread, bold_font_path, 100, None)
        .expect("Couldn't load font oof");
    font_50pt_bold
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
        game_rect: get_game_rect(&rl),
        ui_rect: get_ui_rect(&rl),
        fonts: Fonts {
            regular: font_50pt,
            bold: font_50pt_bold,
        }
    };

    // Get the pixel positions of each cell in the game, and each element in the UI
    g.update_positions();
    ui.update_positions(state.ui_rect);

    // Centre the camera
    g.centre_camera(state.game_rect);

    // Load the symbols
    g.player_1 = get_player_from_symbol(&ui.state.player_1);
    g.player_2 = get_player_from_symbol(&ui.state.player_2);
    update_window_title(&mut rl, &mut thread, &g);

    println!("//------Look Ma, I'm a hacker now!------//");

    while !rl.window_should_close() {
        // Get the time it took to render the last frame
        let delta = rl.get_frame_time();

        //----------// Handle input //----------//

        // Handle all input, returning the currently hovered cell
        let hovered_cell = handle_input(&mut rl, &mut thread, &mut g, &mut ui, &mut state);
        
        // If needed, call the AI
        if (g.players == 0 || (g.players == 1 && g.turn == Turn::Player2))
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

        // Send all queued messages
        for message in state.message_queue.drain(0..state.message_queue.len()) {
            tx.send(message).unwrap();
        }

        // Recieve any sent messages, and queue all moves
        let mv = rx.try_recv();
        match mv {
            Ok(mv) => {
                if state.waiting_for_move {
                    if let Message::Move(Some(y)) = mv {
                        state.move_queue.insert(0, y);
                        println!("{:?}", state.move_queue);
                    }
                }
            }
            Err(e) => match e {
                mpsc::TryRecvError::Empty => {}
                mpsc::TryRecvError::Disconnected => panic!("Thread disconnected"),
            },
        }

        // If the delay between moves is 0, play the next queued move
        if state.response_time <= 0.0 {
            if let Some(mv) = state.move_queue.pop() {
                println!("Some move");
                dbg!(&state.move_queue);
                g.play(&mv).unwrap();
                state.waiting_for_move = false;
            }
        }

        let gr = get_game_rect(&rl);
        let real_origin = rl.get_screen_to_world2D(Vector2 {x: gr.x, y: gr.y}, g.camera);
        let real_maximum = rl.get_screen_to_world2D(Vector2 {x: gr.x + gr.width, y: gr.y + gr.height}, g.camera);
        let on_screen_rect = Rectangle {
            x: real_origin.x,
            y: real_origin.y,
            width: real_maximum.x - real_origin.x,
            height: real_maximum.y - real_origin.y,
        };

        let mut d = rl.begin_drawing(&thread);

        // Set the background
        d.clear_background(Color::BLACK);

        // let world_coord = rl.get_screen_to_world2D(mouse_pos, g.camera);

        
        // Draw the game
        g.draw(
            get_board_rect(g.depth),
            &on_screen_rect,
            &gr,
            &mut d,
            false,
            true,
            hovered_cell.as_deref(),
        );

        // Draw the UI
        ui.draw(state.ui_rect, &mut d, &g, &state);

        // Draw the FPS counter
        if state.show_fps {
            d.draw_fps(10, 10);
        }

        // Decrement the response delay by the frame time
        state.response_time -= delta;
        if state.response_time < 0.0 {
            state.response_time = 0.0;
        }
    }

    Ok(())
}
