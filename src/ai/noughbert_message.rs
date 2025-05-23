use crate::game::game::Turn;

use super::{monte_carlo_settings::MonteCarloSettings, thoughts::Thoughts};

/// Defines the messages that may be passed between the main and Monte Carlo threads
pub enum NoughbertMessage {
    /// Requests that the thread begin simulating
    Start(MonteCarloSettings),

    /// Requests that the Monte Carlo thread return a move
    Return(),

    /// Requests the information on the current gamestate
    GetThoughts(Turn),

    /// Returns the information on the current gamestate
    Thoughts(Thoughts),

    /// Sends a move
    Move(Option<Vec<usize>>),

    /// Stops the calculation of a move
    Interrupt,
}

pub enum ExplorationRequest {
    Stop,
    Return { result: f32 },
}
