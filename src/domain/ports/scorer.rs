use crate::domain::entities::brick::Brick;

/// Calculates the points awarded when a brick is destroyed.
///
/// Separated from `CollisionDetector` (ISP) — scoring and collision are
/// independent concerns. `GameService` can receive different scorer
/// implementations (e.g., `SpeedBonusScorer`) without touching collision logic.
pub trait Scorer: Send + Sync {
    /// Returns the score for destroying `brick`, given the current `combo` count.
    fn score_for_brick(&self, brick: &Brick, combo: u32) -> u32;
}
