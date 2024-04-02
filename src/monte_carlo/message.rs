use super::monte_carlo::MonteCarloSettings;

/// Defines the messages that may be passed between the main and Monte Carlo threads
pub enum Message {
    /// Requests that the thread begin simulating
    Start(MonteCarloSettings),

    /// Requests that the Monte Carlo thread return a move
    GetMoveNow(),
    GetThoughts(),
    Move(Vec<usize>),
    Interrupt,
}
