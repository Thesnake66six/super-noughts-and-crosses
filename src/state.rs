use raylib::math::{Rectangle, Vector2};

use crate::{
    fonts::Fonts,
    noughbert::message::{Message, Thoughts},
    ui::textbox::Textbox,
};

/// Struct holding the main application state
pub struct State {
    /// Declares whether, if right-click is held, the game should be panned
    pub good_right_click: bool,
    /// Stores the position of the mouse last frame
    pub mouse_prev: Vector2,
    /// Stores whether the main thread is waiting for a move
    pub waiting_for_move: bool,
    /// Stores whether the main thread is waiting for thoughts
    pub waiting_for_thoughts: bool,
    /// Stores a queue of messages to be sent to the AI thread
    pub message_queue: Vec<Message>,
    /// Stores a list of the incoming moves from the AI thread
    pub move_queue: Vec<Vec<usize>>,
    /// Stores a list of the incoming moves from the AI thread
    pub currrent_thoughts: Option<Thoughts>,
    /// Stores how long the incoming move should be delayed by
    pub move_delay: f32,
    /// Stores how long the main thread should wait before callin for new thoughts
    pub thoughts_timer: f32,
    /// Stores whether the fps should be displayed
    pub show_fps: bool,
    /// Stores the current textbox that text is being entered into
    pub typing: Textbox,
    /// Stores whether previous attempts to export a game have been successful
    pub can_export: bool,
    /// Stores the rectangle into which the game should be drawn
    pub num_cpus: usize,
    /// Stores the rectangle into which the game should be drawn
    pub game_rect: Rectangle,
    /// Stores the rectangle into which the ui should be drawn
    pub ui_rect: Rectangle,
    /// Stores the loaded fonts
    pub fonts: Fonts,
}
