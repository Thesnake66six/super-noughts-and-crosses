#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Textbox {
    /// The Max Sims textbox
    MaxSims,
    /// The Max Time textbox
    MaxTime,
    /// No textbox selected
    None,
}