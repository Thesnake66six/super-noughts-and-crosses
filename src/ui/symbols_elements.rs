use raylib::math::Rectangle;

pub struct SymbolsElements {
    pub back: Rectangle,
    pub player_1: Rectangle,
    pub player_1_forward: Rectangle,
    pub player_1_backward: Rectangle,
    pub player_2: Rectangle,
    pub player_2_forward: Rectangle,
    pub player_2_backward: Rectangle,
}

impl SymbolsElements {
    pub fn new() -> SymbolsElements {
        SymbolsElements {
            back: Rectangle::EMPTY,
            player_1: Rectangle::EMPTY,
            player_1_forward: Rectangle::EMPTY,
            player_1_backward: Rectangle::EMPTY,
            player_2: Rectangle::EMPTY,
            player_2_forward: Rectangle::EMPTY,
            player_2_backward: Rectangle::EMPTY,
        }
    }
}
