use crate::game::Turn;

#[derive(Debug)]
pub struct MonteCarloNode {
    /// The move that the node represents
    pub play: Vec<usize>,
    /// The number of simulations where this move was made
    pub playouts: f32,
    /// The score of simulations
    pub score: f32,
    /// The number of children that node will have once fully expanded
    pub child_count: usize,
    /// The turn for which the node's move is
    pub turn: Turn,
}

impl MonteCarloNode {
    /// Constructor function
    pub fn new(play: Vec<usize>, child_count: usize, turn: Turn) -> MonteCarloNode {
        MonteCarloNode {
            play,
            playouts: 0.0,
            score: 0.0,
            child_count,
            turn,
        }
    }

    /// Calculates the UCB1 value for the node
    pub fn ucb1(&self, exploration_factor: f32, parent_playouts: f32, opt_for: Turn) -> f32 {
        (self.score(opt_for) / self.playouts)
            + (parent_playouts.ln() * exploration_factor / self.playouts).sqrt()
    }

    /// Calculates the relative score of the node based on the turn of the node
    pub fn score(&self, opt_for: Turn) -> f32 {
        if self.turn == opt_for {
            self.score
        } else {
            -1.0 * self.score
        }
    }
}
