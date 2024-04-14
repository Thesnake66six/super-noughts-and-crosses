use std::fs;

use raylib::{ffi::MouseButton, math::Vector2, RaylibHandle, RaylibThread};

use crate::{
    common::{get_board_rect, get_player_from_symbol, update_window_title},
    game::game::{Game, Turn},
    noughbert::message::Message,
    state::{self, State},
    styles::{COMPUTER_LEVEL_1_SIMS, COMPUTER_LEVEL_2_SIMS, COMPUTER_LEVEL_3_SIMS, COMPUTER_RESPONSE_DELAY, COMPUTER_SIM_SCALING, RULES_URL},
    ui::{textbox::Textbox, ui::UI, ui_tab::UITab},
};

pub fn handle_click(
    rl: &mut RaylibHandle,
    rlthread: &mut RaylibThread,
    g: &mut Game,
    ui: &mut UI,
    state: &mut State,
    mouse_pos: Vector2,
    hovered_cell: &Option<Vec<usize>>,
) {
    if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
        state.typing = Textbox::None;
        if state.ui_rect.check_collision_point_rec(mouse_pos) {
            // This means that the mouse click was in the UI.
            // Oh boy, get ready
            if ui
                .constant_elements
                .game
                .check_collision_point_rec(mouse_pos)
            {
                // If the Game tab was clicked, change the tab to Game
                ui.tab = UITab::Game;
            } else if ui
                .constant_elements
                .settings
                .check_collision_point_rec(mouse_pos)
            {
                // If the Settings tab was clicked, change the tab to Settings
                ui.tab = UITab::Settings;
            } else if ui
                .constant_elements
                .inner_content
                .check_collision_point_rec(mouse_pos)
            {
                // If the tab content was clicked...
                match ui.tab {
                    UITab::Game => {
                        handle_game_tab_click(ui, mouse_pos, g, state);
                    }
                    UITab::Settings => {
                        handle_settings_tab_click(mouse_pos, ui, state, g, rl, rlthread);
                    }
                    UITab::Keybinds => {
                        handle_keybinds_tab_click(mouse_pos, ui)
                    },
                    UITab::Symbols => {
                        handle_symbols_tab_click(mouse_pos, ui, g, rl, rlthread)
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

fn handle_game_tab_click(ui: &mut UI, mouse_pos: Vector2, g: &mut Game, state: &mut State) {
    // Export the game to a file if Export is clicked
    if ui.game_elements.export.check_collision_point_rec(mouse_pos) {
        let game_serial = serde_json::to_string(g).unwrap();
        let _ = fs::create_dir("./exports");
        let filename =
            &format!("{:x}", md5::compute(game_serial.clone()))[..16];
        match fs::write(format!("./exports/{filename}.xo"), game_serial) {
            Ok(()) => {
                println!("Game exported as file \"./exports/{filename}.xo\"");
            }
            Err(_) => {
                eprintln!("Game export failed");
                state.can_export = false;
            }
        }
    }
}

fn handle_settings_tab_click(mouse_pos: Vector2, ui: &mut UI, state: &mut State, g: &mut Game, rl: &mut RaylibHandle, rlthread: &mut RaylibThread) {
    // Account for scroll offset
    let offset = Vector2 {
        x: mouse_pos.x,
        y: mouse_pos.y - ui.scroll_offset_settings,
    };
    // Increment the depth if Depth Plus is clicked
    if ui
        .settings_elements
        .depth_plus
        .check_collision_point_rec(offset)
    {
        ui.state.depth += 1;

    // Decrement the depth if Depth Minus is clicked, saturating at 1
    } else if ui
        .settings_elements
        .depth_minus
        .check_collision_point_rec(offset)
    {
        ui.state.depth -= 1;
        if ui.state.depth < 1 {
            ui.state.depth = 1;
        };

    // Set the player to 1 if Player 1 is clicked
    } else if ui
        .settings_elements
        .players_0
        .check_collision_point_rec(offset)
    {
        ui.state.players = 0;

    // Set the player to 1 if Player 1 is clicked
    } else if ui
        .settings_elements
        .players_1
        .check_collision_point_rec(offset)
    {
        ui.state.players = 1;

    // Set the player to 2 if Player 2 is clicked
    } else if ui
        .settings_elements
        .players_2
        .check_collision_point_rec(offset)
    {
        ui.state.players = 2;

    // Start a new Game with the selected settings if New Game is clicked
    } else if ui
        .settings_elements
        .new_game
        .check_collision_point_rec(offset)
    {
        // Stop any currently calculating moves
        state
            .message_queue
            .insert(state.message_queue.len(), Message::Interrupt);
        // Stop waiting to receive a move
        state.waiting_for_move = false;
        // Set a new game based on the current UI state
        *g = Game::new_depth(
            get_board_rect(ui.state.depth),
            ui.state.depth,
            ui.state.players,
        );
        // Re-initialise the game
        g.update_positions();
        g.centre_camera(state.game_rect);
        g.camera.offset = Vector2 {
            x: state.game_rect.width / 2.0f32,
            y: state.game_rect.height,
        };
        if !ui.state.is_ai_modified {
            let l = match ui.state.ai_strength {
                1 => { COMPUTER_LEVEL_1_SIMS },
                2 => { COMPUTER_LEVEL_2_SIMS },
                3 => { COMPUTER_LEVEL_3_SIMS },
                _ => { 0 }
            };
            ui.state.max_sims = l * (COMPUTER_SIM_SCALING.pow((g.depth - 1).try_into().unwrap()));
        }
        g.player_1 = get_player_from_symbol(&ui.state.player_1);
        g.player_2 = get_player_from_symbol(&ui.state.player_2);
        update_window_title(rl, rlthread, g);
        
        

    } else if ui.settings_elements.ai_1.check_collision_point_rec(offset) {
        ui.state.ai_strength = 1;
        ui.state.max_sims = COMPUTER_LEVEL_1_SIMS * (COMPUTER_SIM_SCALING.pow((g.depth - 1).try_into().unwrap()));
        ui.state.is_ai_modified = false
    } else if ui.settings_elements.ai_2.check_collision_point_rec(offset) {
        ui.state.ai_strength = 2;
        ui.state.max_sims = COMPUTER_LEVEL_2_SIMS * (COMPUTER_SIM_SCALING.pow((g.depth - 1).try_into().unwrap()));
        ui.state.is_ai_modified = false
    } else if ui.settings_elements.ai_3.check_collision_point_rec(offset) {
        ui.state.ai_strength = 3;
        ui.state.max_sims = COMPUTER_LEVEL_3_SIMS * (COMPUTER_SIM_SCALING.pow((g.depth - 1).try_into().unwrap()));
        ui.state.is_ai_modified = false
    } else if ui
        .settings_elements
        .ai_max_sims
        .check_collision_point_rec(offset)
    {
        state.typing = Textbox::MaxSims;
    } else if ui
        .settings_elements
        .ai_max_time
        .check_collision_point_rec(offset)
    {
        state.typing = Textbox::MaxTime;
    } else if ui.settings_elements.rules.check_collision_point_rec(offset) {
        let _ = webbrowser::open(RULES_URL);
    } else if ui.settings_elements.keybinds.check_collision_point_rec(offset) {
        ui.tab = UITab::Keybinds;
    } else if ui.settings_elements.symbols.check_collision_point_rec(offset) {
        ui.tab = UITab::Symbols;
    }
}

fn handle_keybinds_tab_click(mouse_pos: Vector2, ui: &mut UI) {
    if ui.keybinds_elements.back.check_collision_point_rec(mouse_pos) {
        ui.tab = UITab::Settings
    }
}

fn handle_symbols_tab_click(mouse_pos: Vector2, ui: &mut UI, g: &mut Game, rl: &mut RaylibHandle, rlthread: &mut RaylibThread) {
    let mut update = false;
    if ui.symbols_elements.back.check_collision_point_rec(mouse_pos) {
        ui.tab = UITab::Settings
    } else if ui.symbols_elements.player_1_backward.check_collision_point_rec(mouse_pos) {
        update = true;
        ui.state.player_1 = ui.state.player_1.prev();
    } else if ui.symbols_elements.player_1_forward.check_collision_point_rec(mouse_pos) {
        update = true;
        ui.state.player_1 = ui.state.player_1.next();
    } else if ui.symbols_elements.player_2_backward.check_collision_point_rec(mouse_pos) {
        update = true;
        ui.state.player_2 = ui.state.player_2.prev();
    } else if ui.symbols_elements.player_2_forward.check_collision_point_rec(mouse_pos) {
        update = true;
        ui.state.player_2 = ui.state.player_2.next();
    }

    if update {
        g.player_1 = get_player_from_symbol(&ui.state.player_1);
        g.player_2 = get_player_from_symbol(&ui.state.player_2);
        update_window_title(rl, rlthread, g)
    }
}