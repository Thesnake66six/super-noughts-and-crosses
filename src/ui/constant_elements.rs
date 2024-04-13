use raylib::math::Rectangle;

pub struct ConstantElements {
    pub game: Rectangle,
    pub settings: Rectangle,
    pub content: Rectangle,
    pub inner_content: Rectangle,
}

impl ConstantElements {
    pub fn new() -> ConstantElements {
        ConstantElements {
            game: Rectangle::EMPTY,
            settings: Rectangle::EMPTY,
            content: Rectangle::EMPTY,
            inner_content: Rectangle::EMPTY,
        }
    }
}