use crate::game::Turn;

#[derive(Debug)]
pub struct MonteCarloNode {
    pub play: Vec<usize>,
    pub playouts: f32,
    pub score: f32,
    pub child_count: usize,
    pub turn: Turn,
}

impl MonteCarloNode {
    pub fn new(play: Vec<usize>, child_count: usize, turn: Turn) -> MonteCarloNode {
        MonteCarloNode {
            play,
            playouts: 0.0,
            score: 0.0,
            child_count,
            turn,
        }
    }

    pub fn ucb1(&self, exploration_factor: f32, parent_playouts: f32, opt_for: Turn) -> f32 {
        if self.child_count == 0 {
            // eprintln!("({} / {}) + {} * sqrt(ln({}) / {})", self.wins, self.playouts, exploration_factor, parent_playouts, self.playouts);
            // eprintln!("{} + {} = {}", (self.wins / self.playouts), (exploration_factor * (parent_playouts.ln() / self.playouts).sqrt()), ((self.wins / self.playouts)
            // + exploration_factor * (parent_playouts.ln() / self.playouts).sqrt()));
        }
        (self.score(opt_for) / self.playouts)
            + (parent_playouts.ln() * exploration_factor / self.playouts).sqrt()
    }

    pub fn score(&self, opt_for: Turn) -> f32 {
        if self.turn == opt_for {
            self.score
        } else {
            -1.0 * self.score
        }
    }
}
