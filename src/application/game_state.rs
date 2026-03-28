use crate::domain::entities::ball::Ball;
use crate::domain::entities::brick::{Brick, BrickKind};
use crate::domain::entities::paddle::Paddle;

/// The set of possible game states (state machine).
///
/// Each variant maps to a specific screen/behaviour in the game loop.
/// Transitions are driven by `GameService::update` — never from within
/// the entities themselves (Single Responsibility Principle).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameStatus {
    /// Ball is on the paddle, waiting for the player to press SPACE.
    WaitingToLaunch,
    /// Active gameplay.
    Playing,
    /// Game is paused by the player.
    Paused,
    /// All destroyable bricks cleared — advance to next level.
    LevelComplete,
    /// Player ran out of lives.
    GameOver,
    /// All levels completed — final victory screen.
    Victory,
}

/// Aggregates all mutable game state for a single level.
///
/// Passed by mutable reference to `GameService::update` each frame.
/// Infrastructure adapters (renderer, input) receive a shared reference
/// to read the state they need — never write to it.
#[derive(Debug, Clone)]
pub struct GameState {
    pub ball:         Ball,
    pub paddle:       Paddle,
    pub bricks:       Vec<Brick>,
    pub score:        u32,
    pub lives:        u8,
    pub status:       GameStatus,
    pub level:        u32,
    pub combo:        u32,
    pub world_width:  f32,
    pub world_height: f32,
}

impl GameState {
    pub fn new(
        ball: Ball,
        paddle: Paddle,
        bricks: Vec<Brick>,
        world_width: f32,
        world_height: f32,
    ) -> Self {
        Self {
            ball,
            paddle,
            bricks,
            score: 0,
            lives: 3,
            status: GameStatus::WaitingToLaunch,
            level: 1,
            combo: 0,
            world_width,
            world_height,
        }
    }

    /// Returns an iterator over bricks that have not been destroyed.
    pub fn active_bricks(&self) -> impl Iterator<Item = &Brick> {
        self.bricks.iter().filter(|b| !b.is_destroyed())
    }

    /// Returns `true` when every destroyable brick has been cleared.
    pub fn all_destroyable_bricks_gone(&self) -> bool {
        self.bricks
            .iter()
            .all(|b| b.is_destroyed() || b.kind == BrickKind::Indestructible)
    }

    /// Returns `true` while the game is in an active/playing state.
    pub fn is_active(&self) -> bool {
        matches!(self.status, GameStatus::Playing | GameStatus::WaitingToLaunch)
    }
}

// ─── TDD: Unit tests ─────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::dimensions::Dimensions;
    use crate::domain::value_objects::position::Position;
    use crate::domain::value_objects::velocity::Velocity;

    fn make_state(bricks: Vec<Brick>) -> GameState {
        let ball = Ball::new(Position::new(400.0, 300.0), Velocity::new(0.0, 0.0), 8.0);
        let paddle = Paddle::new(
            Position::new(400.0, 560.0),
            Dimensions::new(100.0, 14.0),
            400.0,
        );
        GameState::new(ball, paddle, bricks, 800.0, 600.0)
    }

    fn normal_brick() -> Brick {
        Brick::new(
            Position::new(0.0, 0.0),
            Dimensions::new(60.0, 20.0),
            BrickKind::Normal,
        )
    }

    // ── Initial state ─────────────────────────────────────────────────────────
    #[test]
    fn test_initial_status_is_waiting_to_launch() {
        let state = make_state(vec![]);
        assert_eq!(state.status, GameStatus::WaitingToLaunch);
    }

    #[test]
    fn test_initial_lives_are_three() {
        let state = make_state(vec![]);
        assert_eq!(state.lives, 3);
    }

    #[test]
    fn test_initial_score_is_zero() {
        let state = make_state(vec![]);
        assert_eq!(state.score, 0);
    }

    // ── all_destroyable_bricks_gone ───────────────────────────────────────────
    #[test]
    fn test_no_bricks_returns_all_gone() {
        let state = make_state(vec![]);
        assert!(state.all_destroyable_bricks_gone());
    }

    #[test]
    fn test_intact_brick_returns_not_all_gone() {
        let state = make_state(vec![normal_brick()]);
        assert!(!state.all_destroyable_bricks_gone());
    }

    #[test]
    fn test_all_bricks_destroyed_returns_all_gone() {
        let destroyed = normal_brick().hit(); // hits_remaining = 0
        let state = make_state(vec![destroyed]);
        assert!(state.all_destroyable_bricks_gone());
    }

    #[test]
    fn test_indestructible_bricks_count_as_gone() {
        let indestructible = Brick::new(
            Position::new(0.0, 0.0),
            Dimensions::new(60.0, 20.0),
            BrickKind::Indestructible,
        );
        let state = make_state(vec![indestructible]);
        assert!(state.all_destroyable_bricks_gone());
    }

    // ── is_active ─────────────────────────────────────────────────────────────
    #[test]
    fn test_waiting_to_launch_is_active() {
        let state = make_state(vec![]);
        assert!(state.is_active()); // WaitingToLaunch is active
    }

    #[test]
    fn test_game_over_is_not_active() {
        let mut state = make_state(vec![]);
        state.status = GameStatus::GameOver;
        assert!(!state.is_active());
    }

    // ── active_bricks ─────────────────────────────────────────────────────────
    #[test]
    fn test_active_bricks_excludes_destroyed() {
        let alive = normal_brick();
        let dead  = normal_brick().hit();
        let state = make_state(vec![alive, dead]);
        assert_eq!(state.active_bricks().count(), 1);
    }
}
