use macroquad::prelude::*;
use crate::application::game_state::{GameState, GameStatus};
use crate::application::ports::renderer::Renderer;
use crate::domain::entities::brick::BrickKind;

// ── Colour palette ─────────────────────────────────────────────────────────────
const BG:              Color = Color::new(0.06, 0.06, 0.12, 1.0);
const PADDLE_COLOR:    Color = Color::new(0.31, 0.86, 0.63, 1.0);
const BALL_COLOR:      Color = Color::new(1.0,  0.94, 0.39, 1.0);
const BRICK_NORMAL:    Color = Color::new(0.31, 0.63, 0.94, 1.0);
const BRICK_TOUGH_2:   Color = Color::new(0.94, 0.47, 0.24, 1.0);
const BRICK_TOUGH_1:   Color = Color::new(0.94, 0.78, 0.24, 1.0);
const BRICK_SOLID:     Color = Color::new(0.39, 0.39, 0.47, 1.0);
const BRICK_BORDER:    Color = Color::new(1.0,  1.0,  1.0,  0.15);
const HUD_COLOR:       Color = WHITE;
const OVERLAY_BG:      Color = Color::new(0.0,  0.0,  0.0,  0.63);
const SUBTITLE_COLOR:  Color = Color::new(0.71, 0.71, 1.0,  1.0);

/// Concrete adapter that renders the game via `macroquad`.
///
/// Lives in the infrastructure layer — the domain and application layers
/// know nothing about `macroquad`. Dependency Inversion is achieved because
/// both this struct and the game loop depend on the `Renderer` trait defined
/// in `application::ports::renderer`, not on each other.
pub struct MacroquadRenderer;

impl MacroquadRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MacroquadRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer for MacroquadRenderer {
    fn draw(&mut self, state: &GameState) {
        clear_background(BG);

        match &state.status {
            GameStatus::WaitingToLaunch => {
                self.draw_play_field(state);
                self.draw_hint(
                    state,
                    "Press SPACE to launch",
                    state.world_height * 0.70,
                );
            }
            GameStatus::Playing => {
                self.draw_play_field(state);
            }
            GameStatus::Paused => {
                self.draw_play_field(state);
                self.draw_overlay(state, "PAUSED", "Press P to resume");
            }
            GameStatus::LevelComplete => {
                self.draw_overlay(
                    state,
                    &format!("LEVEL {} COMPLETE", state.level),
                    "Press SPACE for next level",
                );
            }
            GameStatus::GameOver => {
                self.draw_overlay(
                    state,
                    "GAME OVER",
                    &format!("Score: {}  ·  Press R to restart", state.score),
                );
            }
            GameStatus::Victory => {
                self.draw_overlay(
                    state,
                    "YOU WIN!",
                    &format!("Final score: {}", state.score),
                );
            }
        }

        self.draw_hud(state);
    }
}

impl MacroquadRenderer {
    fn draw_play_field(&self, state: &GameState) {
        self.draw_bricks(state);
        self.draw_paddle(state);
        self.draw_ball(state);
    }

    fn draw_bricks(&self, state: &GameState) {
        for brick in state.active_bricks() {
            let color = match brick.kind {
                BrickKind::Normal        => BRICK_NORMAL,
                BrickKind::Tough         => {
                    if brick.hits_remaining == 2 { BRICK_TOUGH_2 } else { BRICK_TOUGH_1 }
                }
                BrickKind::Indestructible => BRICK_SOLID,
            };
            draw_rectangle(
                brick.left(), brick.top(),
                brick.dimensions.width, brick.dimensions.height,
                color,
            );
            draw_rectangle_lines(
                brick.left(), brick.top(),
                brick.dimensions.width, brick.dimensions.height,
                1.0,
                BRICK_BORDER,
            );
        }
    }

    fn draw_paddle(&self, state: &GameState) {
        let p = &state.paddle;
        let rx = p.dimensions.height / 2.0; // corner radius ≈ half height
        draw_rectangle(p.left(), p.top(), p.dimensions.width, p.dimensions.height, PADDLE_COLOR);
        // Highlight strip at top of paddle
        draw_rectangle(
            p.left(), p.top(),
            p.dimensions.width, 3.0,
            Color::new(1.0, 1.0, 1.0, 0.35),
        );
        let _ = rx; // used for future rounded-rect support
    }

    fn draw_ball(&self, state: &GameState) {
        let b = &state.ball;
        draw_circle(b.position.x, b.position.y, b.radius, BALL_COLOR);
        // Small specular highlight
        draw_circle(
            b.position.x - b.radius * 0.3,
            b.position.y - b.radius * 0.3,
            b.radius * 0.3,
            Color::new(1.0, 1.0, 1.0, 0.5),
        );
    }

    fn draw_hud(&self, state: &GameState) {
        let w = state.world_width;
        // Left: score
        draw_text(&format!("SCORE  {}", state.score),  12.0, 24.0, 22.0, HUD_COLOR);
        // Centre: level
        let level_str = format!("LEVEL  {}", state.level);
        draw_text(&level_str, w / 2.0 - 40.0, 24.0, 22.0, HUD_COLOR);
        // Right: lives (represented as ● symbols)
        let lives_str: String = "● ".repeat(state.lives as usize);
        draw_text(&lives_str, w - 100.0, 24.0, 22.0, BALL_COLOR);
    }

    fn draw_overlay(&self, state: &GameState, title: &str, subtitle: &str) {
        draw_rectangle(0.0, 0.0, state.world_width, state.world_height, OVERLAY_BG);
        let cx = state.world_width  / 2.0;
        let cy = state.world_height / 2.0;
        // Title
        let title_w = title.len() as f32 * 22.0;
        draw_text(title, cx - title_w / 2.0, cy - 20.0, 48.0, WHITE);
        // Subtitle
        let sub_w = subtitle.len() as f32 * 10.0;
        draw_text(subtitle, cx - sub_w / 2.0, cy + 34.0, 22.0, SUBTITLE_COLOR);
    }

    fn draw_hint(&self, state: &GameState, text: &str, y: f32) {
        let text_w = text.len() as f32 * 10.0;
        draw_text(
            text,
            state.world_width / 2.0 - text_w / 2.0,
            y,
            22.0,
            SUBTITLE_COLOR,
        );
    }
}
