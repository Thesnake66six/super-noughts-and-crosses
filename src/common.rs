
use raylib::{
    color::Color,
    drawing::RaylibDraw,
    math::{Rectangle},
    text::{measure_text_ex, Font},
    RaylibHandle, RaylibThread,
};

use crate::{game::{game::{Game, Turn}, player::Player, symbol::{Symbol}}, styles::{BARBEQUE, COLOUR_BOARD_BG_GREYED, COLOUR_CELL_BG_GREYED, COLOUR_DRAW_FG, CROSS, DO_COLOURED_GREYS, FISH, IRELAND, NOUGHT, THORN, UI_PANEL_WIDTH}};

/// Draws a draw (`Value::Draw`) into the given rectangle 'rect' onto `d`.
pub fn draw_draw<T: RaylibDraw>(rect: Rectangle, d: &mut T) {
    // Calculating the spacing between the lines.
    let s = rect.height / 7.0;

    // Creating and drawing the first rectangle based on s.
    let mut target_rec = Rectangle {
        x: rect.x + s,
        y: rect.y + s,
        width: rect.width - s * 2.0,
        height: s,
    };
    d.draw_rectangle_rec(target_rec, COLOUR_DRAW_FG);

    // Adjusting the starting height of the second and third rectangles and then drawing them.
    target_rec.y += 2.0 * s;
    d.draw_rectangle_rec(target_rec, COLOUR_DRAW_FG);

    target_rec.y += 2.0 * s;
    d.draw_rectangle_rec(target_rec, COLOUR_DRAW_FG);
}

//----------// Miscellaneous quick procedures //----------//

/// Returns the correct colour for a greyed out cell.
pub fn get_greyed_colour_cell(turn: Turn, player_1: &Player, player_2: &Player) -> Color {
    if DO_COLOURED_GREYS {
        match turn {
            Turn::Player1 => player_1.get_greyed_colour(),
            Turn::Player2 => player_2.get_greyed_colour(),
        }
    } else {
        COLOUR_CELL_BG_GREYED
    }
}

/// Returns the correct colour for a greyed out board.
pub fn get_greyed_colour_board(turn: Turn, player_1: &Player, player_2: &Player) -> Color {
    if DO_COLOURED_GREYS {
        match turn {
            Turn::Player1 => player_1.get_greyed_colour(),
            Turn::Player2 => player_2.get_greyed_colour(),
        }
    } else {
        COLOUR_BOARD_BG_GREYED
    }
}

/// Returns a rectangle fitting the given text, given parameters.
/// Adds a small offset due to rendering bugs (best fix I could find)
pub fn centre_text_rec(
    font: &Font,
    text: &str,
    size: f32,
    spacing: f32,
    rect: Rectangle,
) -> Rectangle {
    let text_size = measure_text_ex(font, text, size, spacing);
    Rectangle {
        x: rect.x + 0.5 * (rect.width - text_size.x) - 2.0,
        y: rect.y + 0.5 * (rect.height - text_size.y),
        width: text_size.x + 2.0,
        height: text_size.y,
    }
}

/// Returns the rectangle in which the game should be drawn
pub fn get_game_rect(rl: &RaylibHandle) -> Rectangle {
    Rectangle {
        x: 0.0,
        y: 0.0,
        width: (rl.get_screen_width() - UI_PANEL_WIDTH as i32) as f32,
        height: rl.get_screen_height() as f32,
    }
}

/// Returns the rectangle in which the UI panel should be drawn
pub fn get_ui_rect(rl: &RaylibHandle) -> Rectangle {
    let r = get_game_rect(rl);
    Rectangle {
        x: r.width,
        y: 0.0,
        width: UI_PANEL_WIDTH as f32,
        height: (rl.get_screen_height()) as f32,
    }
}

/// Returns an appropriately-sized rectangle for drawing the board
pub fn get_board_rect(depth: usize) -> Rectangle {
    Rectangle {
        x: 0.0,
        y: 0.0,
        width: 60.0 * 3f32.powi(depth as i32),
        height: 60.0 * 3f32.powi(depth as i32),
    }
}

pub fn get_rgb_from_rgba(fg: Color, bg: Color) -> Color {
    let alpha = fg.a as f32 / 255.0;
    let r = (fg.r as f32 * alpha + bg.r as f32 * (1.0 - alpha)) as u8;
    let g = (fg.g as f32 * alpha + bg.g as f32 * (1.0 - alpha)) as u8;
    let b = (fg.b as f32 * alpha + bg.b as f32 * (1.0 - alpha)) as u8;

    Color {
        r,
        g,
        b,
        a: 255,
    }
}

pub fn update_window_title(rl: &mut RaylibHandle, rlthread: &mut RaylibThread, g: &Game) {
    let mut out = String::new();

    for _ in 0..g.depth - 1 {
        out += "Super "
    }
    
    out += &(g.player_2.symbol.name() + " and " + &g.player_1.symbol.name());

    rl.set_window_title(rlthread, &out)
}

pub fn get_player_from_symbol(symbol: &Symbol) -> Player {
    match symbol {
        Symbol::Cross => CROSS,
        Symbol::Nought => NOUGHT,
        Symbol::Thorn => THORN,
        Symbol::Barbeque => BARBEQUE,
        Symbol::Fish => FISH,
        Symbol::Ireland => IRELAND,
    }
}