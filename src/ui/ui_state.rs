use crate::{BOARD_DEFAULT_DEPTH, BOARD_DEFAULT_PLAYERS, DEFAULT_MAX_SIMS, DEFAULT_MAX_TIME};

pub struct UIState {
    pub depth: usize,
    pub players: usize,
    pub ai_strength: usize,
    pub max_sims: usize,
    pub max_time: usize,
}

impl UIState {
    pub fn new() -> UIState {
        UIState {
            depth: BOARD_DEFAULT_DEPTH,
            players: BOARD_DEFAULT_PLAYERS,
            ai_strength: 1,
            max_sims: DEFAULT_MAX_SIMS,
            max_time: DEFAULT_MAX_TIME,
        }
    }
}
