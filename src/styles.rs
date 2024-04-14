use std::f32::consts::SQRT_2;

use raylib::color::Color;

use crate::game::{player::Player, symbol::Symbol};

//----------// Constants determining debug settings //----------//

/// Sets the default toggle position of the FPS counter
pub const DEFAULT_SHOW_FPS_COUNTER: bool = false;

/// Enables the FPS counter keybind
pub const ALLOW_FPS_COUNTER: bool = true;

/// Enables the logging of tree graphs
pub const OUTPUT_GRAPHVIS_FILES: bool = false;

/// Enables the automatic compilation of tree graphs to .svg files
pub const AUTOCOMPILE_GRAPHVIS_FILES: bool = true;

//----------// Constants determining UI settings //----------//

/// Specifies the width of the UI panel
pub const UI_PANEL_WIDTH: usize = 500;

/// Specifies the thickness of the line separating the UI navbar and the content
pub const UI_DIVIDER_THICKNESS: usize = 10;

/// Specifies the thickness of the lines in button symbols
pub const UI_BUTTON_LINE_THICKNESS: usize = 8;

/// Specifies the minimum height of the UI panel
pub const UI_PANEL_MIN_HEIGHT: usize = 600;

/// Specifies the height of the UI navbar (pixels)
pub const UI_NAVBAR_HEIGHT: usize = 100;

/// Specifies the UI content padding (percentage)
pub const UI_CONTENT_PADDING: f32 = 0.05;

/// Specifies how far the UI content scrolls per scroll tick
pub const UI_SCROLL_SPEED: f32 = 35.0;

//----------// Constants determining default game settings //----------//

/// Specifies the default depth of a board
pub const BOARD_DEFAULT_DEPTH: usize = 2;

/// Specifies the default number of players
pub const BOARD_DEFAULT_PLAYERS: usize = 2;

//----------// Constants determining default AI settings //----------//

/// The default exploration factor for the `UCB1` function;
pub const DEFAULT_EXPLORATION_FACTOR: f32 = SQRT_2 / 2.0;

/// The time between the computer making a move
pub const DEFAULT_MAX_TIME: usize = 10;

/// The default value for `Max Time`
pub const COMPUTER_RESPONSE_DELAY: f32 = 0.1;

/// The default sims for a level 1 AI
pub const COMPUTER_LEVEL_1_SIMS: usize = 15;

/// The default sims for a level 2 AI
pub const COMPUTER_LEVEL_2_SIMS: usize = 30;

/// The default sims for a level 3 AI
pub const COMPUTER_LEVEL_3_SIMS: usize = 100;

/// The default scale factor for each depth
pub const COMPUTER_SIM_SCALING: usize = 6;

/// The default scale factor for each depth
pub const COMPUTER_DEFAULT_STRENGTH: usize = 2;

//----------// Constants determining other items //----------//

pub const RULES_URL: &str = "https://thesnake66six.gitlab.io/snvc-rules";

//----------// Constants determining the properties of the camera //----------//

/// Governs how fast the camera moves when panning.
pub const CAMERA_MOVE_SPEED: f32 = -1.0;

/// The initial zoom of the camera.
///
///
/// It represents the proportion of the screen that the board takes up.
pub const CAMERA_DEFAULT_ZOOM: f32 = 0.8;

/// Governs how far the camera zooms per scroll tick
pub const CAMERA_SCROLL_SPEED: f32 = 0.1;

//----------// Constants of settings for rendering the board and cells //----------//

/// Alters which cells are highlighted based on their legality.
///
///  
/// When false: illegal cells are chosen to be highlighted.
///
/// When true: legal cells are chosen to be highlighted.
pub const INVERT_GREYS: bool = false;

/// Changes the colours with which 'greyed out' cells are rendered.
///
///
/// When true: cells that should be 'greyed out' are instead coloured based on which player's turn it is.
pub const DO_COLOURED_GREYS: bool = true;

/// The fractional width of the margin between the edge of the board and the cell.
pub const BOARD_CELL_MARGIN: f32 = 0.02;

/// The fractional thickness of the board lines.
pub const BOARD_LINE_THICK: f32 = 0.02;

//----------// Constants of colours used in rendering the UI //----------//

/// Background colour for the UI menu
pub const COLOUR_UI_BG: Color = Color::WHITE;

/// Element default colour for the UI menu
pub const COLOUR_UI_ELEMENT: Color = Color {
    r: 200,
    g: 200,
    b: 200,
    a: 255,
};

/// Background colour of a UI button
pub const COLOUR_UI_BUTTON: Color = Color {
    r: 150,
    g: 150,
    b: 150,
    a: 255,
};

/// Foreground colour of a UI radial
pub const COLOUR_UI_RADIAL: Color = Color::BLACK;

/// Colour of the divider between the navbar and the tab content
pub const COLOUR_UI_DIVIDER: Color = Color::BLACK;

//----------// Constants of colours used in rendering the board //----------//

/// Line colour of the board.
pub const COLOUR_BOARD_FG: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};

/// Background colour of the board.
pub const COLOUR_BOARD_BG: Color = COLOUR_CELL_BG;

/// Background colour of a greyed board.
pub const COLOUR_BOARD_BG_GREYED: Color = COLOUR_CELL_BG_GREYED;

//----------// Constants of colours used in rendering cells //----------//

//-----// Default cell //-----//

/// Background colour of a cell.
pub const COLOUR_CELL_BG: Color = Color::WHITE;

/// Background colour of a greyed cell.
pub const COLOUR_CELL_BG_GREYED: Color = Color {
    r: 200,
    g: 200,
    b: 200,
    a: 255,
};

//-----// Crosses //-----//

pub const CROSS: Player = Player {
    foreground: Color {
        r: 230,
        g: 41,
        b: 55,
        a: 255,
    },
    background: COLOUR_CELL_BG,
    background_alpha: Color {
        r: 230,
        g: 41,
        b: 55,
        a: 127,
    },
    symbol: Symbol::Cross,
};

//-----// Noughts //-----//

pub const NOUGHT: Player = Player {
    foreground: Color {
        r: 49,
        g: 148,
        b: 243,
        a: 255,
    },
    background: COLOUR_CELL_BG,
    background_alpha: Color {
        r: 49,
        g: 148,
        b: 243,
        a: 127,
    },
    symbol: Symbol::Nought,
};

pub const THORN: Player = Player {
    foreground: Color {
        r: 88,
        g: 201,
        b: 154,
        a: 255,
    },
    background: COLOUR_CELL_BG,
    background_alpha: Color {
        r: 88,
        g: 201,
        b: 154,
        a: 127,
    },
    symbol: Symbol::Thorn,
};

pub const BARBEQUE: Player = Player {
    foreground: Color {
        r: 75,
        g: 0,
        b: 130,
        a: 255,
    },
    background: COLOUR_CELL_BG,
    background_alpha: Color {
        r: 75,
        g: 0,
        b: 130,
        a: 127,
    },
    symbol: Symbol::Barbeque,
};

pub const FISH: Player = Player {
    foreground: Color {
        r: 124,
        g: 210,
        b: 213,
        a: 255,
    },
    background: COLOUR_CELL_BG,
    background_alpha: Color {
        r: 124,
        g: 210,
        b: 213,
        a: 127,
    },
    symbol: Symbol::Fish,
};

//-----// Drawn cells //-----//

/// Foreground colour of the draw symbol.
pub const COLOUR_DRAW_FG: Color = Color {
    r: 160,
    g: 160,
    b: 160,
    a: 255,
};

/// Specific background colour of drawn cells.
pub const COLOUR_DRAW_BG: Color = COLOUR_CELL_BG;

/// Specific transparent backgruond colour of drawn cells.
pub const COLOUR_DRAW_BGA: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 127,
};

//-----// Miscellaneous //-----//

/// The overlay imposed upon a hovered cell
pub const COLOUR_CELL_HOVER: Color = Color {
    r: 190,
    g: 190,
    b: 190,
    a: 220,
};
