use raylib::{
    color::Color,
    drawing::RaylibDraw,
    math::{Rectangle, Vector2},
};
use std::f32::consts::SQRT_2;

pub const CAMERA_MOVE_SPEED: f32 = -1.0;
pub const CAMERA_DEFAULT_ZOOM: f32 = 0.5;
pub const CAMERA_SCROLL_SPEED: f32 = 0.1;
pub const BOARD_DEPTH: usize = 2;

pub const COLOUR_CELL_BG: Color = Color::WHITE;
/// When false: illegal cells are chosen to be highlighted
/// When true: legal cells are chosen to be highlighted
pub const INVERT_GREYS: bool = false;
/// When true: cells that should be 'greyed out' are instead coloured based on which player's turn it is
pub const DO_COLOURED_GREYS: bool = true;
pub const COLOUR_CELL_BG_GREYED: Color = Color {
    r: 200,
    g: 200,
    b: 200,
    a: 255,
};
pub const COLOUR_CELL_BG_GREYED_P1: Color = Color {
    r: 243,
    g: 148,
    b: 155,
    a: 255,
};
pub const COLOUR_CELL_BG_GREYED_P2: Color = Color {
    r: 152,
    g: 202,
    b: 249,
    a: 255,
};

// Colours and rendering settings for a Cross - `Cell::Player1`
pub const COLOUR_CROSS_FG: Color = Color {
    r: 230,
    g: 41,
    b: 55,
    a: 255,
};
pub const COLOUR_CROSS_BGA: Color = Color {
    r: 230,
    g: 41,
    b: 55,
    a: 127,
};

pub const COLOUR_CROSS_BG: Color = COLOUR_CELL_BG;
pub const CROSS_THICK: f32 = 0.15; // Thickness of the line

// Colours and rendering settings for a Nought - `Cell::Player2`
pub const COLOUR_NOUGHT_FG: Color = Color {
    r: 49,
    g: 148,
    b: 243,
    a: 255,
};
pub const COLOUR_NOUGHT_BGA: Color = Color {
    r: 49,
    g: 148,
    b: 243,
    a: 127,
};

pub const COLOUR_NOUGHT_BG: Color = COLOUR_CELL_BG;
pub const NOUGHT_THICK: f32 = 0.15; // Thickness of the line
pub const NOUGHT_PADDING: f32 = 0.05; // Padding between the circle and the box

// Colours and rendering settings for a Draw - `Value::Draw`
pub const COLOUR_DRAW_BG: Color = COLOUR_CELL_BG;
pub const COLOUR_DRAW_FG: Color = Color {
    r: 160,
    g: 160,
    b: 160,
    a: 255,
};
pub const COLOUR_DRAW_BGA: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 127,
};

// Colours and rendering settings for a `Board`
pub const COLOUR_BOARD_BG: Color = COLOUR_CELL_BG;

pub const COLOUR_BOARD_BG_GREYED: Color = COLOUR_CELL_BG_GREYED;
pub const COLOUR_BOARD_BG_GREYED_P1: Color = COLOUR_CELL_BG_GREYED_P1;
pub const COLOUR_BOARD_BG_GREYED_P2: Color = COLOUR_CELL_BG_GREYED_P2;
pub const COLOUR_BOARD_LINE: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};
pub const BOARD_CELL_MARGIN: f32 = 0.02;
pub const BOARD_LINE_THICK: f32 = 0.02;

pub const COLOUR_CELL_HOVER: Color = Color {
    r: 190,
    g: 190,
    b: 190,
    a: 220,
};

/// Draws a `Cell::Player1` or `Value::Player1`
pub fn draw_cross<T: RaylibDraw>(rect: Rectangle, d: &mut T) {
    let ln_x = rect.x + (CROSS_THICK * rect.width / SQRT_2);
    let ln_y = rect.y + (CROSS_THICK * rect.height / SQRT_2);
    let ln_fx = rect.x + rect.width - (CROSS_THICK * rect.width / SQRT_2);
    let ln_fy = rect.y + rect.height - (CROSS_THICK * rect.height / SQRT_2);

    d.draw_line_ex(
        Vector2 { x: ln_x, y: ln_y },
        Vector2 { x: ln_fx, y: ln_fy },
        rect.width * CROSS_THICK,
        COLOUR_CROSS_FG,
    );

    let ln_x = rect.x + (CROSS_THICK * rect.width / SQRT_2);
    let ln_y = rect.y + rect.height - (CROSS_THICK * rect.height / SQRT_2);
    let ln_fx = rect.x + rect.width - (CROSS_THICK * rect.width / SQRT_2);
    let ln_fy = rect.y + (CROSS_THICK * rect.height / SQRT_2);

    d.draw_line_ex(
        Vector2 { x: ln_x, y: ln_y },
        Vector2 { x: ln_fx, y: ln_fy },
        rect.width * CROSS_THICK,
        COLOUR_CROSS_FG,
    );
}

/// Draws a `Cell::Player2` or `Value::Player2` with a transparent background
pub fn draw_nought<T: RaylibDraw>(rect: Rectangle, d: &mut T) {
    let cx = rect.x + (rect.width / 2.0);
    let cy = rect.y + (rect.height / 2.0);

    let ro = (rect.width / 2.0) - NOUGHT_PADDING * rect.width;
    let ri = (rect.width / 2.0) - (NOUGHT_THICK + NOUGHT_PADDING) * rect.width;

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

/// Draws a `Value::Draw`
pub fn draw_draw<T: RaylibDraw>(rect: Rectangle, d: &mut T) {
    let s = rect.height / 7.0;

    d.draw_rectangle(
        (rect.x + s) as i32,
        (rect.y + s) as i32,
        (rect.width - s * 2.0) as i32,
        (s) as i32,
        COLOUR_DRAW_FG,
    );
    d.draw_rectangle(
        (rect.x + s) as i32,
        (rect.y + 3.0 * s) as i32,
        (rect.width - s * 2.0) as i32,
        (s) as i32,
        COLOUR_DRAW_FG,
    );
    d.draw_rectangle(
        (rect.x + s) as i32,
        (rect.y + 5.0 * s) as i32,
        (rect.width - s * 2.0) as i32,
        (s) as i32,
        COLOUR_DRAW_FG,
    );
}

/// Quick function to get the correct greyed background colour of a cell
pub fn get_greyed_colour_cell(turn: u8) -> Color {
    if DO_COLOURED_GREYS {
        if turn == 1 {
            COLOUR_CELL_BG_GREYED_P1
        } else {
            COLOUR_CELL_BG_GREYED_P2
        }
    } else {
        COLOUR_CELL_BG_GREYED
    }
}

/// Quick function to get the correct greyed background colour of a board
pub fn get_greyed_colour_board(turn: u8) -> Color {
    if DO_COLOURED_GREYS {
        if turn == 1 {
            COLOUR_BOARD_BG_GREYED_P1
        } else {
            COLOUR_BOARD_BG_GREYED_P2
        }
    } else {
        COLOUR_BOARD_BG_GREYED
    }
}
