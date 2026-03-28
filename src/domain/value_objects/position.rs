/// 2D position in world space.
///
/// Immutable value object — all operations return a new instance.
/// Demonstrates the Value Object pattern from DDD: equality by value, no identity.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Returns a new Position translated by (dx, dy).
    pub fn translate(&self, dx: f32, dy: f32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

// ─── TDD: Unit tests ─────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    // RED → GREEN: basic construction
    #[test]
    fn test_position_stores_coordinates() {
        let p = Position::new(3.0, 4.0);
        assert_eq!(p.x, 3.0);
        assert_eq!(p.y, 4.0);
    }

    // RED → GREEN: translate produces new position
    #[test]
    fn test_translate_returns_new_position() {
        let p = Position::new(1.0, 2.0);
        let moved = p.translate(3.0, -1.0);
        assert_eq!(moved.x, 4.0);
        assert_eq!(moved.y, 1.0);
    }

    // RED → GREEN: immutability guarantee
    #[test]
    fn test_translate_does_not_mutate_original() {
        let p = Position::new(1.0, 2.0);
        let _ = p.translate(5.0, 5.0);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
    }

    // RED → GREEN: PartialEq derived
    #[test]
    fn test_equality_by_value() {
        let a = Position::new(1.0, 2.0);
        let b = Position::new(1.0, 2.0);
        assert_eq!(a, b);
    }

    // RED → GREEN: translate with zero is identity
    #[test]
    fn test_translate_by_zero_is_identity() {
        let p = Position::new(7.0, 3.0);
        assert_eq!(p.translate(0.0, 0.0), p);
    }
}
