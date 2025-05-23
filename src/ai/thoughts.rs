/// Struct holding the information returned from a `Message::GetThoughts()` message
#[derive(Debug, Clone, Copy)]
pub struct Thoughts {
    /// Number of simulations carried out on a move
    pub sims: usize,

    /// Number of wins on a move
    pub score: f32,
}