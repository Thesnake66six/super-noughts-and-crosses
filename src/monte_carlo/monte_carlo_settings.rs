use std::time::Duration;

use crate::game::game::{Game, Turn};

use super::monte_carlo_policy::MonteCarloPolicy;

#[derive(Debug, Clone)]
/// A struct to govern the settings of the AI
pub struct MonteCarloSettings {
    /// The game that is being evaluated
    pub game: Game,
    /// The maximum time allowed for calculation
    pub timeout: Duration,
    /// The maximum number of simulations allowed for calculation
    pub max_sims: usize,
    /// The exploration factor for the UCB1 algorithm
    pub exploration_factor: f32,
    /// The player for which the move should be optimised
    pub opt_for: Turn,
    /// Whether the tree should carry forward (unused)
    pub carry_forward: bool,
    /// The policy with which the move should be selected
    pub policy: MonteCarloPolicy,
}
