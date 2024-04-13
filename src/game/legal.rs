#[derive(PartialEq, Clone, Copy)]
/// An enum used to represent the legal moves in the `Game` draw function
pub enum Legal<'a> {
    /// The relative coordinates of the legal board
    Pos(&'a [usize]),
    /// There are no legal moves below this point.
    None,
    /// Forces the default board background colour (`COLOUR_BOUARD_BG`).
    ForceDefaultBg,
}
