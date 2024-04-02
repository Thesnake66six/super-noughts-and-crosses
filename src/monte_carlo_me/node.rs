use crate::game::Move;

#[derive(Debug)]
pub struct MonteCarloNode {
    pub play: Move,
    pub playouts: f32,
    pub wins: f32,
    pub child_count: usize,
}

impl MonteCarloNode {
    pub fn new(play: Move, child_count: usize) -> MonteCarloNode {
        MonteCarloNode {
            play,
            playouts: 0.0,
            wins: 0.0,
            child_count,
        }
    }

    pub fn ucb1(&self, exploration_factor: f32, parent_playouts: f32) -> f32 {
        if self.child_count == 0 {
            // eprintln!("({} / {}) + {} * sqrt(ln({}) / {})", self.wins, self.playouts, exploration_factor, parent_playouts, self.playouts);
            // eprintln!("{} + {} = {}", (self.wins / self.playouts), (exploration_factor * (parent_playouts.ln() / self.playouts).sqrt()), ((self.wins / self.playouts)
            // + exploration_factor * (parent_playouts.ln() / self.playouts).sqrt()));
        }
        (self.wins / self.playouts)
            + (parent_playouts.ln() * exploration_factor / self.playouts).sqrt()

    }
}
