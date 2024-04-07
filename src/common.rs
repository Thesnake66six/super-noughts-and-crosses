
use std::f32::consts::SQRT_2;

use raylib::{
    color::Color,
    drawing::RaylibDraw,
    math::{Rectangle, Vector2},
    text::{measure_text_ex, Font},
    RaylibHandle,
};

use crate::{game::Turn, monte_carlo::message::Message, styles::*};

//----------// Miscelaneous structs //----------//

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

pub struct State {
    /// Declares whether, if right-click is held, the game should be panned
    pub good_right_click: bool,
    /// Stores the position of the mouse last frame
    pub mouse_prev: Vector2,
    /// Stores whether the main thread is waiting for a move
    pub waiting_for_move: bool,
    /// Stores a queue of messages to be sent to the AI thread
    pub message_queue: Vec<Message>,
    /// Stores a list of the incoming moves from the AI thread
    pub move_queue: Vec<Vec<usize>>,
    /// Stores how long the incoming response should be delayed by
    pub response_time: f32,
    /// Stores whether the fps should be displayed
    pub show_fps: bool,
    /// Stores the current textbox that text is being entered into
    pub typing: Textbox,
    /// Stores whether previous attempts to export a game have been successful
    pub can_export: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Textbox {
    /// The Max Sims textbox
    MaxSims,
    /// The Max Time textbox
    MaxTime,
    /// No textbox selected
    None,
}

//----------// Symbol rendering functions //----------//

/// Draws a cross (`Cell::Player1` or `Value::Player1`) into the given rectangle 'rect' onto `d`.
pub fn draw_cross<T: RaylibDraw>(rect: Rectangle, d: &mut T) {
    // Calculating the starting point...
    let ln_x = rect.x + (CROSS_THICK * rect.width / SQRT_2);
    let ln_y = rect.y + (CROSS_THICK * rect.height / SQRT_2);
    // ...and the ending point of the first line...
    let ln_fx = rect.x + rect.width - (CROSS_THICK * rect.width / SQRT_2);
    let ln_fy = rect.y + rect.height - (CROSS_THICK * rect.height / SQRT_2);

    // ...and drawing the given line with the correct colour and relative thickness.
    d.draw_line_ex(
        Vector2 { x: ln_x, y: ln_y },
        Vector2 { x: ln_fx, y: ln_fy },
        rect.width * CROSS_THICK,
        COLOUR_CROSS_FG,
    );

    // Calculating the starting point...
    let ln_x = rect.x + (CROSS_THICK * rect.width / SQRT_2);
    let ln_y = rect.y + rect.height - (CROSS_THICK * rect.height / SQRT_2);
    // ...and the ending point of the second line...
    let ln_fx = rect.x + rect.width - (CROSS_THICK * rect.width / SQRT_2);
    let ln_fy = rect.y + (CROSS_THICK * rect.height / SQRT_2);

    // ...and drawing the given line with the correct colour and relative thickness.
    d.draw_line_ex(
        Vector2 { x: ln_x, y: ln_y },
        Vector2 { x: ln_fx, y: ln_fy },
        rect.width * CROSS_THICK,
        COLOUR_CROSS_FG,
    );
}

/// Draws a nought (`Cell::Player2` or `Value::Player2`) into the given rectangle 'rect' onto `d`.
pub fn draw_nought<T: RaylibDraw>(rect: Rectangle, d: &mut T) {
    // Calculating the position of the centre of the ring...
    let cx = rect.x + (rect.width / 2.0);
    let cy = rect.y + (rect.height / 2.0);

    // ...then the inner and outer radii of the ring based on the relative thickness...
    let ro = (rect.width / 2.0) - NOUGHT_PADDING * rect.width;
    let ri = (rect.width / 2.0) - (NOUGHT_THICK + NOUGHT_PADDING) * rect.width;

    // ...then drawing that ring with the correct colour.
    d.draw_ring(
        Vector2 { x: cx, y: cy },
        ri,
        ro,
        0.0,
        360.0,
        100,
        COLOUR_NOUGHT_FG,
    )
}

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

//----------// Miscelaneous quick procedures //----------//

/// Returns the correct colour for a greyed out cell.
pub fn get_greyed_colour_cell(turn: Turn) -> Color {
    if DO_COLOURED_GREYS {
        match turn {
            Turn::Player1 => COLOUR_CELL_BG_GREYED_P1,
            Turn::Player2 => COLOUR_CELL_BG_GREYED_P2,
        }
    } else {
        COLOUR_CELL_BG_GREYED
    }
}

/// Returns the correct colour for a greyed out board.
pub fn get_greyed_colour_board(turn: Turn) -> Color {
    if DO_COLOURED_GREYS {
        match turn {
            Turn::Player1 => COLOUR_BOARD_BG_GREYED_P1,
            Turn::Player2 => COLOUR_BOARD_BG_GREYED_P2,
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
