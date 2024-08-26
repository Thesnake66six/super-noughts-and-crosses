use raylib::math::Rectangle;

pub struct GameElements {
    pub turn_display: Rectangle,
    pub padding_1: Rectangle,
    pub moves: Rectangle,
    pub padding_2: Rectangle,
    pub export: Rectangle,
}

impl GameElements {
    pub fn new() -> GameElements {
        GameElements {
            turn_display: Rectangle::EMPTY,
            padding_1: Rectangle::EMPTY,
            moves: Rectangle::EMPTY,
            padding_2: Rectangle::EMPTY,
            export: Rectangle::EMPTY,
        }
    }
}
