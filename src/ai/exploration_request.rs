use crate::{common::Move, game::game::Turn};

pub enum ExplorationRequest {
    Stop,
    Return {
        plays: Vec<Move>,
        result: f32,
        opt_for: Turn,
    },
}
