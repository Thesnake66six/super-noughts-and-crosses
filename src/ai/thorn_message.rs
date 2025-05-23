use crate::game::game::Turn;

use super::{monte_carlo_settings::MonteCarloSettings, thoughts::Thoughts};

pub enum ThornMessage {
    /// Starts position evaluation
    Start(MonteCarloSettings),

    /// Requests the information on the current gamestate
    GetThoughts(Turn),

    /// Returns the information on the current gamestate
    Thoughts(Thoughts),

    /// Sends a move
    Move(Option<Vec<usize>>),

    /// Calls for the tree to be clipped

    /// Stops the calculation of a move
    Interrupt,
}