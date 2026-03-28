use macroquad::prelude::*;
use crate::application::ports::input_provider::{InputProvider, InputSnapshot};

/// Reads player input from the keyboard via `macroquad`.
///
/// `is_key_down` handles held keys (smooth movement); `is_key_pressed`
/// handles one-shot actions (launch, pause). This adapter translates
/// raw key states into the domain-agnostic `InputSnapshot` — the
/// application layer never imports `macroquad`.
pub struct MacroquadInput;

impl MacroquadInput {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MacroquadInput {
    fn default() -> Self {
        Self::new()
    }
}

impl InputProvider for MacroquadInput {
    fn snapshot(&self) -> InputSnapshot {
        InputSnapshot {
            move_left:  is_key_down(KeyCode::Left)  || is_key_down(KeyCode::A),
            move_right: is_key_down(KeyCode::Right) || is_key_down(KeyCode::D),
            pause:      is_key_pressed(KeyCode::P),
            launch:     is_key_pressed(KeyCode::Space),
            quit:       is_key_pressed(KeyCode::Escape),
        }
    }
}
