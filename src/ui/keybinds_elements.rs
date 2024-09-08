use raylib::math::Rectangle;

pub struct KeybindsElements {
    pub back: Rectangle,
    pub padding: Rectangle,
    pub binds: Rectangle,
}

impl KeybindsElements {
    pub fn new() -> KeybindsElements {
        KeybindsElements {
            back: Rectangle::EMPTY,
            padding: Rectangle::EMPTY,
            binds: Rectangle::EMPTY,
        }
    }
}
