use std::{
    sync::mpsc,
    thread,
    time::Duration,
};

use anyhow::Result;
use raylib::{core::texture::RaylibTexture2D, prelude::*};
use styles::*;

use crate::{
    ai_thread::noughbert, common::*, game::{
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

fn main() -> Result<()> {
    // Main thread comms with Noughbert
    let (tx_0, rx_0) = mpsc::sync_channel::<Message>(0);

    // Noughbert comms with main thread
    let (tx_1, rx_1) = mpsc::sync_channel::<Message>(1);

    let _thread = thread::spawn(move || {
        noughbert(rx_0, tx_1)
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
    };

    // Get the pixel positions of each cell in the game, and each element in the UI
    g.update_positions();
    ui.update_positions(state.ui_rect);

    // Centre
    g.centre_camera(state.game_rect);

    println!("//------Look Ma, I'm a hacker now!------//");

    while !rl.window_should_close() {
        // Get the time it took to render the last frame
        let delta = rl.get_frame_time();

        //----------// Handle input //----------//

        let hovered_cell = handle_input(&mut rl, &mut g, &mut ui, &mut state);

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

        for message in state.message_queue.drain(0..state.message_queue.len()) {
            tx.send(message).unwrap();
        }

        let mv = rx.try_recv();
        match mv {
            Ok(mv) => {
                if state.waiting_for_move {
                    if let Message::Move(Some(y)) = mv {
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

        ui.draw(state.ui_rect, &mut d, &g, &font_50pt, &state);

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
