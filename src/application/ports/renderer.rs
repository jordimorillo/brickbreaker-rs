use crate::application::game_state::GameState;

/// Output port: renders the current game state to any visual surface.
///
/// Dependency Inversion Principle: `GameService` depends only on this trait
/// and never on `macroquad` or any concrete renderer.
pub trait Renderer {
    fn draw(&mut self, state: &GameState);
}
