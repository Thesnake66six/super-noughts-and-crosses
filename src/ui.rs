use std::collections::HashMap;

use raylib::{
    color::Color,
    drawing::RaylibDraw,
    math::{Rectangle, Vector2},
    text::Font,
};

use crate::{cell::Value, game::Game, styles::*};

pub enum UITab {
    Game,
    Settings,
}

pub struct UI<'a> {
    pub tab: UITab,
    pub scroll_offset_game: f32,
    pub scroll_offset_settings: f32,
    pub constant_elements: HashMap<&'a str, Rectangle>,
    pub game_elements: HashMap<&'a str, Rectangle>,
    pub settings_elements: HashMap<&'a str, Rectangle>,
    pub state: HashMap<&'a str, usize>,
}

impl UI<'_> {
    /// Returns a new UI
    pub fn new() -> Self {
        UI {
            tab: UITab::Game,
            scroll_offset_game: 0.0,
            scroll_offset_settings: 0.0,
            constant_elements: HashMap::from([
                ("Game", Rectangle::EMPTY),
                ("Settings", Rectangle::EMPTY),
                ("Content", Rectangle::EMPTY),
                ("Inner Content", Rectangle::EMPTY),
            ]),
            game_elements: HashMap::from([
                ("Turn Display", Rectangle::EMPTY),
                ("Padding", Rectangle::EMPTY),
                ("Moves", Rectangle::EMPTY),
            ]),
            settings_elements: HashMap::from([
                ("Depth", Rectangle::EMPTY),
                ("Depth Plus", Rectangle::EMPTY),
                ("Depth Minus", Rectangle::EMPTY),
                ("Players", Rectangle::EMPTY),
                ("Player 1", Rectangle::EMPTY),
                ("Player 2", Rectangle::EMPTY),
                ("New Game", Rectangle::EMPTY),
            ]),
            state: HashMap::from([("Depth", BOARD_DEFAULT_DEPTH), ("Players", 2)]),
        }
    }

    /// A function that updates the elements hashmaps' values.
    ///
    /// Called every frame before rendereing, so as to save on rl.get_screen_size() calls
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
        *self.constant_elements.get_mut("Game").unwrap() = r;

        let r = Rectangle {
            x: r.x + r.width + UI_DIVIDER_THICKNESS as f32,
            y: r.y,
            width: r.width,
            height: r.height,
        };
        *self.constant_elements.get_mut("Settings").unwrap() = r;

        let r = Rectangle {
            x: rect.x,
            y: rect.y + UI_NAVBAR_HEIGHT as f32 + UI_DIVIDER_THICKNESS as f32,
            width: rect.width,
            height: rect.height - UI_NAVBAR_HEIGHT as f32 - UI_DIVIDER_THICKNESS as f32,
        };
        *self.constant_elements.get_mut("Content").unwrap() = r;

        let content_pading = UI_CONTENT_PADDING * UI_PANEL_WIDTH as f32;

        let r = Rectangle {
            x: r.x + content_pading,
            y: r.y + content_pading,
            width: r.width - 2.0 * content_pading,
            height: r.height - 2.0 * content_pading,
        };
        *self.constant_elements.get_mut("Inner Content").unwrap() = r;

        // Calculate the game elements' positions

        // Calculate the position of the Turn Display
        let r = Rectangle {
            x: rect.x + padding,
            y: rect.y + padding + UI_NAVBAR_HEIGHT as f32 + UI_DIVIDER_THICKNESS as f32,
            width: rect.width - 2.0 * padding,
            height: 100.0,
        };
        *self.game_elements.get_mut("Turn Display").unwrap() = r;

        // Calculate the position of the Padding between the Turn Display and the Moves
        let p = Rectangle {
            x: r.x,
            y: r.y + r.height,
            width: r.width,
            height: padding,
        };
        *self.game_elements.get_mut("Padding").unwrap() = p;

        // Calculate the position of the Moves list
        let r = Rectangle {
            x: r.x,
            y: r.y + r.height + padding,
            width: r.width,
            height: rect.height - r.height - 3.0 * padding,
        };
        *self.game_elements.get_mut("Moves").unwrap() = r;

        // Calculate the settings elements' positions

        // Calculate the position of the Depth buttons
        let r = Rectangle {
            x: rect.x + padding,
            y: rect.y + padding + UI_NAVBAR_HEIGHT as f32 + UI_DIVIDER_THICKNESS as f32,
            width: rect.width - 2.0 * padding,
            height: 100.0,
        };
        *self.settings_elements.get_mut("Depth").unwrap() = r;

        // Calculate the position of the Players selection buttons
        let r = Rectangle {
            x: r.x,
            y: r.y + r.height + padding,
            width: r.width,
            height: 200.0,
        };
        *self.settings_elements.get_mut("Players").unwrap() = r;

        // Calculate the position of the New Game button
        let r = Rectangle {
            x: r.x,
            y: r.y + r.height + padding,
            width: r.width,
            height: 100.0,
        };
        *self.settings_elements.get_mut("New Game").unwrap() = r;

        // Calculate the positions of the clickable content
        let padding = UI_CONTENT_PADDING * self.constant_elements["Inner Content"].width;

        let dp = self.settings_elements["Depth"];
        let r = Rectangle {
            x: dp.x + dp.width - 2.0 * padding - 2.0 * (dp.height - 2.0 * padding),
            y: dp.y + padding,
            width: dp.height - 2.0 * padding,
            height: dp.height - 2.0 * padding,
        };
        *self.settings_elements.get_mut("Depth Minus").unwrap() = r;

        let r = Rectangle {
            x: r.x + r.width + padding,
            y: r.y,
            width: r.width,
            height: r.height,
        };
        *self.settings_elements.get_mut("Depth Plus").unwrap() = r;

        // Draw Players selection
        let pl = self.settings_elements["Players"];
        let r = Rectangle {
            x: pl.x + padding + (pl.width - 2.0 * padding) - (100.0 - 2.0 * padding),
            y: pl.y + padding,
            width: 100.0 - 2.0 * padding,
            height: 100.0 - 2.0 * padding,
        };
        *self.settings_elements.get_mut("Player 1").unwrap() = r;

        let r = Rectangle {
            x: pl.x + padding + (pl.width - 2.0 * padding) - (100.0 - 2.0 * padding),
            y: pl.y + 2.0 * padding + (pl.height - 3.0 * padding) / 2.0,
            width: 100.0 - 2.0 * padding,
            height: 100.0 - 2.0 * padding,
        };
        *self.settings_elements.get_mut("Player 2").unwrap() = r;
    }

    pub fn draw<T: RaylibDraw>(&self, rect: Rectangle, d: &mut T, g: &Game, font: &Font) {
        // Draw the background for the UI
        d.draw_rectangle_rec(rect, COLOUR_UI_BG);

        let content_rec = self.constant_elements["Content"];

        let content_rec_inner = self.constant_elements["Inner Content"];

        match self.tab {
            UITab::Game => self.draw_game(content_rec_inner, d, g, font),
            UITab::Settings => self.draw_settings(content_rec_inner, d, g, font),
        }

        // Redraw the padding of the tab content and navbar (with divider) to remove any overspill
        d.draw_rectangle_lines_ex(
            content_rec,
            (UI_CONTENT_PADDING * UI_PANEL_WIDTH as f32) as i32,
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

        let tab_rect = self.constant_elements["Game"];

        d.draw_rectangle_rec(
            tab_rect,
            match self.tab {
                UITab::Settings => COLOUR_UI_ELEMENT,
                UITab::Game => COLOUR_UI_BG,
            },
        );

        let text_rec = centre_text_rec(font, "Game", 50.0, 0.0, tab_rect);

        d.draw_text_rec(font, "Game", text_rec, 50.0, 0.0, false, Color::BLACK);

        // Draw the Settings tab button
        let tab_rect = self.constant_elements["Settings"];

        d.draw_rectangle_rec(
            tab_rect,
            match self.tab {
                UITab::Settings => COLOUR_UI_BG,
                UITab::Game => COLOUR_UI_ELEMENT,
            },
        );

        let text_rec = centre_text_rec(font, "Settings", 50.0, 0.0, tab_rect);

        d.draw_text_rec(font, "Settings", text_rec, 50.0, 0.0, false, Color::BLACK);

        // Draw the lower divider based on the selected tab
        match self.tab {
            UITab::Settings => {
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
            UITab::Game => {
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

    pub fn draw_game<T: RaylibDraw>(&self, rect: Rectangle, d: &mut T, g: &Game, font: &Font) {
        // Draw the move history
        let mv = self.game_elements["Moves"];

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
                    COLOUR_UI_HIGHLIGHT_P2
                } else {
                    COLOUR_UI_HIGHLIGHT_P1
                },
            );
            let t = x[0]
                .iter()
                .map(|x| (x + 1).to_string())
                .collect::<Vec<String>>()
                .join(", ");
            let r = centre_text_rec(font, &t, 50.0, 0.0, rect);
            d.draw_text_rec(font, &t, r, 50.0, 0.0, false, Color::BLACK)
        }

        // Redraw the blank padding
        let p = self.game_elements["Padding"];

        d.draw_rectangle_rec(p, COLOUR_UI_BG);

        // Draw the turn counter
        let tc = self.game_elements["Turn Display"];
        d.draw_rectangle_rec(tc, COLOUR_UI_ELEMENT);

        if g.board.check() != Value::None {
            let r = g.board.check();
            let text = match r {
                Value::None => "Hardware error encountered",
                Value::Draw => "Draw",
                Value::Player1 => "Crosses Win",
                Value::Player2 => "Noughts Win",
            };
            let rec = centre_text_rec(font, text, 50.0, 0.0, tc);
            d.draw_text_rec(
                font,
                text,
                rec,
                50.0,
                0.0,
                false,
                match r {
                    Value::None => Color::RED,
                    Value::Draw => Color::BLACK,
                    Value::Player1 => COLOUR_UI_HIGHLIGHT_P1,
                    Value::Player2 => COLOUR_UI_HIGHLIGHT_P2,
                },
            );
        } else if g.turn == 1 {
            let text = "Crosses' Turn";
            let rec = centre_text_rec(font, text, 50.0, 0.0, tc);

            d.draw_text_rec(font, text, rec, 50.0, 0.0, false, COLOUR_UI_HIGHLIGHT_P1);
        } else {
            let text = "Noughts' Turn";
            let rec = centre_text_rec(font, text, 50.0, 0.0, tc);
            d.draw_text_rec(font, text, rec, 50.0, 0.0, true, COLOUR_UI_HIGHLIGHT_P2);
        }
    }

    pub fn draw_settings<T: RaylibDraw>(&self, rect: Rectangle, d: &mut T, g: &Game, font: &Font) {
        let padding = UI_CONTENT_PADDING * rect.width;

        // Draw the depth selector
        let mut dp = self.settings_elements["Depth"];
        dp.y += self.scroll_offset_settings;
        let button_side = dp.height - 2.0 * padding;
        let text_rec = Rectangle {
            x: dp.x + padding,
            y: dp.y + padding,
            width: dp.width - 4.0 * padding - 2.0 * button_side,
            height: button_side,
        };
        let text = "Depth: ".to_owned() + &self.state["Depth"].to_string();

        // Draw the background
        d.draw_rectangle_rec(dp, COLOUR_UI_ELEMENT);

        // Draw the current depth text
        d.draw_text_rec(font, &text, text_rec, 50.0, 0.0, false, Color::BLACK);

        // Draw the buttons
        let mut brec = self.settings_elements["Depth Plus"];
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

        let mut brec = self.settings_elements["Depth Minus"];
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
        let pl = self.settings_elements["Players"];

        d.draw_rectangle_rec(pl, COLOUR_UI_ELEMENT);
        let button_side = 100.0 - 2.0 * padding;
        let p1 = Rectangle {
            x: pl.x + padding,
            y: pl.y + padding,
            width: pl.width - 2.0 * padding,
            height: (pl.height - 3.0 * padding) / 2.0,
        };

        let mut brec = self.settings_elements["Player 1"];
        brec.y += self.scroll_offset_settings;

        d.draw_rectangle_rec(brec, COLOUR_UI_BUTTON);
        let p = button_side * UI_CONTENT_PADDING * 2.0;
        if self.state["Players"] == 1 {
            d.draw_rectangle_rec(
                Rectangle {
                    x: brec.x + p,
                    y: brec.y + p,
                    width: brec.width - 2.0 * p,
                    height: brec.height - 2.0 * p,
                },
                Color::BLACK,
            )
        }
        let trec = Rectangle {
            x: p1.x,
            y: p1.y,
            width: p1.width - button_side - p,
            height: button_side,
        };
        let trec = centre_text_rec(font, "1 Player", 50.0, 0.0, trec);
        d.draw_text_rec(font, "1 Player", trec, 50.0, 0.0, false, Color::BLACK);

        let p2 = Rectangle {
            x: pl.x + padding,
            y: pl.y + 2.0 * padding + (pl.height - 3.0 * padding) / 2.0,
            width: pl.width - 2.0 * padding,
            height: (pl.height - 3.0 * padding) / 2.0,
        };

        let mut brec = self.settings_elements["Player 2"];
        brec.y += self.scroll_offset_settings;

        d.draw_rectangle_rec(brec, COLOUR_UI_BUTTON);
        let p = button_side * UI_CONTENT_PADDING * 2.0;
        if self.state["Players"] == 2 {
            d.draw_rectangle_rec(
                Rectangle {
                    x: brec.x + p,
                    y: brec.y + p,
                    width: brec.width - 2.0 * p,
                    height: brec.height - 2.0 * p,
                },
                Color::BLACK,
            )
        }
        let trec = Rectangle {
            x: p2.x,
            y: p2.y,
            width: p1.width - button_side - p,
            height: button_side,
        };
        let trec = centre_text_rec(font, "2 Players", 50.0, 0.0, trec);
        d.draw_text_rec(font, "2 Players", trec, 50.0, 0.0, false, Color::BLACK);

        // Draw New Game button
        let mut ng = self.settings_elements["New Game"];
        ng.y += self.scroll_offset_settings;

        d.draw_rectangle_rec(ng, COLOUR_UI_ELEMENT);

        let trec = centre_text_rec(font, "New game", 50.0, 0.0, ng);
        d.draw_text_rec(font, "New game", trec, 50.0, 0.0, false, Color::BLACK);
    }
}
