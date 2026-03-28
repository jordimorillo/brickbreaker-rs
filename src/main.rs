mod application;
mod domain;
mod infrastructure;

use application::game_service::GameService;
use application::game_state::{GameState, GameStatus};
use application::level::{create_level, MAX_LEVEL};
use application::ports::input_provider::InputProvider;
use application::ports::renderer::Renderer;
use domain::entities::ball::Ball;
use domain::entities::paddle::Paddle;
use domain::services::collision_service::CollisionService;
use domain::services::scoring_service::ScoringService;
use domain::value_objects::dimensions::Dimensions;
use domain::value_objects::position::Position;
use domain::value_objects::velocity::Velocity;
use infrastructure::macroquad_input::MacroquadInput;
use infrastructure::macroquad_renderer::MacroquadRenderer;
use macroquad::prelude::*;

// ── World constants ────────────────────────────────────────────────────────────
const WORLD_W:      f32 = 800.0;
const WORLD_H:      f32 = 600.0;
const BALL_RADIUS:  f32 = 8.0;
const PADDLE_W:     f32 = 100.0;
const PADDLE_H:     f32 = 14.0;
const PADDLE_SPEED: f32 = 420.0;

fn window_conf() -> Conf {
    Conf {
        window_title:    String::from("Brickbreaker RS  ·  SOLID + DDD + TDD"),
        window_width:    WORLD_W as i32,
        window_height:   WORLD_H as i32,
        window_resizable: false,
        ..Default::default()
    }
}

fn make_state(level: u32, lives: u8, score: u32) -> GameState {
    let paddle = Paddle::new(
        Position::new(WORLD_W / 2.0, WORLD_H - 40.0),
        Dimensions::new(PADDLE_W, PADDLE_H),
        PADDLE_SPEED,
    );
    let ball = Ball::new(
        Position::new(WORLD_W / 2.0, WORLD_H - 60.0),
        Velocity::new(0.0, 0.0),
        BALL_RADIUS,
    );
    let level_data = create_level(level, WORLD_W);
    let mut state = GameState::new(ball, paddle, level_data.bricks, WORLD_W, WORLD_H);
    state.level = level;
    state.lives = lives;
    state.score = score;
    state
}

#[macroquad::main(window_conf)]
async fn main() {
    // ── Dependency injection ──────────────────────────────────────────────────
    // GameService depends on *traits*, not on concrete types.
    // Swap CollisionService / ScoringService for any other impl here.
    let service  = GameService::new(
        Box::new(CollisionService),
        Box::new(ScoringService),
    );
    let mut renderer = MacroquadRenderer::new();
    let     input    = MacroquadInput::new();

    let mut state    = make_state(1, 3, 0);

    loop {
        // Cap dt to avoid "tunnelling" when the window is dragged/minimised
        let dt = get_frame_time().min(0.05);
        let snap = input.snapshot();

        match state.status {
            // ── Restart from Game Over / Victory ─────────────────────────────
            GameStatus::GameOver | GameStatus::Victory
                if is_key_pressed(KeyCode::R) =>
            {
                state = make_state(1, 3, 0);
            }
            // ── Next level or final Victory ───────────────────────────────────
            GameStatus::LevelComplete if is_key_pressed(KeyCode::Space) => {
                if state.level >= MAX_LEVEL {
                    state.status = GameStatus::Victory;
                } else {
                    let next_level = state.level + 1;
                    state = make_state(next_level, state.lives, state.score);
                }
            }
            // ── Active update (Playing, Paused, WaitingToLaunch) ─────────────
            _ if state.is_active() || state.status == GameStatus::Paused => {
                service.update(&mut state, &snap, dt);
            }
            _ => {}
        }

        renderer.draw(&state);
        next_frame().await;
    }
}
