use crate::domain::entities::brick::Brick;
use crate::domain::ports::scorer::Scorer;

/// Calculates score with an optional combo multiplier.
///
/// Combo multiplier increases every 5 consecutive brick destructions, rewarding
/// skilled play without overpowering the score with low-level arithmetic.
pub struct ScoringService;

impl Scorer for ScoringService {
    fn score_for_brick(&self, brick: &Brick, combo: u32) -> u32 {
        // Multiplier: 1× up to combo 4, 2× at 5–9, 3× at 10–14, …
        let multiplier = 1 + combo / 5;
        brick.points * multiplier
    }
}

// ─── TDD: Unit tests ─────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::brick::BrickKind;
    use crate::domain::value_objects::dimensions::Dimensions;
    use crate::domain::value_objects::position::Position;

    const SVC: ScoringService = ScoringService;

    fn normal_brick() -> Brick {
        Brick::new(Position::new(0.0, 0.0), Dimensions::new(60.0, 20.0), BrickKind::Normal)
    }

    fn tough_brick() -> Brick {
        Brick::new(Position::new(0.0, 0.0), Dimensions::new(60.0, 20.0), BrickKind::Tough)
    }

    #[test]
    fn test_normal_brick_no_combo_gives_10() {
        assert_eq!(SVC.score_for_brick(&normal_brick(), 0), 10);
    }

    #[test]
    fn test_tough_brick_no_combo_gives_25() {
        assert_eq!(SVC.score_for_brick(&tough_brick(), 0), 25);
    }

    #[test]
    fn test_combo_multiplier_activates_at_5() {
        // combo=0..4 → ×1, combo=5 → ×2
        assert_eq!(SVC.score_for_brick(&normal_brick(), 4), 10);
        assert_eq!(SVC.score_for_brick(&normal_brick(), 5), 20);
    }

    #[test]
    fn test_combo_10_gives_triple_multiplier() {
        assert_eq!(SVC.score_for_brick(&normal_brick(), 10), 30);
    }

    #[test]
    fn test_combo_multiplier_increases_every_5() {
        for combo in 0u32..=20 {
            let expected_multiplier = 1 + combo / 5;
            let score = SVC.score_for_brick(&normal_brick(), combo);
            assert_eq!(score, 10 * expected_multiplier, "Failed at combo={combo}");
        }
    }

    #[test]
    fn test_indestructible_brick_always_zero() {
        use crate::domain::entities::brick::BrickKind;
        let b = Brick::new(Position::new(0.0, 0.0), Dimensions::new(60.0, 20.0), BrickKind::Indestructible);
        assert_eq!(SVC.score_for_brick(&b, 99), 0);
    }
}
