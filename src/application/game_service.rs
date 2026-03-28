use crate::application::game_state::{GameState, GameStatus};
use crate::application::ports::input_provider::InputSnapshot;
use crate::domain::ports::collision_detector::{CollisionDetector, CollisionSide, WallCollision};
use crate::domain::ports::scorer::Scorer;
use crate::domain::value_objects::position::Position;
use crate::domain::value_objects::velocity::Velocity;

/// Orchestrates one frame of game logic.
///
/// Demonstrates Dependency Inversion Principle: `GameService` owns
/// `Box<dyn CollisionDetector>` and `Box<dyn Scorer>` — it never
/// refers to `CollisionService` or `ScoringService` directly.
/// In tests, stub implementations can be injected freely.
pub struct GameService {
    collision: Box<dyn CollisionDetector>,
    scorer:    Box<dyn Scorer>,
}

impl GameService {
    pub fn new(
        collision: Box<dyn CollisionDetector>,
        scorer:    Box<dyn Scorer>,
    ) -> Self {
        Self { collision, scorer }
    }

    /// Update game state by one frame (`dt` seconds).
    pub fn update(&self, state: &mut GameState, input: &InputSnapshot, dt: f32) {
        match state.status {
            GameStatus::WaitingToLaunch => self.handle_waiting(state, input, dt),
            GameStatus::Playing         => self.handle_playing(state, input, dt),
            GameStatus::Paused          => self.handle_paused(state, input),
            // Terminal / transitional states: caller handles restart/progression
            GameStatus::LevelComplete | GameStatus::GameOver | GameStatus::Victory => {}
        }
    }

    // ── State handlers ────────────────────────────────────────────────────────

    fn handle_waiting(&self, state: &mut GameState, input: &InputSnapshot, dt: f32) {
        self.move_paddle(state, input, dt);
        // Keep ball glued to the centre of the paddle
        state.ball = state.ball.with_position(Position::new(
            state.paddle.position.x,
            state.paddle.top() - state.ball.radius - 1.0,
        ));
        if input.launch {
            state.status = GameStatus::Playing;
            // Launch at a fixed angle: slightly to the right, upward
            state.ball = state.ball.with_velocity(Velocity::new(180.0, -320.0));
        }
        if input.quit {
            state.status = GameStatus::GameOver;
        }
    }

    fn handle_playing(&self, state: &mut GameState, input: &InputSnapshot, dt: f32) {
        if input.pause {
            state.status = GameStatus::Paused;
            return;
        }
        if input.quit {
            state.status = GameStatus::GameOver;
            return;
        }
        self.move_paddle(state, input, dt);
        state.ball = state.ball.advance(dt);
        self.resolve_collisions(state);
        // Only check win condition if no collision already changed the status
        if state.status == GameStatus::Playing {
            self.check_win_condition(state);
        }
    }

    fn handle_paused(&self, state: &mut GameState, input: &InputSnapshot) {
        if input.pause {
            state.status = GameStatus::Playing;
        }
    }

    // ── Helpers ───────────────────────────────────────────────────────────────

    fn move_paddle(&self, state: &mut GameState, input: &InputSnapshot, dt: f32) {
        if input.move_left  { state.paddle = state.paddle.move_left(dt, state.world_width); }
        if input.move_right { state.paddle = state.paddle.move_right(dt, state.world_width); }
    }

    fn resolve_collisions(&self, state: &mut GameState) {
        self.resolve_wall(state);
        if state.status != GameStatus::Playing { return; } // ball was lost
        self.resolve_paddle(state);
        self.resolve_bricks(state);
    }

    fn resolve_wall(&self, state: &mut GameState) {
        match self.collision.ball_hits_wall(&state.ball, state.world_width, state.world_height) {
            WallCollision::Left | WallCollision::Right => {
                let v = state.ball.velocity.reflect_horizontal();
                state.ball = state.ball.with_velocity(v);
                state.combo = 0;
            }
            WallCollision::Top => {
                let v = state.ball.velocity.reflect_vertical();
                state.ball = state.ball.with_velocity(v);
                state.combo = 0;
            }
            WallCollision::Bottom => {
                state.lives = state.lives.saturating_sub(1);
                state.combo = 0;
                if state.lives == 0 {
                    state.status = GameStatus::GameOver;
                } else {
                    state.status = GameStatus::WaitingToLaunch;
                    state.ball = state.ball.with_velocity(Velocity::new(0.0, 0.0));
                }
            }
            WallCollision::None => {}
        }
    }

    fn resolve_paddle(&self, state: &mut GameState) {
        if !self.collision.ball_hits_paddle(&state.ball, &state.paddle) {
            return;
        }
        // Enforce a minimum ball speed so it never gets too slow to play.
        const MIN_SPEED: f32 = 320.0;
        let current_speed = state.ball.velocity.speed();
        if current_speed < MIN_SPEED && current_speed > 0.0 {
            let factor = MIN_SPEED / current_speed;
            state.ball = state.ball.with_velocity(state.ball.velocity.scale(factor));
        }
        // Vary outgoing angle based on hit position relative to paddle centre.
        // Centre → straight up; edges → up to ±75°.
        let half_w  = state.paddle.dimensions.half_width();
        let rel_x   = (state.ball.position.x - state.paddle.position.x) / half_w;
        let rel_x   = rel_x.clamp(-1.0, 1.0);
        let speed   = state.ball.velocity.speed().max(MIN_SPEED);
        let angle   = rel_x * 75.0_f32.to_radians();
        let new_vx  = speed * angle.sin();
        let new_vy  = -(speed * angle.cos());
        state.ball  = state.ball.with_velocity(Velocity::new(new_vx, new_vy));
        state.combo = 0;
    }

    fn resolve_bricks(&self, state: &mut GameState) {
        // Find first colliding brick (single collision per frame avoids tunnelling)
        let hit = state.bricks.iter().enumerate().find_map(|(i, brick)| {
            self.collision
                .ball_hits_brick(&state.ball, brick)
                .map(|side| (i, side))
        });

        if let Some((idx, side)) = hit {
            let points = self.scorer.score_for_brick(&state.bricks[idx], state.combo);
            state.bricks[idx] = state.bricks[idx].hit();

            if state.bricks[idx].is_destroyed() {
                state.score += points;
                state.combo += 1;
            }

            let new_vel = match side {
                CollisionSide::Top | CollisionSide::Bottom => {
                    state.ball.velocity.reflect_vertical()
                }
                CollisionSide::Left | CollisionSide::Right => {
                    state.ball.velocity.reflect_horizontal()
                }
            };
            state.ball = state.ball.with_velocity(new_vel);
        }
    }

    fn check_win_condition(&self, state: &mut GameState) {
        if state.all_destroyable_bricks_gone() {
            state.status = GameStatus::LevelComplete;
        }
    }
}

// ─── TDD: Unit tests ─────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::ball::Ball;
    use crate::domain::entities::brick::{Brick, BrickKind};
    use crate::domain::entities::paddle::Paddle;
    use crate::domain::ports::collision_detector::{CollisionDetector, CollisionSide, WallCollision};
    use crate::domain::ports::scorer::Scorer;
    use crate::domain::value_objects::dimensions::Dimensions;
    use crate::domain::value_objects::position::Position;
    use crate::domain::value_objects::velocity::Velocity;

    // ── Test doubles ──────────────────────────────────────────────────────────

    /// A collision detector that never reports any collision.
    struct NoCollision;
    impl CollisionDetector for NoCollision {
        fn ball_hits_wall(&self, _: &Ball, _: f32, _: f32) -> WallCollision { WallCollision::None }
        fn ball_hits_paddle(&self, _: &Ball, _: &Paddle) -> bool { false }
        fn ball_hits_brick(&self, _: &Ball, _: &Brick) -> Option<CollisionSide> { None }
    }

    /// A collision detector that always reports bottom wall → ball lost.
    struct AlwaysBottomWall;
    impl CollisionDetector for AlwaysBottomWall {
        fn ball_hits_wall(&self, _: &Ball, _: f32, _: f32) -> WallCollision { WallCollision::Bottom }
        fn ball_hits_paddle(&self, _: &Ball, _: &Paddle) -> bool { false }
        fn ball_hits_brick(&self, _: &Ball, _: &Brick) -> Option<CollisionSide> { None }
    }

    /// A scorer that always returns a fixed value.
    struct FixedScorer(u32);
    impl Scorer for FixedScorer {
        fn score_for_brick(&self, _: &Brick, _: u32) -> u32 { self.0 }
    }

    /// A collision detector that always hits a brick from the top.
    struct AlwaysBrickHit;
    impl CollisionDetector for AlwaysBrickHit {
        fn ball_hits_wall(&self, _: &Ball, _: f32, _: f32) -> WallCollision { WallCollision::None }
        fn ball_hits_paddle(&self, _: &Ball, _: &Paddle) -> bool { false }
        fn ball_hits_brick(&self, _: &Ball, _b: &Brick) -> Option<CollisionSide> {
            if _b.is_destroyed() { None } else { Some(CollisionSide::Top) }
        }
    }

    // ── Helpers ────────────────────────────────────────────────────────────────

    fn make_state() -> GameState {
        let ball   = Ball::new(Position::new(400.0, 300.0), Velocity::new(0.0, -300.0), 8.0);
        let paddle = Paddle::new(Position::new(400.0, 560.0), Dimensions::new(100.0, 14.0), 400.0);
        GameState::new(ball, paddle, vec![], 800.0, 600.0)
    }

    fn svc_no_collision() -> GameService {
        GameService::new(Box::new(NoCollision), Box::new(FixedScorer(0)))
    }

    fn launch(state: &mut GameState) {
        state.status = GameStatus::Playing;
    }

    // ── Tests ──────────────────────────────────────────────────────────────────

    #[test]
    fn test_launch_transitions_status_to_playing() {
        let svc = svc_no_collision();
        let mut state = make_state();
        let input = InputSnapshot { launch: true, ..Default::default() };
        svc.update(&mut state, &input, 0.016);
        assert_eq!(state.status, GameStatus::Playing);
    }

    #[test]
    fn test_ball_does_not_move_before_launch() {
        let svc = svc_no_collision();
        let mut state = make_state();
        let x_before = state.ball.position.x;
        let input = InputSnapshot::default(); // no launch
        svc.update(&mut state, &input, 0.016);
        // Still WaitingToLaunch — ball glued to paddle, x unchanged
        assert_eq!(state.ball.position.x, state.paddle.position.x);
        let _ = x_before; // used to satisfy compiler
    }

    #[test]
    fn test_ball_moves_when_playing() {
        let svc = svc_no_collision();
        let mut state = make_state();
        launch(&mut state);
        let y_before = state.ball.position.y;
        let input = InputSnapshot::default();
        svc.update(&mut state, &input, 0.016);
        assert_ne!(state.ball.position.y, y_before);
    }

    #[test]
    fn test_pause_toggles_status() {
        let svc = svc_no_collision();
        let mut state = make_state();
        launch(&mut state);
        let pause_input = InputSnapshot { pause: true, ..Default::default() };
        svc.update(&mut state, &pause_input, 0.016);
        assert_eq!(state.status, GameStatus::Paused);
        svc.update(&mut state, &pause_input, 0.016);
        assert_eq!(state.status, GameStatus::Playing);
    }

    #[test]
    fn test_bottom_wall_decrements_life() {
        let svc = GameService::new(Box::new(AlwaysBottomWall), Box::new(FixedScorer(0)));
        let mut state = make_state();
        launch(&mut state);
        let input = InputSnapshot::default();
        svc.update(&mut state, &input, 0.016);
        assert_eq!(state.lives, 2);
        assert_eq!(state.status, GameStatus::WaitingToLaunch);
    }

    #[test]
    fn test_game_over_when_zero_lives_remain() {
        let svc = GameService::new(Box::new(AlwaysBottomWall), Box::new(FixedScorer(0)));
        let mut state = make_state();
        state.lives = 1;
        launch(&mut state);
        let input = InputSnapshot::default();
        svc.update(&mut state, &input, 0.016);
        assert_eq!(state.status, GameStatus::GameOver);
    }

    #[test]
    fn test_brick_destruction_awards_score() {
        let svc = GameService::new(Box::new(AlwaysBrickHit), Box::new(FixedScorer(50)));
        let mut state = make_state();
        let brick = Brick::new(
            Position::new(390.0, 295.0),
            Dimensions::new(60.0, 20.0),
            BrickKind::Normal,
        );
        state.bricks.push(brick);
        launch(&mut state);
        let input = InputSnapshot::default();
        svc.update(&mut state, &input, 0.016);
        assert_eq!(state.score, 50);
    }

    #[test]
    fn test_level_complete_when_all_bricks_gone() {
        let svc = GameService::new(Box::new(AlwaysBrickHit), Box::new(FixedScorer(10)));
        let mut state = make_state();
        let brick = Brick::new(
            Position::new(390.0, 295.0),
            Dimensions::new(60.0, 20.0),
            BrickKind::Normal,
        );
        state.bricks.push(brick);
        launch(&mut state);
        let input = InputSnapshot::default();
        svc.update(&mut state, &input, 0.016); // hits and destroys brick
        assert_eq!(state.status, GameStatus::LevelComplete);
    }

    #[test]
    fn test_quit_in_playing_state_causes_game_over() {
        let svc = svc_no_collision();
        let mut state = make_state();
        launch(&mut state);
        let input = InputSnapshot { quit: true, ..Default::default() };
        svc.update(&mut state, &input, 0.016);
        assert_eq!(state.status, GameStatus::GameOver);
    }
}
