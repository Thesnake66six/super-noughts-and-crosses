use raylib::math::{Rectangle, Vector2};

use crate::{monte_carlo::message::Message, ui::textbox::Textbox, };

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
    /// Stores the rectangle into which the game should be drawn
    pub game_rect: Rectangle,
    /// Stores the rectangle into which the ui should be drawn
    pub ui_rect: Rectangle,
}