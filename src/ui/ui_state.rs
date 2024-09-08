use crate::{
    game::symbol::Symbol,
    styles::{
        COMPUTER_DEFAULT_STRENGTH, COMPUTER_LEVEL_1_SIMS, COMPUTER_LEVEL_2_SIMS,
        COMPUTER_LEVEL_3_SIMS, COMPUTER_SIM_SCALING,
    },
    BOARD_DEFAULT_DEPTH, BOARD_DEFAULT_PLAYERS, DEFAULT_MAX_TIME,
};

pub struct UIState {
    pub depth: usize,
    pub players: usize,
    pub ai_strength: usize,
    pub max_sims: usize,
    pub max_time: usize,
    pub is_ai_modified: bool,
    pub player_1: Symbol,
    pub player_2: Symbol,
}

impl UIState {
    pub fn new() -> UIState {
        UIState {
            depth: BOARD_DEFAULT_DEPTH,
            players: BOARD_DEFAULT_PLAYERS,
            ai_strength: COMPUTER_DEFAULT_STRENGTH,
            max_sims: {
                let l = match COMPUTER_DEFAULT_STRENGTH {
                    1 => COMPUTER_LEVEL_1_SIMS,
                    2 => COMPUTER_LEVEL_2_SIMS,
                    3 => COMPUTER_LEVEL_3_SIMS,
                    _ => 0,
                };
                l * (COMPUTER_SIM_SCALING.pow((BOARD_DEFAULT_DEPTH - 1).try_into().unwrap()))
            },
            max_time: DEFAULT_MAX_TIME,
            is_ai_modified: false,
            player_1: Symbol::Cross,
            player_2: Symbol::Nought,
        }
    }
}
