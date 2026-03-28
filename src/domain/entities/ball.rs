use crate::domain::value_objects::position::Position;
use crate::domain::value_objects::velocity::Velocity;

/// The game ball — an Entity with identity defined by its unique role in the world.
///
/// All mutation methods follow the immutable-update pattern, returning a new
/// `Ball`. This makes game state transitions explicit and trivially testable.
#[derive(Debug, Clone)]
pub struct Ball {
    pub position: Position,
    pub velocity: Velocity,
    pub radius: f32,
}

impl Ball {
    pub fn new(position: Position, velocity: Velocity, radius: f32) -> Self {
        debug_assert!(radius > 0.0, "Ball radius must be positive");
        Self { position, velocity, radius }
    }

    /// Advances the ball by `dt` seconds along its velocity vector.
    pub fn advance(&self, dt: f32) -> Self {
        Self {
            position: self.position.translate(
                self.velocity.vx * dt,
                self.velocity.vy * dt,
            ),
            ..*self
        }
    }

    /// Returns a copy of the ball with a new velocity.
    pub fn with_velocity(&self, velocity: Velocity) -> Self {
        Self { velocity, ..*self }
    }

    /// Returns a copy of the ball with a new position.
    pub fn with_position(&self, position: Position) -> Self {
        Self { position, ..*self }
    }

    // ── AABB helpers (used by collision detection) ───────────────────────────
    pub fn left(&self)   -> f32 { self.position.x - self.radius }
    pub fn right(&self)  -> f32 { self.position.x + self.radius }
    pub fn top(&self)    -> f32 { self.position.y - self.radius }
    pub fn bottom(&self) -> f32 { self.position.y + self.radius }
}

// ─── TDD: Unit tests ─────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    fn make_ball(x: f32, y: f32, vx: f32, vy: f32) -> Ball {
        Ball::new(
            Position::new(x, y),
            Velocity::new(vx, vy),
            8.0,
        )
    }

    #[test]
    fn test_advance_moves_ball_by_velocity_times_dt() {
        let ball = make_ball(100.0, 200.0, 50.0, -100.0);
        let advanced = ball.advance(0.5);
        assert!((advanced.position.x - 125.0).abs() < 0.001);
        assert!((advanced.position.y - 150.0).abs() < 0.001);
    }

    #[test]
    fn test_advance_does_not_mutate_original() {
        let ball = make_ball(100.0, 200.0, 50.0, -100.0);
        let _ = ball.advance(1.0);
        assert_eq!(ball.position.x, 100.0);
    }

    #[test]
    fn test_with_velocity_replaces_velocity() {
        let ball = make_ball(0.0, 0.0, 10.0, 20.0);
        let updated = ball.with_velocity(Velocity::new(-5.0, 5.0));
        assert_eq!(updated.velocity.vx, -5.0);
        assert_eq!(updated.velocity.vy, 5.0);
        assert_eq!(ball.velocity.vx, 10.0); // original unchanged
    }

    #[test]
    fn test_with_position_replaces_position() {
        let ball = make_ball(0.0, 0.0, 10.0, 20.0);
        let updated = ball.with_position(Position::new(50.0, 60.0));
        assert_eq!(updated.position.x, 50.0);
        assert_eq!(ball.position.x, 0.0); // original unchanged
    }

    #[test]
    fn test_aabb_bounds_computed_correctly() {
        let ball = Ball::new(Position::new(100.0, 200.0), Velocity::new(0.0, 0.0), 8.0);
        assert_eq!(ball.left(),   92.0);
        assert_eq!(ball.right(), 108.0);
        assert_eq!(ball.top(),   192.0);
        assert_eq!(ball.bottom(), 208.0);
    }

    #[test]
    fn test_advance_with_zero_dt_is_identity() {
        let ball = make_ball(50.0, 80.0, 200.0, -300.0);
        let same = ball.advance(0.0);
        assert_eq!(same.position.x, 50.0);
        assert_eq!(same.position.y, 80.0);
    }
}
