use crate::domain::value_objects::dimensions::Dimensions;
use crate::domain::value_objects::position::Position;

/// The type of a brick, determining its durability and point value.
///
/// Open/Closed Principle: new brick behaviours can be added here without
/// modifying the collision or rendering logic (each layer switches on `BrickKind`
/// independently).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrickKind {
    /// Destroyed by one hit. Worth 10 points.
    Normal,
    /// Destroyed by two hits. Worth 25 points total.
    Tough,
    /// Never destroyed. Worth 0 points. Used for border/obstacle levels.
    Indestructible,
}

/// A brick on the playing field. Position is the *top-left* corner.
///
/// `hit()` returns a new brick with decremented `hits_remaining`, modelling
/// immutable state transitions (no interior mutability needed).
#[derive(Debug, Clone)]
pub struct Brick {
    pub position: Position,
    pub dimensions: Dimensions,
    pub kind: BrickKind,
    pub hits_remaining: u8,
    pub points: u32,
}

impl Brick {
    pub fn new(position: Position, dimensions: Dimensions, kind: BrickKind) -> Self {
        let (hits, points) = match kind {
            BrickKind::Normal        => (1, 10),
            BrickKind::Tough         => (2, 25),
            BrickKind::Indestructible => (u8::MAX, 0),
        };
        Self { position, dimensions, kind, hits_remaining: hits, points }
    }

    /// Applies one hit. Returns a new `Brick` with decremented `hits_remaining`.
    /// Indestructible bricks are unaffected.
    pub fn hit(&self) -> Self {
        if self.kind == BrickKind::Indestructible {
            return self.clone();
        }
        Self {
            hits_remaining: self.hits_remaining.saturating_sub(1),
            ..self.clone()
        }
    }

    /// Returns true when the brick has been hit enough times to be removed.
    pub fn is_destroyed(&self) -> bool {
        self.kind != BrickKind::Indestructible && self.hits_remaining == 0
    }

    // ── AABB helpers (position = top-left) ───────────────────────────────────
    pub fn left(&self)   -> f32 { self.position.x }
    pub fn right(&self)  -> f32 { self.position.x + self.dimensions.width }
    pub fn top(&self)    -> f32 { self.position.y }
    pub fn bottom(&self) -> f32 { self.position.y + self.dimensions.height }
}

// ─── TDD: Unit tests ─────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    fn normal_brick() -> Brick {
        Brick::new(Position::new(0.0, 0.0), Dimensions::new(60.0, 20.0), BrickKind::Normal)
    }

    fn tough_brick() -> Brick {
        Brick::new(Position::new(0.0, 0.0), Dimensions::new(60.0, 20.0), BrickKind::Tough)
    }

    fn indestructible_brick() -> Brick {
        Brick::new(Position::new(0.0, 0.0), Dimensions::new(60.0, 20.0), BrickKind::Indestructible)
    }

    // ── Normal brick lifecycle ────────────────────────────────────────────────
    #[test]
    fn test_normal_brick_has_one_hit() {
        assert_eq!(normal_brick().hits_remaining, 1);
    }

    #[test]
    fn test_normal_brick_destroyed_after_one_hit() {
        let b = normal_brick().hit();
        assert!(b.is_destroyed());
    }

    #[test]
    fn test_normal_brick_not_destroyed_before_hit() {
        assert!(!normal_brick().is_destroyed());
    }

    // ── Tough brick lifecycle ─────────────────────────────────────────────────
    #[test]
    fn test_tough_brick_has_two_hits() {
        assert_eq!(tough_brick().hits_remaining, 2);
    }

    #[test]
    fn test_tough_brick_not_destroyed_after_one_hit() {
        let b = tough_brick().hit();
        assert!(!b.is_destroyed());
        assert_eq!(b.hits_remaining, 1);
    }

    #[test]
    fn test_tough_brick_destroyed_after_two_hits() {
        let b = tough_brick().hit().hit();
        assert!(b.is_destroyed());
    }

    // ── Indestructible brick ──────────────────────────────────────────────────
    #[test]
    fn test_indestructible_never_destroyed() {
        let b = indestructible_brick().hit().hit().hit();
        assert!(!b.is_destroyed());
    }

    // ── Points ───────────────────────────────────────────────────────────────
    #[test]
    fn test_normal_brick_worth_10_points() {
        assert_eq!(normal_brick().points, 10);
    }

    #[test]
    fn test_tough_brick_worth_25_points() {
        assert_eq!(tough_brick().points, 25);
    }

    #[test]
    fn test_indestructible_worth_zero_points() {
        assert_eq!(indestructible_brick().points, 0);
    }

    // ── Immutability ──────────────────────────────────────────────────────────
    #[test]
    fn test_hit_does_not_mutate_original() {
        let b = normal_brick();
        let _ = b.hit();
        assert_eq!(b.hits_remaining, 1); // original unchanged
    }

    // ── AABB ──────────────────────────────────────────────────────────────────
    #[test]
    fn test_aabb_bounds() {
        let b = Brick::new(Position::new(10.0, 20.0), Dimensions::new(60.0, 20.0), BrickKind::Normal);
        assert_eq!(b.left(),   10.0);
        assert_eq!(b.right(),  70.0);
        assert_eq!(b.top(),    20.0);
        assert_eq!(b.bottom(), 40.0);
    }
}
