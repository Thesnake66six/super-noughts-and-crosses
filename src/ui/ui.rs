use raylib::{
    color::Color,
    drawing::RaylibDraw,
    math::{Rectangle, Vector2},
    text,
};

use crate::{
    common::centre_text_rec,
    game::{
        game::{Game, Turn},
        value::Value,
    },
    state::State,
    styles::{
        COLOUR_UI_BG, COLOUR_UI_BUTTON, COLOUR_UI_DIVIDER, COLOUR_UI_ELEMENT, COLOUR_UI_RADIAL,
        UI_BUTTON_LINE_THICKNESS, UI_CONTENT_PADDING, UI_DIVIDER_THICKNESS, UI_NAVBAR_HEIGHT,
        UI_PANEL_WIDTH,
    },
};

use super::{
    constant_elements::ConstantElements, game_elements::GameElements,
    keybinds_elements::KeybindsElements, settings_elements::SettingsElements,
    symbols_elements::SymbolsElements, textbox::Textbox, ui_state::UIState, ui_tab::UITab,
};

pub struct UI {
    /// The current selected `UITab`
    pub tab: UITab,
    /// The current amount that the game tab has scrolled
    pub scroll_offset_game: f32,
    /// The current amount that the settings tab has scrolled
    pub scroll_offset_settings: f32,
    /// The current amount that the settings tab has scrolled
    pub scroll_offset_keybinds: f32,
    /// Stores the positions of the constant elements
    pub constant_elements: ConstantElements,
    /// Stores the positions of the elements of the game tab
    pub game_elements: GameElements,
    /// Stores the positions of the elements of the settings tab
    pub settings_elements: SettingsElements,
    /// Stores the positions of the elements of the keybinds tab
    pub keybinds_elements: KeybindsElements,
    /// Stores the positions of the elements of the symbols tab
    pub symbols_elements: SymbolsElements,
    /// Stores the current UI state
    pub state: UIState,
}

impl UI {
    /// Returns a new UI
    pub fn new() -> Self {
        UI {
            tab: UITab::Game,
            scroll_offset_game: 0.0,
            scroll_offset_settings: 0.0,
            scroll_offset_keybinds: 0.0,
            constant_elements: ConstantElements::new(),
            game_elements: GameElements::new(),
            settings_elements: SettingsElements::new(),
            keybinds_elements: KeybindsElements::new(),
            symbols_elements: SymbolsElements::new(),
            state: UIState::new(),
        }
    }

    /// A function that updates the elements hashmaps' values.
    ///
    /// Called every frame before rendering, so as to save on `rl.get_screen_size()` calls
    pub fn update_positions(&mut self, rect: Rectangle) {
        // Calculate the size of the padding in pixels
        let padding = UI_CONTENT_PADDING * rect.width;

        // Calculate the constant elements' positions
        let navbar_tab_width = (rect.width - UI_DIVIDER_THICKNESS as f32) / 2.0;

        let r = Rectangle {
            x: rect.x,
            y: rect.y,
            width: navbar_tab_width,
            height: UI_NAVBAR_HEIGHT as f32,
        };
        self.constant_elements.game = r;

        let r = Rectangle {
            x: r.x + r.width + UI_DIVIDER_THICKNESS as f32,
            y: r.y,
            width: r.width,
            height: r.height,
        };
        self.constant_elements.settings = r;

        let r = Rectangle {
            x: rect.x,
            y: rect.y + UI_NAVBAR_HEIGHT as f32 + UI_DIVIDER_THICKNESS as f32,
            width: rect.width,
            height: rect.height - UI_NAVBAR_HEIGHT as f32 - UI_DIVIDER_THICKNESS as f32,
        };
        self.constant_elements.content = r;

        let content_pading = UI_CONTENT_PADDING * UI_PANEL_WIDTH as f32;

        let inner_content = Rectangle {
            x: r.x + content_pading,
            y: r.y + content_pading,
            width: r.width - 2.0 * content_pading,
            height: r.height - 2.0 * content_pading,
        };
        self.constant_elements.inner_content = inner_content;

        // Calculate the game elements' positions --------

        // Calculate the position of the Turn Display
        let r = Rectangle {
            x: rect.x + padding,
            y: rect.y + padding + UI_NAVBAR_HEIGHT as f32 + UI_DIVIDER_THICKNESS as f32,
            width: rect.width - 2.0 * padding,
            height: 100.0,
        };
        self.game_elements.turn_display = r;

        // Calculate the position of the Padding between the Turn Display and the Moves
        let p = Rectangle {
            x: r.x,
            y: r.y + r.height,
            width: r.width,
            height: padding,
        };
        self.game_elements.padding_1 = p;

        // Calculate the position of the Moves list
        let r = Rectangle {
            x: r.x,
            y: r.y + r.height + padding,
            width: r.width,
            height: inner_content.height - r.height * 2.0 - p.height * 2.0,
        };
        self.game_elements.moves = r;

        let p = Rectangle {
            x: p.x,
            y: r.y + r.height,
            width: p.width,
            height: padding,
        };
        self.game_elements.padding_2 = p;

        let r = Rectangle {
            x: r.x,
            y: p.y + p.height,
            width: r.width,
            height: 100.0,
        };
        self.game_elements.export = r;

        // Calculate the settings elements' positions --------

        // Calculate the position of the Depth buttons
        let r = Rectangle {
            x: rect.x + padding,
            y: rect.y + padding + UI_NAVBAR_HEIGHT as f32 + UI_DIVIDER_THICKNESS as f32,
            width: rect.width - 2.0 * padding,
            height: 100.0,
        };
        self.settings_elements.depth = r;

        // Calculate the position of the Players selection buttons
        let r = Rectangle {
            x: r.x,
            y: r.y + r.height + padding,
            width: r.width,
            height: 300.0,
        };
        self.settings_elements.players = r;

        // Calculate the position of the New Game button
        let r = Rectangle {
            x: r.x,
            y: r.y + r.height + padding,
            width: r.width,
            height: 100.0,
        };
        self.settings_elements.new_game = r;

        // Calculate the position of the AI strength buttons
        let r = Rectangle {
            x: r.x,
            y: r.y + r.height + padding,
            width: r.width,
            height: 200.0,
        };
        self.settings_elements.ai_strength = r;

        let r = Rectangle {
            x: r.x,
            y: r.y + r.height + padding,
            width: r.width,
            height: r.height,
        };
        self.settings_elements.ai_settings = r;

        // Calculate the position of the Rules button
        let r = Rectangle {
            x: r.x,
            y: r.y + r.height + padding,
            width: r.width,
            height: 100.0,
        };
        self.settings_elements.rules = r;

        // Calculate the position of the Keybinds button
        let r = Rectangle {
            x: r.x,
            y: r.y + r.height + padding,
            width: r.width,
            height: 100.0,
        };
        self.settings_elements.keybinds = r;

        // Calculate the position of the Symbols button
        let r = Rectangle {
            x: r.x,
            y: r.y + r.height + padding,
            width: r.width,
            height: 100.0,
        };
        self.settings_elements.symbols = r;

        // Calculate the keybinds elements positions

        // Calculate the position of the Back button
        let r = Rectangle {
            x: rect.x + padding,
            y: rect.y + padding + UI_NAVBAR_HEIGHT as f32 + UI_DIVIDER_THICKNESS as f32,
            width: rect.width - 2.0 * padding,
            height: 100.0,
        };
        self.keybinds_elements.back = r;

        // Calculate the position of the Padding
        let p = Rectangle {
            x: r.x,
            y: r.y + r.height,
            width: r.width,
            height: padding,
        };
        self.keybinds_elements.padding = p;

        // Calculate the position of the binds element
        let r = Rectangle {
            x: r.x,
            y: r.y + r.height + padding,
            width: r.width,
            height: 700.0,
        };
        self.keybinds_elements.binds = r;

        // Calculate the Symbols elements positions

        // Calculate the position of the Back button
        let r = Rectangle {
            x: rect.x + padding,
            y: rect.y + padding + UI_NAVBAR_HEIGHT as f32 + UI_DIVIDER_THICKNESS as f32,
            width: rect.width - 2.0 * padding,
            height: 100.0,
        };
        self.symbols_elements.back = r;

        let r = Rectangle {
            x: r.x,
            y: r.y + r.height + padding,
            width: r.width,
            height: 200.0,
        };
        self.symbols_elements.player_1 = r;

        let r = Rectangle {
            x: r.x,
            y: r.y + r.height + padding,
            width: r.width,
            height: 200.0,
        };
        self.symbols_elements.player_2 = r;

        // Calculate the positions of the clickable content -------

        let padding = UI_CONTENT_PADDING * self.constant_elements.inner_content.width;

        let dp = self.settings_elements.depth;
        let button_side = dp.height - 2.0 * padding;

        let r = Rectangle {
            x: dp.x + dp.width - 2.0 * padding - 2.0 * button_side,
            y: dp.y + padding,
            width: button_side,
            height: button_side,
        };
        self.settings_elements.depth_minus = r;

        let r = Rectangle {
            x: r.x + r.width + padding,
            y: r.y,
            width: r.width,
            height: r.height,
        };
        self.settings_elements.depth_plus = r;

        // Calculate the position of the player selection
        let pl = self.settings_elements.players;

        let r = Rectangle {
            x: pl.x + padding + (pl.width - 2.0 * padding) - button_side,
            y: pl.y + padding,
            width: 100.0 - 2.0 * padding,
            height: 100.0 - 2.0 * padding,
        };
        self.settings_elements.players_0 = r;

        let r = Rectangle {
            x: pl.x + padding + (pl.width - 2.0 * padding) - button_side,
            y: pl.y + 2.0 * padding + (pl.height - 3.0 * padding) / 3.0,
            width: 100.0 - 2.0 * padding,
            height: 100.0 - 2.0 * padding,
        };
        self.settings_elements.players_1 = r;

        let r = Rectangle {
            x: pl.x + padding + (pl.width - 2.0 * padding) - button_side,
            y: pl.y + 3.0 * padding + ((pl.height - 3.0 * padding) / 3.0) * 2.0,
            width: 100.0 - 2.0 * padding,
            height: 100.0 - 2.0 * padding,
        };
        self.settings_elements.players_2 = r;

        // Calculate the position of the AI strength values
        let ai = self.settings_elements.ai_strength;
        let column_width = (ai.width - 2.0 * padding) / 3.0;
        let r = Rectangle {
            x: ai.x + column_width - button_side,
            y: ai.y + 3.0 * padding + button_side,
            width: button_side,
            height: button_side,
        };
        self.settings_elements.ai_1 = r;

        let r = Rectangle {
            x: r.x + column_width,
            y: r.y,
            width: button_side,
            height: button_side,
        };
        self.settings_elements.ai_2 = r;

        let r = Rectangle {
            x: r.x + column_width,
            y: r.y,
            width: button_side,
            height: button_side,
        };
        self.settings_elements.ai_3 = r;

        // Calculate positions of the AI text boxes
        let ai = self.settings_elements.ai_settings;
        let column_width = (ai.width - padding) / 2.0;
        let p = column_width * UI_CONTENT_PADDING;
        let r = Rectangle {
            x: ai.x + column_width + p,
            y: ai.y + (100.0 - button_side) / 2.0,
            width: column_width - p,
            height: button_side,
        };
        self.settings_elements.ai_max_sims = r;

        let r = Rectangle {
            x: r.x,
            y: r.y + 100.0,
            width: column_width - p,
            height: button_side,
        };
        self.settings_elements.ai_max_time = r;

        let p1 = self.symbols_elements.player_1;
        let b = Rectangle {
            x: p1.x + padding,
            y: p1.y + padding + 100.0,
            width: button_side,
            height: button_side,
        };
        self.symbols_elements.player_1_backward = b;
        let f = Rectangle {
            x: p1.x + p1.width - padding - button_side,
            y: p1.y + padding + 100.0,
            width: button_side,
            height: button_side,
        };
        self.symbols_elements.player_1_forward = f;

        let p2 = self.symbols_elements.player_2;
        let b = Rectangle {
            x: p2.x + padding,
            y: p2.y + padding + 100.0,
            width: button_side,
            height: button_side,
        };
        self.symbols_elements.player_2_backward = b;
        let f = Rectangle {
            x: p2.x + p1.width - padding - button_side,
            y: p2.y + padding + 100.0,
            width: button_side,
            height: button_side,
        };
        self.symbols_elements.player_2_forward = f;
    }

    /// Draws the constant elements onto the screen
    pub fn draw<T: RaylibDraw>(&self, rect: Rectangle, d: &mut T, g: &Game, state: &State) {
        // Draw the background for the UI
        d.draw_rectangle_rec(rect, COLOUR_UI_BG);

        let content_rec = self.constant_elements.content;

        let content_rec_inner = self.constant_elements.inner_content;

        match self.tab {
            UITab::Game => self.draw_game(content_rec_inner, d, g, state),
            UITab::Settings => self.draw_settings(content_rec_inner, d, g, state),
            UITab::Keybinds => self.draw_keybinds(rect, d, g, state),
            UITab::Symbols => self.draw_symbols(rect, d, g, state),
            UITab::None => {}
        }

        // Redraw the padding of the tab content and navbar (with divider) to remove any overspill
        d.draw_rectangle_lines_ex(
            content_rec,
            UI_CONTENT_PADDING * UI_PANEL_WIDTH as f32,
            COLOUR_UI_BG,
        );
        d.draw_rectangle_rec(
            Rectangle {
                x: rect.x,
                y: rect.y,
                width: rect.width,
                height: UI_NAVBAR_HEIGHT as f32 + UI_DIVIDER_THICKNESS as f32,
            },
            COLOUR_UI_BG,
        );

        let tab_rect = self.constant_elements.game;

        d.draw_rectangle_rec(
            tab_rect,
            match self.tab {
                UITab::Game => COLOUR_UI_BG,
                _ => COLOUR_UI_ELEMENT,
            },
        );

        let text_rec = centre_text_rec(&state.fonts.regular, "Game", 50.0, 0.0, tab_rect);

        // d.draw_text_ex(&state.fonts.regular, "Game", Vector2 { x: text_rec.x, y: text_rec.y }, 50.0, 0.0, Color::BLACK);
        d.draw_text_ex(
            &state.fonts.regular,
            "Game",
            Vector2 {
                x: text_rec.x,
                y: text_rec.y,
            },
            50.0,
            0.0,
            Color::BLACK,
        );

        // Draw the Settings tab button
        let tab_rect = self.constant_elements.settings;

        d.draw_rectangle_rec(
            tab_rect,
            match self.tab {
                UITab::Settings => COLOUR_UI_BG,
                _ => COLOUR_UI_ELEMENT,
            },
        );

        let text_rec = centre_text_rec(&state.fonts.regular, "Settings", 50.0, 0.0, tab_rect);

        d.draw_text_ex(
            &state.fonts.regular,
            "Settings",
            Vector2 {
                x: text_rec.x,
                y: text_rec.y,
            },
            50.0,
            0.0,
            Color::BLACK,
        );

        // Draw the lower divider based on the selected tab
        if self.tab != UITab::Game {
            d.draw_rectangle_rec(
                Rectangle {
                    x: rect.x,
                    y: rect.y + UI_NAVBAR_HEIGHT as f32,
                    width: rect.width * 0.5,
                    height: UI_DIVIDER_THICKNESS as f32,
                },
                COLOUR_UI_DIVIDER,
            );
        }
        if self.tab != UITab::Settings {
            d.draw_rectangle_rec(
                Rectangle {
                    x: rect.x + rect.width * 0.5,
                    y: rect.y + UI_NAVBAR_HEIGHT as f32,
                    width: rect.width * 0.5,
                    height: UI_DIVIDER_THICKNESS as f32,
                },
                COLOUR_UI_DIVIDER,
            );
        }

        d.draw_line_ex(
            Vector2 {
                x: rect.x + rect.width * 0.5,
                y: rect.y,
            },
            Vector2 {
                x: rect.x + rect.width * 0.5,
                y: rect.y + UI_NAVBAR_HEIGHT as f32 + UI_DIVIDER_THICKNESS as f32,
            },
            UI_DIVIDER_THICKNESS as f32,
            COLOUR_UI_DIVIDER,
        );
    }

    /// Draws the game tab
    pub fn draw_game<T: RaylibDraw>(&self, rect: Rectangle, d: &mut T, g: &Game, state: &State) {
        // Draw the move history
        let mv = self.game_elements.moves;

        d.draw_rectangle_rec(mv, COLOUR_UI_ELEMENT);

        for (i, x) in g.moves.iter().enumerate() {
            let rect = Rectangle {
                x: mv.x
                    + if i % 2 == 1 { mv.width * 0.5 } else { 0.0 }
                    + (UI_CONTENT_PADDING * 75.0),
                y: mv.y
                    + self.scroll_offset_game
                    + (75.0 * f32::floor(i as f32 / 2.0))
                    + (UI_CONTENT_PADDING * 75.0),
                width: mv.width * 0.5 - (UI_CONTENT_PADDING * 75.0 * 2.0),
                height: 75.0 - (UI_CONTENT_PADDING * 75.0 * 2.0),
            };

            d.draw_rectangle_rec(
                rect,
                if i % 2 == 1 {
                    g.player_2.foreground
                } else {
                    g.player_1.foreground
                },
            );
            let t = x[0]
                .iter()
                .map(|x| (x + 1).to_string())
                .collect::<Vec<String>>()
                .join(", ");
            let r = centre_text_rec(&state.fonts.regular, &t, 50.0, 0.0, rect);
            d.draw_text_ex(
                &state.fonts.regular,
                &t,
                Vector2 { x: r.x, y: r.y },
                50.0,
                0.0,
                Color::BLACK,
            );
        }

        // Redraw the blank padding
        let p = self.game_elements.padding_1;

        d.draw_rectangle_rec(p, COLOUR_UI_BG);

        // Draw the turn counter
        let tc = self.game_elements.turn_display;
        d.draw_rectangle_rec(tc, COLOUR_UI_ELEMENT);

        if g.board.check() != Value::None {
            let r = g.board.check();
            let p1text = g.player_2.symbol.name() + " Win";
            let p2text = g.player_2.symbol.name() + " Win";
            let text = match r {
                Value::None => "Hardware error encountered",
                Value::Draw => "Draw",
                Value::Player1 => &p2text,
                Value::Player2 => &p1text,
            };
            let rec = centre_text_rec(&state.fonts.regular, text, 50.0, 0.0, tc);
            d.draw_text_ex(
                &state.fonts.regular,
                text,
                Vector2 { x: rec.x, y: rec.y },
                50.0,
                0.0,
                match r {
                    Value::None => Color::RED,
                    Value::Draw => Color::BLACK,
                    Value::Player1 => g.player_1.foreground,
                    Value::Player2 => g.player_2.foreground,
                },
            );
        } else if g.turn == Turn::Player1 {
            let text = &(g.player_1.symbol.name_apostrophe() + " Turn");
            let rec = centre_text_rec(&state.fonts.regular, text, 50.0, 0.0, tc);

            d.draw_text_ex(
                &state.fonts.regular,
                text,
                Vector2 { x: rec.x, y: rec.y },
                50.0,
                0.0,
                g.player_1.foreground,
            );
        } else {
            let text = &(g.player_2.symbol.name_apostrophe() + " Turn");
            let rec = centre_text_rec(&state.fonts.regular, text, 50.0, 0.0, tc);
            d.draw_text_ex(
                &state.fonts.regular,
                text,
                Vector2 { x: rec.x, y: rec.y },
                50.0,
                0.0,
                g.player_2.foreground,
            );
        }

        let p = self.game_elements.padding_2;
        d.draw_rectangle_rec(p, COLOUR_UI_BG);

        let eb = self.game_elements.export;
        d.draw_rectangle_rec(eb, COLOUR_UI_ELEMENT);
        let text = "Export game";
        let trec = centre_text_rec(&state.fonts.regular, text, 50.0, 0.0, eb);
        d.draw_text_ex(
            &state.fonts.regular,
            text,
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            50.0,
            0.0,
            if state.can_export {
                Color::BLACK
            } else {
                Color::RED
            },
        );
    }

    /// Draw the settings tab
    pub fn draw_settings<T: RaylibDraw>(
        &self,
        rect: Rectangle,
        d: &mut T,
        g: &Game,
        state: &State,
    ) {
        let padding = UI_CONTENT_PADDING * rect.width;

        // Draw the depth selector
        let mut dp = self.settings_elements.depth;
        dp.y += self.scroll_offset_settings;
        let button_side = dp.height - 2.0 * padding;
        let text_rec = Rectangle {
            x: dp.x + padding,
            y: dp.y + padding,
            width: dp.width - 4.0 * padding - 2.0 * button_side,
            height: button_side,
        };
        let text = "Depth: ".to_owned() + &self.state.depth.to_string();

        // Draw the background
        d.draw_rectangle_rec(dp, COLOUR_UI_ELEMENT);

        // Change the colour of the text based on the current depth
        let colour = if self.state.depth >= 6 {
            Color::RED
        } else {
            Color::BLACK
        };
        // Draw the current depth text
        d.draw_text_ex(
            &state.fonts.regular,
            &text,
            Vector2 {
                x: text_rec.x,
                y: text_rec.y,
            },
            50.0,
            0.0,
            colour,
        );

        // Draw the buttons
        let mut brec = self.settings_elements.depth_plus;
        brec.y += self.scroll_offset_settings;
        d.draw_rectangle_rec(brec, COLOUR_UI_BUTTON);
        let p = button_side * UI_CONTENT_PADDING * 2.0;
        d.draw_line_ex(
            Vector2 {
                x: brec.x + p,
                y: brec.y + 0.5 * brec.height,
            },
            Vector2 {
                x: brec.x + brec.width - p,
                y: brec.y + 0.5 * brec.height,
            },
            UI_BUTTON_LINE_THICKNESS as f32,
            Color::BLACK,
        );
        d.draw_line_ex(
            Vector2 {
                x: brec.x + 0.5 * brec.width,
                y: brec.y + p,
            },
            Vector2 {
                x: brec.x + 0.5 * brec.width,
                y: brec.y + brec.height - p,
            },
            UI_BUTTON_LINE_THICKNESS as f32,
            Color::BLACK,
        );

        let mut brec = self.settings_elements.depth_minus;
        brec.y += self.scroll_offset_settings;
        d.draw_rectangle_rec(brec, COLOUR_UI_BUTTON);
        d.draw_line_ex(
            Vector2 {
                x: brec.x + p,
                y: brec.y + 0.5 * brec.height,
            },
            Vector2 {
                x: brec.x + brec.width - p,
                y: brec.y + 0.5 * brec.height,
            },
            UI_BUTTON_LINE_THICKNESS as f32,
            Color::BLACK,
        );

        // Draw Players selection
        let mut pl = self.settings_elements.players;
        pl.y += self.scroll_offset_settings;

        d.draw_rectangle_rec(pl, COLOUR_UI_ELEMENT);
        let button_side = 100.0 - 2.0 * padding;

        let p0 = Rectangle {
            x: pl.x + padding,
            y: pl.y + padding,
            width: pl.width - 2.0 * padding,
            height: (pl.height - 4.0 * padding) / 3.0,
        };

        let mut brec = self.settings_elements.players_0;
        brec.y += self.scroll_offset_settings;

        d.draw_rectangle_rec(brec, COLOUR_UI_BUTTON);
        if self.state.players == 0 {
            d.draw_rectangle_rec(
                Rectangle {
                    x: brec.x + p,
                    y: brec.y + p,
                    width: brec.width - 2.0 * p,
                    height: brec.height - 2.0 * p,
                },
                COLOUR_UI_RADIAL,
            );
        }

        let trec = Rectangle {
            x: p0.x,
            y: p0.y,
            width: p0.width - button_side - p,
            height: button_side,
        };
        let trec = centre_text_rec(&state.fonts.regular, "0 Players", 50.0, 0.0, trec);
        d.draw_text_ex(
            &state.fonts.regular,
            "0 Players",
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            50.0,
            0.0,
            Color::BLACK,
        );

        let p1 = Rectangle {
            x: pl.x + padding,
            y: pl.y + 2.0 * padding + (pl.height - 4.0 * padding) / 3.0,
            width: pl.width - 2.0 * padding,
            height: (pl.height - 4.0 * padding) / 3.0,
        };

        let mut brec = self.settings_elements.players_1;
        brec.y += self.scroll_offset_settings;

        d.draw_rectangle_rec(brec, COLOUR_UI_BUTTON);
        let p = button_side * UI_CONTENT_PADDING * 2.0;
        if self.state.players == 1 {
            d.draw_rectangle_rec(
                Rectangle {
                    x: brec.x + p,
                    y: brec.y + p,
                    width: brec.width - 2.0 * p,
                    height: brec.height - 2.0 * p,
                },
                COLOUR_UI_RADIAL,
            );
        }
        let trec = Rectangle {
            x: p1.x,
            y: p1.y,
            width: p1.width - button_side - p,
            height: button_side,
        };
        let trec = centre_text_rec(&state.fonts.regular, "1 Player", 50.0, 0.0, trec);
        d.draw_text_ex(
            &state.fonts.regular,
            "1 Player",
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            50.0,
            0.0,
            Color::BLACK,
        );

        let p2 = Rectangle {
            x: pl.x + padding,
            y: pl.y + 3.0 * padding + ((pl.height - 4.0 * padding) / 3.0) * 2.0,
            width: pl.width - 2.0 * padding,
            height: (pl.height - 4.0 * padding) / 3.0,
        };

        let mut brec = self.settings_elements.players_2;
        brec.y += self.scroll_offset_settings;

        d.draw_rectangle_rec(brec, COLOUR_UI_BUTTON);
        let inner_button_padding = button_side * UI_CONTENT_PADDING * 2.0;
        if self.state.players == 2 {
            d.draw_rectangle_rec(
                Rectangle {
                    x: brec.x + inner_button_padding,
                    y: brec.y + inner_button_padding,
                    width: brec.width - 2.0 * inner_button_padding,
                    height: brec.height - 2.0 * inner_button_padding,
                },
                COLOUR_UI_RADIAL,
            );
        }
        let trec = Rectangle {
            x: p2.x,
            y: p2.y,
            width: p1.width - button_side - inner_button_padding,
            height: button_side,
        };
        let trec = centre_text_rec(&state.fonts.regular, "2 Players", 50.0, 0.0, trec);
        d.draw_text_ex(
            &state.fonts.regular,
            "2 Players",
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            50.0,
            0.0,
            Color::BLACK,
        );

        // Draw New Game button
        let mut ng = self.settings_elements.new_game;
        ng.y += self.scroll_offset_settings;

        d.draw_rectangle_rec(ng, COLOUR_UI_ELEMENT);

        let trec = centre_text_rec(&state.fonts.regular, "New game", 50.0, 0.0, ng);
        d.draw_text_ex(
            &state.fonts.regular,
            "New game",
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            50.0,
            0.0,
            Color::BLACK,
        );

        // Draw AI strength buttons
        let mut ai = self.settings_elements.ai_strength;
        ai.y += self.scroll_offset_settings;
        d.draw_rectangle_rec(ai, COLOUR_UI_ELEMENT);

        let text = "AI Strength:";
        let text_rec = Rectangle {
            x: ai.x + padding,
            y: ai.y + padding,
            width: ai.width - 2.0 * padding,
            height: button_side,
        };

        let text_rec = centre_text_rec(&state.fonts.regular, text, 50.0, 0.0, text_rec);
        d.draw_text_ex(
            &state.fonts.regular,
            text,
            Vector2 {
                x: text_rec.x,
                y: text_rec.y,
            },
            50.0,
            0.0,
            Color::BLACK,
        );

        let column_width = (ai.width - 2.0 * padding) / 3.0;

        let mut a1 = self.settings_elements.ai_1;
        a1.y += self.scroll_offset_settings;
        d.draw_rectangle_rec(a1, COLOUR_UI_BUTTON);
        let p = button_side * UI_CONTENT_PADDING * 2.0;
        if self.state.ai_strength == 1 && !self.state.is_ai_modified {
            d.draw_rectangle_rec(
                Rectangle {
                    x: a1.x + p,
                    y: a1.y + p,
                    width: a1.width - 2.0 * p,
                    height: a1.height - 2.0 * p,
                },
                COLOUR_UI_RADIAL,
            );
        }

        let text = "1";
        let trec = Rectangle {
            x: a1.x - column_width + button_side,
            y: a1.y,
            width: column_width - a1.width,
            height: a1.height,
        };
        let ntrec = centre_text_rec(&state.fonts.regular, text, 50.0, 0.0, trec);
        d.draw_text_ex(
            &state.fonts.regular,
            text,
            Vector2 {
                x: ntrec.x,
                y: ntrec.y,
            },
            50.0,
            0.0,
            Color::BLACK,
        );

        let mut a2 = self.settings_elements.ai_2;
        a2.y += self.scroll_offset_settings;
        d.draw_rectangle_rec(a2, COLOUR_UI_BUTTON);
        let p = button_side * UI_CONTENT_PADDING * 2.0;
        if self.state.ai_strength == 2 && !self.state.is_ai_modified {
            d.draw_rectangle_rec(
                Rectangle {
                    x: a2.x + p,
                    y: a2.y + p,
                    width: a2.width - 2.0 * p,
                    height: a2.height - 2.0 * p,
                },
                COLOUR_UI_RADIAL,
            );
        }

        let text = "2";
        let trec = Rectangle {
            x: a2.x - column_width + button_side,
            y: a2.y,
            width: column_width - a2.width,
            height: a2.height,
        };
        let ntrec = centre_text_rec(&state.fonts.regular, text, 50.0, 0.0, trec);
        d.draw_text_ex(
            &state.fonts.regular,
            text,
            Vector2 {
                x: ntrec.x,
                y: ntrec.y,
            },
            50.0,
            0.0,
            Color::BLACK,
        );

        let mut a3 = self.settings_elements.ai_3;
        a3.y += self.scroll_offset_settings;
        d.draw_rectangle_rec(a3, COLOUR_UI_BUTTON);
        let p = button_side * UI_CONTENT_PADDING * 2.0;
        if self.state.ai_strength == 3 && !self.state.is_ai_modified {
            d.draw_rectangle_rec(
                Rectangle {
                    x: a3.x + p,
                    y: a3.y + p,
                    width: a3.width - 2.0 * p,
                    height: a3.height - 2.0 * p,
                },
                COLOUR_UI_RADIAL,
            );
        }

        let text = "3";
        let trec = Rectangle {
            x: a3.x - column_width + button_side,
            y: a3.y,
            width: column_width - a3.width,
            height: a3.height,
        };
        let ntrec = centre_text_rec(&state.fonts.regular, text, 50.0, 0.0, trec);
        d.draw_text_ex(
            &state.fonts.regular,
            text,
            Vector2 {
                x: ntrec.x,
                y: ntrec.y,
            },
            50.0,
            0.0,
            Color::BLACK,
        );

        let mut ai = self.settings_elements.ai_settings;
        ai.y += self.scroll_offset_settings;
        d.draw_rectangle_rec(ai, COLOUR_UI_ELEMENT);
        let column_width = (ai.width - padding) / 2.0;
        let p = column_width * UI_CONTENT_PADDING;
        let trec = Rectangle {
            x: ai.x + padding,
            y: ai.y + padding,
            width: column_width,
            height: 100.0,
        };
        let text = "Max sims:";
        d.draw_text_ex(
            &state.fonts.regular,
            text,
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            50.0,
            0.0,
            Color::BLACK,
        );
        let mut tbox = self.settings_elements.ai_max_sims;
        tbox.y += self.scroll_offset_settings;
        d.draw_rectangle_rec(tbox, COLOUR_UI_BUTTON);
        let text = format!(
            "{}{}",
            self.state.max_sims,
            if state.typing == Textbox::MaxSims {
                "_"
            } else {
                ""
            }
        );
        let font = if state.typing == Textbox::MaxSims {
            &state.fonts.bold
        } else {
            &state.fonts.regular
        };

        d.draw_text_ex(
            font,
            &text,
            Vector2 {
                x: tbox.x,
                y: tbox.y,
            },
            50.0,
            0.0,
            Color::BLACK,
        );

        let trec = Rectangle {
            x: trec.x,
            y: trec.y + 100.0,
            width: column_width,
            height: 100.0,
        };
        let text = "Max time:";
        d.draw_text_ex(
            &state.fonts.regular,
            text,
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            50.0,
            0.0,
            Color::BLACK,
        );
        let mut tbox = self.settings_elements.ai_max_time;
        tbox.y += self.scroll_offset_settings;
        d.draw_rectangle_rec(tbox, COLOUR_UI_BUTTON);
        let text = format!(
            "{}{}",
            self.state.max_time,
            if state.typing == Textbox::MaxTime {
                "_"
            } else {
                ""
            }
        );
        let font = if state.typing == Textbox::MaxTime {
            &state.fonts.bold
        } else {
            &state.fonts.regular
        };

        d.draw_text_ex(
            font,
            &text,
            Vector2 {
                x: tbox.x,
                y: tbox.y,
            },
            50.0,
            0.0,
            Color::BLACK,
        );

        let mut rs = self.settings_elements.rules;
        rs.y += self.scroll_offset_settings;
        d.draw_rectangle_rec(rs, COLOUR_UI_ELEMENT);
        let text = "Rules";
        let trec = centre_text_rec(&state.fonts.regular, text, 50.0, 0.0, rs);
        d.draw_text_ex(
            &state.fonts.regular,
            text,
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            50.0,
            0.0,
            Color::BLACK,
        );

        let mut kb = self.settings_elements.keybinds;
        kb.y += self.scroll_offset_settings;
        d.draw_rectangle_rec(kb, COLOUR_UI_ELEMENT);
        let text = "Keybinds";
        let trec = centre_text_rec(&state.fonts.regular, text, 50.0, 0.0, kb);
        d.draw_text_ex(
            &state.fonts.regular,
            text,
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            50.0,
            0.0,
            Color::BLACK,
        );

        let mut sy = self.settings_elements.symbols;
        sy.y += self.scroll_offset_settings;
        d.draw_rectangle_rec(sy, COLOUR_UI_ELEMENT);
        let text = "Symbols";
        let trec = centre_text_rec(&state.fonts.regular, text, 50.0, 0.0, sy);
        d.draw_text_ex(
            &state.fonts.regular,
            text,
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            50.0,
            0.0,
            Color::BLACK,
        );
    }

    pub fn draw_keybinds<T: RaylibDraw>(
        &self,
        rect: Rectangle,
        d: &mut T,
        g: &Game,
        state: &State,
    ) {
        let mut kb = self.keybinds_elements.binds;

        kb.y += self.scroll_offset_keybinds;

        d.draw_rectangle_rec(kb, COLOUR_UI_ELEMENT);

        let a = Rectangle {
            x: kb.x,
            y: kb.y,
            width: kb.width / 2.0,
            height: 100.0,
        };
        let text = "Left Click:";
        let trec = centre_text_rec(&state.fonts.regular, text, 40.0, 0.0, a);
        d.draw_text_ex(
            &state.fonts.regular,
            text,
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            40.0,
            0.0,
            Color::BLACK,
        );
        let b = Rectangle {
            x: a.x + a.width,
            y: a.y,
            width: a.width,
            height: a.height,
        };
        let text = "Play Move";
        let trec = centre_text_rec(&state.fonts.regular, text, 40.0, 0.0, b);
        d.draw_text_ex(
            &state.fonts.regular,
            text,
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            40.0,
            0.0,
            Color::BLACK,
        );

        let a = Rectangle {
            x: a.x,
            y: a.y + a.height,
            width: a.width,
            height: a.height,
        };
        let text = "Right Click:";
        let trec = centre_text_rec(&state.fonts.regular, text, 40.0, 0.0, a);
        d.draw_text_ex(
            &state.fonts.regular,
            text,
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            40.0,
            0.0,
            Color::BLACK,
        );
        let b = Rectangle {
            x: a.x + a.width,
            y: a.y,
            width: a.width,
            height: a.height,
        };
        let text = "Pan";
        let trec = centre_text_rec(&state.fonts.regular, text, 40.0, 0.0, b);
        d.draw_text_ex(
            &state.fonts.regular,
            text,
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            40.0,
            0.0,
            Color::BLACK,
        );

        let a = Rectangle {
            x: a.x,
            y: a.y + a.height,
            width: a.width,
            height: a.height,
        };
        let text = "Scroll:";
        let trec = centre_text_rec(&state.fonts.regular, text, 40.0, 0.0, a);
        d.draw_text_ex(
            &state.fonts.regular,
            text,
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            40.0,
            0.0,
            Color::BLACK,
        );
        let b = Rectangle {
            x: a.x + a.width,
            y: a.y,
            width: a.width,
            height: a.height,
        };
        let text = "Zoom";
        let trec = centre_text_rec(&state.fonts.regular, text, 40.0, 0.0, b);
        d.draw_text_ex(
            &state.fonts.regular,
            text,
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            40.0,
            0.0,
            Color::BLACK,
        );

        let a = Rectangle {
            x: a.x,
            y: a.y + a.height,
            width: a.width,
            height: a.height,
        };
        let text = "Enter:";
        let trec = centre_text_rec(&state.fonts.regular, text, 40.0, 0.0, a);
        d.draw_text_ex(
            &state.fonts.regular,
            text,
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            40.0,
            0.0,
            Color::BLACK,
        );
        let b = Rectangle {
            x: a.x + a.width,
            y: a.y,
            width: a.width,
            height: a.height,
        };
        let text = "Re-centre";
        let trec = centre_text_rec(&state.fonts.regular, text, 40.0, 0.0, b);
        d.draw_text_ex(
            &state.fonts.regular,
            text,
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            40.0,
            0.0,
            Color::BLACK,
        );

        let a = Rectangle {
            x: a.x,
            y: a.y + a.height,
            width: a.width,
            height: a.height,
        };
        let text = "Slash:";
        let trec = centre_text_rec(&state.fonts.regular, text, 40.0, 0.0, a);
        d.draw_text_ex(
            &state.fonts.regular,
            text,
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            40.0,
            0.0,
            Color::BLACK,
        );
        let b = Rectangle {
            x: a.x + a.width,
            y: a.y,
            width: a.width,
            height: a.height,
        };
        let text = "Call AI";
        let trec = centre_text_rec(&state.fonts.regular, text, 40.0, 0.0, b);
        d.draw_text_ex(
            &state.fonts.regular,
            text,
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            40.0,
            0.0,
            Color::BLACK,
        );

        let a = Rectangle {
            x: a.x,
            y: a.y + a.height,
            width: a.width,
            height: a.height,
        };
        let text = "Backspace:";
        let trec = centre_text_rec(&state.fonts.regular, text, 40.0, 0.0, a);
        d.draw_text_ex(
            &state.fonts.regular,
            text,
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            40.0,
            0.0,
            Color::BLACK,
        );
        let b = Rectangle {
            x: a.x + a.width,
            y: a.y,
            width: a.width,
            height: a.height,
        };
        let text = "Undo Move";
        let trec = centre_text_rec(&state.fonts.regular, text, 40.0, 0.0, b);
        d.draw_text_ex(
            &state.fonts.regular,
            text,
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            40.0,
            0.0,
            Color::BLACK,
        );

        let a = Rectangle {
            x: a.x,
            y: a.y + a.height,
            width: a.width,
            height: a.height,
        };
        let text = "Grave:";
        let trec = centre_text_rec(&state.fonts.regular, text, 40.0, 0.0, a);
        d.draw_text_ex(
            &state.fonts.regular,
            text,
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            40.0,
            0.0,
            Color::BLACK,
        );
        let b = Rectangle {
            x: a.x + a.width,
            y: a.y,
            width: a.width,
            height: a.height,
        };
        let text = "Toggle FPS";
        let trec = centre_text_rec(&state.fonts.regular, text, 40.0, 0.0, b);
        d.draw_text_ex(
            &state.fonts.regular,
            text,
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            40.0,
            0.0,
            Color::BLACK,
        );

        let bk = self.keybinds_elements.back;

        d.draw_rectangle_rec(bk, COLOUR_UI_ELEMENT);
        let text = "Back";
        let trec = centre_text_rec(&state.fonts.regular, text, 50.0, 0.0, bk);
        d.draw_text_ex(
            &state.fonts.regular,
            text,
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            50.0,
            0.0,
            Color::BLACK,
        );

        let pd = self.keybinds_elements.padding;
        d.draw_rectangle_rec(pd, COLOUR_UI_BG);
    }

    pub fn draw_symbols<T: RaylibDraw>(&self, rect: Rectangle, d: &mut T, g: &Game, state: &State) {
        let bk = self.symbols_elements.back;
        d.draw_rectangle_rec(bk, COLOUR_UI_ELEMENT);
        let text = "Back";
        let trec = centre_text_rec(&state.fonts.regular, text, 50.0, 0.0, bk);
        d.draw_text_ex(
            &state.fonts.regular,
            text,
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            50.0,
            0.0,
            Color::BLACK,
        );

        // Draw Player 1 Symbol Selection
        let p1 = self.symbols_elements.player_1;
        d.draw_rectangle_rec(p1, COLOUR_UI_ELEMENT);

        let trec = Rectangle {
            x: p1.x,
            y: p1.y,
            width: p1.width,
            height: 100.0,
        };
        let text = "Player 1";
        let trec = centre_text_rec(&state.fonts.regular, text, 50.0, 0.0, trec);
        d.draw_text_ex(
            &state.fonts.regular,
            text,
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            50.0,
            0.0,
            Color::BLACK,
        );

        let p1b = self.symbols_elements.player_1_backward;
        let p1f = self.symbols_elements.player_1_forward;
        let text = self.state.player_1.name();
        let r = Rectangle {
            x: p1b.x + p1b.width,
            y: p1b.y,
            width: p1f.x - p1b.x - p1f.width,
            height: p1b.height,
        };
        let trec = centre_text_rec(&state.fonts.regular, &text, 50.0, 0.0, r);
        d.draw_text_ex(
            &state.fonts.regular,
            &text,
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            50.0,
            0.0,
            Color::BLACK,
        );

        d.draw_rectangle_rec(p1b, COLOUR_UI_BUTTON);
        let arrow_padding = p1b.width * UI_CONTENT_PADDING * 3.0;
        d.draw_line_ex(
            Vector2 {
                x: p1b.x + arrow_padding,
                y: p1b.y + p1b.height / 2.0,
            },
            Vector2 {
                x: p1b.x + p1b.width / 2.0,
                y: p1b.y + arrow_padding,
            },
            p1f.width * 0.15,
            Color::BLACK,
        );
        d.draw_line_ex(
            Vector2 {
                x: p1b.x + arrow_padding - p1f.width * 0.15 * (1.0 / 3.0),
                y: p1b.y + p1b.height / 2.0 - p1f.width * 0.15 * (1.0 / 3.0),
            },
            Vector2 {
                x: p1b.x + p1b.width / 2.0,
                y: p1b.y + p1b.height - arrow_padding,
            },
            p1f.width * 0.15,
            Color::BLACK,
        );
        d.draw_line_ex(
            Vector2 {
                x: p1b.x + arrow_padding,
                y: p1b.y + p1b.height / 2.0,
            },
            Vector2 {
                x: p1b.x + p1b.width - arrow_padding,
                y: p1b.y + p1b.height / 2.0,
            },
            p1f.width * 0.15,
            Color::BLACK,
        );

        d.draw_rectangle_rec(p1f, COLOUR_UI_BUTTON);
        let arrow_padding = p1b.width * UI_CONTENT_PADDING * 3.0;
        d.draw_line_ex(
            Vector2 {
                x: p1f.x + p1f.width - arrow_padding,
                y: p1f.y + p1f.height / 2.0,
            },
            Vector2 {
                x: p1f.x + p1f.width / 2.0,
                y: p1f.y + arrow_padding,
            },
            p1f.width * 0.15,
            Color::BLACK,
        );
        d.draw_line_ex(
            Vector2 {
                x: p1f.x + p1f.width - arrow_padding + p1f.width * 0.15 * (1.0 / 3.0),
                y: p1f.y + p1f.height / 2.0 - p1f.width * 0.15 * (1.0 / 3.0),
            },
            Vector2 {
                x: p1f.x + p1f.width / 2.0,
                y: p1f.y + p1f.height - arrow_padding,
            },
            p1f.width * 0.15,
            Color::BLACK,
        );
        d.draw_line_ex(
            Vector2 {
                x: p1f.x + arrow_padding,
                y: p1f.y + p1f.height / 2.0,
            },
            Vector2 {
                x: p1f.x + p1f.width - arrow_padding,
                y: p1f.y + p1f.height / 2.0,
            },
            p1f.width * 0.15,
            Color::BLACK,
        );

        // Draw Player 2 Symbol Selection
        let p1 = self.symbols_elements.player_2;
        d.draw_rectangle_rec(p1, COLOUR_UI_ELEMENT);

        let trec = Rectangle {
            x: p1.x,
            y: p1.y,
            width: p1.width,
            height: 100.0,
        };
        let text = "Player 2";
        let trec = centre_text_rec(&state.fonts.regular, text, 50.0, 0.0, trec);
        d.draw_text_ex(
            &state.fonts.regular,
            text,
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            50.0,
            0.0,
            Color::BLACK,
        );

        let p1b = self.symbols_elements.player_2_backward;
        let p1f = self.symbols_elements.player_2_forward;
        let text = self.state.player_2.name();
        let r = Rectangle {
            x: p1b.x + p1b.width,
            y: p1b.y,
            width: p1f.x - p1b.x - p1f.width,
            height: p1b.height,
        };
        let trec = centre_text_rec(&state.fonts.regular, &text, 50.0, 0.0, r);
        d.draw_text_ex(
            &state.fonts.regular,
            &text,
            Vector2 {
                x: trec.x,
                y: trec.y,
            },
            50.0,
            0.0,
            Color::BLACK,
        );

        d.draw_rectangle_rec(p1b, COLOUR_UI_BUTTON);
        let arrow_padding = p1b.width * UI_CONTENT_PADDING * 3.0;
        d.draw_line_ex(
            Vector2 {
                x: p1b.x + arrow_padding,
                y: p1b.y + p1b.height / 2.0,
            },
            Vector2 {
                x: p1b.x + p1b.width / 2.0,
                y: p1b.y + arrow_padding,
            },
            p1f.width * 0.15,
            Color::BLACK,
        );
        d.draw_line_ex(
            Vector2 {
                x: p1b.x + arrow_padding - p1f.width * 0.15 * (1.0 / 3.0),
                y: p1b.y + p1b.height / 2.0 - p1f.width * 0.15 * (1.0 / 3.0),
            },
            Vector2 {
                x: p1b.x + p1b.width / 2.0,
                y: p1b.y + p1b.height - arrow_padding,
            },
            p1f.width * 0.15,
            Color::BLACK,
        );
        d.draw_line_ex(
            Vector2 {
                x: p1b.x + arrow_padding,
                y: p1b.y + p1b.height / 2.0,
            },
            Vector2 {
                x: p1b.x + p1b.width - arrow_padding,
                y: p1b.y + p1b.height / 2.0,
            },
            p1f.width * 0.15,
            Color::BLACK,
        );

        d.draw_rectangle_rec(p1f, COLOUR_UI_BUTTON);
        let arrow_padding = p1b.width * UI_CONTENT_PADDING * 3.0;
        d.draw_line_ex(
            Vector2 {
                x: p1f.x + p1f.width - arrow_padding,
                y: p1f.y + p1f.height / 2.0,
            },
            Vector2 {
                x: p1f.x + p1f.width / 2.0,
                y: p1f.y + arrow_padding,
            },
            p1f.width * 0.15,
            Color::BLACK,
        );
        d.draw_line_ex(
            Vector2 {
                x: p1f.x + p1f.width - arrow_padding + p1f.width * 0.15 * (1.0 / 3.0),
                y: p1f.y + p1f.height / 2.0 - p1f.width * 0.15 * (1.0 / 3.0),
            },
            Vector2 {
                x: p1f.x + p1f.width / 2.0,
                y: p1f.y + p1f.height - arrow_padding,
            },
            p1f.width * 0.15,
            Color::BLACK,
        );
        d.draw_line_ex(
            Vector2 {
                x: p1f.x + arrow_padding,
                y: p1f.y + p1f.height / 2.0,
            },
            Vector2 {
                x: p1f.x + p1f.width - arrow_padding,
                y: p1f.y + p1f.height / 2.0,
            },
            p1f.width * 0.15,
            Color::BLACK,
        );
    }
}
