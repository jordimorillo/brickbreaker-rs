use crate::domain::value_objects::dimensions::Dimensions;
use crate::domain::value_objects::position::Position;

/// The player-controlled paddle.
///
/// Position represents the *center* of the paddle. Movement is clamped to the
/// world bounds. All state transitions return a new `Paddle`.
#[derive(Debug, Clone)]
pub struct Paddle {
    pub position: Position,
    pub dimensions: Dimensions,
    pub speed: f32,
}

impl Paddle {
    pub fn new(position: Position, dimensions: Dimensions, speed: f32) -> Self {
        Self { position, dimensions, speed }
    }

    /// Move the paddle left, clamped so it stays inside the world.
    pub fn move_left(&self, dt: f32, _world_width: f32) -> Self {
        let half_w = self.dimensions.half_width();
        let new_x = (self.position.x - self.speed * dt).max(half_w);
        Self {
            position: Position::new(new_x, self.position.y),
            ..*self
        }
    }

    /// Move the paddle right, clamped so it stays inside the world.
    pub fn move_right(&self, dt: f32, world_width: f32) -> Self { // world_width used for right clamp
        let half_w = self.dimensions.half_width();
        let new_x = (self.position.x + self.speed * dt).min(world_width - half_w);
        Self {
            position: Position::new(new_x, self.position.y),
            ..*self
        }
    }

    // ── AABB helpers ─────────────────────────────────────────────────────────
    pub fn left(&self)   -> f32 { self.position.x - self.dimensions.half_width() }
    pub fn right(&self)  -> f32 { self.position.x + self.dimensions.half_width() }
    pub fn top(&self)    -> f32 { self.position.y - self.dimensions.half_height() }
    pub fn bottom(&self) -> f32 { self.position.y + self.dimensions.half_height() }
}

// ─── TDD: Unit tests ─────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    fn make_paddle(x: f32) -> Paddle {
        Paddle::new(
            Position::new(x, 560.0),
            Dimensions::new(100.0, 14.0),
            400.0,
        )
    }

    const WORLD: f32 = 800.0;

    #[test]
    fn test_move_left_decreases_x() {
        let p = make_paddle(400.0);
        let moved = p.move_left(0.016, WORLD);
        assert!(moved.position.x < 400.0);
    }

    #[test]
    fn test_move_right_increases_x() {
        let p = make_paddle(400.0);
        let moved = p.move_right(0.016, WORLD);
        assert!(moved.position.x > 400.0);
    }

    #[test]
    fn test_move_left_clamps_at_left_wall() {
        let p = make_paddle(10.0); // very close to left wall
        let moved = p.move_left(10.0, WORLD); // large dt
        assert_eq!(moved.position.x, p.dimensions.half_width()); // clamped
    }

    #[test]
    fn test_move_right_clamps_at_right_wall() {
        let p = make_paddle(790.0); // very close to right wall
        let moved = p.move_right(10.0, WORLD); // large dt
        assert_eq!(moved.position.x, WORLD - p.dimensions.half_width()); // clamped
    }

    #[test]
    fn test_move_does_not_mutate_original() {
        let p = make_paddle(400.0);
        let _ = p.move_left(1.0, WORLD);
        assert_eq!(p.position.x, 400.0);
    }

    #[test]
    fn test_aabb_bounds() {
        let p = make_paddle(400.0);
        assert_eq!(p.left(),   350.0);
        assert_eq!(p.right(),  450.0);
        assert_eq!(p.top(),    553.0);
        assert_eq!(p.bottom(), 567.0);
    }
}
