use raylib::math::Rectangle;

pub struct SettingsElements {
    pub depth: Rectangle,
    pub depth_plus: Rectangle,
    pub depth_minus: Rectangle,
    pub players: Rectangle,
    pub players_0: Rectangle,
    pub players_1: Rectangle,
    pub players_2: Rectangle,
    pub new_game: Rectangle,
    pub ai_strength: Rectangle,
    pub ai_1: Rectangle,
    pub ai_2: Rectangle,
    pub ai_3: Rectangle,
    pub ai_settings: Rectangle,
    pub ai_max_sims: Rectangle,
    pub ai_max_time: Rectangle,
    pub rules: Rectangle,
    pub keybinds: Rectangle,
    pub symbols: Rectangle,
}

impl SettingsElements {
    pub fn new() -> SettingsElements {
        SettingsElements {
            depth: Rectangle::EMPTY,
            depth_plus: Rectangle::EMPTY,
            depth_minus: Rectangle::EMPTY,
            players: Rectangle::EMPTY,
            players_0: Rectangle::EMPTY,
            players_1: Rectangle::EMPTY,
            players_2: Rectangle::EMPTY,
            new_game: Rectangle::EMPTY,
            ai_strength: Rectangle::EMPTY,
            ai_1: Rectangle::EMPTY,
            ai_2: Rectangle::EMPTY,
            ai_3: Rectangle::EMPTY,
            ai_settings: Rectangle::EMPTY,
            ai_max_sims: Rectangle::EMPTY,
            ai_max_time: Rectangle::EMPTY,
            rules: Rectangle::EMPTY,
            keybinds: Rectangle::EMPTY,
            symbols: Rectangle::EMPTY,
        }
    }
}
