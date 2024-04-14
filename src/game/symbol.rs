use std::f32::consts::SQRT_2;

use raylib::{color::Color, drawing::RaylibDraw, math::{Rectangle, Vector2}};
use serde::{Deserialize, Serialize};

use super::player::Player;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Symbol {
    Cross,
    Nought,
    Thorn,
    Barbeque,
    Fish,
    Ireland
}

impl Symbol {
    pub fn draw<T: RaylibDraw>(&self, player: &Player, rect: Rectangle, d: &mut T) {
        match self {
            Symbol::Cross => {
                let cross_thick = 0.15f32;
                // Calculating the starting point...
                let ln_x = rect.x + (cross_thick * rect.width / SQRT_2);
                let ln_y = rect.y + (cross_thick * rect.height / SQRT_2);
                // ...and the ending point of the first line...
                let ln_fx = rect.x + rect.width - (cross_thick * rect.width / SQRT_2);
                let ln_fy = rect.y + rect.height - (cross_thick * rect.height / SQRT_2);

                // ...and drawing the given line with the correct colour and relative thickness.
                d.draw_line_ex(
                    Vector2 { x: ln_x, y: ln_y },
                    Vector2 { x: ln_fx, y: ln_fy },
                    rect.width * cross_thick,
                    player.foreground,
                );

                // Calculating the starting point...
                let ln_x = rect.x + (cross_thick * rect.width / SQRT_2);
                let ln_y = rect.y + rect.height - (cross_thick * rect.height / SQRT_2);
                // ...and the ending point of the second line...
                let ln_fx = rect.x + rect.width - (cross_thick * rect.width / SQRT_2);
                let ln_fy = rect.y + (cross_thick * rect.height / SQRT_2);

                // ...and drawing the given line with the correct colour and relative thickness.
                d.draw_line_ex(
                    Vector2 { x: ln_x, y: ln_y },
                    Vector2 { x: ln_fx, y: ln_fy },
                    rect.width * cross_thick,
                    player.foreground,
                );
            },
            Symbol::Nought => {
                let nought_padding = 0.05;
                let nought_thick = 0.15;

                // Calculating the position of the centre of the ring...
                let cx = rect.x + (rect.width / 2.0);
                let cy = rect.y + (rect.height / 2.0);

                // ...then the inner and outer radii of the ring based on the relative thickness...
                let ro = (rect.width / 2.0) - nought_padding * rect.width;
                let ri = (rect.width / 2.0) - (nought_thick + nought_padding) * rect.width;

                // ...then drawing that ring with the correct colour.
                d.draw_ring(
                    Vector2 { x: cx, y: cy },
                    ri,
                    ro,
                    0.0,
                    360.0,
                    100,
                    player.foreground,
                );
            },
            Symbol::Thorn => {
                let thorn_thick = 0.15;
                let thorn_padding = 0.05;

                let ln = Vector2 {
                    x: rect.x + (rect.width / 3.0),
                    y: rect.y + (rect.height * thorn_padding),
                };
                let ln_f = Vector2 {
                    x: rect.x + (rect.width / 3.0),
                    y: rect.y + rect.height - (rect.height * thorn_padding),
                };

                d.draw_line_ex(ln, ln_f, rect.width * thorn_thick ,player.foreground);

                let c = Vector2 {
                    x: rect.x + (rect.width / 3.0),
                    y: rect.y + (rect.height * 0.5),
                };
                let ro = (rect.width / 2.5) - thorn_padding * rect.width;
                let ri = (rect.width / 2.5) - (thorn_thick + thorn_padding) * rect.width;

                d.draw_ring(
                    c,
                    ri,
                    ro,
                    0.0,
                    180.0,
                    50,
                    player.foreground,
                );
            },
            Symbol::Barbeque => {
                let barbeque_thick = 0.15;
                let offset_dist = ((barbeque_thick * 0.5 * rect.width).powi(2) * 0.5).sqrt();
                // Calculating the starting point...
                let ln_x = rect.x + (barbeque_thick * rect.width / SQRT_2);
                let ln_y = rect.y + (barbeque_thick * rect.height / SQRT_2);
                // ...and the ending point of the first line...
                let ln_fx = rect.x + rect.width - (barbeque_thick * rect.width / SQRT_2);
                let ln_fy = rect.y + rect.height - (barbeque_thick * rect.height / SQRT_2);

                // ...and drawing the given line with the correct colour and relative thickness.
                d.draw_line_ex(
                    Vector2 { x: ln_x, y: ln_y + (SQRT_2 - 1.0) * offset_dist},
                    Vector2 { x: ln_fx, y: ln_fy + (SQRT_2 - 1.0) * offset_dist},
                    rect.width * barbeque_thick,
                    player.foreground,
                );

                // Calculating the starting point...
                let ln2_x = rect.x + (barbeque_thick * rect.width / SQRT_2);
                let ln2_y = rect.y + rect.height - (barbeque_thick * rect.height / SQRT_2);
                // ...and the ending point of the second line...
                let ln2_fx = rect.x + rect.width - (barbeque_thick * rect.width / SQRT_2);
                let ln2_fy = rect.y + (barbeque_thick * rect.height / SQRT_2);

                // ...and drawing the given line with the correct colour and relative thickness.
                d.draw_line_ex(
                    Vector2 { x: ln2_x, y: ln2_y + (SQRT_2 - 1.0) * offset_dist },
                    Vector2 { x: ln2_fx, y: ln2_fy + (SQRT_2 - 1.0) * offset_dist },
                    rect.width * barbeque_thick,
                    player.foreground,
                );


                d.draw_line_ex(
                    Vector2 { x: ln_x - offset_dist, y: ln_y },
                    Vector2 { x: ln2_fx + offset_dist, y: ln2_fy },
                    rect.width * barbeque_thick,
                    player.foreground,
                );


            },
            Symbol::Fish => {
                let fish_thick = 0.15 * rect.width;
                let fish_padding = 0.05 * rect.width;
                let sect_width = (rect.width - 3.0 * fish_padding) / 3.0;

                let offsetting = ((fish_thick * 0.5).powi(2) * 0.5).sqrt();
                
                // d.draw_line_ex(nose, top, fish_thick, player.foreground);
                // d.draw_line_ex(nose, bottom, fish_thick, player.foreground);
                // d.draw_line_ex(top, tbottom, fish_thick, player.foreground);
                // d.draw_line_ex(bottom, ttop, fish_thick, player.foreground);

                // From nose going up
                let nose_upstroke = Vector2 {
                    x: rect.x + fish_padding + offsetting,
                    y: rect.y + rect.height / 2.0 + offsetting,
                };
                // From nose going down
                let nose_downstroke = Vector2 {
                    x: rect.x + fish_padding + offsetting,
                    y: rect.y + rect.height / 2.0 - offsetting,
                };
                // From nose going up
                let tip_nose = Vector2 {
                    x: rect.x + fish_padding + sect_width + offsetting,
                    y: rect.y + fish_padding + ((rect.height - 2.0 * fish_padding ) / 2.0) - sect_width + offsetting,
                };
                // From nose going down
                let bottom_nose = Vector2 {
                    x: rect.x + fish_padding + sect_width + offsetting,
                    y: rect.y + fish_padding + ((rect.height - 2.0 * fish_padding) / 2.0) + sect_width - offsetting,
                };
                
                
                d.draw_line_ex(nose_upstroke, tip_nose, fish_thick, player.foreground);
                d.draw_line_ex(nose_downstroke, bottom_nose, fish_thick, player.foreground);
                
                let tail_downstroke = Vector2 {
                    x: rect.x + fish_padding + sect_width - offsetting,
                    y: rect.y + fish_padding + ((rect.height - 2.0 * fish_padding ) / 2.0) - sect_width + offsetting,
                };
                // From bottom going up
                let tail_upstroke = Vector2 {
                    x: rect.x + fish_padding + sect_width - offsetting,
                    y: rect.y + fish_padding + ((rect.height - 2.0 * fish_padding) / 2.0) + sect_width - offsetting,
                };
                // From top going down
                let bottom_tail = Vector2 {
                    x: rect.x + rect.width - fish_padding - offsetting,
                    y: rect.y + fish_padding + ((rect.height - 2.0 * fish_padding ) / 2.0) - sect_width - offsetting,
                };
                // From bottom going up
                let top_tail = Vector2 {
                    x: rect.x + rect.width - fish_padding - offsetting,
                    y: rect.y + fish_padding + ((rect.height - 2.0 * fish_padding ) / 2.0) + sect_width + offsetting,
                };
                
                d.draw_line_ex(tail_downstroke, top_tail, fish_thick, player.foreground);
                d.draw_line_ex(tail_upstroke, bottom_tail, fish_thick, player.foreground);
            }
            Symbol::Ireland => {
                let padding = 0.1 * rect.width;
                let flag_width = rect.width - 2.0 * padding;
                let flag_height = flag_width / 2.0;
                let pale_width = flag_width / 3.0;
                let pale_offset = (rect.height - 2.0 * padding - flag_height) / 2.0;

                let orange = Color {
                    r: 255,
                    g: 136,
                    b: 62,
                    a: 255,
                };

                let green_pale = Rectangle {
                    x: rect.x + padding,
                    y: rect.y + padding + pale_offset,
                    width: pale_width,
                    height: flag_height
                };
                let white_pale = Rectangle {
                    x: rect.x + padding + pale_width,
                    y: rect.y + padding + pale_offset,
                    width: pale_width,
                    height: flag_height
                };
                let orange_pale = Rectangle {
                    x: rect.x + padding + pale_width * 2.0,
                    y: rect.y + padding + pale_offset,
                    width: pale_width,
                    height: flag_height
                };

                d.draw_rectangle_rec(green_pale, player.foreground);
                d.draw_rectangle_rec(white_pale, Color::WHITE);
                d.draw_rectangle_rec(orange_pale, orange);

            }
        }
    }
    pub fn name(&self) -> String {
        match self {
            Symbol::Cross => "Crosses".to_owned(),
            Symbol::Nought => "Noughts".to_owned(),
            Symbol::Thorn => "Thorns".to_owned(),
            Symbol::Barbeque => "Barbeques".to_owned(),
            Symbol::Fish => "Fish".to_owned(),
            Symbol::Ireland => "Ireland".to_owned(),
    
        }
    }
    pub fn name_apostrophe(&self) -> String {
        match self {
            Symbol::Cross => "Crosses'".to_owned(),
            Symbol::Nought => "Noughts'".to_owned(),
            Symbol::Thorn => "Thorns'".to_owned(),
            Symbol::Barbeque => "Barbeques'".to_owned(),
            Symbol::Fish => "Fish's".to_owned(),
            Symbol::Ireland => "Ireland's".to_owned(),
        }
    }
    pub fn next(self) -> Symbol {
        match self {
            Symbol::Cross => Self::Nought,
            Symbol::Nought => Self::Thorn,
            Symbol::Thorn => Self::Barbeque,
            Symbol::Barbeque => Self::Fish,
            Symbol::Fish => Self::Ireland,
            Symbol::Ireland => Self::Cross,
        }
    }
    pub fn prev(self) -> Symbol {
        match self {
            Symbol::Cross => Self::Ireland,
            Symbol::Nought => Self::Cross,
            Symbol::Thorn => Self::Nought,
            Symbol::Barbeque => Self::Thorn,
            Symbol::Fish => Self::Barbeque,
            Symbol::Ireland => Self::Fish,
        }
    }
}