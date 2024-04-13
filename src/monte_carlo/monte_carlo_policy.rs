#[derive(Debug, Clone, Copy)]
pub enum MonteCarloPolicy {
    /// Largest number of simulations
    Robust,
    /// Highest total value
    Maximum,
    /// Lowest number of simulations
    Frail,
    /// Lowest total value
    Minimum,
    /// A random move
    Random,
    /// Highest UCB1 value
    UCB1,
}